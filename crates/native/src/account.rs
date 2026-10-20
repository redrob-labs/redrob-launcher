//! Signing this computer in to a Redrob workspace once, and handing that key to the apps.
//!
//! The console implements RFC 8628's device authorization grant. The launcher asks it for a code
//! under the product id `launcher`, the person approves that code on console.redrob.ai, and the
//! launcher polls until the console hands back a workspace API key. The key goes into the shared
//! account file (`docs/shared-account.md`), which every Redrob app on this computer may read.
//!
//! Apps that keep a key file of their own today (Redrob CAD, Redrob Film, Redrob Code) can be handed
//! the same key: [`Connector`] knows each one's path and format. The device code never leaves this
//! module and is never logged or stored; a pending sign-in lives in memory only. Every file holding
//! a key is written readable by this user alone (0600 on Unix), through a temporary file renamed
//! over the old one. Nothing retries a refusal: the console never turns a refused code into a key.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use redrob_launcher_core::account::{AccountBackend, AppLink, ConnectState, LoginState, PendingConnect, Workspace};
use serde_json::{Value, json};

pub const CONSOLE_API: &str = "https://console.redrob.ai/api/backend/v1";
/// The launcher's client id at redrob-auth (the Redrob account sign-in).
pub const LOGIN_CLIENT_ID: &str = "redrob-launcher";
/// Overrides the console base URL (tests, staging).
pub const CONSOLE_API_ENV: &str = "REDROB_CONSOLE_API";
/// Overrides the shared account file's directory.
pub const SHARED_DIR_ENV: &str = "REDROB_SHARED_DIR";
/// The launcher's product id on the console.
pub const PRODUCT: &str = "launcher";
/// The shared account file's name, in [`shared_dir`].
pub const ACCOUNT_FILE: &str = "account.json";

const FALLBACK_EXPIRES: Duration = Duration::from_secs(600);
const MAX_EXPIRES: Duration = Duration::from_secs(1800);
const FALLBACK_INTERVAL: Duration = Duration::from_secs(5);
/// RFC 8628 3.5: on `slow_down` the interval grows by at least five seconds.
const SLOW_DOWN: Duration = Duration::from_secs(5);
/// The console restarts its interval window on every poll; a second of headroom avoids `slow_down`.
const MARGIN: Duration = Duration::from_secs(1);
const MAX_UNREACHABLE: u32 = 5;

/// The console's answer to one POST: HTTP status and JSON body (`{}` when it had none).
pub trait Transport: Send + Sync {
    fn post(&self, url: &str, body: &Value) -> Result<(u16, Value), String>;
}

/// Called from the sign-in thread when its state changes (repaints the UI).
pub type Wake = Arc<dyn Fn() + Send + Sync>;
/// Opens the console's approval page.
pub type Open = Arc<dyn Fn(&str) + Send + Sync>;

/// HTTPS to the console, with the OS verifying its certificate (as [`crate::http::Http`] does).
pub struct Https(ureq::Agent);

impl Https {
    pub fn new() -> Self {
        let tls = ureq::tls::TlsConfig::builder().root_certs(ureq::tls::RootCerts::PlatformVerifier).build();
        Self(
            ureq::Agent::config_builder()
                .tls_config(tls)
                .timeout_global(Some(Duration::from_secs(20)))
                .http_status_as_error(false)
                .user_agent(concat!("Redrob-Launcher/", env!("CARGO_PKG_VERSION")))
                .build()
                .into(),
        )
    }
}

impl Default for Https {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport for Https {
    fn post(&self, url: &str, body: &Value) -> Result<(u16, Value), String> {
        let mut resp = self.0.post(url).header("content-type", "application/json").send(body.to_string()).map_err(|e| e.to_string())?;
        let status = resp.status().as_u16();
        let text = resp.body_mut().read_to_string().unwrap_or_default();
        Ok((status, serde_json::from_str(&text).unwrap_or_else(|_| json!({}))))
    }
}

/// The connected workspace as stored. Only `api_key` is secret.
#[derive(Clone, PartialEq, Eq)]
pub struct Stored {
    pub api_key: String,
    pub api_key_name: String,
    pub account_name: String,
    pub api_base_url: String,
}

impl std::fmt::Debug for Stored {
    /// Never prints the key.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stored")
            .field("api_key", &"<redacted>")
            .field("api_key_name", &self.api_key_name)
            .field("account_name", &self.account_name)
            .field("api_base_url", &self.api_base_url)
            .finish()
    }
}

