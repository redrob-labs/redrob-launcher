//! Downloading the apps' models into the folders the apps read them from.
//!
//! Each model's files come from `assets/models/models.json` (pinned URL, SHA-256 and size). They are
//! downloaded with the same machinery as the apps themselves ([`Http::download`]): resumable, paused
//! by the person or by a dropped connection, checked against their SHA-256 before they are renamed
//! into place, so an app never sees a half-written or wrong file. A model counts as present the way
//! the app itself decides it: every file there with its pinned size.
//!
//! Deleting removes only the files the manifest names (and their partial downloads), never
//! anything else in the folder, and the model's folder only once it is empty.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, PoisonError};

use redrob_launcher_core::device::Device;
use redrob_launcher_core::models::{Model, ModelBackend, ModelState, ModelView, Runtime, RuntimeState, SHARED_APP, Serving};
use redrob_launcher_core::platform::{Arch, Os};
use redrob_launcher_core::protocol::{CANCELLED, PAUSED};
use redrob_launcher_core::registry::{InstalledModel, REGISTRY_FILE, Registry, Served};

use crate::http::{self, Http, Stop, Transfer};
use crate::runtime;

fn env_dir(k: &str) -> Option<PathBuf> {
    std::env::var_os(k).filter(|v| !v.is_empty()).map(PathBuf::from)
}

/// The folder an app reads its downloaded models from, as that app computes it. `None`: unknown.
///
/// Redrob Film: `<data dir>/models`, where the data dir is `FILMCRAFT_DATA_DIR`, else
/// `~/Library/Application Support/Redrob Film`, `%APPDATA%\Redrob Film` or
/// `$XDG_DATA_HOME/redrob-film` (`~/.local/share/redrob-film`). A portable Redrob Film keeps its data
/// beside itself; the launcher installs the regular kind, so that case isn't looked for.
pub fn models_dir(app: &str) -> Option<PathBuf> {
    match app {
        "redrob-film" => {
            let data = env_dir("FILMCRAFT_DATA_DIR").or_else(|| {
                if cfg!(target_os = "macos") {
                    env_dir("HOME").map(|h| h.join("Library/Application Support/Redrob Film"))
                } else if cfg!(windows) {
                    env_dir("APPDATA").map(|h| h.join("Redrob Film"))
                } else {
                    env_dir("XDG_DATA_HOME").map(|d| d.join("redrob-film")).or_else(|| env_dir("HOME").map(|h| h.join(".local/share/redrob-film")))
                }
            })?;
            Some(data.join("models"))
        }
        // Shared models, and every other app's: one folder every Redrob app reads through the
        // installed registry (`installed.json` beside the model folders).
        SHARED_APP | "redrob-canvas" | "redrob-shape" | "redrob-recall" | "redrob-code" => shared_models_dir(),
        _ => None,
    }
}

/// `~/.redrob/models` (`REDROB_MODELS_DIR` overrides): the shared models folder and the registry.
pub fn shared_models_dir() -> Option<PathBuf> {
    env_dir("REDROB_MODELS_DIR").or_else(|| dirs::home_dir().map(|h| h.join(".redrob").join("models")))
}

/// Where the model lives, given its app's models folder.
pub fn model_dir(models: &Path, m: &Model) -> PathBuf {
    models.join(&m.id)
}

/// The model's state from what is on disk.
pub fn disk_state(dir: &Path, m: &Model) -> ModelState {
    let mut done = 0;
    let mut complete = 0;
    for f in &m.files {
        let dest = dir.join(&f.name);
        if std::fs::metadata(&dest).is_ok_and(|md| md.len() == f.size) {
            done += f.size;
            complete += 1;
        } else if let Some(n) = http::partial_bytes(&dest, f.size) {
            done += n.min(f.size);
        }
    }
    if complete == m.files.len() {
        ModelState::Installed
    } else if done > 0 {
        ModelState::Partial { done, total: m.size() }
    } else {
        ModelState::NotDownloaded
    }
}

/// Download every missing file of `m` into `dir`. `progress` gets bytes done over the whole model.
pub fn download(http: &Http, dir: &Path, m: &Model, stop: &Stop, progress: &mut dyn FnMut(u64, u64)) -> Result<(), String> {
    let total = m.size();
    let mut before = 0;
    for f in &m.files {
        let dest = dir.join(&f.name);
        let base = before;
        http.download(&f.url, &dest, f.size, Some(&f.sha256), stop, &mut |t| {
            let (Transfer::Bytes { done, .. } | Transfer::Reconnecting { done, .. }) = t;
            progress(base + done.min(f.size), total);
        })?;
        before += f.size;
    }
    Ok(())
}

