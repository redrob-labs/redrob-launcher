//! The models dialog against a fake model backend: every state's label and buttons, and the
//! requests they send.

use std::cell::RefCell;
use std::rc::Rc;

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use redrob_launcher_core::models::{Model, ModelBackend, ModelState, ModelView, builtin};
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
    ModelView { model: m.clone(), state, folder: Some(format!("/home/u/.local/share/redrob-cut/models/{}", m.id)) }
}

#[test]
fn every_state_offers_the_right_action() {
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let m = builtin();
    let state = Rc::new(RefCell::new(State {
        views: vec![
            view(&m[0], ModelState::Installed),
            view(&m[1], ModelState::Downloading { done: 100 << 20, total: 277 << 20 }),
            view(&m[2], ModelState::Partial { done: 300 << 20, total: 924 << 20 }),
            view(&m[3], ModelState::NotDownloaded),
        ],
        asked: Vec::new(),
    }));
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
    // A download is running, so the header says so.
    h.get_by_label("Models…").click();
    h.run_steps(12);
    h.get_by_label("Whisper base (multilingual)");
    h.get_by_label("Natural voices (Kokoro-82M)");
    h.get_by_label("Downloaded");
    h.get_by_label("Downloading 100 MB of 277 MB");
    h.get_by_label("300 MB of 924 MB downloaded");
    h.get_by_label("Not downloaded");
    if let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") {
        h.render().unwrap().save(std::path::Path::new(&dir).join("models.png")).unwrap();
    }
    h.get_by_label("Pause").click();
    h.run_steps(8);
    h.get_by_label("Resume").click();
    h.run_steps(8);
    h.get_by_label("Download").click();
    h.run_steps(8);
    h.get_all_by_label("Delete").next().unwrap().click();
    h.run_steps(8);
    assert_eq!(
        state.borrow().asked,
        ["pause redrob-cut/whisper-base", "download redrob-cut/whisper-small", "download redrob-cut/kokoro-82m", "delete redrob-cut/whisper-tiny"]
    );
}