impl Stored {
    /// The shared file's and Redrob CAD's / Redrob Film's format.
    fn to_json(&self) -> Value {
        json!({"apiKey": self.api_key, "apiKeyName": self.api_key_name, "accountName": self.account_name, "apiBaseUrl": self.api_base_url})
    }
    fn from_json(v: &Value) -> Option<Self> {
        let s = |k: &str| v[k].as_str().map(str::trim).unwrap_or("").to_string();
        let api_key = s("apiKey");
        (!api_key.is_empty()).then(|| Stored {
            api_key,
            api_key_name: s("apiKeyName"),
            account_name: s("accountName"),
            api_base_url: s("apiBaseUrl"),
        })
    }
}

fn env_dir(k: &str) -> Option<PathBuf> {
    std::env::var_os(k).filter(|v| !v.is_empty()).map(PathBuf::from)
}

/// The per-user configuration root: `%APPDATA%`, `~/Library/Application Support`, or
/// `$XDG_CONFIG_HOME` (`~/.config`).
fn config_root() -> Option<PathBuf> {
    if cfg!(windows) {
        env_dir("APPDATA")
    } else if cfg!(target_os = "macos") {
        env_dir("HOME").map(|h| h.join("Library/Application Support"))
    } else {
        env_dir("XDG_CONFIG_HOME").or_else(|| env_dir("HOME").map(|h| h.join(".config")))
    }
}

/// The shared account file's directory (`docs/shared-account.md`).
pub fn shared_dir() -> Option<PathBuf> {
    if let Some(d) = env_dir(SHARED_DIR_ENV) {
        return Some(d);
    }
    let root = config_root()?;
    Some(if cfg!(windows) {
        root.join("Redrob").join("Shared")
    } else if cfg!(target_os = "macos") {
        root.join("ai.redrob.shared")
    } else {
        root.join("redrob-shared")
    })
}

/// The console base URL: `REDROB_CONSOLE_API` or the production console.
pub fn console_base() -> String {
    std::env::var(CONSOLE_API_ENV)
        .ok()
        .map(|s| s.trim().trim_end_matches('/').to_string())
        .filter(|s| s.starts_with("https://"))
        .unwrap_or_else(|| CONSOLE_API.to_string())
}

fn read_json(path: &Path) -> Option<Value> {
    serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

pub fn load(path: &Path) -> Option<Stored> {
    Stored::from_json(&read_json(path)?)
}

/// Write `value` readable by this user only: a 0600 temp file renamed over the old one.
fn write_private(path: &Path, value: &Value) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("json.tmp");
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    {
        use std::io::Write;
        let mut f = opts.open(&tmp)?;
        f.write_all(serde_json::to_string_pretty(value).unwrap_or_default().as_bytes())?;
        f.sync_all()?;
    }
    std::fs::rename(&tmp, path)
}

pub fn save(path: &Path, s: &Stored) -> std::io::Result<()> {
    write_private(path, &s.to_json())
}

// ---- apps that keep their own key -------------------------------------------------------------

/// How an app that keeps its own key file today stores it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// `account.json`: the same shape as the shared file (Redrob CAD, Redrob Film).
    Account,
    /// Redrob Code's `auth.json`: one entry per provider; ours is `"redrob": {"type": "api", "key"}`.
    /// The other entries are the person's own and are kept as they are.
    RedrobCodeAuth,
}

/// Where one app keeps its key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Connector {
    pub app: &'static str,
    pub path: PathBuf,
    pub format: Format,
}

/// An `account.json` in the app's own config directory: `<env>` overrides it, else
/// `%APPDATA%\<Windows name>`, `~/Library/Application Support/<bundle id>`, `~/.config/<id>`.
fn account_dir(env: &str, windows: &str, bundle: &str, id: &str) -> Option<PathBuf> {
    if let Some(d) = env_dir(env) {
        return Some(d);
    }
    let root = config_root()?;
    Some(if cfg!(windows) {
        root.join(windows)
    } else if cfg!(target_os = "macos") {
        root.join(bundle)
    } else {
        root.join(id)
    })
}

/// Redrob Code's data directory: `$XDG_DATA_HOME/redrob` (`~/.local/share/redrob`) on every
/// platform, as its `xdg-basedir` resolves it.
fn redrob_code_data() -> Option<PathBuf> {
    env_dir("XDG_DATA_HOME").or_else(|| dirs::home_dir().map(|h| h.join(".local").join("share"))).map(|d| d.join("redrob"))
}

/// The app's key file, for the apps that keep their own. `None`: the launcher doesn't know one.
pub fn connector(app: &str) -> Option<Connector> {
    let (path, format) = match app {
        "redrob-cad" => (account_dir("REDROB_CAD_CONFIG_DIR", "Redrob CAD", "ai.redrob.cad", "redrob-cad")?.join("account.json"), Format::Account),
        "redrob-film" => {
            (account_dir("REDROB_CUT_CONFIG_DIR", "Redrob Film", "ai.redrob.film", "redrob-film")?.join("account.json"), Format::Account)
        }
        "redrob-code" => (redrob_code_data()?.join("auth.json"), Format::RedrobCodeAuth),
        _ => return None,
    };
    Some(Connector { app: CONNECTOR_IDS.iter().copied().find(|a| *a == app)?, path, format })
}

