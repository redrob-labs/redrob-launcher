//! The models dialog against a fake model backend: every state's label and buttons, and the
//! requests they send.

use std::cell::RefCell;
use std::rc::Rc;

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use redrob_launcher_core::device::{Device, Fit, Gpu, GpuVendor};
use redrob_launcher_core::models::{Model, ModelBackend, ModelState, ModelView, Runtime, RuntimeState, Serving, builtin};
use redrob_launcher_core::protocol::{Backend, Capabilities, Command, Event};
use redrob_launcher_core::{Arch, Os, Platform};
use redrob_launcher_ui_egui::{Host, LauncherUi, Settings};

#[derive(Default)]
struct State {
    views: Vec<ModelView>,
    asked: Vec<String>,
}

struct FakeModels(Rc<RefCell<State>>);

impl ModelBackend for FakeModels {
    fn models(&self) -> Vec<ModelView> {
        self.0.borrow().views.clone()
    }
    fn download(&mut self, app: &str, id: &str) {
        self.0.borrow_mut().asked.push(format!("download {app}/{id}"));
    }
    fn pause(&mut self, app: &str, id: &str) {
        self.0.borrow_mut().asked.push(format!("pause {app}/{id}"));
    }
    fn delete(&mut self, app: &str, id: &str) -> Result<(), String> {
        self.0.borrow_mut().asked.push(format!("delete {app}/{id}"));
        Ok(())
    }
    fn device(&self) -> Device {
        Device {
            os: Os::MacOs,
            arch: Arch::Arm64,
            cpu: "Apple M3 Pro".into(),
            cores: 12,
            ram_mb: 36 * 1024,
            gpu: Some(Gpu { name: "Apple M3 Pro".into(), vendor: GpuVendor::Apple, memory_mb: None, unified: true }),
            npu: None,
            gpu_probed: true,
        }
    }
    fn remeasure(&mut self) {
        self.0.borrow_mut().asked.push("remeasure".into());
    }
    fn runtime(&self, runtime: Runtime) -> RuntimeState {
        if runtime.served() { RuntimeState::NotInstalled } else { RuntimeState::Installed }
    }
    fn install_runtime(&mut self, _: Runtime) {}
    fn start(&mut self, app: &str, id: &str) {
        self.0.borrow_mut().asked.push(format!("start {app}/{id}"));
    }
    fn stop(&mut self, app: &str, id: &str) {
        self.0.borrow_mut().asked.push(format!("stop {app}/{id}"));
    }
    fn disk_free(&self) -> Option<u64> {
        Some(120 << 30)
    }
}

struct Fake(FakeModels);

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
    fn models(&mut self) -> Option<&mut dyn ModelBackend> {
        Some(&mut self.0)
    }
}

struct NoHost;
impl Host for NoHost {}

fn view(m: &Model, state: ModelState) -> ModelView {
    ModelView {
        model: m.clone(),
        state,
        folder: Some(format!("/home/u/.local/share/redrob-film/models/{}", m.id)),
        fit: Fit::Runs,
        serving: Serving::No,
    }
}

/// A harness on the given rows; the test opens the Models page from the header. kittest cannot
/// scroll, so the window is tall and each test shows only the rows it clicks.
fn open(views: Vec<ModelView>) -> (Harness<'static, Option<LauncherUi>>, Rc<RefCell<State>>) {
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let state = Rc::new(RefCell::new(State { views, asked: Vec::new() }));
    let mut backend = Some(Fake(FakeModels(state.clone())));
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 1500.0)).build_ui_state(
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
    );
    h.run_steps(4);
    (h, state)
}