/// Remove the model's files and partial downloads, then its folder if nothing else is in it.
pub fn delete(dir: &Path, m: &Model) -> Result<(), String> {
    for f in &m.files {
        http::discard(&dir.join(&f.name), f.size);
        let left = dir.join(&f.name);
        if left.exists() {
            std::fs::remove_file(&left).map_err(|e| format!("couldn't delete {} ({e})", left.display()))?;
        }
    }
    // Only an empty folder goes; anything the app put there itself stays.
    let _ = std::fs::remove_dir(dir);
    Ok(())
}

/// Where an app's models go: the app's real folder, or a test's.
pub type FolderOf = Arc<dyn Fn(&str) -> Option<PathBuf> + Send + Sync>;

type Key = (String, String);

struct Job {
    stop: Arc<Stop>,
    done: u64,
    total: u64,
}

/// A served model's lifecycle, as the backend tracks it.
enum Serve {
    Starting,
    Running(runtime::Server),
    Failed(String),
}

struct RuntimeJob {
    done: u64,
    total: u64,
}

/// Everything a worker thread needs to rewrite the installed registry after a change.
#[derive(Clone)]
struct Shared {
    models: Arc<Vec<Model>>,
    dir: FolderOf,
    servers: Arc<Mutex<HashMap<Key, Serve>>>,
    device: Arc<Mutex<Device>>,
    registry_path: Option<PathBuf>,
    wake: Arc<dyn Fn() + Send + Sync>,
}

impl Shared {
    /// Rewrites `installed.json` from what is on disk and what is being served.
    fn write_registry(&self) {
        let Some(path) = &self.registry_path else { return };
        let backend = self.device.lock().unwrap_or_else(PoisonError::into_inner).backend();
        let servers = self.servers.lock().unwrap_or_else(PoisonError::into_inner);
        let mut installed = Vec::new();
        for m in self.models.iter() {
            let Some(dir) = (self.dir)(&m.app).map(|d| model_dir(&d, m)) else { continue };
            if disk_state(&dir, m) != ModelState::Installed {
                continue;
            }
            let served = match servers.get(&key(&m.app, &m.id)) {
                Some(Serve::Running(s)) => Some(Served { url: s.url(), port: s.port, pid: s.pid() }),
                _ => None,
            };
            installed.push(InstalledModel::from_model(m, dir.display().to_string(), served));
        }
        drop(servers);
        let registry = Registry::new(backend.id(), installed);
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Err(e) = crate::paths::write_atomic(path, registry.to_json().as_bytes()) {
            log::warn!("couldn't write the models registry {}: {e}", path.display());
        }
    }
}

/// The launcher's model downloads and local servers: at most one job per model, each on its
/// own thread; one `llama-server` process per served model.
pub struct Models {
    shared: Shared,
    http: Option<Http>,
    jobs: Arc<Mutex<HashMap<Key, Job>>>,
    failed: Arc<Mutex<HashMap<Key, String>>>,
    runtime_jobs: Arc<Mutex<HashMap<Runtime, RuntimeJob>>>,
    runtime_failed: Arc<Mutex<HashMap<Runtime, String>>>,
    runtimes_root: Option<PathBuf>,
    code_config: Option<PathBuf>,
    /// The last answers to `models()` and `disk_free()`, with when they were made. The UI asks
    /// every frame; the file scan (two `metadata` calls per catalogue file) and the free-space probe
    /// (a `df`/PowerShell process) are too slow for that and made the Models page scroll in jerks.
    /// A download in progress keeps the views fresh through its own progress (the cache is short).
    views_cache: Mutex<Option<(Instant, Vec<ModelView>)>>,
    disk_cache: Mutex<Option<(Instant, Option<u64>)>>,
}

/// How long `models()` reuses its last scan, and `disk_free()` its last probe.
const VIEWS_FRESH_FOR: Duration = Duration::from_millis(700);
const DISK_FRESH_FOR: Duration = Duration::from_secs(5);

/// The device record before the probe has run: the platform, nothing measured.
fn unmeasured() -> Device {
    Device {
        os: if cfg!(target_os = "macos") {
            Os::MacOs
        } else if cfg!(windows) {
            Os::Windows
        } else {
            Os::Linux
        },
        arch: if cfg!(target_arch = "aarch64") { Arch::Arm64 } else { Arch::X64 },
        cpu: String::new(),
        cores: 0,
        ram_mb: 0,
        gpu: None,
        npu: None,
        gpu_probed: false,
    }
}