/// The apps [`connector`] knows.
pub const CONNECTOR_IDS: [&str; 3] = ["redrob-cad", "redrob-film", "redrob-code"];

/// The key an app's file holds now, and its name when the format records one.
pub fn read_app_key(c: &Connector) -> Option<(String, String)> {
    let v = read_json(&c.path)?;
    match c.format {
        Format::Account => Stored::from_json(&v).map(|s| (s.api_key, s.api_key_name)),
        Format::RedrobCodeAuth => {
            let e = &v["redrob"];
            let key = e["key"].as_str().map(str::trim).filter(|k| !k.is_empty() && e["type"] == "api")?;
            Some((key.to_string(), String::new()))
        }
    }
}

/// Put `s`'s key into the app's own file.
pub fn write_app_key(c: &Connector, s: &Stored) -> std::io::Result<()> {
    match c.format {
        Format::Account => save(&c.path, s),
        Format::RedrobCodeAuth => {
            // Keep the person's other providers; an unreadable file is not overwritten.
            let mut v = match std::fs::read_to_string(&c.path) {
                Ok(text) => serde_json::from_str::<Value>(&text)
                    .ok()
                    .filter(Value::is_object)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{} isn't a JSON object", c.path.display())))?,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => json!({}),
                Err(e) => return Err(e),
            };
            v["redrob"] = json!({"type": "api", "key": s.api_key});
            write_private(&c.path, &v)
        }
    }
}

// ---- the device flow --------------------------------------------------------------------------

/// How one poll ended.
#[derive(Debug, PartialEq, Eq)]
pub enum Poll {
    Pending,
    SlowDown,
    Connected(Stored),
    Denied,
    Expired,
    Unreachable,
    Failed(String),
}

/// What the console said when asked for a code. `device_code` never leaves this module.
pub struct Started {
    device_code: String,
    pub user_code: String,
    pub url: String,
    pub expires_in: Duration,
    pub interval: Duration,
}

pub fn authorize(t: &dyn Transport, base: &str) -> Result<Started, String> {
    let (status, b) =
        t.post(&format!("{base}/device/authorize"), &json!({"product": PRODUCT})).map_err(|_| "Couldn't reach console.redrob.ai.".to_string())?;
    if !(200..300).contains(&status) {
        return Err("console.redrob.ai wouldn't start the sign-in.".into());
    }
    let s = |k: &str| b[k].as_str().map(str::trim).unwrap_or("").to_string();
    let (device_code, user_code, uri) = (s("deviceCode"), s("userCode"), s("verificationUri"));
    if device_code.is_empty() || user_code.is_empty() || !uri.starts_with("https://") {
        return Err("console.redrob.ai didn't return a usable code.".into());
    }
    let complete = s("verificationUriComplete");
    let secs = |k: &str| b[k].as_f64().filter(|v| v.is_finite() && *v > 0.0).map(Duration::from_secs_f64);
    Ok(Started {
        device_code,
        user_code,
        url: if complete.starts_with("https://") { complete } else { uri },
        expires_in: secs("expiresIn").unwrap_or(FALLBACK_EXPIRES).min(MAX_EXPIRES),
        interval: secs("interval").unwrap_or(FALLBACK_INTERVAL).max(Duration::from_secs(1)),
    })
}

pub fn poll_once(t: &dyn Transport, base: &str, started: &Started) -> Poll {
    let Ok((status, b)) = t.post(&format!("{base}/device/token"), &json!({"deviceCode": started.device_code})) else { return Poll::Unreachable };
    if (200..300).contains(&status) {
        let mut v = b.clone();
        if v["apiBaseUrl"].as_str().is_none_or(str::is_empty) {
            v["apiBaseUrl"] = json!(base);
        }
        return Stored::from_json(&v).map(Poll::Connected).unwrap_or(Poll::Failed("missing_key".into()));
    }
    match b["error"].as_str().unwrap_or("") {
        "authorization_pending" => Poll::Pending,
        "slow_down" => Poll::SlowDown,
        "access_denied" => Poll::Denied,
        "expired_token" => Poll::Expired,
        "" => Poll::Failed(format!("http_{status}")),
        other => Poll::Failed(other.to_string()),
    }
}

fn plain(p: &Poll) -> String {
    match p {
        Poll::Denied => "The sign-in was declined on console.redrob.ai.".into(),
        Poll::Expired => "The code expired. Sign in again for a new one.".into(),
        Poll::Unreachable => "Couldn't reach console.redrob.ai. Check the network and sign in again.".into(),
        Poll::Failed(code) => format!("console.redrob.ai refused the sign-in ({code})."),
        _ => String::new(),
    }
}

