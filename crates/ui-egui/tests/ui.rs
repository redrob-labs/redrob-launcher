//! Drives the launcher UI against a fake backend: what's shown for each status, and which
//! commands the buttons send. `REDROB_LAUNCHER_SNAPSHOTS=<dir>` also saves screenshots; run that
//! with `-- --test-threads=1`, as rendering on several threads at once can crash Vulkan loaders
//! older than 1.4.350 (KhronosGroup/Vulkan-Loader#1863).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use redrob_launcher_core::protocol::{
    Backend, CANCELLED, Capabilities, Command, Event, INSTALL_CANCELLED, JobPhase, LatestRelease, Origin, PausedDownload, Source,
};
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
        html_url: format!("https://github.com/storytold/{id}/releases/tag/v{v}"),
        notes: "## What's Changed\n* Faster things by @a in https://github.com/storytold/x/pull/1".into(),
        prerelease: false,
        assets: vec![Asset {
            name: name.clone(),
            size: 150 << 20,
            url: format!("https://github.com/storytold/{id}/releases/download/v{v}/{name}"),
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
    ev.push(Event::ReadyToInstall { app: "redrob-cad".into(), ready: true });
    ev.push(Event::CheckFinished { at: NOW - 180, source: Source::GitHubApi, rate_limited_until: None });
    ev
}

fn harness(theme: egui::Theme, manage: bool) -> (Harness<'static, Option<LauncherUi>>, Rc<RefCell<Vec<Command>>>) {
    harness_at(theme, manage, 1.0)
}

fn harness_at(theme: egui::Theme, manage: bool, scale: f32) -> (Harness<'static, Option<LauncherUi>>, Rc<RefCell<Vec<Command>>>) {
    harness_with(events(), theme, manage, scale)
}

fn harness_with(
    events: Vec<Event>,
    theme: egui::Theme,
    manage: bool,
    scale: f32,
) -> (Harness<'static, Option<LauncherUi>>, Rc<RefCell<Vec<Command>>>) {
    // Integration tests compile without cfg!(test), so system_lang() picks up the
    // developer's locale. Force English so labels match the assertions.
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let sent = Rc::new(RefCell::new(Vec::new()));
    let sent2 = sent.clone();
    let mut backend = Some(Fake { events, sent: sent2, manage, clock: Rc::new(Cell::new(NOW)) });
    let settings = Settings { notify: false, ..Settings::default() };
    // No `.wgpu()`: it gives every test its own GPU device, and those segfault on parallel test threads (see above).
    // The default renderer creates one only when a test renders.
    let harness = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).with_pixels_per_point(scale).with_theme(theme).build_ui_state(
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

#[test]
fn keeps_working_with_no_window_then_draws_in_a_new_one() {
    let sent = Rc::new(RefCell::new(Vec::new()));
    let backend = Fake { events: events(), sent: sent.clone(), manage: true, clock: Rc::new(Cell::new(NOW)) };
    let windowless = egui::Context::default();
    let mut ui = LauncherUi::new(&windowless, Box::new(backend), Settings { notify: false, ..Settings::default() });
    // No window: the backend's news is still taken in.
    ui.logic(&windowless, &mut NoHost);
    assert!(sent.borrow().contains(&Command::CheckForUpdates { force: false }));
    assert_eq!(ui.update_count(), 2);
    assert_eq!(ui.settings.last_check, Some(NOW - 180));

    let mut attached = false;
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |egui_ui, state: &mut LauncherUi| {
            if !attached {
                attached = true;
                state.attach(egui_ui.ctx());
            }
            state.logic(&egui_ui.ctx().clone(), &mut NoHost);
            state.ui(egui_ui, &mut NoHost);
        },
        ui,
    );
    h.run_steps(4);
    h.get_by_label("Update to 0.5.0");
    h.get_by_label("Update to 0.6.0");
}

#[test]
fn downloads_can_be_paused_resumed_and_discarded() {
    // The fixture's reconnecting download is Redrob Office's.
    let (mut h, sent) = harness_with(events(), egui::Theme::Dark, true, 1.0);
    h.run_steps(4);
    // Bring the paused and reconnecting apps on screen (the sidebar leaves the grid three
    // columns, so the third row needs a taller window).
    if let Some(l) = h.state_mut().as_mut() {
        l.settings.filter = redrob_launcher_ui_egui::settings::Filter::NotInstalled;
    }
    h.set_size(egui::vec2(1180.0, 1100.0));
    h.run_steps(6);
    snapshot(&mut h, "grid-downloads-dark");
    h.get_by_label_contains(": Download paused");
    // The badge, and in a narrow tile the progress caption too.
    assert!(h.query_all_by_label("Reconnecting…").count() >= 1);
    h.get_by_label_contains("· 25%").click();
    h.run_steps(2);
    assert!(sent.borrow().contains(&Command::Install { app: "redrob-cowork".into() }));
    h.get_by_label("Discard download").click();
    h.run_steps(2);
    assert!(sent.borrow().contains(&Command::Cancel { app: "redrob-cowork".into() }));
    h.get_all_by_label("Pause download").next().unwrap().click();
    h.run_steps(2);
    assert!(sent.borrow().iter().any(|c| matches!(c, Command::Pause { .. })));
}

#[test]
fn a_kept_download_offers_retry_install_and_download_again() {
    let (mut h, sent) = harness(egui::Theme::Dark, true);
    h.run_steps(4);
    if let Some(l) = h.state_mut().as_mut() {
        l.settings.filter = redrob_launcher_ui_egui::settings::Filter::NotInstalled;
    }
    h.run_steps(6);
    // Below the fold in the restyled grid: bring the card on screen first.
    h.get_by_label("Retry install").scroll_to_me();
    h.run_steps(4);
    snapshot(&mut h, "grid-retry-install-dark");
    sent.borrow_mut().clear();
    let retry = h.get_by_label("Retry install");
    let row = retry.rect();
    retry.click();
    h.run_steps(2);
    assert_eq!(*sent.borrow(), [Command::Install { app: "redrob-cad".into() }]);
    sent.borrow_mut().clear();
    // The named button beside this uninstalled app exposes the same command as its context menu.
    let beside = |r: egui::Rect| row.y_range().contains(r.center().y) && (row.right()..row.right() + 60.0).contains(&r.left());
    assert!(h.query_all_by_label("More actions").any(|n| beside(n.rect())), "… beside Retry install");
    open_card_more_button(&mut h, "Redrob CAD");
    h.get_by_label("Download again");
    h.key_press(egui::Key::Escape);
    h.run_steps(2);
    // The card's own controls sit over the card and keep the pointer: right-clicking the button
    // neither opens the card's menu nor presses it.
    h.get_by_label("Retry install").click_secondary();
    h.run_steps(3);
    assert!(h.query_by_label("Download again").is_none(), "a right-click on a button is the button's");
    assert!(sent.borrow().is_empty(), "{:?}", sent.borrow());
    open_card_menu(&mut h, "Redrob CAD");
    snapshot(&mut h, "grid-retry-install-menu-dark");
    h.get_by_label("Download again").click();
    h.run_steps(2);
    assert_eq!(*sent.borrow(), [Command::Cancel { app: "redrob-cad".into() }, Command::Install { app: "redrob-cad".into() }]);
    // Apps without a kept download don't offer it.
    assert_eq!(h.query_all_by_label("Retry install").count(), 1);
}

#[test]
fn a_dismissed_install_prompt_is_not_called_a_cancelled_download() {
    let finished = |app: &str, why: &str| Event::JobFinished { app: app.into(), result: Err(why.into()) };
    // The password prompt was dismissed with the download kept (redrob-cad) and without (redrob-office);
    // a download was cancelled (redrob-shape).
    let mut ev = events();
    ev.push(finished("redrob-cad", INSTALL_CANCELLED));
    ev.push(finished("redrob-office", INSTALL_CANCELLED));
    ev.push(finished("redrob-shape", CANCELLED));
    let (mut h, _) = harness_with(ev, egui::Theme::Dark, true, 1.0);
    h.run_steps(6);
    snapshot(&mut h, "grid-install-cancelled-dark");
    for toast in ["Install cancelled. The download is kept for the next try.", "Install cancelled", "Download cancelled"] {
        assert_eq!(h.query_all_by_label(toast).count(), 1, "{toast}");
    }
}

#[test]
fn a_kept_update_names_its_version_and_an_up_to_date_app_is_not_downloaded_again() {
    let mut ev = events();
    for app in ["redrob-browser", "redrob-code"] {
        ev.push(Event::ReadyToInstall { app: app.into(), ready: true });
    }
    let (mut h, sent) = harness_with(ev, egui::Theme::Dark, true, 1.0);
    h.run_steps(8);
    h.get_by_label("Retry update to 0.6.0").scroll_to_me();
    h.run_steps(4);
    snapshot(&mut h, "grid-retry-update-dark");
    sent.borrow_mut().clear();
    assert!(h.query_by_label("Update to 0.6.0").is_none());
    h.get_by_label("Retry update to 0.6.0").click();
    h.run_steps(2);
    assert_eq!(*sent.borrow(), [Command::Install { app: "redrob-browser".into() }]);
    // Redrob Code is up to date: its … button opens the menu, which offers Reinstall (it uses the
    // kept file).
    open_card_more_button(&mut h, "Redrob Code");
    h.get_by_label("Reinstall");
    assert!(h.query_by_label("Download again").is_none());
}

#[test]
fn an_install_waiting_for_another_says_so_and_can_be_cancelled() {
    let mut events = events();
    // Two updates reached the system's installer together: one runs, the other waits its turn.
    events.push(Event::Progress { app: "redrob-canvas".into(), phase: JobPhase::Installing });
    events.push(Event::Progress { app: "redrob-shape".into(), phase: JobPhase::Waiting });
    let (mut h, sent) = harness_with(events, egui::Theme::Dark, true, 1.0);
    h.run_steps(6);
    snapshot(&mut h, "grid-waiting-dark");
    h.get_by_label("Installing…");
    h.get_by_label("Waiting for another app…");
    // Only the waiting one: the install that is running can't be called off. (Redrob Shape is on
    // the third shelf now: bring it on screen first.)
    h.get_by_label("Cancel").scroll_to_me();
    h.run_steps(4);
    h.get_by_label("Cancel").click();
    h.run_steps(2);
    let cancels: Vec<_> = sent.borrow().iter().filter(|c| matches!(c, Command::Cancel { .. })).cloned().collect();
    assert_eq!(cancels, [Command::Cancel { app: "redrob-shape".into() }]);
}

#[test]
fn the_launcher_update_waiting_for_other_installs_can_be_cancelled() {
    let mut ev = events();
    ev.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    ev.push(Event::Progress { app: "redrob-launcher".into(), phase: JobPhase::Waiting });
    let (mut h, sent) = harness_with(ev, egui::Theme::Dark, true, 1.0);
    h.run_steps(6);
    snapshot(&mut h, "grid-launcher-waiting-dark");
    h.get_by_label("Waiting for other installs…");
    sent.borrow_mut().clear();
    h.get_by_label("Cancel update").click();
    h.run_steps(2);
    assert_eq!(*sent.borrow(), [Command::Cancel { app: "redrob-launcher".into() }]);
}

#[test]
fn each_cancellation_says_what_it_called_off() {
    let finished = |app: &str, why: &str| Event::JobFinished { app: app.into(), result: Err(why.into()) };
    let mut ev = events();
    // An uninstall called off while it waited for the installer.
    ev.push(Event::Progress { app: "redrob-film".into(), phase: JobPhase::Removing });
    ev.push(Event::Progress { app: "redrob-film".into(), phase: JobPhase::Waiting });
    ev.push(finished("redrob-film", CANCELLED));
    // The launcher's own update, cancelled while it waited.
    ev.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    ev.push(Event::Progress { app: "redrob-launcher".into(), phase: JobPhase::Waiting });
    ev.push(finished("redrob-launcher", CANCELLED));
    // A download cancelled mid-way.
    ev.push(finished("redrob-film", CANCELLED));
    let (mut h, _) = harness_with(ev, egui::Theme::Dark, true, 1.0);
    h.run_steps(6);
    for toast in ["Uninstall cancelled", "Update cancelled", "Download cancelled"] {
        assert_eq!(h.query_all_by_label(toast).count(), 1, "{toast}");
    }
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

/// Opens the Settings page: the sidebar's account row (it reads "Settings" with no account
/// backend) opens its menu, and the menu's Settings item opens the page.
fn open_settings(h: &mut Harness<'_, Option<LauncherUi>>) {
    from_account_menu(h, "Settings", "Settings");
}

/// Opens `app`'s menu by right-clicking its tile (on its name: the card under the label takes the
/// click). Every tile also has the named … button, including read-only and job states.
fn open_card_menu(h: &mut Harness<'_, Option<LauncherUi>>, app: &str) {
    h.get_all_by_label(app).next().unwrap().scroll_to_me();
    h.run_steps(4);
    h.get_all_by_label(app).next().unwrap().click_secondary();
    h.run_steps(3);
}

/// Opens `app`'s menu with its … button: the nearest one right of its name and under it, in the
/// same tile.
fn open_card_more_button(h: &mut Harness<'_, Option<LauncherUi>>, app: &str) {
    h.get_all_by_label(app).next().unwrap().scroll_to_me();
    h.run_steps(4);
    {
        let name = h.get_all_by_label(app).next().unwrap().rect();
        let menu = h
            .get_all_by_label("More actions")
            .filter(|n| {
                let r = n.rect();
                r.left() > name.left() && r.top() > name.bottom() && r.top() < name.top() + 180.0
            })
            .min_by(|a, b| a.rect().left().total_cmp(&b.rect().left()))
            .unwrap();
        menu.click();
    }
    h.run_steps(3);
}

#[test]
fn settings_link_to_redrob() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    open_settings(&mut h);
    snapshot(&mut h, "settings-dark");
    h.get_by_label("News and source code for every Redrob app");
    assert!(h.query_all_by_label_contains("Discord").next().is_none());
}

#[test]
fn links_to_the_website_github_and_each_apps_pages() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    // The Apps page has no links: no footer, nothing on the cards.
    assert_eq!(h.query_all_by_label("GitHub").count(), 0);
    assert_eq!(h.query_all_by_label("Website").count(), 0);
    // A card's menu has the app's GitHub repo and no website entry (no app has a web page yet).
    open_card_menu(&mut h, "Redrob Code");
    h.get_by_label("GitHub repository");
    assert_eq!(h.query_all_by_label_contains("Website").count(), 0, "no app has a website page yet");
    // Redrob's own pages are in Settings.
    h.key_press(egui::Key::Escape);
    h.run_steps(3);
    open_settings(&mut h);
    h.get_by_label("Website");
    h.get_by_label("GitHub");
}

