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
    /// The app reads the shared file from a release that is not out yet.
    SharedSoon,
    /// The latest release reads the shared file, but the installed copy is older.
    SharedAfterUpdate,
    /// The app keeps its own key file, and it holds a key. `same_key`: it is this computer's key.
    Connected { same_key: bool, key_name: String },
    /// The app keeps its own key file, and it holds no key yet.
    NotConnected,
    /// The app never talks to the console, so it needs no sign-in.
    NoKey,
    /// The launcher doesn't know where this app keeps its key yet.
    Unsupported,
}

/// Which apps read the shared account file themselves, and from which released version -- the
/// FALLBACK for releases made before apps shipped `redrob-launcher.json` (see
/// [`crate::release::Manifest`]). A release that carries the manifest is believed over this table,
/// so new releases need no launcher change.
///
/// `None`: the change is merged (every one on `develop`) but no release carries it yet, so an installed
/// copy cannot read the file. The source of each is in `docs/app-integration.md`.
pub const SHARED_READERS: [(&str, Option<&str>); 10] = [
    ("redrob-cad", Some("0.0.3")),
    ("redrob-cut", Some("0.0.1")),
    ("redrob-shape", None),
    ("redrob-browser", None),
    ("redrob-cowork", None),
    ("redrob-office", None),
    ("redrob-design", None),
    ("redrob-query", None),
    ("redrob-recall", None),
    ("redrob-canvas", None),
];

/// Apps that never hold a console key.
pub const NO_KEY: [&str; 2] = ["redrob-studio", "redrob-launcher"];

/// The released version of `app` from which it reads the shared file. `Some(None)`: merged, unreleased.
pub fn shared_since(app: &str) -> Option<Option<crate::version::Version>> {
    SHARED_READERS.iter().find(|(id, _)| *id == app).map(|(_, v)| v.and_then(crate::version::parse))
}