struct Attempt {
    state: ConnectState,
    expires: Option<Instant>,
    cancelled: Arc<AtomicBool>,
}

/// The Redrob account (redrob-auth) sign-in, next to the workspace key.
struct Login {
    state: LoginState,
    session: Option<Arc<redrob_login::Session>>,
    /// Bumped by every sign-in and sign-out, so a thread that finishes late can't undo a newer one.
    generation: u64,
}

/// Sets the account's state unless a newer sign-in or sign-out started after `generation`.
fn set_login(login: &Mutex<Login>, generation: u64, state: LoginState, session: Option<Arc<redrob_login::Session>>) -> bool {
    let mut l = login.lock().unwrap_or_else(PoisonError::into_inner);
    if l.generation != generation {
        return false;
    }
    l.state = state;
    l.session = session;
    true
}

/// What the account row shows: the person's name, else their email.
fn who(session: &redrob_login::Session) -> Result<String, redrob_login::Error> {
    let u = session.userinfo()?;
    Ok(u.name.or(u.email).unwrap_or_default())
}

fn login_plain(e: &redrob_login::Error) -> String {
    use redrob_login::Error as E;
    match e {
        E::Denied => "The sign-in was declined.".into(),
        E::TimedOut => "The sign-in took too long. Sign in again.".into(),
        E::StateMismatch => "The sign-in answer didn't match. Sign in again.".into(),
        E::Browser(_) => "Couldn't open the browser.".into(),
        E::Network(_) => "Couldn't reach auth.redrob.io. Check the network and sign in again.".into(),
        E::Keychain(_) => "The keychain wouldn't keep the sign-in.".into(),
        E::NotAvailable(_) | E::NotSignedIn | E::Expired | E::Server(_) => "auth.redrob.io refused the sign-in.".into(),
    }
}

/// This computer's workspace: the shared key, and at most one sign-in running in the background.
pub struct DeviceAccount {
    path: Option<PathBuf>,
    stored: Arc<Mutex<Option<Stored>>>,
    /// The shared file as last read (modified time, size) and when that was checked, so a change
    /// made by an app or by another launcher instance is picked up without a restart.
    seen: Mutex<(Option<FileStamp>, Option<Instant>)>,
    attempt: Arc<Mutex<Attempt>>,
    transport: Arc<dyn Transport>,
    base: String,
    wake: Wake,
    /// Opens the approval page (the browser, in the app; nothing in tests).
    open: Open,
    login: Arc<Mutex<Login>>,
}

/// How often the shared file is checked for changes made outside this launcher.
const RECHECK_FILE: Duration = Duration::from_secs(2);

/// What identifies a file's contents cheaply: its modified time and size.
type FileStamp = (std::time::SystemTime, u64);

fn file_stamp(path: &Path) -> Option<FileStamp> {
    let m = std::fs::metadata(path).ok()?;
    Some((m.modified().ok()?, m.len()))
}

impl DeviceAccount {
    pub fn new(path: Option<PathBuf>, transport: Arc<dyn Transport>, base: String, wake: Wake, open: Open) -> Self {
        let stored = path.as_deref().and_then(load);
        let stamp = path.as_deref().and_then(file_stamp);
        Self {
            path,
            stored: Arc::new(Mutex::new(stored)),
            seen: Mutex::new((stamp, Some(Instant::now()))),
            attempt: Arc::new(Mutex::new(Attempt { state: ConnectState::Idle, expires: None, cancelled: Arc::new(AtomicBool::new(false)) })),
            transport,
            base: base.trim_end_matches('/').to_string(),
            wake,
            open,
            login: Arc::new(Mutex::new(Login { state: LoginState::SignedOut, session: None, generation: 0 })),
        }
    }

    /// The launcher's own account: the shared file, the production console, the browser, and the
    /// Redrob account sign-in kept in the keychain.
    pub fn desktop(wake: Wake) -> Self {
        let open: Open = Arc::new(|url: &str| {
            if url.starts_with("https://")
                && let Err(e) = crate::process::open_url(url)
            {
                log::warn!("couldn't open the approval page: {e}");
            }
        });
        let acct = Self::new(shared_dir().map(|d| d.join(ACCOUNT_FILE)), Arc::new(Https::new()), console_base(), wake, open);
        acct.restore_login();
        acct
    }

