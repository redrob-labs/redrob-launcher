//! The engine's chat channels and bridged apps, from the desktop launcher.
//!
//! Reads and writes the engine's `channels.json` and `channels/secrets.json` (the shapes live in
//! `redrob_launcher_core::channels`), asks the running daemon for `/channels/status` and
//! `/v1/apps`, turns a channel on or off through the engine's own `redrob channels enable|disable`
//! (so the engine installs or removes its start-at-login registration), and reads the WhatsApp
//! pairing folder. Everything runs on a thread and wakes the UI when done, like `engine.rs`: the
//! UI sees snapshots and never waits. Secrets go from the file to the UI's fields and back; they
//! are never logged and never put in an error.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use redrob_engine_client as engine;
use redrob_launcher_core::channels::{
    self as chan, Apps, AppsList, ChannelConfig, ChannelDraft, ChannelName, ChannelsBackend, ChannelsReload, ChannelsStatus, Live, Loaded,
    SaveOutcome, SaveState, WhatsAppPairing,
};
use serde::de::DeserializeOwned;

use crate::account::Wake;

/// How long `/channels/status` is trusted before the next look.
const LIVE_FRESH_FOR: Duration = Duration::from_secs(3);
/// How long the WhatsApp QR / pairing read is trusted.
const WHATSAPP_FRESH_FOR: Duration = Duration::from_secs(2);
/// How long `/v1/apps` is trusted.
const APPS_FRESH_FOR: Duration = Duration::from_secs(5);
/// `redrob channels enable|disable` installs a login item; give it time.
const CLI_TIMEOUT: Duration = Duration::from_secs(60);

/// What the backend does to the files, the daemon and the engine binary, so tests can stand in.
pub trait Ops {
    /// The engine's channels home: `REDROB_CHANNELS_HOME`, else `~/.redrob`.
    fn home(&self) -> PathBuf;
    /// The user's home, for the start-at-login registration paths.
    fn user_home(&self) -> Option<PathBuf>;
    /// The daemon record as written, whether or not the daemon answers.
    fn record(&self) -> Option<engine::Record>;
    fn status(&self, record: &engine::Record) -> Result<ChannelsStatus, String>;
    fn reload(&self, record: &engine::Record) -> Result<ChannelsReload, String>;
    fn apps(&self, record: &engine::Record) -> Result<AppsList, String>;
    /// `redrob channels <args>`.
    fn cli(&self, args: &[&str]) -> Result<(), String>;
}

struct RealOps;

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(3))).http_status_as_error(false).build().into()
}

fn url(record: &engine::Record, path: &str) -> String {
    format!("{}{}", record.url.trim_end_matches('/'), path)
}

fn read_json<T: DeserializeOwned>(path: &str, mut response: ureq::http::Response<ureq::Body>) -> Result<T, String> {
    let status = response.status().as_u16();
    if status != 200 {
        return Err(format!("{path} answered {status}"));
    }
    let text = response.body_mut().read_to_string().map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

fn get_json<T: DeserializeOwned>(record: &engine::Record, path: &str) -> Result<T, String> {
    let response =
        agent().get(&url(record, path)).header("Authorization", &engine::authorization(&record.secret)).call().map_err(|e| e.to_string())?;
    read_json(path, response)
}

fn post_json<T: DeserializeOwned>(record: &engine::Record, path: &str) -> Result<T, String> {
    let response =
        agent().post(&url(record, path)).header("Authorization", &engine::authorization(&record.secret)).send_empty().map_err(|e| e.to_string())?;
    read_json(path, response)
}

impl Ops for RealOps {
    fn home(&self) -> PathBuf {
        if let Some(h) = std::env::var_os("REDROB_CHANNELS_HOME").filter(|v| !v.is_empty()) {
            return PathBuf::from(h);
        }
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".redrob")
    }
    fn user_home(&self) -> Option<PathBuf> {
        dirs::home_dir()
    }
    fn record(&self) -> Option<engine::Record> {
        engine::read_record()
    }
    fn status(&self, record: &engine::Record) -> Result<ChannelsStatus, String> {
        get_json(record, "/channels/status")
    }
    fn reload(&self, record: &engine::Record) -> Result<ChannelsReload, String> {
        post_json(record, "/channels/reload")
    }
    fn apps(&self, record: &engine::Record) -> Result<AppsList, String> {
        get_json(record, "/v1/apps")
    }
    fn cli(&self, args: &[&str]) -> Result<(), String> {
        let bin = engine::redrob_bin();
        let mut full = vec!["channels"];
        full.extend_from_slice(args);
        crate::process::run(&bin.to_string_lossy(), &full, CLI_TIMEOUT).map(|_| ())
    }
}

/// A value read on a thread and kept until it is older than `fresh_for`; `get` hands back the
/// last one and starts the next read when it is due, one at a time.
struct Polled<T> {
    value: Arc<Mutex<(T, Option<Instant>)>>,
    in_flight: Arc<AtomicBool>,
    fresh_for: Duration,
}