/// Clicks the `n`th widget labelled `label` and returns the URL the app asked to open.
fn click_nth_opens(h: &mut Harness<'_, Option<LauncherUi>>, label: &str, n: usize) -> Option<String> {
    h.get_all_by_label(label).nth(n)?.click();
    for _ in 0..3 {
        h.step();
        let url = h.output().platform_output.commands.iter().find_map(|c| match c {
            egui::OutputCommand::OpenUrl(o) => Some(o.url.clone()),
            _ => None,
        });
        if url.is_some() {
            return url;
        }
    }
    None
}

fn click_opens(h: &mut Harness<'_, Option<LauncherUi>>, label: &str) -> Option<String> {
    click_nth_opens(h, label, 0)
}

#[test]
fn links_open_the_right_pages() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    // Each app's pages, from its card's menu.
    open_card_menu(&mut h, "Redrob Canvas");
    assert_eq!(click_opens(&mut h, "GitHub repository").as_deref(), Some("https://github.com/redrob-labs/redrob-canvas"));
    open_card_menu(&mut h, "Redrob Shape");
    assert_eq!(click_opens(&mut h, "All releases").as_deref(), Some("https://github.com/redrob-labs/redrob-shape/releases"));
    // Redrob's own, from Settings.
    open_settings(&mut h);
    assert_eq!(click_opens(&mut h, "Website").as_deref(), Some("https://redrob.ai"));
    assert_eq!(click_opens(&mut h, "GitHub").as_deref(), Some("https://github.com/redrob-labs"));
}