    /// Picks up the Redrob account kept in the keychain by an earlier run, on a thread (the keychain
    /// and redrob-auth may be slow), then asks redrob-auth who it is.
    fn restore_login(&self) {
        let (login, wake) = (self.login.clone(), self.wake.clone());
        let generation = login.lock().unwrap_or_else(PoisonError::into_inner).generation;
        std::thread::spawn(move || {
            let session = match redrob_login::Session::restore(LOGIN_CLIENT_ID) {
                Ok(Some(s)) => Arc::new(s),
                Ok(None) => return,
                Err(e) => {
                    log::warn!("couldn't read the Redrob sign-in: {e}");
                    return;
                }
            };
            if !set_login(&login, generation, LoginState::SignedIn(String::new()), Some(session.clone())) {
                return;
            }
            wake();
            match who(&session) {
                Ok(name) => {
                    set_login(&login, generation, LoginState::SignedIn(name), Some(session));
                }
                Err(redrob_login::Error::NotSignedIn) => {
                    set_login(&login, generation, LoginState::SignedOut, None);
                }
                Err(redrob_login::Error::NotAvailable(why)) => {
                    log::info!("Redrob sign-in isn't available: {why}");
                    set_login(&login, generation, LoginState::Unavailable, None);
                }
                // Offline or a passing refusal: still signed in; the name shows next time.
                Err(e) => log::warn!("couldn't ask auth.redrob.io who is signed in: {e}"),
            }
            wake();
        });
    }

    /// The shared key as the file has it now: re-read when the file changed since it was last
    /// looked at (an app or another launcher instance may write or remove it; the contract says
    /// readers read it when they need it, and the launcher is a reader too).
    fn stored(&self) -> Option<Stored> {
        if let Some(path) = &self.path {
            let mut seen = self.seen.lock().unwrap_or_else(PoisonError::into_inner);
            let due = seen.1.is_none_or(|t| t.elapsed() >= RECHECK_FILE);
            if due {
                let stamp = file_stamp(path);
                if stamp != seen.0 {
                    *self.stored.lock().unwrap_or_else(PoisonError::into_inner) = load(path);
                    seen.0 = stamp;
                }
                seen.1 = Some(Instant::now());
            }
        }
        self.stored.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }

    /// After this launcher wrote or removed the file: remember its new stamp, so the next look
    /// doesn't re-read what is already in memory.
    fn note_written(&self) {
        let stamp = self.path.as_deref().and_then(file_stamp);
        *self.seen.lock().unwrap_or_else(PoisonError::into_inner) = (stamp, Some(Instant::now()));
    }
}

impl AccountBackend for DeviceAccount {
    fn workspace(&self) -> Option<Workspace> {
        let s = self.stored()?;
        Some(Workspace { account_name: if s.account_name.is_empty() { "Redrob workspace".into() } else { s.account_name }, key_name: s.api_key_name })
    }

    fn state(&mut self) -> ConnectState {
        let a = self.attempt.lock().unwrap_or_else(PoisonError::into_inner);
        match (&a.state, a.expires) {
            (ConnectState::Waiting(p), Some(exp)) => {
                ConnectState::Waiting(PendingConnect { seconds_left: exp.saturating_duration_since(Instant::now()).as_secs(), ..p.clone() })
            }
            (s, _) => s.clone(),
        }
    }

    fn connect(&mut self) {
        let cancelled = Arc::new(AtomicBool::new(false));
        {
            let mut a = self.attempt.lock().unwrap_or_else(PoisonError::into_inner);
            if matches!(a.state, ConnectState::Starting | ConnectState::Waiting(_)) {
                return;
            }
            *a = Attempt { state: ConnectState::Starting, expires: None, cancelled: cancelled.clone() };
        }
        let (attempt, stored, transport, base, path, wake, open) = (
            self.attempt.clone(),
            self.stored.clone(),
            self.transport.clone(),
            self.base.clone(),
            self.path.clone(),
            self.wake.clone(),
            self.open.clone(),
        );
        let cancelled2 = cancelled.clone();
        let set = move |state: ConnectState, expires: Option<Instant>| {
            {
                let mut a = attempt.lock().unwrap_or_else(PoisonError::into_inner);
                if Arc::ptr_eq(&a.cancelled, &cancelled) && !cancelled.load(Ordering::SeqCst) {
                    a.state = state;
                    a.expires = expires;
                }
            }
            wake();
        };
        std::thread::spawn(move || {
            let started = match authorize(transport.as_ref(), &base) {
                Ok(s) => s,
                Err(e) => return set(ConnectState::Failed(e), None),
            };
            let deadline = Instant::now() + started.expires_in;
            open(&started.url);
            set(
                ConnectState::Waiting(PendingConnect {
                    user_code: started.user_code.clone(),
                    url: started.url.clone(),
                    seconds_left: started.expires_in.as_secs(),
                }),
                Some(deadline),
            );
            let mut interval = started.interval;
            let mut unreachable = 0;
            loop {
                std::thread::sleep(interval + MARGIN);
                if cancelled2.load(Ordering::SeqCst) {
                    return;
                }
                if Instant::now() >= deadline {
                    return set(ConnectState::Failed(plain(&Poll::Expired)), None);
                }
                match poll_once(transport.as_ref(), &base, &started) {
                    Poll::Pending => unreachable = 0,
                    Poll::SlowDown => interval += SLOW_DOWN,
                    Poll::Unreachable => {
                        unreachable += 1;
                        if unreachable >= MAX_UNREACHABLE {
                            return set(ConnectState::Failed(plain(&Poll::Unreachable)), None);
                        }
                    }
                    Poll::Connected(s) => {
                        if let Some(p) = &path
                            && let Err(e) = save(p, &s)
                        {
                            return set(ConnectState::Failed(format!("Signed in, but the key couldn't be saved: {e}")), None);
                        }
                        *stored.lock().unwrap_or_else(PoisonError::into_inner) = Some(s);
                        return set(ConnectState::Idle, None);
                    }
                    other => return set(ConnectState::Failed(plain(&other)), None),
                }
            }
        });
    }

