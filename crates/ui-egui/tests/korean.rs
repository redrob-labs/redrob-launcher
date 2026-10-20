//! The Korean UI, in its own test binary: the UI language is process-wide, so it can't share one
//! with the English UI tests. `REDROB_LAUNCHER_SNAPSHOTS=<dir>` also saves screenshots (run with
//! `-- --test-threads=1`).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use redrob_launcher_core::protocol::{Backend, Capabilities, Command, Event, JobPhase, LatestRelease, Origin, PausedDownload, Source};
use redrob_launcher_core::release::{Asset, Release};
use redrob_launcher_core::status::InstallSource;
use redrob_launcher_core::{Arch, Installed, Os, Platform, SUITE, version};
use redrob_launcher_ui_egui::{Host, LauncherUi, Settings};

const NOW: i64 = 1_791_500_000;

struct Fake {
    events: Vec<Event>,
    sent: Rc<RefCell<Vec<Command>>>,
    manage: bool,
    clock: Rc<Cell<i64>>,
}

impl Backend for Fake {
    fn send(&mut self, command: Command) {
        self.sent.borrow_mut().push(command);
    }
    fn poll(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }
    fn capabilities(&self) -> Capabilities {
        Capabilities { manage_apps: self.manage, reveal_label: "Show in Finder", self_update: self.manage }
    }
    fn platform(&self) -> Option<Platform> {
        Some(Platform { os: Os::MacOs, arch: Arch::Arm64 })
    }
    fn now(&self) -> i64 {
        self.clock.get()
    }
}

struct NoHost;
impl Host for NoHost {}

fn release(id: &str, v: &str, at: i64) -> Release {
    let name = format!("{id}-{v}-macos-universal.dmg");
    Release {
        tag: format!("v{v}"),
        version: version::parse(v),
        name: format!("v{v}"),
        published_at: Some(at),
        html_url: format!("https://github.com/redrob-labs/{id}/releases/tag/v{v}"),
        notes: "## What's Changed\n* Faster things by @a in https://github.com/redrob-labs/x/pull/1".into(),
        prerelease: false,
        assets: vec![Asset {
            name: name.clone(),
            size: 150 << 20,
            url: format!("https://github.com/redrob-labs/{id}/releases/download/v{v}/{name}"),
            sha256: None,
        }],
        manifest: None,
    }
}

/// A realistic mix: updates, up-to-date apps, a download in progress, one app with no release.
fn events() -> Vec<Event> {
    let mut ev = Vec::new();
    let versions = [
        ("redrob-canvas", "0.5.0"),
        ("redrob-shape", "0.7.0"),
        ("redrob-film", "0.4.0"),
        ("redrob-office", "0.4.0"),
        ("redrob-browser", "0.6.0"),
        ("redrob-code", "0.3.0"),
        ("redrob-cowork", "0.3.0"),
        ("redrob-cad", "0.3.0"),
    ];
    for (i, (id, v)) in versions.iter().enumerate() {
        ev.push(Event::Latest {
            app: (*id).into(),
            latest: LatestRelease::Found(release(id, v, NOW - 86_400 - i as i64 * 600)),
            from: Origin::RedrobApi,
        });
    }
    ev.push(Event::Latest { app: "redrob-design".into(), latest: LatestRelease::None, from: Origin::RedrobApi });
    let installed = [("redrob-canvas", "0.4.1"), ("redrob-code", "0.3.0"), ("redrob-browser", "0.5.0")];
    for app in &SUITE {
        let inst = installed.iter().find(|(id, _)| *id == app.id).map(|(_, v)| Installed {
            version: version::parse(v),
            source: InstallSource::MacApp,
            location: format!("/Applications/{}.app", app.name),
            package: None,
        });
        ev.push(Event::Detected { app: app.id.into(), installed: inst });
    }
    ev.push(Event::Progress { app: "redrob-film".into(), phase: JobPhase::Downloading { done: 48 << 20, total: 131 << 20 } });
    ev.push(Event::Paused { app: "redrob-cowork".into(), download: Some(PausedDownload { done: 30 << 20, total: 120 << 20 }) });
    ev.push(Event::Progress { app: "redrob-office".into(), phase: JobPhase::Reconnecting { done: 10 << 20, total: 90 << 20 } });
    ev.push(Event::CheckFinished { at: NOW - 180, source: Source::GitHubApi, rate_limited_until: None });
    ev
}