#[test]
fn looks_again_at_installed_apps_every_few_minutes_not_every_frame() {
    let sent = Rc::new(RefCell::new(Vec::new()));
    let clock = Rc::new(Cell::new(NOW));
    let mut backend = Some(Fake { events: events(), sent: sent.clone(), manage: true, clock: clock.clone() });
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    );
    let detects = |sent: &Rc<RefCell<Vec<Command>>>| sent.borrow().iter().filter(|c| **c == Command::Detect).count();
    h.run_steps(10);
    assert_eq!(detects(&sent), 0, "the backend's own startup detection is enough");
    clock.set(NOW + 4 * 60);
    h.run_steps(10);
    assert_eq!(detects(&sent), 0, "not before five minutes");
    clock.set(NOW + 5 * 60);
    h.run_steps(10);
    assert_eq!(detects(&sent), 1, "once at five minutes, not once per frame");
}

#[test]
fn the_launcher_updates_itself_and_quits_to_restart() {
    struct QuitHost(Rc<Cell<bool>>);
    impl Host for QuitHost {
        fn quit(&mut self) {
            self.0.set(true);
        }
    }
    let sent = Rc::new(RefCell::new(Vec::new()));
    let mut events = events();
    events.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    let shared_events = Rc::new(RefCell::new(Vec::new()));
    struct Scripted {
        inner: Fake,
        later: Rc<RefCell<Vec<Event>>>,
    }
    impl Backend for Scripted {
        fn send(&mut self, c: Command) {
            self.inner.send(c);
        }
        fn poll(&mut self) -> Vec<Event> {
            let mut out = self.inner.poll();
            out.extend(self.later.borrow_mut().drain(..));
            out
        }
        fn capabilities(&self) -> Capabilities {
            self.inner.capabilities()
        }
        fn platform(&self) -> Option<Platform> {
            self.inner.platform()
        }
        fn now(&self) -> i64 {
            self.inner.now()
        }
    }
    let quit = Rc::new(Cell::new(false));
    let mut host = QuitHost(quit.clone());
    let mut backend =
        Some(Scripted { inner: Fake { events, sent: sent.clone(), manage: true, clock: Rc::new(Cell::new(NOW)) }, later: shared_events.clone() });
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut host);
                l.ui(ui, &mut host);
            }
        },
        None,
    );
    h.run_steps(6);
    h.get_by_label_contains("Redrob Launcher 0.2.0 is available");
    h.get_by_label("Update and restart").click();
    h.run_steps(2);
    assert!(sent.borrow().contains(&Command::UpdateLauncher));
    // Downloading: the banner shows progress instead of the button.
    shared_events
        .borrow_mut()
        .push(Event::Progress { app: "redrob-launcher".into(), phase: JobPhase::Downloading { done: 5 << 20, total: 10 << 20 } });
    h.run_steps(3);
    assert!(h.query_by_label("Update and restart").is_none());
    assert!(!quit.get());
    // In place: the launcher quits so the helper can start the new one.
    shared_events.borrow_mut().push(Event::LauncherRestarting);
    h.run_steps(3);
    assert!(quit.get());
}