impl<T: Clone + Default + Send + 'static> Polled<T> {
    fn new(fresh_for: Duration) -> Self {
        Self { value: Arc::new(Mutex::new((T::default(), None))), in_flight: Arc::new(AtomicBool::new(false)), fresh_for }
    }

    fn get(&self, wake: &Wake, fetch: impl FnOnce() -> T + Send + 'static) -> T {
        let (value, taken) = self.value.lock().unwrap_or_else(PoisonError::into_inner).clone();
        let due = taken.is_none_or(|t| t.elapsed() >= self.fresh_for);
        if due && !self.in_flight.swap(true, Ordering::SeqCst) {
            let (slot, in_flight, wake) = (self.value.clone(), self.in_flight.clone(), wake.clone());
            std::thread::spawn(move || {
                let v = fetch();
                *slot.lock().unwrap_or_else(PoisonError::into_inner) = (v, Some(Instant::now()));
                in_flight.store(false, Ordering::SeqCst);
                wake();
            });
        }
        value
    }

    /// The next `get` reads again.
    fn invalidate(&self) {
        self.value.lock().unwrap_or_else(PoisonError::into_inner).1 = None;
    }
}

pub struct Channels {
    ops: Arc<dyn Ops + Send + Sync>,
    wake: Wake,
    loaded: Arc<Mutex<Option<Loaded>>>,
    loading: Arc<AtomicBool>,
    live: Polled<Live>,
    whatsapp: Polled<WhatsAppPairing>,
    apps: Polled<Apps>,
    save: Arc<Mutex<SaveState>>,
}

impl Channels {
    pub fn desktop(wake: Wake) -> Self {
        Self::new(Arc::new(RealOps), wake)
    }

    pub fn new(ops: Arc<dyn Ops + Send + Sync>, wake: Wake) -> Self {
        Self {
            ops,
            wake,
            loaded: Arc::new(Mutex::new(None)),
            loading: Arc::new(AtomicBool::new(false)),
            live: Polled::new(LIVE_FRESH_FOR),
            whatsapp: Polled::new(WHATSAPP_FRESH_FOR),
            apps: Polled::new(APPS_FRESH_FOR),
            save: Arc::new(Mutex::new(SaveState::Idle)),
        }
    }

    fn read_files(ops: &dyn Ops) -> Loaded {
        let home = ops.home();
        let config = chan::read_config(&home).unwrap_or_else(|e| {
            log::warn!("channels: {e}");
            chan::ChannelsFile::default()
        });
        let secrets = chan::read_secrets(&home).unwrap_or_else(|e| {
            log::warn!("channels: {e}");
            chan::SecretsFile::default()
        });
        let channels = ChannelName::ALL
            .into_iter()
            .map(|name| {
                let c = config.get(name);
                let secrets = name.secret_keys().iter().map(|k| ((*k).to_string(), secrets.get(name, k).unwrap_or_default().to_string())).collect();
                (name, ChannelDraft { enabled: c.enabled, allowlist: c.allowlist, secrets })
            })
            .collect();
        let autostart = ops.user_home().is_some_and(|h| chan::autostart_installed(&h));
        Loaded { channels, autostart }
    }

    fn look_live(ops: &dyn Ops) -> Live {
        let Some(record) = ops.record() else { return Live::NoEngine };
        match ops.status(&record) {
            Ok(status) => Live::Up(status),
            // A record whose daemon is gone reads the same as no record.
            Err(e) if is_connect_error(&e) => Live::NoEngine,
            Err(e) => Live::Unreachable(e),
        }
    }

    fn look_apps(ops: &dyn Ops) -> Apps {
        let Some(record) = ops.record() else { return Apps::NoEngine };
        match ops.apps(&record) {
            Ok(list) => Apps::Up { apps: list.apps, now_ms: now_ms() },
            Err(e) if is_connect_error(&e) => Apps::NoEngine,
            Err(e) => Apps::Unreachable(e),
        }
    }

    fn look_whatsapp(home: &Path) -> WhatsAppPairing {
        let dir = chan::whatsapp_dir(home);
        if creds_paired(&dir.join("creds.json")) {
            return WhatsAppPairing::Paired;
        }
        match std::fs::read(chan::whatsapp_qr_path(home)) {
            Ok(png) if !png.is_empty() => WhatsAppPairing::Pairing(Arc::from(png)),
            _ => WhatsAppPairing::Unpaired,
        }
    }