impl Models {
    pub fn new(models: Vec<Model>, http: Option<Http>, wake: Arc<dyn Fn() + Send + Sync>, dir: FolderOf) -> Self {
        let registry_path = dir(SHARED_APP).map(|d| d.join(REGISTRY_FILE));
        Self {
            shared: Shared {
                models: Arc::new(models),
                dir,
                servers: Arc::default(),
                device: Arc::new(Mutex::new(unmeasured())),
                registry_path,
                wake,
            },
            http,
            jobs: Arc::default(),
            failed: Arc::default(),
            runtime_jobs: Arc::default(),
            runtime_failed: Arc::default(),
            runtimes_root: None,
            code_config: None,
            views_cache: Mutex::new(None),
            disk_cache: Mutex::new(None),
        }
    }

    /// Forget the cached scan: something changed on disk or in the jobs.
    fn invalidate(&self) {
        *self.views_cache.lock().unwrap_or_else(PoisonError::into_inner) = None;
    }

    /// The built-in manifest, the apps' real folders, the real runtimes folder and Redrob Code's
    /// config; measures the computer on a thread right away.
    pub fn desktop(http: Option<Http>, wake: Arc<dyn Fn() + Send + Sync>) -> Self {
        let mut m = Self::new(redrob_launcher_core::models::builtin(), http, wake, Arc::new(models_dir));
        m.runtimes_root = runtime::runtimes_dir();
        m.code_config = runtime::code_config_path();
        m.remeasure();
        m
    }

    /// Test hook: where runtimes unpack and where Redrob Code's config is.
    pub fn with_paths(mut self, runtimes_root: Option<PathBuf>, code_config: Option<PathBuf>) -> Self {
        self.runtimes_root = runtimes_root;
        self.code_config = code_config;
        self
    }

    /// Test hook: a device record instead of a probe.
    pub fn with_device(self, device: Device) -> Self {
        *self.shared.device.lock().unwrap_or_else(PoisonError::into_inner) = device;
        self
    }

    fn find(&self, app: &str, id: &str) -> Option<&Model> {
        self.shared.models.iter().find(|m| m.app == app && m.id == id)
    }

    fn served_ports(&self) -> Vec<u16> {
        self.shared
            .servers
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .values()
            .filter_map(|s| match s {
                Serve::Running(s) => Some(s.port),
                _ => None,
            })
            .collect()
    }

    /// Stops every server. Called when the launcher quits so no `llama-server` outlives it.
    pub fn stop_all(&mut self) {
        let keys: Vec<Key> = self.shared.servers.lock().unwrap_or_else(PoisonError::into_inner).keys().cloned().collect();
        for (app, id) in keys {
            self.stop(&app, &id);
        }
    }
}

impl Models {
    /// The real scan behind `models()`: what is on disk, downloading, failed or served, per model.
    fn scan(&self) -> Vec<ModelView> {
        let jobs = self.jobs.lock().unwrap_or_else(PoisonError::into_inner);
        let failed = self.failed.lock().unwrap_or_else(PoisonError::into_inner);
        let servers = self.shared.servers.lock().unwrap_or_else(PoisonError::into_inner);
        let device = self.shared.device.lock().unwrap_or_else(PoisonError::into_inner).clone();
        self.shared
            .models
            .iter()
            .map(|m| {
                let folder = (self.shared.dir)(&m.app).map(|d| model_dir(&d, m));
                let k = key(&m.app, &m.id);
                let state = match (&folder, jobs.get(&k), failed.get(&k)) {
                    (None, ..) => ModelState::NoFolder,
                    (Some(_), Some(j), _) => ModelState::Downloading { done: j.done, total: j.total },
                    (Some(d), None, f) => match (disk_state(d, m), f) {
                        (ModelState::Installed, _) => ModelState::Installed,
                        (_, Some(e)) => ModelState::Failed(e.clone()),
                        (s, None) => s,
                    },
                };
                let serving = match servers.get(&k) {
                    None => Serving::No,
                    Some(Serve::Starting) => Serving::Starting,
                    Some(Serve::Running(s)) => Serving::Running { url: s.url() },
                    Some(Serve::Failed(e)) => Serving::Failed(e.clone()),
                };
                ModelView { model: m.clone(), state, folder: folder.map(|d| d.display().to_string()), fit: device.fit(&m.requires), serving }
            })
            .collect()
    }
}

impl Drop for Models {
    fn drop(&mut self) {
        self.stop_all();
    }
}

