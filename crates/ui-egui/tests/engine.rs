// ADDED-LINE-RATIONALE 250e5756265617a5:1: Gives `engine` direct access to `redrob_launcher_core / protocol / Backend` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 1bc576a9a6f3b1e8:1: Gives `engine` direct access to `redrob_launcher_core / release / Asset` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE b15bb0dff4935232:1: Gives `engine` direct access to `redrob_launcher_core / status / InstallSource` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 9af8fb73d9b74f9d:1: Gives `engine` direct access to `redrob_launcher_core / Arch / Installed` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 3de81395a1c6a0e6:1: Supports `State` while preserving the nearby constraint: Catalog events for the first poll (e.g. an installed engine and a newer release).
// ADDED-LINE-RATIONALE 97553ef14a968992:1: Delegates `poll`'s `std / mem / take` work to `std::mem::take` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6987605c246883bf:1: Supports `a_running_engine_lists_its_apps_and_offers_stop_and_restart` while preserving the nearby constraint: Two apps attached: the sidebar row says the engine is in use.
// ADDED-LINE-RATIONALE b0a554aedd09fde4:1: Supports `a_running_engine_lists_its_apps_and_offers_stop_and_restart` while preserving the nearby constraint: The engine is a page now: it replaces the suite grid, so each attached app is listed once.
// ADDED-LINE-RATIONALE 47cef389627ae3ef:1: Makes `a_running_engine_lists_its_apps_and_offers_stop_and_restart` fail visibly when `assert_eq / h / get_all_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE a2cfb4982a3fb88b:1: Supports `a_running_engine_lists_its_apps_and_offers_stop_and_restart` while preserving the nearby constraint: The footer's actions and the apps' pids share the card's trailing edge, Stop before Restart.
// ADDED-LINE-RATIONALE b1231ee8c4e6480e:1: Freezes `pid` for `a_running_engine_lists_its_apps_and_offers_stop_and_restart` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 061496c1d4c30c41:1: Freezes `assert` for `a_running_engine_lists_its_apps_and_offers_stop_and_restart` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3544835e8e852ea3:1: Makes `a_running_engine_lists_its_apps_and_offers_stop_and_restart` fail visibly when `assert / h / get_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 73858ed0aa1f2e09:1: Supports `a_running_engine_lists_its_apps_and_offers_stop_and_restart` while preserving the nearby constraint: The attached apps are list rows, not 65 px bands.
// ADDED-LINE-RATIONALE 9e9b4dd3ff0cd505:1: Freezes `cowork` for `a_running_engine_lists_its_apps_and_offers_stop_and_restart` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ef397ebf2caffff3:1: Makes `a_running_engine_lists_its_apps_and_offers_stop_and_restart` fail visibly when `assert / cowork / center` violates the behavior this path promises.
// ADDED-LINE-RATIONALE bd64e12fdeedfc54:1: Supports `a_running_engine_lists_its_apps_and_offers_stop_and_restart` while preserving the nearby constraint: A page, not a dialog: no Done; Escape and the back button lead to the apps.
// ADDED-LINE-RATIONALE d337c611be97aa10:1: Delegates `a_running_engine_lists_its_apps_and_offers_stop_and_restart`'s `h / key_press / egui` work to `h.key_press` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6f744418c3c4cd50:4: Delegates `a_running_engine_lists_its_apps_and_offers_stop_and_restart`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4db1a7da17d3f6f1:1: Makes `a_running_engine_lists_its_apps_and_offers_stop_and_restart` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 6987605c246883bf:2: Delegates `a_running_engine_lists_its_apps_and_offers_stop_and_restart`'s `h / get_by_label / AI` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d83f3818ff5cffa4:1: Delegates `a_running_engine_lists_its_apps_and_offers_stop_and_restart`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 47b05acee09bcd55:1: Delegates `a_running_engine_lists_its_apps_and_offers_stop_and_restart`'s `h / get_by_label / Back` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6f744418c3c4cd50:5: Delegates `a_running_engine_lists_its_apps_and_offers_stop_and_restart`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4db1a7da17d3f6f1:2: Makes `a_running_engine_lists_its_apps_and_offers_stop_and_restart` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE d4b89abb9f1c432e:1: Delegates `an_idle_engine_says_when_it_stops_and_a_stopped_one_can_be_started`'s `h / get_by_label / AI` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c78f90ed92c6f1b1:2: Freezes `if` for `an_idle_engine_says_when_it_stops_and_a_stopped_one_can_be_started` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 244741a921f460cb:1: Delegates `an_idle_engine_says_when_it_stops_and_a_stopped_one_can_be_started`'s `h / render / unwrap` work to `h.render` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 18adcb5505067674:14: Bounds the `an_idle_engine_says_when_it_stops_and_a_stopped_one_can_be_started` implementation here so `h / render / unwrap` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60220930a1db5abf:1: Supports `an_idle_engine_says_when_it_stops_and_a_stopped_one_can_be_started` while preserving the nearby constraint: Stopped is its resting state: the row says so plainly, not "off".
// ADDED-LINE-RATIONALE c78f90ed92c6f1b1:3: Freezes `if` for `an_idle_engine_says_when_it_stops_and_a_stopped_one_can_be_started` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE eae1cd8adcea0bbc:1: Delegates `an_idle_engine_says_when_it_stops_and_a_stopped_one_can_be_started`'s `h / render / unwrap` work to `h.render` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 18adcb5505067674:15: Bounds the `an_idle_engine_says_when_it_stops_and_a_stopped_one_can_be_started` implementation here so `h / render / unwrap` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 16c5b07d05a03f25:1: Supports `without_the_engine_installed_the_page_installs_redrob_code` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 4a7b5fae66309636:1: Delegates `without_the_engine_installed_the_page_installs_redrob_code`'s `h / get_by_label / AI` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c78f90ed92c6f1b1:4: Freezes `if` for `without_the_engine_installed_the_page_installs_redrob_code` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 562833658ef3693d:1: Delegates `without_the_engine_installed_the_page_installs_redrob_code`'s `h / render / unwrap` work to `h.render` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 18adcb5505067674:16: Bounds the `without_the_engine_installed_the_page_installs_redrob_code` implementation here so `h / render / unwrap` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8cc4ea56c8a3aebd:1: Supports `engine_versions` while preserving the nearby constraint: Redrob Code `installed` on this Mac, and `latest` released.
// ADDED-LINE-RATIONALE 5d2dde1c66395158:1: Freezes `name` for `engine_versions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 95632a1cbfc973f0:1: Freezes `release` for `engine_versions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 02f4a888fa6f9311:1: Keeps `tag / format / v` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 24fddca65926274c:1: Keeps `version / version / parse` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 62d330d891aabfaa:1: Keeps `format / v / latest` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 34af17a6f7aac597:1: Keeps `published_at / _791_400_000` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 3492c340f1bc2e7d:1: Keeps `html_url / format / https` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 264f98bc4f75380e:1: Keeps `notes / String / new` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 866db6ffab5066f8:1: Keeps `prerelease` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE ff9d3200110678ad:1: Delegates `engine_versions`'s `assets / vec / Asset` work to `vec` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e9174fc83cc427f1:1: Keeps `clone` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 02b25afa7f9a8264:1: Keeps `size` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 72bceae6a7d2b3a9:1: Keeps `url / format / https` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE c4e6c79c1dd0f856:1: Keeps `sha256` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 3ab82150734f5495:1: Keeps `sha256` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 1fb7887b002bb38c:1: Keeps `manifest` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE f5c6d07789fb083d:1: Bounds the `engine_versions` implementation here so `manifest` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 4f62fb7c4bfa9179:1: Delegates `engine_versions`'s `vec` work to `vec` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e8fa54bee1a94cc1:1: Keeps `Event / Latest / app` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 376c86c959fbb494:1: Makes `Event / Detected` an explicit part of `engine_versions` because the regression suite can pin the refreshed behavior before release.
// ADDED-LINE-RATIONALE 3212c6a0cc0c483b:1: Keeps `app / redrob / code` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 1289666ed26afe94:1: Delegates `engine_versions`'s `installed / Installed` work to `Some` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 92898613b5ea28b5:1: Keeps `version / version / parse` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 2580093d9d872292:1: Keeps `source / InstallSource / MacApp` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 214919d40f7e4640:1: Keeps `location / Applications / Redrob` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE c7c4ee6e02ba117d:1: Keeps `package` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 80d2a0f25479acf1:1: Keeps `package` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE d3affb74b5860978:2: Keeps `package` in `engine_versions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 18adab5505064014:1: Bounds the `engine_versions` implementation here so `package` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 81f7a190a85ae724:9: Bounds the `engine_versions` implementation here so `test` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0db1dff71fa4043a:4: Declares the `test` contract on `engine_versions` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 0f8e0b819ab69ef6:1: Supports `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 246d1a0d3e4a514a:4: Delegates `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install`'s `redrob_launcher_ui_egui / i18n / set_test_lang` work to `redrob_launcher_ui_egui::i18n::set_test_lang` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 984dfcc95aa101f8:1: Freezes `state` for `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 364e383114384b9e:1: Delegates `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install`'s `Rc / new / RefCell` work to `Rc::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 63a6a34367087a56:4: Freezes `h` for `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6f744418c3c4cd50:10: Delegates `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d4b89abb9f1c432e:2: Delegates `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install`'s `h / get_by_label / AI` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 86ee3c3f0b49f7b3:4: Delegates `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c78f90ed92c6f1b1:5: Freezes `if` for `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 62d596c0d880dcf6:1: Delegates `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install`'s `h / render / unwrap` work to `h.render` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 18adcb5505067674:17: Bounds the `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install` implementation here so `h / render / unwrap` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f4e1d6c472c7174c:1: Supports `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install` while preserving the nearby constraint: Its own alert, under the status card: the versions, what happens, and Update now.
// ADDED-LINE-RATIONALE 39f7d35347eb2f83:1: Delegates `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install`'s `h / get_by_label` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6e52ac1c5c93b6ef:1: Delegates `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install`'s `h / get_by_label / It` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2e1bdac028d3769d:1: Freezes `update` for `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f6ddcd7950d70c58:1: Makes `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install` fail visibly when `assert / update / top` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 0753f17882143972:1: Supports `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install` while preserving the nearby constraint: Update now stops the idle engine first (then installs and starts it again).
// ADDED-LINE-RATIONALE 6f744418c3c4cd50:11: Delegates `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 544aac3f6f4d800f:1: Makes `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install` fail visibly when `assert / state / borrow` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 81f7a190a85ae724:10: Bounds the `a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install` implementation here so `assert / state / borrow` cannot leak into the adjacent declaration or control path.
//! The AI engine page against a fake engine backend: each state's label and buttons, and the
//! requests they send.