    /// Writes the files, toggles through the CLI, reloads the daemon.
    fn do_save(ops: &dyn Ops, drafts: Vec<(ChannelName, ChannelDraft)>, toggled: Vec<ChannelName>) -> Result<SaveOutcome, String> {
        let home = ops.home();
        // Re-read right before writing so a key the engine added in the meantime survives.
        let mut config = chan::read_config(&home)?;
        let mut secrets = chan::read_secrets(&home)?;
        for (name, draft) in &drafts {
            let allowlist = draft.allowlist.iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            config.set(*name, &ChannelConfig { enabled: draft.enabled, allowlist });
            for (key, value) in &draft.secrets {
                secrets.set(*name, key, value.trim());
            }
        }
        chan::write_config(&home, &config)?;
        chan::write_secrets(&home, &secrets)?;
        for name in toggled {
            let enabled = drafts.iter().find(|(n, _)| *n == name).is_some_and(|(_, d)| d.enabled);
            ops.cli(&[if enabled { "enable" } else { "disable" }, name.id()])?;
        }
        let Some(record) = ops.record() else { return Ok(SaveOutcome::TakesEffectOnStart) };
        match ops.reload(&record) {
            Ok(_) => Ok(SaveOutcome::Reloaded),
            Err(e) if is_connect_error(&e) => Ok(SaveOutcome::TakesEffectOnStart),
            Err(e) => Err(format!("saved, but the engine didn't reload: {e}")),
        }
    }
}

/// Baileys' `creds.json` carries the account (`me`) and `registered: true` once the device is
/// linked; before that the file holds only fresh keys.
fn creds_paired(path: &Path) -> bool {
    let Ok(text) = std::fs::read_to_string(path) else { return false };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else { return false };
    value.get("registered").and_then(serde_json::Value::as_bool).unwrap_or(false) || value.get("me").is_some_and(serde_json::Value::is_object)
}

/// ureq's wording when nothing listens on the record's port.
fn is_connect_error(e: &str) -> bool {
    let e = e.to_ascii_lowercase();
    e.contains("connection refused") || (e.contains("connect") && e.contains("failed")) || e.contains("timed out") || e.contains("timeout")
}

fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
}

impl ChannelsBackend for Channels {
    fn load(&mut self) {
        if self.loading.swap(true, Ordering::SeqCst) {
            return;
        }
        let (ops, loaded, loading, wake) = (self.ops.clone(), self.loaded.clone(), self.loading.clone(), self.wake.clone());
        std::thread::spawn(move || {
            let read = Self::read_files(ops.as_ref());
            *loaded.lock().unwrap_or_else(PoisonError::into_inner) = Some(read);
            loading.store(false, Ordering::SeqCst);
            wake();
        });
    }

    fn loaded(&mut self) -> Option<Loaded> {
        self.loaded.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }

    fn live(&mut self) -> Live {
        let ops = self.ops.clone();
        self.live.get(&self.wake, move || Self::look_live(ops.as_ref()))
    }

    fn whatsapp(&mut self) -> WhatsAppPairing {
        let home = self.ops.home();
        self.whatsapp.get(&self.wake, move || Self::look_whatsapp(&home))
    }

    fn unlink_whatsapp(&mut self) {
        let (ops, whatsapp, live, wake) = (self.ops.clone(), self.whatsapp.value.clone(), self.live.value.clone(), self.wake.clone());
        std::thread::spawn(move || {
            let dir = chan::whatsapp_dir(&ops.home());
            if let Err(e) = std::fs::remove_dir_all(&dir)
                && e.kind() != std::io::ErrorKind::NotFound
            {
                log::warn!("channels: couldn't remove the WhatsApp session: {e}");
            }
            // The running transport keeps its socket; a reload restarts the channel so it comes
            // back with a fresh QR code.
            if let Some(record) = ops.record()
                && let Err(e) = ops.reload(&record)
            {
                log::warn!("channels: reload after unlink: {e}");
            }
            whatsapp.lock().unwrap_or_else(PoisonError::into_inner).1 = None;
            live.lock().unwrap_or_else(PoisonError::into_inner).1 = None;
            wake();
        });
    }

    fn save(&mut self, drafts: Vec<(ChannelName, ChannelDraft)>, toggled: Vec<ChannelName>) {
        {
            let mut state = self.save.lock().unwrap_or_else(PoisonError::into_inner);
            if *state == SaveState::Saving {
                return;
            }
            *state = SaveState::Saving;
        }
        let (ops, save, loaded, live, wake) = (self.ops.clone(), self.save.clone(), self.loaded.clone(), self.live.value.clone(), self.wake.clone());
        std::thread::spawn(move || {
            let result = Self::do_save(ops.as_ref(), drafts, toggled);
            if let Err(e) = &result {
                log::warn!("channels: save: {e}");
            }
            // What is on disk now, so the page edits the saved state.
            *loaded.lock().unwrap_or_else(PoisonError::into_inner) = Some(Self::read_files(ops.as_ref()));
            live.lock().unwrap_or_else(PoisonError::into_inner).1 = None;
            *save.lock().unwrap_or_else(PoisonError::into_inner) = match result {
                Ok(outcome) => SaveState::Saved(outcome),
                Err(e) => SaveState::Failed(e),
            };
            wake();
        });
    }

    fn save_state(&mut self) -> SaveState {
        self.save.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }

    fn apps(&mut self) -> Apps {
        let ops = self.ops.clone();
        self.apps.get(&self.wake, move || Self::look_apps(ops.as_ref()))
    }

    fn refresh(&mut self) {
        self.live.invalidate();
        self.apps.invalidate();
        self.whatsapp.invalidate();
    }
}