fn key(app: &str, id: &str) -> (String, String) {
    (app.to_string(), id.to_string())
}

/// Free bytes on the volume holding `dir`, through the OS's own tool.
fn free_bytes(dir: &Path) -> Option<u64> {
    let probe = std::time::Duration::from_secs(3);
    let mut at = dir.to_path_buf();
    while !at.exists() {
        at = at.parent()?.to_path_buf();
    }
    let s = at.display().to_string();
    if cfg!(windows) {
        let script = format!("(Get-Item -LiteralPath '{}').PSDrive.Free", s.replace('\'', "''"));
        crate::process::capture("powershell", &["-NoProfile", "-Command", &script], probe)?.trim().parse().ok()
    } else {
        // POSIX `df -Pk`: the fourth column of the last line is the free KiB.
        let out = crate::process::capture("df", &["-Pk", &s], probe)?;
        let line = out.lines().last()?;
        let kib: u64 = line.split_whitespace().nth(3)?.parse().ok()?;
        Some(kib * 1024)
    }
}

impl ModelBackend for Models {
    fn models(&self) -> Vec<ModelView> {
        {
            let cache = self.views_cache.lock().unwrap_or_else(PoisonError::into_inner);
            if let Some((at, views)) = cache.as_ref()
                && at.elapsed() < VIEWS_FRESH_FOR
            {
                return views.clone();
            }
        }
        let views = self.scan();
        *self.views_cache.lock().unwrap_or_else(PoisonError::into_inner) = Some((Instant::now(), views.clone()));
        views
    }

    fn download(&mut self, app: &str, id: &str) {
        self.invalidate();
        let Some(m) = self.find(app, id).cloned() else { return };
        let Some(dir) = (self.shared.dir)(app).map(|d| model_dir(&d, &m)) else { return };
        let k = key(app, id);
        let stop = Arc::new(Stop::default());
        {
            let mut jobs = self.jobs.lock().unwrap_or_else(PoisonError::into_inner);
            if jobs.contains_key(&k) {
                return;
            }
            let done = match disk_state(&dir, &m) {
                ModelState::Partial { done, .. } => done,
                _ => 0,
            };
            jobs.insert(k.clone(), Job { stop: stop.clone(), done, total: m.size() });
        }
        self.failed.lock().unwrap_or_else(PoisonError::into_inner).remove(&k);
        let Some(http) = self.http.clone() else {
            self.jobs.lock().unwrap_or_else(PoisonError::into_inner).remove(&k);
            self.failed.lock().unwrap_or_else(PoisonError::into_inner).insert(k, "HTTPS isn't available on this computer.".into());
            (self.shared.wake)();
            return;
        };
        let (jobs, failed, shared) = (self.jobs.clone(), self.failed.clone(), self.shared.clone());
        std::thread::spawn(move || {
            let mut last = std::time::Instant::now();
            let result = download(&http, &dir, &m, &stop, &mut |done, total| {
                if let Some(j) = jobs.lock().unwrap_or_else(PoisonError::into_inner).get_mut(&k) {
                    j.done = done;
                    j.total = total;
                }
                // A repaint every quarter second is plenty for a progress bar.
                if last.elapsed() >= std::time::Duration::from_millis(250) {
                    last = std::time::Instant::now();
                    (shared.wake)();
                }
            });
            jobs.lock().unwrap_or_else(PoisonError::into_inner).remove(&k);
            match result {
                Ok(()) => shared.write_registry(),
                // Paused or cancelled: what's on disk says where it stands.
                Err(e) if e == PAUSED || e == CANCELLED => {}
                Err(e) => {
                    log::warn!("download of model {} for {} failed: {e}", m.id, m.app);
                    failed.lock().unwrap_or_else(PoisonError::into_inner).insert(k, e);
                }
            }
            (shared.wake)();
        });
        (self.shared.wake)();
    }

    fn pause(&mut self, app: &str, id: &str) {
        self.invalidate();
        if let Some(j) = self.jobs.lock().unwrap_or_else(PoisonError::into_inner).get(&key(app, id)) {
            j.stop.pause();
        }
    }

