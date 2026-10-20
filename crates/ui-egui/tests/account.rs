//! The account dialog against a fake account backend: signing in, the code to approve, the
//! signed-in workspace, and handing the key to an app.

use std::cell::RefCell;
use std::rc::Rc;

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use redrob_launcher_core::account::{AccountBackend, AppLink, ConnectState, PendingConnect, Workspace};
use redrob_launcher_core::protocol::{Backend, Capabilities, Command, Event};
use redrob_launcher_core::{Arch, Os, Platform};
use redrob_launcher_ui_egui::{Host, LauncherUi, Settings};

/// What the fake account was asked to do.
#[derive(Debug, PartialEq, Eq)]
enum Asked {
    Connect,
    Cancel,
    Disconnect,
    ConnectApp(String),
}

#[derive(Default)]
struct State {
    workspace: Option<Workspace>,
    connect: Option<ConnectState>,
    links: Vec<(&'static str, AppLink)>,
    asked: Vec<Asked>,
}

struct FakeAccount(Rc<RefCell<State>>);

impl AccountBackend for FakeAccount {
    fn workspace(&self) -> Option<Workspace> {
        self.0.borrow().workspace.clone()
    }
    fn state(&mut self) -> ConnectState {
        self.0.borrow().connect.clone().unwrap_or(ConnectState::Idle)
    }
    fn connect(&mut self) {
        self.0.borrow_mut().asked.push(Asked::Connect);
    }
    fn cancel(&mut self) {
        self.0.borrow_mut().asked.push(Asked::Cancel);
    }
    fn disconnect(&mut self) {
        self.0.borrow_mut().asked.push(Asked::Disconnect);
    }
    fn app_link(
        &self,
        app: &str,
        _installed: Option<&redrob_launcher_core::version::Version>,
        _latest: Option<&redrob_launcher_core::release::Release>,
    ) -> AppLink {
        self.0.borrow().links.iter().find(|(a, _)| *a == app).map_or(AppLink::Unsupported, |(_, l)| l.clone())
    }
    fn connect_app(&mut self, app: &str) -> Result<(), String> {
        self.0.borrow_mut().asked.push(Asked::ConnectApp(app.into()));
        Ok(())
    }
}

struct Fake {
    account: FakeAccount,
}

impl Backend for Fake {
    fn send(&mut self, _: Command) {}
    fn poll(&mut self) -> Vec<Event> {
        Vec::new()
    }
    fn capabilities(&self) -> Capabilities {
        Capabilities { manage_apps: true, reveal_label: "Show in Finder", self_update: false }
    }
    fn platform(&self) -> Option<Platform> {
        Some(Platform { os: Os::MacOs, arch: Arch::Arm64 })
    }
    fn now(&self) -> i64 {
        1_791_500_000
    }
    fn account(&mut self) -> Option<&mut dyn AccountBackend> {
        Some(&mut self.account)
    }
}

struct NoHost;
impl Host for NoHost {}

fn harness(state: Rc<RefCell<State>>) -> Harness<'static, Option<LauncherUi>> {
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let mut backend = Some(Fake { account: FakeAccount(state) });
    Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, s: &mut Option<LauncherUi>| {
            if s.is_none()
                && let Some(b) = backend.take()
            {
                *s = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
            }
            if let Some(l) = s.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    )
}

/// Clicks the sidebar's account row labelled `row`, then the item `item` of the menu it opens
/// above itself.
fn from_account_menu(h: &mut Harness<'_, Option<LauncherUi>>, row: &str, item: &str) {
    let row_rect = h.get_by_label(row).rect();
    h.get_by_label(row).click();
    h.run_steps(4);
    {
        // The menu's item: the nearest one above the row (a page title can share its label).
        let node = h
            .get_all_by_label(item)
            .filter(|n| n.rect().bottom() <= row_rect.top() + 0.5)
            .max_by(|a, b| a.rect().bottom().total_cmp(&b.rect().bottom()))
            .unwrap();
        node.click();
    }
    h.run_steps(4);
}

fn snapshot(h: &mut Harness<'_, Option<LauncherUi>>, name: &str) {
    let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") else { return };
    let img = h.render().unwrap();
    img.save(std::path::Path::new(&dir).join(format!("{name}.png"))).unwrap();
}