#[test]
fn the_current_desktop_setting_is_only_offered_where_the_os_has_desktops() {
    // NoHost has no desktops (Windows, Linux, web): no setting.
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    open_settings(&mut h);
    assert!(h.query_by_label("Show the window on the current desktop").is_none());
    // On by default for everyone; macOS shows it in Settings (DesktopHost::has_desktops).
    assert!(Settings::default().show_on_current_desktop);
}

#[test]
fn feed_announcements_and_alerts_show_until_dismissed() {
    use redrob_launcher_core::feed::Notice;
    let notice = |id: &str, title: &str, link: Option<&str>| Notice {
        id: id.into(),
        title: title.into(),
        body: format!("{title} body"),
        link: link.map(str::to_string),
        created_at: None,
    };
    let sent = Rc::new(RefCell::new(Vec::new()));
    let mut evs = events();
    evs.push(Event::Notices {
        announcements: vec![notice("a1", "Redrob CAD 1.0 is out", Some("https://redrob.ai/news"))],
        alerts: vec![notice("x1", "Security update", Some("https://redrob.ai/security"))],
    });
    let mut backend = Some(Fake { events: evs, sent, manage: true, clock: Rc::new(Cell::new(NOW)) });
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    );
    h.run_steps(6);
    // The alert pops up first; OK acknowledges it for good.
    h.get_by_label("Security update");
    h.get_by_label("Open link");
    h.get_by_label("OK").click();
    h.run_steps(4);
    assert!(h.query_by_label("Security update").is_none());
    assert!(h.state().as_ref().unwrap().settings.dismissed_notices.contains("x1"));
    // The announcement stays inline until dismissed.
    h.get_by_label("Redrob CAD 1.0 is out");
    h.get_by_label("Learn more");
    h.get_all_by_label("Dismiss").next().unwrap().click();
    h.run_steps(4);
    assert!(h.query_by_label("Redrob CAD 1.0 is out").is_none());
    assert!(h.state().as_ref().unwrap().settings.dismissed_notices.contains("a1"));
}