    fn delete(&mut self, app: &str, id: &str) -> Result<(), String> {
        self.invalidate();
        let m = self.find(app, id).cloned().ok_or("unknown model")?;
        let dir = (self.shared.dir)(app).map(|d| model_dir(&d, &m)).ok_or("the app's models folder isn't known")?;
        self.stop(app, id);
        if let Some(j) = self.jobs.lock().unwrap_or_else(PoisonError::into_inner).get(&key(app, id)) {
            // Stop it first: its thread deletes its partial file and ends.
            j.stop.cancel();
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
        self.failed.lock().unwrap_or_else(PoisonError::into_inner).remove(&key(app, id));
        let r = delete(&dir, &m);
        self.shared.write_registry();
        (self.shared.wake)();
        r
    }

    fn device(&self) -> Device {
        self.shared.device.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }

    fn remeasure(&mut self) {
        let shared = self.shared.clone();
        std::thread::spawn(move || {
            let d = crate::device::measure();
            *shared.device.lock().unwrap_or_else(PoisonError::into_inner) = d;
            shared.write_registry();
            (shared.wake)();
        });
    }

    fn runtime(&self, rt: Runtime) -> RuntimeState {
        if !rt.served() {
            return RuntimeState::Installed;
        }
        if let Some(j) = self.runtime_jobs.lock().unwrap_or_else(PoisonError::into_inner).get(&rt) {
            return RuntimeState::Downloading { done: j.done, total: j.total };
        }
        if let Some(e) = self.runtime_failed.lock().unwrap_or_else(PoisonError::into_inner).get(&rt) {
            return RuntimeState::Failed(e.clone());
        }
        let device = self.device();
        let Some((release, _, _)) = runtime::asset_for(&device, rt) else { return RuntimeState::Unavailable };
        match &self.runtimes_root {
            Some(root) if runtime::installed_binary(&runtime::release_dir(root, rt, &release), &release).is_some() => RuntimeState::Installed,
            Some(_) => RuntimeState::NotInstalled,
            None => RuntimeState::Unavailable,
        }
    }

    fn install_runtime(&mut self, rt: Runtime) {
        self.invalidate();
        if !rt.served() || matches!(self.runtime(rt), RuntimeState::Installed | RuntimeState::Downloading { .. } | RuntimeState::Unavailable) {
            return;
        }
        let (Some(http), Some(root)) = (self.http.clone(), self.runtimes_root.clone()) else {
            self.runtime_failed.lock().unwrap_or_else(PoisonError::into_inner).insert(rt, "HTTPS isn't available on this computer.".into());
            return;
        };
        let stop = Arc::new(Stop::default());
        self.runtime_jobs.lock().unwrap_or_else(PoisonError::into_inner).insert(rt, RuntimeJob { done: 0, total: 0 });
        self.runtime_failed.lock().unwrap_or_else(PoisonError::into_inner).remove(&rt);
        let (jobs, failed, shared) = (self.runtime_jobs.clone(), self.runtime_failed.clone(), self.shared.clone());
        let device = self.device();
        std::thread::spawn(move || {
            let r = runtime::install(&http, &root, &device, rt, &stop, &mut |done, total| {
                if let Some(j) = jobs.lock().unwrap_or_else(PoisonError::into_inner).get_mut(&rt) {
                    j.done = done;
                    j.total = total;
                }
                (shared.wake)();
            });
            jobs.lock().unwrap_or_else(PoisonError::into_inner).remove(&rt);
            if let Err(e) = r {
                log::warn!("installing {} failed: {e}", rt.id());
                failed.lock().unwrap_or_else(PoisonError::into_inner).insert(rt, e);
            }
            (shared.wake)();
        });
        (self.shared.wake)();
    }

    fn start(&mut self, app: &str, id: &str) {
        self.invalidate();
        let Some(m) = self.find(app, id).cloned() else { return };
        let k = key(app, id);
        let fail = |servers: &Arc<Mutex<HashMap<Key, Serve>>>, e: &str| {
            servers.lock().unwrap_or_else(PoisonError::into_inner).insert(k.clone(), Serve::Failed(e.into()));
        };
        if !m.runtime.served() {
            return;
        }
        {
            let servers = self.shared.servers.lock().unwrap_or_else(PoisonError::into_inner);
            if matches!(servers.get(&k), Some(Serve::Starting | Serve::Running(_))) {
                return;
            }
        }
        let Some(dir) = (self.shared.dir)(app).map(|d| model_dir(&d, &m)) else {
            fail(&self.shared.servers, "the models folder isn't known");
            return;
        };
        if disk_state(&dir, &m) != ModelState::Installed {
            fail(&self.shared.servers, "download the model first");
            return;
        }
        let (Some(http), Some(root)) = (self.http.clone(), self.runtimes_root.clone()) else {
            fail(&self.shared.servers, "HTTPS isn't available on this computer.");
            return;
        };
        let port = match runtime::free_port(4100, &self.served_ports()) {
            Some(p) => p,
            None => {
                fail(&self.shared.servers, "no free port between 4100 and 4300");
                return;
            }
        };
        self.shared.servers.lock().unwrap_or_else(PoisonError::into_inner).insert(k.clone(), Serve::Starting);
        let device = self.device();
        let shared = self.shared.clone();
        let code_config = self.code_config.clone();
        let (rjobs, rfailed) = (self.runtime_jobs.clone(), self.runtime_failed.clone());
        std::thread::spawn(move || {
            // The runtime first, if it isn't here yet (shown as the runtime's own download).
            let stop = Arc::new(Stop::default());
            rjobs.lock().unwrap_or_else(PoisonError::into_inner).entry(m.runtime).or_insert(RuntimeJob { done: 0, total: 0 });
            let bin = runtime::install(&http, &root, &device, m.runtime, &stop, &mut |done, total| {
                if let Some(j) = rjobs.lock().unwrap_or_else(PoisonError::into_inner).get_mut(&m.runtime) {
                    j.done = done;
                    j.total = total;
                }
                (shared.wake)();
            });
            rjobs.lock().unwrap_or_else(PoisonError::into_inner).remove(&m.runtime);
            let bin = match bin {
                Ok(b) => b,
                Err(e) => {
                    rfailed.lock().unwrap_or_else(PoisonError::into_inner).insert(m.runtime, e.clone());
                    shared.servers.lock().unwrap_or_else(PoisonError::into_inner).insert(k, Serve::Failed(e));
                    (shared.wake)();
                    return;
                }
            };
            let result = runtime::Server::start(&bin, &m, &dir, port, &http, std::time::Duration::from_secs(240));
            let entry = match result {
                Ok(server) => {
                    if let Some(cfg) = &code_config
                        && let Err(e) = runtime::set_code_provider(cfg, &m, Some(&server.url()))
                    {
                        log::warn!("couldn't add the local provider to Redrob Code's config: {e}");
                    }
                    Serve::Running(server)
                }
                Err(e) => {
                    log::warn!("serving {} failed: {e}", m.id);
                    Serve::Failed(e)
                }
            };
            shared.servers.lock().unwrap_or_else(PoisonError::into_inner).insert(k, entry);
            shared.write_registry();
            (shared.wake)();
        });
        (self.shared.wake)();
    }

    fn stop(&mut self, app: &str, id: &str) {
        self.invalidate();
        let k = key(app, id);
        let removed = self.shared.servers.lock().unwrap_or_else(PoisonError::into_inner).remove(&k);
        let Some(entry) = removed else { return };
        if let Serve::Running(server) = entry {
            server.stop();
            if let (Some(cfg), Some(m)) = (&self.code_config, self.find(app, id))
                && let Err(e) = runtime::set_code_provider(cfg, m, None)
            {
                log::warn!("couldn't remove the local provider from Redrob Code's config: {e}");
            }
        }
        self.shared.write_registry();
        (self.shared.wake)();
    }

    fn disk_free(&self) -> Option<u64> {
        {
            let cache = self.disk_cache.lock().unwrap_or_else(PoisonError::into_inner);
            if let Some((at, free)) = cache.as_ref()
                && at.elapsed() < DISK_FRESH_FOR
            {
                return *free;
            }
        }
        let free = (self.shared.dir)(SHARED_APP).and_then(|dir| free_bytes(&dir));
        *self.disk_cache.lock().unwrap_or_else(PoisonError::into_inner) = Some((Instant::now(), free));
        free
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use redrob_launcher_core::models::ModelFile;

    fn model(files: &[(&str, &[u8])]) -> Model {
        Model {
            id: "m".into(),
            app: "redrob-film".into(),
            name: "M".into(),
            summary: String::new(),
            description: String::new(),
            license: String::new(),
            source: "https://x".into(),
            task: "stt".into(),
            runtime: Runtime::Files,
            requires: redrob_launcher_core::models::Requires { memory_mb: 1024, backends: vec![redrob_launcher_core::models::Backend::Cpu] },
            serve: None,
            files: files
                .iter()
                .map(|(n, b)| ModelFile {
                    name: (*n).into(),
                    url: format!("https://example.invalid/{n}"),
                    sha256: http::hex(&sha2_of(b)),
                    size: b.len() as u64,
                })
                .collect(),
        }
    }

    fn sha2_of(b: &[u8]) -> Vec<u8> {
        use sha2::Digest;
        sha2::Sha256::digest(b).to_vec()
    }

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("redrob-launcher-models-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn state_follows_what_is_on_disk() {
        let dir = temp("state");
        let m = model(&[("a.bin", b"aaaa"), ("b.bin", b"bbbbbbbb")]);
        assert_eq!(disk_state(&dir, &m), ModelState::NotDownloaded);
        std::fs::write(dir.join("a.bin"), b"aaaa").unwrap();
        assert_eq!(disk_state(&dir, &m), ModelState::Partial { done: 4, total: 12 });
        // A partial download of the second file counts too.
        std::fs::write(dir.join("b.bin.8.part"), b"bbb").unwrap();
        assert_eq!(disk_state(&dir, &m), ModelState::Partial { done: 7, total: 12 });
        // A file of the wrong size is not "there".
        std::fs::write(dir.join("b.bin"), b"short").unwrap();
        assert_eq!(disk_state(&dir, &m), ModelState::Partial { done: 7, total: 12 });
        std::fs::write(dir.join("b.bin"), b"bbbbbbbb").unwrap();
        assert_eq!(disk_state(&dir, &m), ModelState::Installed);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn delete_removes_only_the_models_files() {
        let root = temp("delete");
        let dir = root.join("m");
        std::fs::create_dir_all(&dir).unwrap();
        let m = model(&[("a.bin", b"aaaa"), ("b.bin", b"bbbbbbbb")]);
        std::fs::write(dir.join("a.bin"), b"aaaa").unwrap();
        std::fs::write(dir.join("b.bin.8.part"), b"bb").unwrap();
        std::fs::write(dir.join("notes.txt"), b"the app's own").unwrap();
        delete(&dir, &m).unwrap();
        assert!(!dir.join("a.bin").exists() && !dir.join("b.bin.8.part").exists());
        assert!(dir.join("notes.txt").exists(), "a file the manifest doesn't name stays");
        std::fs::remove_file(dir.join("notes.txt")).unwrap();
        delete(&dir, &m).unwrap();
        assert!(!dir.exists(), "an emptied folder goes");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn views_report_unknown_folders_and_failures() {
        let root = temp("views");
        let r = root.clone();
        let m = model(&[("a.bin", b"aaaa")]);
        let mut models = Models::new(
            vec![m.clone(), Model { app: "redrob-code".into(), ..m.clone() }],
            None,
            Arc::new(|| {}),
            Arc::new(move |app: &str| (app == "redrob-film").then(|| r.clone())),
        );
        let v = models.models();
        assert_eq!(v[0].state, ModelState::NotDownloaded);
        assert_eq!(v[1].state, ModelState::NoFolder);
        assert!(v[0].folder.as_deref().is_some_and(|f| f.ends_with("m")));
        // Without HTTPS the download fails with a reason, and nothing is left running.
        models.download("redrob-film", "m");
        assert_eq!(models.models()[0].state, ModelState::Failed("HTTPS isn't available on this computer.".into()));
        // Present on disk wins over an old failure.
        std::fs::create_dir_all(root.join("m")).unwrap();
        std::fs::write(root.join("m").join("a.bin"), b"aaaa").unwrap();
        assert_eq!(models.models()[0].state, ModelState::Installed);
        models.delete("redrob-film", "m").unwrap();
        assert_eq!(models.models()[0].state, ModelState::NotDownloaded);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn redrob_film_models_go_where_it_reads_them() {
        let dir = models_dir("redrob-film");
        assert!(dir.as_ref().is_some_and(|d| d.ends_with("models")), "{dir:?}");
        assert_eq!(models_dir("redrob-office"), None);
        assert_eq!(models_dir("redrob"), shared_models_dir());
        assert_eq!(models_dir("redrob-canvas"), shared_models_dir());
    }

    #[test]
    fn the_registry_lists_installed_models_and_the_backend() {
        let root = temp("registry");
        let r = root.clone();
        let m = model(&[("a.bin", b"aaaa")]);
        let mut models = Models::new(vec![m.clone()], None, Arc::new(|| {}), Arc::new(move |_: &str| Some(r.clone())));
        std::fs::create_dir_all(root.join("m")).unwrap();
        std::fs::write(root.join("m").join("a.bin"), b"aaaa").unwrap();
        models.shared.write_registry();
        let reg = Registry::parse(&std::fs::read_to_string(root.join(REGISTRY_FILE)).unwrap()).unwrap();
        assert_eq!(reg.backend, "cpu", "nothing measured yet means the CPU");
        assert_eq!(reg.models.len(), 1);
        assert_eq!((reg.models[0].id.as_str(), reg.models[0].task.as_str(), reg.models[0].served.is_none()), ("m", "stt", true));
        assert!(reg.models[0].path.ends_with("m"));
        assert_eq!(reg.models[0].files, ["a.bin"]);
        // Deleting rewrites it empty, and the verdicts follow the device record.
        models.delete("redrob-film", "m").unwrap();
        let reg = Registry::parse(&std::fs::read_to_string(root.join(REGISTRY_FILE)).unwrap()).unwrap();
        assert!(reg.models.is_empty());
        assert_eq!(models.models()[0].fit, redrob_launcher_core::device::Fit::WontRun, "0 MB RAM before the probe");
        assert_eq!(models.models()[0].serving, Serving::No);
        assert_eq!(models.runtime(Runtime::Files), RuntimeState::Installed);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn starting_a_plain_files_model_or_an_absent_one_does_nothing_harmful() {
        let root = temp("start");
        let r = root.clone();
        let m = model(&[("a.bin", b"aaaa")]);
        let served = Model {
            id: "s".into(),
            app: "redrob".into(),
            runtime: Runtime::LlamaServer,
            serve: Some(redrob_launcher_core::models::Serve { model: "a.bin".into(), mmproj: None }),
            ..m.clone()
        };
        let mut models = Models::new(vec![m, served], None, Arc::new(|| {}), Arc::new(move |_: &str| Some(r.clone())));
        models.start("redrob-film", "m");
        assert_eq!(models.models()[0].serving, Serving::No);
        // Not downloaded: refused with a reason, no thread started.
        models.start("redrob", "s");
        assert_eq!(models.models()[1].serving, Serving::Failed("download the model first".into()));
        models.stop("redrob", "s");
        assert_eq!(models.models()[1].serving, Serving::No);
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Live, end to end through the same backend the UI drives: Whisper tiny (146 MB) from
    /// Hugging Face, paused part way, resumed, finished, and every file's SHA-256 checked again
    /// (`cargo test -p redrob-launcher-native -- --ignored a_real_model_pauses_resumes_and_verifies`).
    #[test]
    #[ignore = "needs the network; downloads 146 MB"]
    fn a_real_model_pauses_resumes_and_verifies() {
        let root = temp("live-full");
        let r = root.clone();
        let tiny = redrob_launcher_core::models::builtin().into_iter().find(|m| m.id == "whisper-tiny").unwrap();
        let mut models = Models::new(vec![tiny.clone()], Some(Http::new().unwrap()), Arc::new(|| {}), Arc::new(move |_: &str| Some(r.clone())));
        let state = |m: &Models| m.models()[0].state.clone();
        let wait = |m: &Models, done: &dyn Fn(&ModelState) -> bool| {
            let until = std::time::Instant::now() + std::time::Duration::from_secs(600);
            while !done(&state(m)) && std::time::Instant::now() < until {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        };
        models.download("redrob-film", "whisper-tiny");
        wait(&models, &|s| matches!(s, ModelState::Downloading { done, .. } if *done > 20 << 20));
        models.pause("redrob-film", "whisper-tiny");
        wait(&models, &|s| !matches!(s, ModelState::Downloading { .. }));
        let paused = state(&models);
        assert!(matches!(paused, ModelState::Partial { done, total } if done > 0 && done < total), "{paused:?}");
        models.download("redrob-film", "whisper-tiny");
        wait(&models, &|s| *s == ModelState::Installed);
        assert_eq!(state(&models), ModelState::Installed);
        for f in &tiny.files {
            assert_eq!(http::sha256_file(&root.join("whisper-tiny").join(&f.name)).unwrap(), f.sha256, "{}", f.name);
        }
        eprintln!("paused at {paused:?}, then finished and verified {} files", tiny.files.len());
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Live: downloads Whisper tiny's config from Hugging Face and checks its pin
    /// (`cargo test -p redrob-launcher-native -- --ignored real_model_file`).
    #[test]
    #[ignore = "needs the network"]
    fn real_model_file() {
        let models = redrob_launcher_core::models::builtin();
        let tiny = models.iter().find(|m| m.id == "whisper-tiny").unwrap();
        let one = Model { files: vec![tiny.files[0].clone()], ..tiny.clone() };
        let dir = temp("live");
        let http = Http::new().unwrap();
        download(&http, &dir, &one, &Stop::default(), &mut |_, _| {}).unwrap();
        assert_eq!(disk_state(&dir, &one), ModelState::Installed);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