    fn cancel(&mut self) {
        let mut a = self.attempt.lock().unwrap_or_else(PoisonError::into_inner);
        a.cancelled.store(true, Ordering::SeqCst);
        *a = Attempt { state: ConnectState::Idle, expires: None, cancelled: Arc::new(AtomicBool::new(false)) };
    }

    fn disconnect(&mut self) {
        // The key itself stays valid on the console; it is revoked there (Settings > API keys).
        // Apps that were handed a copy keep theirs: each is disconnected in that app.
        if let Some(p) = &self.path {
            let _ = std::fs::remove_file(p);
        }
        *self.stored.lock().unwrap_or_else(PoisonError::into_inner) = None;
        self.note_written();
    }

    fn app_link(
        &self,
        app: &str,
        installed: Option<&redrob_launcher_core::version::Version>,
        latest: Option<&redrob_launcher_core::release::Release>,
    ) -> AppLink {
        let c = connector(app);
        let own_key = c.as_ref().and_then(read_app_key).map(|(key, key_name)| {
            let same_key = self.stored().is_some_and(|s| s.api_key == key);
            (same_key, key_name)
        });
        redrob_launcher_core::account::link_for(app, own_key, installed, c.is_some(), latest)
    }

    fn connect_app(&mut self, app: &str) -> Result<(), String> {
        let s = self.stored().ok_or("Sign in first.")?;
        let c = connector(app).ok_or("The launcher can't connect this app yet.")?;
        write_app_key(&c, &s).map_err(|e| format!("Couldn't write the app's key file: {e}"))
    }

    fn login(&mut self) -> LoginState {
        self.login.lock().unwrap_or_else(PoisonError::into_inner).state.clone()
    }

    fn sign_in(&mut self) {
        let generation = {
            let mut l = self.login.lock().unwrap_or_else(PoisonError::into_inner);
            if matches!(l.state, LoginState::SigningIn) {
                return;
            }
            l.generation += 1;
            l.state = LoginState::SigningIn;
            l.session = None;
            l.generation
        };
        let (login, wake) = (self.login.clone(), self.wake.clone());
        std::thread::spawn(move || {
            match redrob_login::login(LOGIN_CLIENT_ID) {
                Ok(session) => {
                    let session = Arc::new(session);
                    let name = who(&session).unwrap_or_else(|e| {
                        log::warn!("signed in, but couldn't ask auth.redrob.io who: {e}");
                        String::new()
                    });
                    set_login(&login, generation, LoginState::SignedIn(name), Some(session));
                }
                Err(redrob_login::Error::NotAvailable(why)) => {
                    log::info!("Redrob sign-in isn't available: {why}");
                    set_login(&login, generation, LoginState::Unavailable, None);
                }
                Err(e) => {
                    log::warn!("Redrob sign-in failed: {e}");
                    set_login(&login, generation, LoginState::Failed(login_plain(&e)), None);
                }
            }
            wake();
        });
    }