/// What an app can do with this sign-in, given what it holds itself, which version is installed, and
/// what its latest release declares.
///
/// `own_key`: the key the app's own file holds, if it has one (that key always wins inside the app).
/// `installed`: the installed version when known; `None` also when the app is not installed, in which
/// case the version the launcher would install is the latest release, so a released reader counts.
/// `latest`: the latest release, whose `redrob-launcher.json` (when it has one) is the authority.
pub fn link_for(
    app: &str,
    own_key: Option<(bool, String)>,
    installed: Option<&crate::version::Version>,
    knows_key_file: bool,
    latest: Option<&crate::release::Release>,
) -> AppLink {
    if let Some((same_key, key_name)) = own_key {
        return AppLink::Connected { same_key, key_name };
    }
    if NO_KEY.contains(&app) {
        return AppLink::NoKey;
    }
    if let Some(r) = latest.filter(|r| r.manifest.is_some()) {
        // The release speaks for itself. Known shared-reading version: the latest one; an installed
        // copy at or past it reads the file, an older copy needs the update first.
        if r.reads_shared_account() {
            return match (installed, &r.version) {
                (Some(i), Some(v)) if i < v => AppLink::SharedAfterUpdate,
                _ => AppLink::Shared,
            };
        }
        return if knows_key_file { AppLink::NotConnected } else { AppLink::Unsupported };
    }
    match shared_since(app) {
        Some(Some(since)) if installed.is_none_or(|v| *v >= since) => AppLink::Shared,
        Some(_) if knows_key_file => AppLink::NotConnected,
        Some(_) => AppLink::SharedSoon,
        None if knows_key_file => AppLink::NotConnected,
        None => AppLink::Unsupported,
    }
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
    /// `installed`: the app's installed version when the caller knows it; `latest`: its latest release
    /// (whose manifest says what it can do). Together they decide whether the installed copy reads
    /// the shared file yet.
    fn app_link(&self, app: &str, installed: Option<&crate::version::Version>, latest: Option<&crate::release::Release>) -> AppLink;
    /// Write this computer's key into the app's own key file. `Err` says why not, in plain words.
    fn connect_app(&mut self, app: &str) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::parse;

    fn v(t: &str) -> crate::version::Version {
        parse(t).unwrap()
    }

    #[test]
    fn every_shared_reader_and_no_key_app_is_in_the_suite() {
        for (id, since) in SHARED_READERS {
            assert!(crate::CraftApp::by_id(id).is_some(), "{id}");
            if let Some(s) = since {
                assert!(parse(s).is_some(), "{id}: {s}");
            }
        }
        for id in NO_KEY {
            assert!(id == "redrob-launcher" || crate::CraftApp::by_id(id).is_some(), "{id}");
            assert!(shared_since(id).is_none(), "{id} cannot both read the file and need no key");
        }
    }

    #[test]
    fn an_apps_own_key_always_wins() {
        let l = link_for("redrob-cad", Some((true, "Redrob Launcher".into())), Some(&v("0.0.1")), true, None);
        assert_eq!(l, AppLink::Connected { same_key: true, key_name: "Redrob Launcher".into() });
    }

    #[test]
    fn a_released_reader_is_shared_from_its_version_on() {
        assert_eq!(link_for("redrob-cad", None, Some(&v("0.0.3")), true, None), AppLink::Shared);
        assert_eq!(link_for("redrob-cad", None, Some(&v("0.1.0")), true, None), AppLink::Shared);
        // Not installed: the launcher would install the latest release, which reads the file.
        assert_eq!(link_for("redrob-cad", None, None, true, None), AppLink::Shared);
        // An older copy with a key file of its own can still be handed the key.
        assert_eq!(link_for("redrob-cad", None, Some(&v("0.0.2")), true, None), AppLink::NotConnected);
        // An older copy with no key file the launcher knows: only an update helps.
        assert_eq!(link_for("redrob-shape", None, Some(&v("0.0.1")), false, None), AppLink::SharedSoon);
    }

    #[test]
    fn a_merged_but_unreleased_reader_says_so() {
        assert_eq!(link_for("redrob-office", None, None, false, None), AppLink::SharedSoon);
        assert_eq!(link_for("redrob-office", None, Some(&v("9.9.9")), false, None), AppLink::SharedSoon);
        // Redrob Code: unreleased reader, but the launcher knows its auth.json, so it can still connect.
        assert_eq!(link_for("redrob-code", None, None, true, None), AppLink::NotConnected);
    }

    fn release(version: &str, manifest: Option<crate::release::Manifest>) -> crate::release::Release {
        crate::release::Release { version: parse(version), tag: format!("v{version}"), manifest, ..Default::default() }
    }

    #[test]
    fn a_release_manifest_outranks_the_table() {
        let shared = Some(crate::release::Manifest { schema: 1, shared_account: true });
        let not = Some(crate::release::Manifest { schema: 1, shared_account: false });
        // Office is "unreleased" in the table, but its latest release says it reads the file.
        let r = release("0.9.1", shared.clone());
        assert_eq!(link_for("redrob-office", None, None, false, Some(&r)), AppLink::Shared);
        assert_eq!(link_for("redrob-office", None, Some(&v("0.9.1")), false, Some(&r)), AppLink::Shared);
        assert_eq!(link_for("redrob-office", None, Some(&v("0.9.0")), false, Some(&r)), AppLink::SharedAfterUpdate);
        // A manifest that says "no" is believed too, even where the table says yes.
        let r = release("0.0.4", not);
        assert_eq!(link_for("redrob-cad", None, Some(&v("0.0.4")), true, Some(&r)), AppLink::NotConnected);
        assert_eq!(link_for("redrob-shape", None, None, false, Some(&r)), AppLink::Unsupported);
        // No manifest on the release: the table decides, as before.
        let r = release("0.0.4", None);
        assert_eq!(link_for("redrob-cad", None, Some(&v("0.0.4")), true, Some(&r)), AppLink::Shared);
        // The app's own key still wins over everything.
        let r = release("0.9.1", shared);
        assert_eq!(
            link_for("redrob-office", Some((false, "k".into())), None, false, Some(&r)),
            AppLink::Connected { same_key: false, key_name: "k".into() }
        );
    }

    #[test]
    fn apps_without_a_console_key_and_unknown_apps() {
        assert_eq!(link_for("redrob-launcher", None, None, false, None), AppLink::NoKey);
        assert_eq!(link_for("redrob-studio", None, Some(&v("1.0.0")), false, None), AppLink::NoKey);
        assert_eq!(link_for("something-else", None, None, false, None), AppLink::Unsupported);
    }
}