#[test]
fn signing_in_shows_the_code_then_the_workspace_and_connects_apps() {
    let state = Rc::new(RefCell::new(State {
        links: vec![
            ("redrob-cad", AppLink::NotConnected),
            ("redrob-code", AppLink::Connected { same_key: false, key_name: String::new() }),
            ("redrob-film", AppLink::Connected { same_key: true, key_name: "Redrob Launcher".into() }),
            ("redrob-shape", AppLink::Shared),
            ("redrob-office", AppLink::SharedSoon),
            ("redrob-design", AppLink::SharedAfterUpdate),
        ],
        ..State::default()
    }));
    let mut h = harness(state.clone());
    h.run_steps(4);

    // Signed out: the account row says so; its menu leads to the account page and its sign-in.
    from_account_menu(&mut h, "Log in or sign up", "Log in or sign up");
    h.run_steps(8);
    h.get_by_label("Redrob account");
    h.get_by_label("Sign in to Redrob").click();
    h.run_steps(2);
    assert_eq!(state.borrow().asked, [Asked::Connect]);
    // App buttons wait for a sign-in.
    h.get_by_label("Connect").click();
    h.run_steps(2);
    assert_eq!(state.borrow().asked, [Asked::Connect]);

    // Waiting: the code to approve, with a way out.
    state.borrow_mut().connect = Some(ConnectState::Waiting(PendingConnect {
        user_code: "ABCD-EFGH".into(),
        url: "https://console.redrob.ai/connect?code=ABCD-EFGH".into(),
        seconds_left: 599,
    }));
    h.run_steps(12);
    h.get_by_label("ABCD-EFGH");
    h.get_by_label("9:59 left");
    snapshot(&mut h, "account-waiting");
    h.get_by_label("Cancel").click();
    h.run_steps(2);
    assert_eq!(state.borrow().asked.last(), Some(&Asked::Cancel));

    // Signed in: the workspace, the key's name, and each app's state.
    {
        let mut s = state.borrow_mut();
        s.connect = None;
        s.workspace = Some(Workspace { account_name: "Acme".into(), key_name: "Redrob Launcher".into() });
    }
    h.run_steps(12);
    h.get_by_label("Workspace: Acme");
    h.get_by_label("Key: Redrob Launcher");
    h.get_by_label("Connected");
    h.get_by_label("Connected with another key");
    h.get_by_label("Uses this sign-in");
    h.get_by_label("Next release uses this sign-in");
    h.get_by_label("Update to use this sign-in");
    // Every app the Apps page shows without a link above is listed as unsupported.
    assert_eq!(h.query_all_by_label("Not supported yet").count(), redrob_launcher_core::SUITE.len() - 6);
    snapshot(&mut h, "account-signed-in");
    h.get_by_label("Connect").click();
    h.run_steps(2);
    assert_eq!(state.borrow().asked.last(), Some(&Asked::ConnectApp("redrob-cad".into())));
    h.get_by_label_contains("Redrob CAD is connected");
    h.get_by_label("Use this sign-in").click();
    h.run_steps(2);
    assert_eq!(state.borrow().asked.last(), Some(&Asked::ConnectApp("redrob-code".into())));
    h.get_by_label("Sign out").click();
    h.run_steps(2);
    assert_eq!(state.borrow().asked.last(), Some(&Asked::Disconnect));
    // A page, not a dialog: no Done; the back button and Escape lead to the apps.
    assert!(h.query_by_label("Done").is_none());
    let on_page = |h: &egui_kittest::Harness<'_, _>| h.query_by_label("Sign out").is_some() || h.query_by_label("Sign in to Redrob").is_some();
    assert!(on_page(&h));
    h.get_by_label("Back to apps").click();
    h.run_steps(4);
    assert!(!on_page(&h));
    // The sidebar's account row opens it again, through its menu (the fake still reports Acme).
    from_account_menu(&mut h, "Acme", "Redrob account");
    h.run_steps(2);
    assert!(on_page(&h));
    h.key_press(egui::Key::Escape);
    h.run_steps(4);
    assert!(!on_page(&h));
}

#[test]
fn the_header_names_the_workspace_once_signed_in() {
    let state = Rc::new(RefCell::new(State {
        workspace: Some(Workspace { account_name: "Acme".into(), key_name: "Redrob Launcher".into() }),
        ..State::default()
    }));
    let mut h = harness(state);
    h.run_steps(4);
    assert!(h.query_by_label("Log in or sign up").is_none());
    from_account_menu(&mut h, "Acme", "Redrob account");
    h.get_by_label("Workspace: Acme");
}

#[test]
fn signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings() {
    let state = Rc::new(RefCell::new(State::default()));
    let mut h = harness(state.clone());
    h.run_steps(4);
    // One row, no Settings row of its own.
    let row = h.get_by_label("Log in or sign up").rect();
    assert!(h.query_by_label("Settings").is_none());
    h.get_by_label("Log in or sign up").click();
    h.run_steps(4);
    snapshot(&mut h, "account-menu-signed-out");
    // Log in or sign up first, then Settings and Quit, then the version; all above the row.
    let login = h.get_all_by_label("Log in or sign up").map(|n| n.rect()).find(|r| r.bottom() <= row.top() + 0.5).unwrap();
    let settings = h.get_by_label("Settings").rect();
    let quit = h.get_by_label("Quit Redrob Launcher").rect();
    assert!(login.top() < settings.top() && settings.top() < quit.top() && quit.bottom() <= row.top() + 0.5);
    h.get_by_label("Settings").click();
    h.run_steps(4);
    h.get_by_label("News and source code for every Redrob app");
    // While the code waits for approval the row says so; signed in, it names the workspace.
    state.borrow_mut().connect = Some(ConnectState::Starting);
    h.run_steps(4);
    h.get_by_label("Log in or sign up");
    state.borrow_mut().connect = None;
    state.borrow_mut().workspace = Some(Workspace { account_name: "Acme".into(), key_name: String::new() });
    h.run_steps(4);
    from_account_menu(&mut h, "Acme", "Settings");
    h.get_by_label("News and source code for every Redrob app");
}