fn harness_language(theme: egui::Theme, manage: bool, language: &str) -> (Harness<'static, Option<LauncherUi>>, Rc<RefCell<Vec<Command>>>) {
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let sent = Rc::new(RefCell::new(Vec::new()));
    let sent2 = sent.clone();
    let mut backend = Some(Fake { events: events(), sent: sent2, manage, clock: Rc::new(Cell::new(NOW)) });
    let settings = Settings { notify: false, language: language.into(), ..Settings::default() };
    let harness = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).with_theme(theme).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                let mut s = settings.clone();
                s.theme = if theme == egui::Theme::Dark {
                    redrob_launcher_ui_egui::theme::ThemeChoice::Dark
                } else {
                    redrob_launcher_ui_egui::theme::ThemeChoice::Light
                };
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), s));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    );
    (harness, sent)
}

fn snapshot(h: &mut Harness<'_, Option<LauncherUi>>, name: &str) {
    let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") else { return };
    let img = h.render().unwrap();
    img.save(std::path::Path::new(&dir).join(format!("{name}.png"))).unwrap();
}

// One test: the UI language is process-wide.
#[test]
fn korean_switches_persists_and_keeps_install_actions() {
    let (mut h, sent) = harness_language(egui::Theme::Dark, true, "en");
    h.run_steps(6);
    // Settings is in the account row's menu.
    let row = h.get_by_label("Settings").rect();
    h.get_by_label("Settings").click();
    h.run_steps(3);
    h.get_all_by_label("Settings").find(|n| n.rect().bottom() <= row.top() + 0.5).unwrap().click();
    h.run_steps(3);
    h.get_by_value("English").click();
    h.run_steps(2);
    h.get_by_label("한국어").click();
    h.run_steps(3);
    h.get_by_value("한국어");
    h.get_by_label("설치된 앱에 업데이트가 있으면 알림");
    // The last check is in Settings (the Apps page has no footer).
    h.get_by_label_contains("3분 전 확인함 · Redrob API 사용");
    snapshot(&mut h, "settings-ko-dark");
    let settings = h.state().as_ref().unwrap().settings.clone();
    assert_eq!(settings.language, "ko");
    let saved = serde_json::to_string(&settings).unwrap();
    let restored: Settings = serde_json::from_str(&saved).unwrap();
    assert_eq!(restored.language, "ko");
    h.get_by_label("앱으로 돌아가기").click();
    h.run_steps(3);
    h.get_by_label("에이전트와 함께 그림을 그리고 이미지를 보정하고 편집해요");
    // Cards show the full description and no "kind of app" caption.
    assert!(h.query_by_label("그래픽").is_none());
    h.get_by_label("0.5.0 버전으로 업데이트");
    // The account row's menu, in Korean.
    let row = h.get_by_label("설정").rect();
    h.get_by_label("설정").click();
    h.run_steps(3);
    h.get_by_label("Redrob Launcher 종료");
    snapshot(&mut h, "account-menu-ko-dark");
    h.key_press(egui::Key::Escape);
    h.run_steps(3);
    assert!(h.query_by_label("Redrob Launcher 종료").is_none(), "Escape closed the menu");
    assert_eq!(h.get_by_label("설정").rect(), row);
    // Product names stay in Latin script.
    h.get_by_label("Redrob Launcher");
    h.get_by_label("Redrob Canvas");
    snapshot(&mut h, "grid-ko-dark");
    h.get_by_label("모두 업데이트(2)").click();
    h.run_steps(2);
    let installs: Vec<_> = sent.borrow().iter().filter_map(|c| if let Command::Install { app } = c { Some(app.clone()) } else { None }).collect();
    assert_eq!(installs, ["redrob-canvas", "redrob-browser"]);

    // Version lines in Korean on the tiles, without a release date (that is in Release notes).
    let (mut light, _) = harness_language(egui::Theme::Light, true, &restored.language);
    light.run_steps(6);
    assert!(light.query_all_by_label_contains("버전 ").count() >= 1);
    assert_eq!(light.query_all_by_label_contains("2026년 10월").count(), 0);
    snapshot(&mut light, "grid-ko-light");
    let (mut web, _) = harness_language(egui::Theme::Dark, false, &restored.language);
    web.run_steps(6);
    assert!(web.query_all_by_label("macOS용 다운로드").count() >= 3);
}
