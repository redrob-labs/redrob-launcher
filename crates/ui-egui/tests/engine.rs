//! The engine dialog against a fake engine backend: each state's label and buttons, and the
//! requests they send.

use std::cell::RefCell;
use std::rc::Rc;

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use redrob_launcher_core::engine::{AttachedApp, EngineBackend, EngineState, RunningEngine};
use redrob_launcher_core::protocol::{Backend, Capabilities, Command, Event};
use redrob_launcher_core::{Arch, Os, Platform};
use redrob_launcher_ui_egui::{Host, LauncherUi, Settings};

#[derive(Default)]
struct State {
    state: Option<EngineState>,
    asked: Vec<String>,
    commands: Vec<Command>,
}

struct FakeEngine(Rc<RefCell<State>>);

impl EngineBackend for FakeEngine {
    fn state(&mut self) -> EngineState {
        self.0.borrow().state.clone().unwrap_or(EngineState::Stopped)
    }
    fn refresh(&mut self) {
        self.0.borrow_mut().asked.push("refresh".into());
    }
    fn start(&mut self) {
        self.0.borrow_mut().asked.push("start".into());
    }
    fn stop(&mut self) {
        self.0.borrow_mut().asked.push("stop".into());
    }
    fn restart(&mut self) {
        self.0.borrow_mut().asked.push("restart".into());
    }
    fn last_error(&self) -> Option<String> {
        None
    }
}

struct Fake(FakeEngine, Rc<RefCell<State>>);

impl Backend for Fake {
    fn send(&mut self, c: Command) {
        self.1.borrow_mut().commands.push(c);
    }
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
    fn engine(&mut self) -> Option<&mut dyn EngineBackend> {
        Some(&mut self.0)
    }
}

struct NoHost;
impl Host for NoHost {}

fn harness(state: Rc<RefCell<State>>) -> Harness<'static, Option<LauncherUi>> {
    let mut backend = Some(Fake(FakeEngine(state.clone()), state));
    Harness::builder().with_size(egui::vec2(1180.0, 1200.0)).build_ui_state(
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

fn running(attached: Vec<AttachedApp>, idle_for_secs: Option<u64>) -> EngineState {
    EngineState::Running(RunningEngine {
        version: "0.6.0".into(),
        url: "http://127.0.0.1:4096".into(),
        pid: 4242,
        attached,
        idle_for_secs,
        idle_exit_after_secs: Some(600),
    })
}

#[test]
fn a_running_engine_lists_its_apps_and_offers_stop_and_restart() {
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let state = Rc::new(RefCell::new(State {
        state: Some(running(vec![AttachedApp { app: "design".into(), pid: Some(7) }, AttachedApp { app: "redrob-cowork".into(), pid: None }], None)),
        ..State::default()
    }));
    let mut h = harness(state.clone());
    h.run_steps(4);
    // Two apps attached: the header counts them.
    h.get_by_label("Engine · 2").click();
    h.run_steps(12);
    h.get_by_label("Running");
    h.get_by_label("version 0.6.0");
    // The engine is a page now: it replaces the suite grid, so each attached app is listed once.
    assert_eq!(h.get_all_by_label("Redrob Design").count(), 1);
    assert_eq!(h.get_all_by_label("Redrob Cowork").count(), 1);
    if let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") {
        h.render().unwrap().save(std::path::Path::new(&dir).join("engine.png")).unwrap();
    }
    // The footer's actions and the apps' pids share the card's trailing edge, Stop before Restart.
    let restart = h.get_by_label("Restart").rect();
    let pid = h.get_by_label("pid 7").rect();
    assert!((restart.right() - pid.right()).abs() <= 1.0, "restart {restart:?} pid {pid:?}");
    assert!(h.get_by_label("Stop").rect().right() < restart.left());
    // The attached apps are list rows, not 65 px bands.
    let design = h.get_by_label("Redrob Design").rect();
    let cowork = h.get_by_label("Redrob Cowork").rect();
    assert!(cowork.center().y - design.center().y < 44.0, "app row pitch {}", cowork.center().y - design.center().y);
    h.get_by_label("Restart").click();
    h.run_steps(4);
    h.get_by_label("Stop").click();
    h.run_steps(4);
    assert_eq!(state.borrow().asked, ["refresh", "restart", "stop"]);
    // A page, not a dialog: no Done; Escape and the back button lead to the apps.
    assert!(h.query_by_label("Done").is_none());
    h.key_press(egui::Key::Escape);
    h.run_steps(4);
    assert!(h.query_by_label("Restart").is_none());
    h.get_by_label("Engine · 2").click();
    h.run_steps(8);
    h.get_by_label("Back to apps").click();
    h.run_steps(4);
    assert!(h.query_by_label("Restart").is_none());
}

#[test]
fn an_idle_engine_says_when_it_stops_and_a_stopped_one_can_be_started() {
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let state = Rc::new(RefCell::new(State { state: Some(running(vec![], Some(300))), ..State::default() }));
    let mut h = harness(state.clone());
    h.run_steps(4);
    h.get_by_label("Engine").click();
    h.run_steps(12);
    h.get_by_label("No app is using it. It stops on its own in about 5 min.");
    if let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") {
        h.render().unwrap().save(std::path::Path::new(&dir).join("engine-idle.png")).unwrap();
    }

    state.borrow_mut().state = Some(EngineState::Stopped);
    h.run_steps(8);
    h.get_by_label("Not running");
    if let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") {
        h.render().unwrap().save(std::path::Path::new(&dir).join("engine-stopped.png")).unwrap();
    }
    h.get_by_label("Start").click();
    h.run_steps(4);
    assert_eq!(state.borrow().asked, ["refresh", "start"]);
}

#[test]
fn without_the_engine_installed_the_dialog_installs_redrob_code() {
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let state = Rc::new(RefCell::new(State { state: Some(EngineState::NotInstalled), ..State::default() }));
    let mut h = harness(state.clone());
    h.run_steps(4);
    h.get_by_label("Engine").click();
    h.run_steps(12);
    h.get_by_label("Not installed");
    if let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") {
        h.render().unwrap().save(std::path::Path::new(&dir).join("engine-not-installed.png")).unwrap();
    }
    h.get_by_label("Install Redrob Code").click();
    h.run_steps(4);
    assert!(state.borrow().commands.contains(&Command::Install { app: "redrob-code".into() }));
}