use std::cell::RefCell;
use std::rc::Rc;

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use redrob_launcher_core::engine::{AttachedApp, EngineBackend, EngineState, RunningEngine};
use redrob_launcher_core::protocol::{Backend, Capabilities, Command, Event, LatestRelease, Origin};
use redrob_launcher_core::release::{Asset, Release};
use redrob_launcher_core::status::InstallSource;
use redrob_launcher_core::{Arch, Installed, Os, Platform, version};
use redrob_launcher_ui_egui::{Host, LauncherUi, Settings};

#[derive(Default)]
struct State {
    state: Option<EngineState>,
    asked: Vec<String>,
    commands: Vec<Command>,
    /// Catalog events for the first poll (e.g. an installed engine and a newer release).
    events: Vec<Event>,
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
        std::mem::take(&mut self.1.borrow_mut().events)
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
    // Two apps attached: the sidebar row says the engine is in use.
    h.get_by_label("AI engine: In use").click();
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
    h.get_by_label("AI engine: In use").click();
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
    h.get_by_label("AI engine: Ready").click();
    h.run_steps(12);
    h.get_by_label("No app is using it. It stops on its own in about 5 min.");
    if let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") {
        h.render().unwrap().save(std::path::Path::new(&dir).join("engine-idle.png")).unwrap();
    }

    state.borrow_mut().state = Some(EngineState::Stopped);
    h.run_steps(8);
    h.get_by_label("Not running");
    // Stopped is its resting state: the row says so plainly, not "off".
    h.get_by_label("AI engine: Standby");
    if let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") {
        h.render().unwrap().save(std::path::Path::new(&dir).join("engine-stopped.png")).unwrap();
    }
    h.get_by_label("Start").click();
    h.run_steps(4);
    assert_eq!(state.borrow().asked, ["refresh", "start"]);
}

