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
            ("redrob-cut", AppLink::Connected { same_key: true, key_name: "Redrob Launcher".into() }),
            ("redrob-shape", AppLink::Shared),
            ("redrob-office", AppLink::SharedSoon),
            ("redrob-query", AppLink::SharedAfterUpdate),
            ("redrob-studio", AppLink::NoKey),
        ],
        ..State::default()
    }));
    let mut h = harness(state.clone());
    h.run_steps(4);

    // Signed out: the header says so, and the dialog offers the sign-in.
    h.get_by_label("Sign in").click();
    h.run_steps(12);
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
    h.get_by_label("Needs no sign-in");
    // Every suite app without a link above is listed as unsupported.
    assert_eq!(h.query_all_by_label("Not supported yet").count(), redrob_launcher_core::SUITE.len() - 7);
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
}

#[test]
fn the_header_names_the_workspace_once_signed_in() {
    let state = Rc::new(RefCell::new(State {
        workspace: Some(Workspace { account_name: "Acme".into(), key_name: "Redrob Launcher".into() }),
        ..State::default()
    }));
    let mut h = harness(state);
    h.run_steps(4);
    assert!(h.query_by_label("Sign in").is_none());
    h.get_by_label("Acme").click();
    h.run_steps(3);
    h.get_by_label("Workspace: Acme");
}
