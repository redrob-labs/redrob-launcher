//! Using this PC from the web and the phone ("desk"), as the UI sees it.
//!
//! Redrob Chat on the web and the phone app reach this computer's shared Redrob Code daemon
//! through a relay. The daemon holds the relay connection, the sign-in and the list of devices it
//! trusts; the launcher is where a person signs in and allows a new device once. This module holds
//! the daemon's `GET /desk/status` answer shape, the checks that keep what it says safe to show and
//! to put in a request path, the judgement of what the UI shows ([`view`]), which pending devices
//! are new since the last look ([`new_pending`]), and the [`DeskBackend`] trait the desktop backend
//! implements. The HTTP calls and the processes live in the desktop crate.

use std::collections::BTreeSet;

use serde::Deserialize;

/// The longest `/desk/status` answer read. A real one is a few hundred bytes.
pub const MAX_STATUS_BYTES: usize = 256 * 1024;
/// At most this many pending devices and trusted devices are shown.
pub const MAX_LISTED: usize = 32;
/// A device name is cut to this many characters.
pub const MAX_NAME: usize = 64;
/// How often the launcher asks the daemon while it runs.
pub const POLL_SECS: u64 = 3;

/// Whether the device is the phone app or a web browser profile.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceKind {
    /// The phone app. A missing `kind` means a phone (v1 apps).
    #[default]
    Phone,
    Web,
    /// A kind this launcher doesn't know yet.
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct DeskAccount {
    pub email: String,
}

/// A device-grant sign-in the daemon started: the browser page that finishes it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DeskLogin {
    pub user_code: String,
    pub verification_uri_complete: String,
    /// Unix milliseconds.
    pub expires_at: i64,
}

/// A device asking to be allowed. The same 6-digit code shows on the device.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PendingDevice {
    pub code: String,
    pub name: String,
    pub kind: DeviceKind,
    /// Unix milliseconds.
    pub expires_at: i64,
}

/// A device the person allowed.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DeskDevice {
    pub id: String,
    pub name: String,
    pub kind: DeviceKind,
    /// Unix milliseconds.
    pub added_at: i64,
}

/// `GET /desk/status`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DeskStatus {
    /// The relay connection is on (`channels.json`, channel `desk`).
    pub enabled: bool,
    pub signed_in: bool,
    pub account: Option<DeskAccount>,
    /// The daemon is connected to the relay right now.
    pub connected: bool,
    pub login: Option<DeskLogin>,
    pub pending: Vec<PendingDevice>,
    pub devices: Vec<DeskDevice>,
}

/// A pairing code: exactly six ASCII digits. Anything else never reaches a request path.
pub fn is_code(code: &str) -> bool {
    code.len() == 6 && code.bytes().all(|b| b.is_ascii_digit())
}

/// A device id safe as one path segment: 1 to 128 of `[A-Za-z0-9_-]`.
pub fn is_device_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 128 && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

/// A browser page the launcher may open for a sign-in: `https://`, or a loopback `http://` for a
/// developer's own server. Anything else is dropped.
pub fn is_openable_url(url: &str) -> bool {
    let url = url.trim();
    if url.len() > 2048 || url.bytes().any(|b| b.is_ascii_control() || b == b' ') {
        return false;
    }
    let lower = url.to_ascii_lowercase();
    lower.starts_with("https://") || lower.starts_with("http://127.0.0.1") || lower.starts_with("http://localhost")
}

/// The name as shown: control characters dropped, trimmed, at most [`MAX_NAME`] characters.
fn clean_name(name: &str) -> String {
    let name: String = name.chars().filter(|c| !c.is_control()).collect();
    name.trim().chars().take(MAX_NAME).collect()
}

