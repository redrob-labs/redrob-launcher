//! The Redrob workspace this computer is signed in to, as the UI sees it.
//!
//! The launcher signs in once per computer through the console's device flow (product id
//! `launcher`) and keeps the workspace key in the shared account file every Redrob app can read
//! (`docs/shared-account.md`). It can also hand that key to an app that keeps its own key file
//! today. The UI only ever sees names and states, never the key; the desktop backend implements
//! [`AccountBackend`], and the web build has none.

/// What may be shown about the signed-in workspace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Workspace {
    /// The workspace's name on the console (`Acme`).
    pub account_name: String,
    /// The key's name on the console (`Redrob Launcher`), so it can be found and revoked there.
    pub key_name: String,
}

/// A sign-in the person is approving on the console.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingConnect {
    pub user_code: String,
    /// The approval page, with the code filled in.
    pub url: String,
    /// Seconds left before the code expires.
    pub seconds_left: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConnectState {
    Idle,
    /// Asking the console for a code.
    Starting,
    Waiting(PendingConnect),
    /// The attempt ended without a key; the text says why, in plain English (the UI translates it).
    Failed(String),
}

/// Whether one app can use the workspace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppLink {
    /// The app reads the shared account file itself: signing in here is all it needs.
    Shared,
    /// The app keeps its own key file, and it holds a key. `same_key`: it is this computer's key.
    Connected { same_key: bool, key_name: String },
    /// The app keeps its own key file, and it holds no key yet.
    NotConnected,
    /// The launcher doesn't know where this app keeps its key yet.
    Unsupported,
}

/// The desktop backend's sign-in and key hand-off. Every call returns at once; the device flow
/// runs on a thread and its progress shows up in [`AccountBackend::state`].
pub trait AccountBackend {
    fn workspace(&self) -> Option<Workspace>;
    fn state(&mut self) -> ConnectState;
    /// Start the device flow (no-op while one is running).
    fn connect(&mut self);
    fn cancel(&mut self);
    /// Forget this computer's key (it stays valid on the console until revoked there).
    fn disconnect(&mut self);
    fn app_link(&self, app: &str) -> AppLink;
    /// Write this computer's key into the app's own key file. `Err` says why not, in plain words.
    fn connect_app(&mut self, app: &str) -> Result<(), String>;
}
