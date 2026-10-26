//! Using this PC from the web and the phone, from the desktop launcher.
//!
//! The shared Redrob Code daemon holds the desk relay connection (the engine's desk relay v2
//! contract, section 3). The launcher keeps that daemon running with desk on, asks it for
//! `GET /desk/status` every [`POLL_SECS`] seconds while it runs, and sends the person's answers:
//! sign in, allow or deny a device, remove one. Everything runs on a thread and wakes the UI when
//! done, like `engine.rs`: the UI sees [`DeskState`] snapshots and never waits. The daemon record's
//! secret stays in this module.

use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use redrob_engine_client as engine;
use redrob_launcher_core::desk::{self, AutoAllEvent, DeskBackend, DeskLogin, DeskState, MAX_EVENTS_BYTES, MAX_STATUS_BYTES, POLL_SECS};

use crate::account::Wake;

/// A daemon the launcher starts for desk stays up with nobody attached: phones and browsers reach
/// it at any time. `0` is the engine's "never exit on idle".
const IDLE_EXIT_MINUTES: u32 = 0;
/// How long one request to the daemon may take.
const TIMEOUT: Duration = Duration::from_secs(5);

/// What the backend does to the engine, so tests can stand in.
pub trait Ops {
    fn installed(&self) -> bool;
    /// The daemon's record if it answers.
    fn record(&self) -> Option<engine::Record>;
    /// Finds the daemon or starts it: `redrob service start --json --idle-exit 0`.
    fn start(&self) -> Result<engine::Record, String>;
    /// `GET <path>`: the status code and the body.
    fn get(&self, record: &engine::Record, path: &str) -> Result<(u16, String), String>;
    /// `POST <path>` with a JSON body (or none): the status code and the body.
    fn post(&self, record: &engine::Record, path: &str, body: Option<&str>) -> Result<(u16, String), String>;
    /// Opens a sign-in page in the default browser.
    fn open(&self, url: &str) -> Result<(), String>;
}

struct RealOps;

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder().timeout_global(Some(TIMEOUT)).http_status_as_error(false).build().into()
}

fn url(record: &engine::Record, path: &str) -> String {
    format!("{}{}", record.url.trim_end_matches('/'), path)
}

fn answer(mut response: ureq::http::Response<ureq::Body>) -> Result<(u16, String), String> {
    let status = response.status().as_u16();
    let limit = u64::try_from(MAX_STATUS_BYTES).unwrap_or(u64::MAX);
    let text = response.body_mut().with_config().limit(limit).read_to_string().map_err(|e| e.to_string())?;
    Ok((status, text))
}

impl Ops for RealOps {
    fn installed(&self) -> bool {
        engine::installed()
    }
    fn record(&self) -> Option<engine::Record> {
        engine::current()
    }
    fn start(&self) -> Result<engine::Record, String> {
        engine::connect_or_start(IDLE_EXIT_MINUTES)
    }
    fn get(&self, record: &engine::Record, path: &str) -> Result<(u16, String), String> {
        let response =
            agent().get(&url(record, path)).header("Authorization", &engine::authorization(&record.secret)).call().map_err(|e| e.to_string())?;
        answer(response)
    }
    fn post(&self, record: &engine::Record, path: &str, body: Option<&str>) -> Result<(u16, String), String> {
        let request = agent().post(&url(record, path)).header("Authorization", &engine::authorization(&record.secret));
        let response = match body {
            Some(body) => request.header("Content-Type", "application/json").send(body),
            None => request.send_empty(),
        }
        .map_err(|e| e.to_string())?;
        answer(response)
    }
    fn open(&self, url: &str) -> Result<(), String> {
        crate::process::open_url(url)
    }
}

fn is_connect_error(e: &str) -> bool {
    let e = e.to_ascii_lowercase();
    e.contains("connection refused") || (e.contains("connect") && e.contains("failed")) || e.contains("timed out") || e.contains("timeout")
}

/// A 2xx answer is done; anything else is said in plain words.
fn expect_ok(answer: (u16, String)) -> Result<String, String> {
    match answer.0 {
        200..=299 => Ok(answer.1),
        404 => Err("That device is no longer waiting".into()),
        status => Err(format!("The AI engine refused that ({status})")),
    }
}

struct Snapshot {
    state: DeskState,
    taken: Option<Instant>,
}

pub struct Desk {
    ops: Arc<dyn Ops + Send + Sync>,
    wake: Wake,
    snapshot: Arc<Mutex<Snapshot>>,
    /// A status look is running.
    looking: Arc<AtomicBool>,
    /// A request (start, sign-in, approve...) is running.
    busy: Arc<AtomicBool>,
    last_error: Arc<Mutex<Option<String>>>,
    /// `~/.redrob/desk/settings.json` (`%USERPROFILE%\.redrob\desk\settings.json` on Windows).
    settings: Option<PathBuf>,
    /// Its `allowAutoAll`, read once and kept as written.
    auto_all: bool,
    /// The newest `auto_all_on` lines of `~/.redrob/desk/events.jsonl`, re-read with each look.
    events: Arc<Mutex<Option<Vec<AutoAllEvent>>>>,
}