#[test]
fn every_download_state_offers_the_right_action() {
    let m = builtin();
    let (mut h, state) = open(vec![
        view(&m[1], ModelState::Downloading { done: 100 << 20, total: 277 << 20 }),
        view(&m[2], ModelState::Partial { done: 300 << 20, total: 924 << 20 }),
        view(&m[3], ModelState::NotDownloaded),
    ]);
    // A download is running, so the header says so.
    h.get_by_label("Models…").click();
    h.run_steps(12);
    // Plain words first, the model's name small under them.
    h.get_by_label("Subtitles for most videos, a good balance of speed and accuracy");
    h.get_by_label("Whisper base (multilingual)");
    h.get_by_label("Nine natural English narration voices");
    h.get_by_label("Speech to text");
    h.get_by_label("Installing 100 MB of 277 MB");
    h.get_by_label("300 MB of 924 MB installed");
    h.get_by_label("Not installed");
    h.get_by_label("Only show what runs on this computer");
    h.get_by_label("Apple M3 Pro, 36 GB unified memory");
    h.get_all_by_label("Runs here").next().unwrap();
    if let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") {
        h.render().unwrap().save(std::path::Path::new(&dir).join("models.png")).unwrap();
    }
    h.get_by_label("Pause").click();
    h.run_steps(8);
    h.get_by_label("Resume").click();
    h.run_steps(8);
    h.get_by_label("Install").click();
    h.run_steps(8);
    h.get_by_label("Measure again").click();
    h.run_steps(8);
    h.get_all_by_label("Delete").next().unwrap().click();
    h.run_steps(8);
    assert_eq!(
        state.borrow().asked,
        [
            "pause redrob-film/whisper-base",
            "download redrob-film/whisper-small",
            "download redrob-film/kokoro-82m",
            "remeasure",
            "delete redrob-film/whisper-small"
        ]
    );
}

#[test]
fn served_models_show_their_verdict_and_start_or_stop() {
    let m = builtin();
    let qwen = m.iter().find(|x| x.id == "qwen3.8-27b-q4").unwrap().clone();
    let coder = m.iter().find(|x| x.id == "qwen3-coder-30b-a3b-q4").unwrap().clone();
    let (mut h, state) = open(vec![
        ModelView { serving: Serving::Running { url: "http://127.0.0.1:4100".into() }, fit: Fit::Slow, ..view(&qwen, ModelState::Installed) },
        ModelView { fit: Fit::WontRun, ..view(&coder, ModelState::Installed) },
    ]);
    h.get_by_label("Models").click();
    h.run_steps(12);
    h.get_all_by_label("for Every app").next().unwrap();
    h.get_by_label("llama-server is installed on first start");
    h.get_by_label("Serving at http://127.0.0.1:4100");
    h.get_by_label("Installed");
    h.get_by_label("Runs slowly");
    h.get_by_label("Won't run here");
    if let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") {
        h.render().unwrap().save(std::path::Path::new(&dir).join("models-served.png")).unwrap();
    }
    h.get_by_label("Stop").click();
    h.run_steps(8);
    h.get_by_label("Start").click();
    h.run_steps(8);
    h.get_all_by_label("Delete").next().unwrap().click();
    h.run_steps(8);
    assert_eq!(state.borrow().asked, ["stop redrob/qwen3.8-27b-q4", "start redrob/qwen3-coder-30b-a3b-q4", "delete redrob/qwen3.8-27b-q4"]);
}

#[test]
fn every_row_starts_its_verdict_on_one_edge_and_ends_its_actions_on_another() {
    let m = builtin();
    let (mut h, _) = open(vec![
        view(&m[1], ModelState::Downloading { done: 100 << 20, total: 277 << 20 }),
        view(&m[2], ModelState::Partial { done: 300 << 20, total: 924 << 20 }),
        view(&m[3], ModelState::NotDownloaded),
    ]);
    h.get_by_label("Models…").click();
    h.run_steps(12);
    // The verdict chips line up on the content edge, whatever the headline's length.
    let lefts: Vec<f32> = h.get_all_by_label("Runs here").map(|n| n.rect().left()).collect();
    assert_eq!(lefts.len(), 3);
    assert!(lefts.iter().all(|l| (l - lefts[0]).abs() <= 1.0), "verdicts on one left edge: {lefts:?}");
    // Each row's last action (the primary where there is one) sits on one trailing edge, the
    // device card's Measure again with them.
    let rights: Vec<f32> = ["Pause", "Resume", "Install", "Measure again"].iter().map(|l| h.get_by_label(l).rect().right()).collect();
    assert!(rights.iter().all(|r| (r - rights[0]).abs() <= 1.0), "actions on one right edge: {rights:?}");
    // The primary is the trailing action: Resume is right of Delete.
    assert!(h.get_by_label("Resume").rect().left() > h.get_by_label("Delete").rect().right());
    // While installing, the bar says so; there is no second "Installing" chip.
    assert!(h.query_by_label("Installing").is_none());
    // A row is about as tall as its three lines: a hairline list, not a box per model.
    let pause = h.get_by_label("Pause").rect();
    let resume = h.get_by_label("Resume").rect();
    assert!(resume.center().y - pause.center().y < 120.0, "row pitch {}", resume.center().y - pause.center().y);
}