/// Reads the daemon's `/desk/status` answer. What doesn't fit its checks is dropped, never shown:
/// a pending device without a 6-digit code, a device without a usable id, a sign-in page that
/// isn't `https://`.
pub fn parse_status(text: &str) -> Result<DeskStatus, String> {
    if text.len() > MAX_STATUS_BYTES {
        return Err("the engine's desk status is too large".into());
    }
    let mut status: DeskStatus = serde_json::from_str(text).map_err(|e| format!("the engine's desk status doesn't parse: {e}"))?;
    status.pending.retain(|p| is_code(&p.code));
    status.pending.truncate(MAX_LISTED);
    for p in &mut status.pending {
        p.name = clean_name(&p.name);
    }
    status.devices.retain(|d| is_device_id(&d.id));
    status.devices.truncate(MAX_LISTED);
    for d in &mut status.devices {
        d.name = clean_name(&d.name);
    }
    if status.login.as_ref().is_some_and(|l| !is_openable_url(&l.verification_uri_complete)) {
        status.login = None;
    }
    if let Some(a) = &mut status.account {
        a.email = clean_name(&a.email);
    }
    Ok(status)
}

/// What the backend knows about desk right now.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DeskState {
    /// No `redrob` binary: nothing to show.
    #[default]
    NotInstalled,
    /// Installed, but no daemon answers.
    NoEngine,
    /// Starting the daemon or sending a request.
    Busy,
    /// The daemon answers but has no `/desk` routes: an older Redrob Code.
    Unsupported,
    /// The daemon answered something unusable.
    Unreachable(String),
    Up(DeskStatus),
}

/// What the Code card and the AI engine page show for desk.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeskView {
    /// Nothing: Redrob Code isn't installed, or its daemon isn't up yet.
    Hidden,
    Working,
    /// An older Redrob Code: update it to use this.
    NeedsUpdate,
    /// Not signed in. `login` is the sign-in waiting in the browser, if one started.
    SignIn { login: Option<DeskLogin> },
    Ready { email: Option<String>, enabled: bool, connected: bool, devices: Vec<DeskDevice> },
    Problem(String),
}

/// What to show for a state.
pub fn view(state: &DeskState) -> DeskView {
    match state {
        DeskState::NotInstalled | DeskState::NoEngine => DeskView::Hidden,
        DeskState::Busy => DeskView::Working,
        DeskState::Unsupported => DeskView::NeedsUpdate,
        DeskState::Unreachable(e) => DeskView::Problem(e.clone()),
        DeskState::Up(s) if !s.signed_in => DeskView::SignIn { login: s.login.clone() },
        DeskState::Up(s) => DeskView::Ready {
            email: s.account.as_ref().map(|a| a.email.clone()).filter(|e| !e.is_empty()),
            enabled: s.enabled,
            connected: s.connected,
            devices: s.devices.clone(),
        },
    }
}

/// The pending devices whose code isn't in `seen`: each gets one notification and brings the
/// launcher forward once.
pub fn new_pending<'a>(seen: &BTreeSet<String>, status: &'a DeskStatus) -> Vec<&'a PendingDevice> {
    status.pending.iter().filter(|p| !seen.contains(&p.code)).collect()
}

/// The desktop backend's desk control. Every call returns at once; requests run on a thread and
/// show up in [`DeskBackend::state`].
pub trait DeskBackend {
    /// The last look, refreshed every [`POLL_SECS`] while the daemon runs.
    fn state(&mut self) -> DeskState;
    /// Makes sure the shared daemon runs and stays up (`redrob service start --json --idle-exit 0`),
    /// then turns desk on (`POST /desk/enable {enabled:true}`).
    fn ensure(&mut self);
    /// Starts the sign-in (`POST /desk/login`) and opens its page in the default browser.
    fn login(&mut self);
    fn logout(&mut self);
    fn approve(&mut self, code: &str);
    fn deny(&mut self, code: &str);
    fn revoke(&mut self, id: &str);
    /// What the last request said when it failed, in plain words; cleared by the next one.
    fn last_error(&self) -> Option<String>;
}