/// The desk settings file the daemon reads: `~/.redrob/desk/settings.json`.
fn settings_path() -> Option<PathBuf> {
    Some(dirs::home_dir()?.join(".redrob").join("desk").join("settings.json"))
}

/// The events file Redrob Code appends to: `~/.redrob/desk/events.jsonl`.
fn events_path() -> Option<PathBuf> {
    Some(dirs::home_dir()?.join(".redrob").join("desk").join("events.jsonl"))
}

/// The newest `auto_all_on` lines from the last [`MAX_EVENTS_BYTES`] of the events file; none
/// when it is missing.
fn read_events(path: &Path) -> Vec<AutoAllEvent> {
    let Ok(mut file) = std::fs::File::open(path) else { return Vec::new() };
    let len = file.metadata().map(|m| m.len()).unwrap_or(0);
    let from = len.saturating_sub(MAX_EVENTS_BYTES);
    if file.seek(SeekFrom::Start(from)).is_err() {
        return Vec::new();
    }
    let mut bytes = Vec::new();
    if file.take(MAX_EVENTS_BYTES).read_to_end(&mut bytes).is_err() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&bytes);
    // A cut first line is half a record.
    let text = if from > 0 { text.split_once('\n').map_or("", |(_, rest)| rest) } else { &text };
    desk::parse_auto_all_events(text)
}

/// `allowAutoAll` from the settings file: true unless it says `false`.
fn read_auto_all(path: &Path) -> bool {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .and_then(|v| v.get("allowAutoAll").and_then(serde_json::Value::as_bool))
        .unwrap_or(true)
}

/// Sets `allowAutoAll`, keeping every other key. The folder is made if missing and the file is
/// replaced in one rename. A file that isn't a JSON object is left alone rather than overwritten.
fn write_auto_all(path: &Path, on: bool) -> Result<(), String> {
    let mut object = match std::fs::read_to_string(path) {
        Ok(text) if text.trim().is_empty() => serde_json::Map::new(),
        Ok(text) => match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(serde_json::Value::Object(map)) => map,
            _ => return Err(format!("{} isn't a settings file", path.display())),
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => serde_json::Map::new(),
        Err(e) => return Err(format!("couldn't read {} ({e})", path.display())),
    };
    object.insert("allowAutoAll".into(), serde_json::Value::Bool(on));
    let mut text = serde_json::to_string_pretty(&serde_json::Value::Object(object)).map_err(|e| e.to_string())?;
    text.push('\n');
    crate::paths::write_atomic(path, text.as_bytes())
}

impl Desk {
    pub fn desktop(wake: Wake) -> Self {
        Self::new(Arc::new(RealOps), wake)
    }

    pub fn new(ops: Arc<dyn Ops + Send + Sync>, wake: Wake) -> Self {
        let settings = settings_path();
        Self {
            ops,
            wake,
            snapshot: Arc::new(Mutex::new(Snapshot { state: DeskState::default(), taken: None })),
            looking: Arc::new(AtomicBool::new(false)),
            busy: Arc::new(AtomicBool::new(false)),
            last_error: Arc::new(Mutex::new(None)),
            auto_all: settings.as_deref().is_none_or(read_auto_all),
            settings,
            events: Arc::new(Mutex::new(None)),
        }
    }

    fn look_now(ops: &dyn Ops) -> DeskState {
        if !ops.installed() {
            return DeskState::NotInstalled;
        }
        let Some(record) = ops.record() else { return DeskState::NoEngine };
        match ops.get(&record, "/desk/status") {
            Ok((200, text)) => match desk::parse_status(&text) {
                Ok(status) => DeskState::Up(status),
                Err(e) => DeskState::Unreachable(e),
            },
            // An older engine with no desk routes.
            Ok((404, _)) => DeskState::Unsupported,
            Ok((status, _)) => DeskState::Unreachable(format!("The AI engine refused that ({status})")),
            Err(e) if is_connect_error(&e) => DeskState::NoEngine,
            Err(e) => DeskState::Unreachable(format!("Couldn't reach the AI engine: {e}")),
        }
    }

    fn store(snapshot: &Mutex<Snapshot>, state: DeskState) {
        *snapshot.lock().unwrap_or_else(PoisonError::into_inner) = Snapshot { state, taken: Some(Instant::now()) };
    }

    /// Run `work` on a thread, then look again. One at a time: a second press while busy is ignored.
    fn run(&mut self, work: impl FnOnce(&dyn Ops) -> Result<(), String> + Send + 'static) {
        if self.busy.swap(true, Ordering::SeqCst) {
            return;
        }
        *self.last_error.lock().unwrap_or_else(PoisonError::into_inner) = None;
        let (ops, snapshot, busy, last_error, wake) =
            (self.ops.clone(), self.snapshot.clone(), self.busy.clone(), self.last_error.clone(), self.wake.clone());
        std::thread::spawn(move || {
            if let Err(e) = work(ops.as_ref()) {
                log::warn!("desk: {e}");
                *last_error.lock().unwrap_or_else(PoisonError::into_inner) = Some(e);
            }
            Self::store(&snapshot, Self::look_now(ops.as_ref()));
            busy.store(false, Ordering::SeqCst);
            wake();
        });
    }