#[test]
fn automatic_checks_run_every_five_minutes() {
    use redrob_launcher_ui_egui::settings::CheckEvery;
    assert_eq!(Settings::default().check_every.seconds(), Some(300));
    // Older saved choices still load, and mean "automatically" now.
    assert_eq!(CheckEvery::Day.seconds(), Some(300));
    assert_eq!(CheckEvery::Never.seconds(), None);
}

#[test]
fn automatic_updates_queue_installs_for_outdated_apps_and_launcher() {
    let mut ev = events();
    ev.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    let sent = Rc::new(RefCell::new(Vec::new()));
    let sent2 = sent.clone();
    let mut backend = Some(Fake { events: ev, sent: sent2, manage: true, clock: Rc::new(Cell::new(NOW)) });
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { auto_update: true, notify: false, ..Settings::default() }));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    );
    h.run_steps(4);
    let commands = sent.borrow().clone();
    assert!(
        commands.contains(&Command::Install { app: "redrob-canvas".into() }),
        "outdated redrob-canvas should be queued for install: {commands:?}"
    );
    assert!(
        commands.contains(&Command::Install { app: "redrob-browser".into() }),
        "outdated redrob-browser should be queued for install: {commands:?}"
    );
    assert!(commands.contains(&Command::UpdateLauncher), "launcher update should be queued: {commands:?}");
}