    fn sign_out(&mut self) {
        let session = {
            let mut l = self.login.lock().unwrap_or_else(PoisonError::into_inner);
            l.generation += 1;
            l.state = LoginState::SignedOut;
            l.session.take()
        };
        // An open browser sign-in is simply abandoned: its result is ignored when it comes back.
        if let Some(session) = session {
            std::thread::spawn(move || {
                if let Err(e) = session.logout() {
                    log::warn!("signed out, but the sign-in couldn't be removed: {e}");
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    type Answer = Result<(u16, Value), String>;

    /// Answers as the console would, in order, and records what was asked.
    type Asked = Vec<(String, Value)>;
    struct Script(Mutex<(VecDeque<Answer>, Asked)>);
    impl Script {
        fn new(answers: Vec<Answer>) -> Self {
            Self(Mutex::new((answers.into(), Vec::new())))
        }
        fn asked(&self) -> Asked {
            self.0.lock().map(|g| g.1.clone()).unwrap_or_default()
        }
    }
    impl Transport for Script {
        fn post(&self, url: &str, body: &Value) -> Result<(u16, Value), String> {
            let Ok(mut g) = self.0.lock() else { return Err("poisoned".into()) };
            g.1.push((url.to_string(), body.clone()));
            g.0.pop_front().unwrap_or(Err("no answer".into()))
        }
    }

    fn started_body() -> Value {
        json!({"deviceCode": "dev-secret", "userCode": "ABCD-EFGH", "verificationUri": "https://console.redrob.ai/connect",
               "verificationUriComplete": "https://console.redrob.ai/connect?code=ABCD-EFGH", "expiresIn": 600, "interval": 1})
    }

    fn key(k: &str) -> Stored {
        Stored { api_key: k.into(), api_key_name: "Redrob Launcher".into(), account_name: "Acme".into(), api_base_url: "https://c".into() }
    }

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("redrob-launcher-acct-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    fn quiet() -> (Wake, Open) {
        (Arc::new(|| {}), Arc::new(|_: &str| {}))
    }

    #[cfg(unix)]
    fn mode(p: &Path) -> u32 {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(p).map(|m| m.permissions().mode() & 0o777).unwrap_or(0)
    }

    #[test]
    fn authorize_asks_as_the_launcher_and_keeps_the_device_code_inside() {
        let t = Script::new(vec![Ok((200, started_body()))]);
        let s = authorize(&t, "https://c/api").unwrap();
        assert_eq!(t.asked(), vec![("https://c/api/device/authorize".to_string(), json!({"product": "launcher"}))]);
        assert_eq!((s.user_code.as_str(), s.url.as_str()), ("ABCD-EFGH", "https://console.redrob.ai/connect?code=ABCD-EFGH"));
        assert!(authorize(&Script::new(vec![Ok((400, json!({"error": "invalid_request"})))]), "b").is_err());
        assert!(authorize(&Script::new(vec![Ok((200, json!({"userCode": "X"})))]), "b").is_err());
        // An approval page that isn't https is never opened.
        let mut bad = started_body();
        bad["verificationUri"] = json!("javascript:alert(1)");
        bad["verificationUriComplete"] = json!("");
        assert!(authorize(&Script::new(vec![Ok((200, bad))]), "b").is_err());
        assert!(authorize(&Script::new(vec![Err("down".into())]), "b").is_err());
    }

    #[test]
    fn poll_maps_every_console_answer() {
        let s = Started {
            device_code: "dev-secret".into(),
            user_code: "U".into(),
            url: "u".into(),
            expires_in: FALLBACK_EXPIRES,
            interval: FALLBACK_INTERVAL,
        };
        let one = |r: Result<(u16, Value), String>| poll_once(&Script::new(vec![r]), "https://c", &s);
        assert_eq!(one(Ok((400, json!({"error": "authorization_pending"})))), Poll::Pending);
        assert_eq!(one(Ok((400, json!({"error": "slow_down"})))), Poll::SlowDown);
        assert_eq!(one(Ok((403, json!({"error": "access_denied"})))), Poll::Denied);
        assert_eq!(one(Ok((400, json!({"error": "expired_token"})))), Poll::Expired);
        assert_eq!(one(Ok((500, json!({})))), Poll::Failed("http_500".into()));
        assert_eq!(one(Err("down".into())), Poll::Unreachable);
        assert_eq!(one(Ok((200, json!({"apiKeyName": "k"})))), Poll::Failed("missing_key".into()));
        assert_eq!(one(Ok((200, json!({"apiKey": "rk_1", "apiKeyName": "Redrob Launcher", "accountName": "Acme"})))), Poll::Connected(key("rk_1")));
    }

    #[test]
    fn a_full_sign_in_saves_the_shared_key_privately() {
        let dir = temp("flow");
        let path = dir.join(ACCOUNT_FILE);
        let t = Arc::new(Script::new(vec![
            Ok((200, started_body())),
            Ok((400, json!({"error": "authorization_pending"}))),
            Ok((200, json!({"apiKey": "rk_1", "apiKeyName": "Redrob Launcher", "accountName": "Acme", "apiBaseUrl": "https://c"}))),
        ]));
        let opened = Arc::new(Mutex::new(Vec::<String>::new()));
        let o = opened.clone();
        let mut acct = DeviceAccount::new(
            Some(path.clone()),
            t.clone(),
            "https://c".into(),
            Arc::new(|| {}),
            Arc::new(move |u: &str| o.lock().unwrap().push(u.into())),
        );
        assert!(acct.workspace().is_none());
        acct.connect();
        let deadline = Instant::now() + Duration::from_secs(15);
        while acct.workspace().is_none() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(50));
        }
        assert_eq!(acct.workspace(), Some(Workspace { account_name: "Acme".into(), key_name: "Redrob Launcher".into() }));
        assert_eq!(acct.state(), ConnectState::Idle);
        assert_eq!(*opened.lock().unwrap(), ["https://console.redrob.ai/connect?code=ABCD-EFGH"]);
        assert_eq!(load(&path), Some(key("rk_1")));
        #[cfg(unix)]
        assert_eq!(mode(&path), 0o600);
        // The device code went only to the token endpoint.
        let asked = t.asked();
        assert!(asked[1..].iter().all(|(u, b)| u == "https://c/device/token" && b == &json!({"deviceCode": "dev-secret"})));
        // A second launcher start reads it back; signing out removes it.
        let (w, o) = quiet();
        let mut again = DeviceAccount::new(Some(path.clone()), Arc::new(Script::new(vec![])), "https://c".into(), w, o);
        assert_eq!(again.workspace().map(|w| w.account_name), Some("Acme".into()));
        again.disconnect();
        assert!(!path.exists() && again.workspace().is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_refused_start_is_reported_in_plain_words() {
        let (w, o) = quiet();
        let mut acct =
            DeviceAccount::new(None, Arc::new(Script::new(vec![Ok((400, json!({"error": "invalid_request"})))])), "https://c".into(), w, o);
        acct.connect();
        let deadline = Instant::now() + Duration::from_secs(5);
        while matches!(acct.state(), ConnectState::Starting) && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(10));
        }
        assert_eq!(acct.state(), ConnectState::Failed("console.redrob.ai wouldn't start the sign-in.".into()));
        assert!(acct.workspace().is_none());
    }

    #[test]
    fn the_key_never_shows_in_debug_output() {
        assert!(!format!("{:?}", key("rk_secret")).contains("rk_secret"));
    }

    #[test]
    fn an_account_json_app_is_handed_the_same_key() {
        let dir = temp("cad");
        let c = Connector { app: "redrob-cad", path: dir.join("account.json"), format: Format::Account };
        assert_eq!(read_app_key(&c), None);
        write_app_key(&c, &key("rk_1")).unwrap();
        assert_eq!(read_app_key(&c), Some(("rk_1".into(), "Redrob Launcher".into())));
        // Exactly the file Redrob CAD and Redrob Film read.
        let v = read_json(&c.path).unwrap();
        assert_eq!(v["apiKey"], "rk_1");
        assert_eq!(v["apiBaseUrl"], "https://c");
        #[cfg(unix)]
        assert_eq!(mode(&c.path), 0o600);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn redrob_code_keeps_its_other_providers() {
        let dir = temp("code");
        std::fs::create_dir_all(&dir).unwrap();
        let c = Connector { app: "redrob-code", path: dir.join("auth.json"), format: Format::RedrobCodeAuth };
        std::fs::write(&c.path, r#"{"anthropic": {"type": "api", "key": "sk-own"}}"#).unwrap();
        write_app_key(&c, &key("rk_1")).unwrap();
        let v = read_json(&c.path).unwrap();
        assert_eq!(v["anthropic"]["key"], "sk-own");
        assert_eq!(v["redrob"], json!({"type": "api", "key": "rk_1"}));
        assert_eq!(read_app_key(&c), Some(("rk_1".into(), String::new())));
        // A file that isn't an object is left alone rather than overwritten.
        std::fs::write(&c.path, "[1, 2]").unwrap();
        assert!(write_app_key(&c, &key("rk_2")).is_err());
        assert_eq!(std::fs::read_to_string(&c.path).unwrap(), "[1, 2]");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn every_known_connector_resolves_and_others_are_unsupported() {
        for id in CONNECTOR_IDS {
            assert!(redrob_launcher_core::CraftApp::by_id(id).is_some(), "{id}");
            if let Some(c) = connector(id) {
                assert_eq!(c.app, id);
            }
        }
        assert_eq!(connector("redrob-office"), None);
        let (w, o) = quiet();
        let mut acct = DeviceAccount::new(None, Arc::new(Script::new(vec![])), "https://c".into(), w, o);
        // Office is merged on develop but unreleased: it will read the file from its next release.
        assert_eq!(acct.app_link("redrob-office", None, None), AppLink::SharedSoon);
        assert_eq!(acct.app_link("redrob-launcher", None, None), AppLink::NoKey);
        assert_eq!(acct.connect_app("redrob-cad"), Err("Sign in first.".into()));
    }
}
