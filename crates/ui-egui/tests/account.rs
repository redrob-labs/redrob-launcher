// ADDED-LINE-RATIONALE 3834571c497db674:1: Supports `from_account_menu` while preserving the nearby constraint: above itself.
// ADDED-LINE-RATIONALE 24ab91c398c98f45:1: Freezes `row_rect` for `from_account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c2c2c4350fabfdec:1: Delegates `from_account_menu`'s `h / get_by_label / row` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ab984039ec5960b7:1: Delegates `from_account_menu`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3ce6f460b2ea90b9:1: Bounds the `from_account_menu` implementation here so `h / run_steps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7330fef8c817cd70:1: Supports `from_account_menu` while preserving the nearby constraint: The menu's item: the nearest one above the row (a page title can share its label).
// ADDED-LINE-RATIONALE 7a2bda03f99b9119:1: Delegates `from_account_menu`'s `get_all_by_label / item` work to `get_all_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8c433ac34fc2bc62:1: Delegates `from_account_menu`'s `filter / n / n` work to `filter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE cb0524886230fb8f:1: Delegates `from_account_menu`'s `max_by / a / b` work to `max_by` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE eebeb2476d79678f:1: Delegates `from_account_menu`'s `unwrap` work to `unwrap` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6d2ca946b666750e:1: Delegates `from_account_menu`'s `node / click` work to `node.click` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3ce6f660b2ea941f:13: Bounds the `from_account_menu` implementation here so `node / click` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ab984039ec5960b7:2: Delegates `from_account_menu`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 396d2fd6531d811f:7: Bounds the `from_account_menu` implementation here so `h / run_steps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE beb7f3278612e1e2:1: Keeps `redrob / cowork / AppLink` in `signing_in_shows_the_code_then_the_workspace_and_connects_apps`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE da43275dbb0fb03e:1: Supports `signing_in_shows_the_code_then_the_workspace_and_connects_apps` while preserving the nearby constraint: Signed out: the account row says so; its menu leads to the account page and its sign-in.
// ADDED-LINE-RATIONALE 88f21439d8b8f94b:1: Delegates `signing_in_shows_the_code_then_the_workspace_and_connects_apps`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c6566f875bf87c84:1: Supports `signing_in_shows_the_code_then_the_workspace_and_connects_apps` while preserving the nearby constraint: above is listed as unsupported; the hidden ones are not listed at all.
// ADDED-LINE-RATIONALE 752e1b2e92ba14fe:1: Makes `signing_in_shows_the_code_then_the_workspace_and_connects_apps` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE a828c80c1e80a855:1: Supports `signing_in_shows_the_code_then_the_workspace_and_connects_apps` while preserving the nearby constraint: A page, not a dialog: no Done; the back button and Escape lead to the apps.
// ADDED-LINE-RATIONALE 1e208c3c122c2618:1: Freezes `on_page` for `signing_in_shows_the_code_then_the_workspace_and_connects_apps` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b85aeb54a6566d85:1: Makes `signing_in_shows_the_code_then_the_workspace_and_connects_apps` fail visibly when `assert / on_page / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 38a251317f8ba746:1: Delegates `signing_in_shows_the_code_then_the_workspace_and_connects_apps`'s `h / get_by_label / Back` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ab984039ec5960b7:4: Delegates `signing_in_shows_the_code_then_the_workspace_and_connects_apps`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0a914ededfffae86:1: Makes `signing_in_shows_the_code_then_the_workspace_and_connects_apps` fail visibly when `assert / on_page / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE b262068c23179a1b:1: Supports `signing_in_shows_the_code_then_the_workspace_and_connects_apps` while preserving the nearby constraint: The sidebar's account row opens it again, through its menu (the fake still reports Acme).
// ADDED-LINE-RATIONALE df91363a09c97ab5:7: Delegates `signing_in_shows_the_code_then_the_workspace_and_connects_apps`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b85aeb54a6566d85:2: Makes `signing_in_shows_the_code_then_the_workspace_and_connects_apps` fail visibly when `assert / on_page / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE c82a4ed6140a3877:1: Delegates `signing_in_shows_the_code_then_the_workspace_and_connects_apps`'s `h / key_press / egui` work to `h.key_press` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ab984039ec5960b7:5: Delegates `signing_in_shows_the_code_then_the_workspace_and_connects_apps`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0a914ededfffae86:2: Makes `signing_in_shows_the_code_then_the_workspace_and_connects_apps` fail visibly when `assert / on_page / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE bb53b85c0143eda5:1: Makes `the_header_names_the_workspace_once_signed_in` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE b262068c23179a1b:2: Delegates `the_header_names_the_workspace_once_signed_in`'s `from_account_menu / h / Acme` work to `from_account_menu` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a7e6b2006ecdc131:3: Declares the `test` contract on `the_header_names_the_workspace_once_signed_in` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE e0131184474da0c8:1: Supports `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 420d3473b788fb19:1: Freezes `state` for `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 941c3007c4e1f8c5:2: Freezes `h` for `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ab984039ec5960b7:7: Delegates `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 51eb892098c94510:1: Supports `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` while preserving the nearby constraint: One row, no Settings row of its own.
// ADDED-LINE-RATIONALE 611e8842748caaf8:1: Makes `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 27ac942b6a74ce4c:1: Delegates `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings`'s `h / get_by_label / Log` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ab984039ec5960b7:8: Delegates `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 67866ba9afa5bd53:1: Delegates `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings`'s `snapshot / h / account` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 184a92b9c7714b75:1: Supports `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` while preserving the nearby constraint: Log in or sign up first, then Settings and Quit, then the version; all above the row.
// ADDED-LINE-RATIONALE e462c13451f1cc4a:1: Freezes `settings` for `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a6612102ea7326c0:1: Freezes `quit` for `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6d4e2a9f50b9186b:1: Freezes `assert` for `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6f396ad74ed4e495:1: Delegates `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings`'s `h / get_by_label / Settings` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ab984039ec5960b7:9: Delegates `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1a3938e9145182e4:1: Delegates `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings`'s `h / get_by_label / News` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b7d8f8bd97801e36:1: Supports `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` while preserving the nearby constraint: While the code waits for approval the row says so; signed in, it names the workspace.
// ADDED-LINE-RATIONALE ab984039ec5960b7:10: Delegates `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 97182e3b277348ab:1: Delegates `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings`'s `h / get_by_label / Log` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 07f39743e227d442:1: Freezes `state` for `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 42c00b78ad76713e:1: Freezes `state` for `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ab984039ec5960b7:11: Delegates `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ee56dc78745b9dc3:1: Delegates `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings`'s `from_account_menu / h / Acme` work to `from_account_menu` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1a3938e9145182e4:2: Delegates `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings`'s `h / get_by_label / News` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 396d2fd6531d811f:11: Bounds the `signed_out_the_row_reads_log_in_or_sign_up_and_its_menu_opens_settings` implementation here so `h / get_by_label / News` cannot leak into the adjacent declaration or control path.
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