#[test]
fn automatic_updates_are_disabled_by_default() {
    let mut ev = events();
    ev.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    let sent = Rc::new(RefCell::new(Vec::new()));
    let sent2 = sent.clone();
    let mut backend = Some(Fake { events: ev, sent: sent2, manage: true, clock: Rc::new(Cell::new(NOW)) });
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { auto_update: false, notify: false, ..Settings::default() }));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    );
    h.run_steps(4);
    let commands = sent.borrow().clone();
    assert!(
        !commands.contains(&Command::Install { app: "redrob-canvas".into() }),
        "redrob-canvas should not be auto-installed when auto_update is false"
    );
}

#[test]
fn web_build_offers_downloads() {
    let (mut h, _) = harness(egui::Theme::Dark, false);
    h.run_steps(8);
    snapshot(&mut h, "web-grid-dark");
    assert!(h.query_all_by_label("Download for macOS").count() >= 6);
    // Read-only web cards still need a discoverable route to alternate packages and links.
    let visible_cards = SUITE.iter().filter(|app| h.query_by_label_contains(&format!("{}:", app.name)).is_some()).count();
    assert_eq!(h.query_all_by_label("More actions").count(), visible_cards);
    assert!(h.query_by_label("Install").is_none());
}

/// The title bar's search field.
fn search_field_rect(h: &Harness<'_, Option<LauncherUi>>) -> egui::Rect {
    h.query_all(egui_kittest::kittest::by().role(egui::accesskit::Role::TextInput).label("Search apps"))
        .map(|n| n.rect())
        .find(|r| r.bottom() <= 48.0)
        .unwrap()
}