#[test]
fn without_the_engine_installed_the_page_installs_redrob_code() {
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let state = Rc::new(RefCell::new(State { state: Some(EngineState::NotInstalled), ..State::default() }));
    let mut h = harness(state.clone());
    h.run_steps(4);
    h.get_by_label("AI engine: Not installed").click();
    h.run_steps(12);
    h.get_by_label("Not installed");
    if let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") {
        h.render().unwrap().save(std::path::Path::new(&dir).join("engine-not-installed.png")).unwrap();
    }
    h.get_by_label("Install Redrob Code").click();
    h.run_steps(4);
    assert!(state.borrow().commands.contains(&Command::Install { app: "redrob-code".into() }));
}

/// Redrob Code `installed` on this Mac, and `latest` released.
fn engine_versions(installed: &str, latest: &str) -> Vec<Event> {
    let name = format!("redrob-code-{latest}-macos-universal.dmg");
    let release = Release {
        tag: format!("v{latest}"),
        version: version::parse(latest),
        name: format!("v{latest}"),
        published_at: Some(1_791_400_000),
        html_url: format!("https://github.com/redrob-labs/redrob-code/releases/tag/v{latest}"),
        notes: String::new(),
        prerelease: false,
        assets: vec![Asset {
            name: name.clone(),
            size: 80 << 20,
            url: format!("https://github.com/redrob-labs/redrob-code/releases/download/v{latest}/{name}"),
            sha256: None,
        }],
        manifest: None,
    };
    vec![
        Event::Latest { app: "redrob-code".into(), latest: LatestRelease::Found(release), from: Origin::RedrobApi },
        Event::Detected {
            app: "redrob-code".into(),
            installed: Some(Installed {
                version: version::parse(installed),
                source: InstallSource::MacApp,
                location: "/Applications/Redrob Code.app".into(),
                package: None,
            }),
        },
    ]
}

#[test]
fn a_newer_engine_shows_its_own_alert_and_update_now_stops_it_to_install() {
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let state =
        Rc::new(RefCell::new(State { state: Some(running(vec![], Some(30))), events: engine_versions("0.5.0", "0.6.0"), ..State::default() }));
    let mut h = harness(state.clone());
    h.run_steps(4);
    h.get_by_label("AI engine: Ready").click();
    h.run_steps(12);
    if let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") {
        h.render().unwrap().save(std::path::Path::new(&dir).join("engine-update.png")).unwrap();
    }
    // Its own alert, under the status card: the versions, what happens, and Update now.
    h.get_by_label("Update available");
    h.get_by_label("0.5.0 → 0.6.0");
    h.get_by_label("It updates on its own when no app is using it. You can also update it now.");
    let update = h.get_by_label("Update now").rect();
    assert!(update.top() > h.get_by_label("Restart").rect().bottom(), "the alert sits below the card's footer");
    // Update now stops the idle engine first (then installs and starts it again).
    h.get_by_label("Update now").click();
    h.run_steps(4);
    assert!(state.borrow().asked.contains(&"stop".to_string()), "{:?}", state.borrow().asked);
}