    /// The running daemon, started if it isn't (for a request the person made).
    fn daemon(ops: &dyn Ops) -> Result<engine::Record, String> {
        match ops.record() {
            Some(record) => Ok(record),
            None => ops.start().map_err(|e| format!("The AI engine didn't start: {e}")),
        }
    }

    fn post(&mut self, path: String, body: Option<&'static str>) {
        self.run(move |ops| {
            let record = Self::daemon(ops)?;
            let answer = ops.post(&record, &path, body).map_err(|e| format!("Couldn't reach the AI engine: {e}"))?;
            expect_ok(answer).map(|_| ())
        });
    }
}

impl DeskBackend for Desk {
    fn state(&mut self) -> DeskState {
        if self.busy.load(Ordering::SeqCst) {
            return DeskState::Busy;
        }
        let (state, due) = {
            let s = self.snapshot.lock().unwrap_or_else(PoisonError::into_inner);
            (s.state.clone(), s.taken.is_none_or(|t| t.elapsed() >= Duration::from_secs(POLL_SECS)))
        };
        if due && !self.looking.swap(true, Ordering::SeqCst) {
            let (ops, snapshot, looking, wake) = (self.ops.clone(), self.snapshot.clone(), self.looking.clone(), self.wake.clone());
            let events = self.events.clone();
            std::thread::spawn(move || {
                let fresh = Self::look_now(ops.as_ref());
                let mut changed = snapshot.lock().unwrap_or_else(PoisonError::into_inner).state != fresh;
                Self::store(&snapshot, fresh);
                let read = events_path().as_deref().map(read_events).unwrap_or_default();
                let mut kept = events.lock().unwrap_or_else(PoisonError::into_inner);
                if kept.as_ref() != Some(&read) {
                    *kept = Some(read);
                    changed = true;
                }
                drop(kept);
                looking.store(false, Ordering::SeqCst);
                if changed {
                    wake();
                }
            });
        }
        state
    }

    fn ensure(&mut self) {
        self.run(|ops| {
            if !ops.installed() {
                return Ok(());
            }
            let record = ops.start().map_err(|e| format!("The AI engine didn't start: {e}"))?;
            match ops.post(&record, "/desk/enable", Some(r#"{"enabled":true}"#)) {
                // An older engine with no desk routes: the page says to update it.
                Ok((404, _)) => Ok(()),
                Ok(answer) => expect_ok(answer).map(|_| ()),
                Err(e) => Err(format!("Couldn't reach the AI engine: {e}")),
            }
        });
    }

    fn login(&mut self) {
        self.run(|ops| {
            let record = Self::daemon(ops)?;
            let answer = ops.post(&record, "/desk/login", None).map_err(|e| format!("Couldn't reach the AI engine: {e}"))?;
            let text = expect_ok(answer)?;
            // Already signed in: the daemon has nothing to open.
            let Ok(login) = serde_json::from_str::<DeskLogin>(&text) else { return Ok(()) };
            let url = login.verification_uri_complete.trim();
            if url.is_empty() {
                return Ok(());
            }
            if !desk::is_openable_url(url) {
                return Err("The sign-in page couldn't be opened".into());
            }
            ops.open(url).map_err(|e| format!("The sign-in page couldn't be opened: {e}"))
        });
    }

    fn logout(&mut self) {
        self.post("/desk/logout".into(), None);
    }

    fn approve(&mut self, code: &str) {
        if desk::is_code(code) {
            self.post(format!("/desk/pending/{code}/approve"), None);
        }
    }

    fn deny(&mut self, code: &str) {
        if desk::is_code(code) {
            self.post(format!("/desk/pending/{code}/deny"), None);
        }
    }

    fn revoke(&mut self, id: &str) {
        if desk::is_device_id(id) {
            self.post(format!("/desk/devices/{id}/revoke"), None);
        }
    }

    fn last_error(&self) -> Option<String> {
        self.last_error.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }

    fn allow_auto_all(&self) -> bool {
        self.auto_all
    }

    fn auto_all_events(&self) -> Option<Vec<AutoAllEvent>> {
        self.events.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }

    fn set_allow_auto_all(&mut self, on: bool) {
        let result = match &self.settings {
            Some(path) => write_auto_all(path, on),
            None => Err("no home folder".into()),
        };
        let error = match result {
            Ok(()) => {
                self.auto_all = on;
                None
            }
            Err(e) => {
                log::warn!("desk: {e}");
                Some(format!("Couldn't save the Auto all setting: {e}"))
            }
        };
        *self.last_error.lock().unwrap_or_else(PoisonError::into_inner) = error;
    }
}