#[test]
fn custom_title_bar_has_window_buttons() {
    struct FramelessHost;
    impl Host for FramelessHost {
        fn title_bar(&self) -> redrob_launcher_ui_egui::TitleBar {
            redrob_launcher_ui_egui::TitleBar::Custom
        }
    }
    let frameless = |width: f32| {
        let mut backend = Some(Fake { events: events(), sent: Rc::new(RefCell::new(Vec::new())), manage: true, clock: Rc::new(Cell::new(NOW)) });
        Harness::builder().with_size(egui::vec2(width, 900.0)).build_ui_state(
            move |ui, state: &mut Option<LauncherUi>| {
                if state.is_none()
                    && let Some(b) = backend.take()
                {
                    *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
                }
                if let Some(l) = state.as_mut() {
                    l.logic(&ui.ctx().clone(), &mut FramelessHost);
                    l.ui(ui, &mut FramelessHost);
                }
            },
            None,
        )
    };
    // Narrow windows shrink the search first, never under the caption buttons, and leave the bar
    // room to drag left of the logo.
    for width in [600.0, 960.0] {
        let mut h = frameless(width);
        h.run_steps(6);
        snapshot(&mut h, &format!("grid-title-bar-dark-{width}"));
        let minimize = h.get_by_label("Minimize").rect();
        let search = search_field_rect(&h);
        let logo = h.get_by_label("Go to console.redrob.ai").rect();
        assert!(search.right() <= minimize.left() && search.width() >= 159.5, "{width}: {search:?}");
        assert!(logo.left() >= 24.0, "{width}: {logo:?}");
    }
    let mut h = frameless(1180.0);
    h.run_steps(6);
    snapshot(&mut h, "grid-title-bar-dark");
    // The custom bar, right to left: window buttons, search, the group logo. No action button.
    let minimize = h.get_by_label("Minimize").rect();
    let search = search_field_rect(&h);
    let logo = h.get_by_label("Go to console.redrob.ai").rect();
    assert!(logo.right() <= search.left() && search.right() <= minimize.left());
    // One row, inside the caption buttons' band (whose target starts under the 5 px resize edge).
    for r in [search, logo] {
        assert!(r.top() >= minimize.top() - 5.0 && r.bottom() <= minimize.bottom(), "{r:?} vs {minimize:?}");
    }
    // Check for updates is in the filter row, with Update all directly left of it on the same row.
    let refresh = h.get_by_label("Check for updates").rect();
    let update = h.get_by_label("Update all (2)").rect();
    assert!(refresh.top() > 48.0 && update.top() > 48.0, "both are under the title bar: {update:?} {refresh:?}");
    assert!(update.right() <= refresh.left() && refresh.left() - update.right() <= 12.0, "{update:?} left of {refresh:?}");
    assert!((update.center().y - refresh.center().y).abs() < 1.0, "same row: {update:?} {refresh:?}");
    // A tile's main button is Sm (32 px) at its label's width, right-aligned just left of its … button.
    let main = h.get_by_label("Update to 0.6.0").rect();
    assert!((main.height() - 32.0).abs() < 0.5 && main.width() < 200.0, "{main:?}");
    assert!(
        h.query_all_by_label("More actions").any(|m| {
            let m = m.rect();
            m.left() >= main.right() && m.left() - main.right() <= 12.0 && (m.center().y - main.center().y).abs() < 1.0
        }),
        "… directly right of {main:?}"
    );
    // The suite's hero leads the page, above the filter row, and stays when a filter narrows the
    // page (only a search hides it).
    let hero = h.get_by_label("Work, create and build in one suite.").rect();
    assert!(hero.bottom() < refresh.top(), "hero above the filter row: {hero:?} {refresh:?}");
    if let Some(l) = h.state_mut().as_mut() {
        l.settings.filter = redrob_launcher_ui_egui::settings::Filter::Updates;
    }
    h.run_steps(3);
    assert!(h.query_by_label("Work, create and build in one suite.").is_some(), "the hero stays on a filtered page");
    if let Some(l) = h.state_mut().as_mut() {
        l.settings.filter = redrob_launcher_ui_egui::settings::Filter::All;
    }
    h.run_steps(3);
    assert!(h.get_all_by_label("All apps").all(|n| n.rect().top() >= 48.0), "the sidebar and page start under the title bar");
    // Restored, the top 5 px of the caption buttons and close's right 5 px stay the resize band.
    // (The harness insets the bar: its top is minimize's minus that band.)
    h.get_by_label("Maximize");
    let restored_close = h.get_by_label("Close").rect();
    let bar_top = minimize.top() - 5.0;
    assert!((restored_close.top() - (bar_top + 5.0)).abs() < 0.5, "{restored_close:?}");
    // Maximized there is no resize band: the buttons run to the top of the screen and close to its
    // right edge, so flinging the pointer into the corner lands on Close (Windows' own buttons).
    for _ in 0..3 {
        h.input_mut().viewports.entry(egui::ViewportId::ROOT).or_default().maximized = Some(true);
        h.step();
    }
    let close = h.get_by_label("Close").rect();
    let restore = h.get_by_label("Restore").rect();
    assert!((close.top() - bar_top).abs() < 0.5 && (close.right() - (restored_close.right() + 5.0)).abs() < 0.5, "{close:?}");
    assert!((restore.top() - bar_top).abs() < 0.5 && (restore.right() - close.left()).abs() < 0.5, "{restore:?}");
    h.get_by_label("Minimize");
    h.get_by_label("Restore");
    h.get_by_label("Close").click();
    h.step();
    let closes = h.output().viewport_output.values().any(|v| v.commands.contains(&egui::ViewportCommand::Close));
    assert!(closes, "the close button asks the window to close (the shell decides whether that hides it to the tray)");
}
