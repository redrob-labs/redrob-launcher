//! Drives the launcher UI against a fake backend: what's shown for each status, and which
//! commands the buttons send. `REDROB_LAUNCHER_SNAPSHOTS=<dir>` also saves screenshots; run that
//! with `-- --test-threads=1`, as rendering on several threads at once can crash Vulkan loaders
//! older than 1.4.350 (KhronosGroup/Vulkan-Loader#1863).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};
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
        ("redrob-query", "0.3.0"),
        ("redrob-cad", "0.3.0"),
    ];
    for (i, (id, v)) in versions.iter().enumerate() {
        ev.push(Event::Latest {
            app: (*id).into(),
            latest: LatestRelease::Found(release(id, v, NOW - 86_400 - i as i64 * 600)),
            from: Origin::RedrobApi,
        });
    }
    ev.push(Event::Latest { app: "redrob-recall".into(), latest: LatestRelease::None, from: Origin::RedrobApi });
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
    ev.push(Event::Progress { app: "redrob-query".into(), phase: JobPhase::Reconnecting { done: 10 << 20, total: 90 << 20 } });
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
fn grid_shows_every_status_and_installs_on_click() {
    let (mut h, sent) = harness(egui::Theme::Dark, true);
    h.run_steps(8);
    snapshot(&mut h, "grid-dark");
    // Statuses the user asked for: available, installed and up to date, update with version. No
    // chip draws them any more; each card carries its status for screen readers.
    assert!(h.query_all_by_label_contains(": Available to install").count() >= 3);
    assert!(h.query_all_by_label_contains(": Installed · up to date").count() >= 1);
    // Named by app as well as status, so two cards in the same state never share a name.
    h.get_by_label("Redrob Shape: Available to install");
    // No links on the cards, and the … button only where the menu holds Uninstall.
    assert!(h.query_by_label("GitHub").is_none() && h.query_by_label("Website").is_none(), "cards carry no links");
    let installed = [": Installed · up to date", ": Update available"].iter().map(|l| h.query_all_by_label_contains(l).count()).sum::<usize>();
    assert_eq!(h.query_all_by_label("More actions").count(), installed, "a … button on each installed app and no other");
    h.get_by_label("Update to 0.5.0");
    h.get_by_label("Update to 0.6.0");
    h.get_by_label_contains("0.4.1 installed → 0.5.0 released");
    h.get_by_label("Graphics");
    // The window has no title bar of its own here (the web build, and the tests).
    assert!(h.query_by_label("Close").is_none());
    // No upstream community links, social icons or trademark notice.
    assert!(h.query_all_by_label_contains("Discord").next().is_none());
    assert!(h.query_all_by_label_contains("ArtCraft").next().is_none());
    assert!(h.query_all_by_label("Coming soon").count() >= 1);
    h.get_by_label_contains("48 MB of 131 MB");
    h.get_by_label("Update all (2)").click();
    h.run_steps(2);
    let installs: Vec<_> = sent.borrow().iter().filter_map(|c| if let Command::Install { app } = c { Some(app.clone()) } else { None }).collect();
    assert_eq!(installs.len(), 2, "{installs:?}");
    assert!(installs.contains(&"redrob-canvas".to_string()) && installs.contains(&"redrob-browser".to_string()));
}

#[test]
fn downloads_can_be_paused_resumed_and_discarded() {
    let (mut h, sent) = harness(egui::Theme::Dark, true);
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
    // Not installed: no … button beside it; right-clicking the card opens the menu.
    let beside = |r: egui::Rect| row.y_range().contains(r.center().y) && (row.right()..row.right() + 60.0).contains(&r.left());
    assert!(h.query_all_by_label("More actions").all(|n| !beside(n.rect())), "no … beside Retry install");
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
    // Only the waiting one: the install that is running can't be called off.
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

#[test]
fn a_search_that_matches_nothing_says_what_to_relax_and_clears() {
    let (mut h, _) = harness(egui::Theme::Light, true);
    h.run_steps(6);
    let field = h.query_all(egui_kittest::kittest::by().role(egui::accesskit::Role::TextInput).label("Search apps")).next().unwrap();
    field.focus();
    field.type_text("zzz");
    h.run_steps(4);
    snapshot(&mut h, "empty-search-light");
    // A DS empty state: what happened, what to relax, and the way back.
    h.get_by_label("No apps match");
    h.get_by_label_contains("Nothing matches");
    assert!(h.query_by_label("Redrob Canvas").is_none());
    h.get_by_label("Clear search").click();
    h.run_steps(4);
    assert!(h.query_by_label("No apps match").is_none());
    h.get_by_label("Redrob Canvas");
}

#[test]
fn switching_filters() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    // The grid is the only view: no Grid / Table control, no sort.
    assert!(h.query_by_label("Table").is_none() && h.query_by_label("Grid").is_none());
    assert!(h.query_by_label("Sort").is_none());
    h.get_by_label_contains("Updates").click();
    h.run_steps(4);
    snapshot(&mut h, "grid-updates-dark");
    assert!(h.query_by_label("Redrob Code").is_none());
    h.get_by_label("Redrob Canvas");
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
/// click). Every tile has this menu; installed ones also have a … button for it.
fn open_card_menu(h: &mut Harness<'_, Option<LauncherUi>>, app: &str) {
    h.get_all_by_label(app).next().unwrap().scroll_to_me();
    h.run_steps(4);
    h.get_all_by_label(app).next().unwrap().click_secondary();
    h.run_steps(3);
}

/// Opens an installed `app`'s menu with its … button: the nearest one right of its name and under
/// it, in the same tile.
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
fn an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it() {
    use redrob_launcher_ui_egui::settings::{SortKey, ViewMode};
    // Saved by an earlier version, when there was a table and a sort.
    let old: Settings = serde_json::from_str(r#"{"view":"Table","sort":"Name","sort_descending":true,"notify":false}"#).unwrap();
    assert_eq!((old.view, old.sort, old.sort_descending), (ViewMode::Table, SortKey::Name, true));
    let mut backend = Some(Fake { events: events(), sent: Rc::new(RefCell::new(Vec::new())), manage: true, clock: Rc::new(Cell::new(NOW)) });
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), old.clone()));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    );
    h.run_steps(6);
    // The grid, grouped in suite order: Create before Work, Redrob Canvas first.
    let create = h.get_by_label("Create").rect();
    let work = h.get_by_label("Work").rect();
    assert!(create.top() < work.top());
    assert!(h.query_by_label("Kind").is_none(), "no table header");
    let canvas = h.get_all_by_label("Redrob Canvas").next().unwrap().rect();
    assert!(canvas.top() > create.bottom() && canvas.top() < work.top());
}

/// How far the middle of these lines of text is below the middle of the app icon beside them, in
/// points. The lines also share one left edge, just right of the icon.
fn below_icon_centre(h: &Harness<'_, Option<LauncherUi>>, lines: &[&str]) -> f32 {
    let rects: Vec<egui::Rect> = lines.iter().map(|l| h.get_by_label(l).rect()).collect();
    let left = rects[0].left();
    for (l, r) in lines.iter().zip(&rects) {
        assert!((r.left() - left).abs() < 0.5, "{l} is not left-aligned with {}: {} vs {left}", lines[0], r.left());
    }
    let text = rects.iter().copied().reduce(|a, b| a.union(b)).unwrap();
    let icon = h
        .query_all(egui_kittest::kittest::by().role(egui::accesskit::Role::Image))
        .map(|n| n.rect())
        .filter(|r| r.right() <= text.left() && r.top() < text.bottom() && r.bottom() > text.top())
        .max_by(|a, b| a.right().total_cmp(&b.right()))
        .unwrap();
    assert!(text.left() - icon.right() < 24.0, "no icon beside {lines:?}");
    assert!(text.left() > icon.right(), "{lines:?} runs into the icon");
    text.center().y - icon.center().y
}

#[test]
fn name_and_tagline_are_left_aligned_beside_the_icon_and_centred_on_it() {
    for (scale, suffix) in [(1.0, "-1x"), (1.5, "-1.5x"), (2.0, "-2x")] {
        let (mut h, _) = harness_at(egui::Theme::Dark, true, scale);
        h.run_steps(8);
        snapshot(&mut h, &format!("grid-dark{suffix}"));
        let off = below_icon_centre(&h, &["Redrob Canvas", "Paint and edit images with an agent", "Graphics"]);
        assert!(off.abs() < 0.25, "grid at {scale}x: {off}");
        let (mut h, _) = harness_at(egui::Theme::Light, true, scale);
        h.run_steps(8);
        snapshot(&mut h, &format!("grid-light{suffix}"));
        let off = below_icon_centre(&h, &["Redrob Canvas", "Paint and edit images with an agent", "Graphics"]);
        assert!(off.abs() < 0.25, "grid (light) at {scale}x: {off}");
    }
}

#[test]
fn pages_lead_back_to_apps_by_the_back_button_and_escape() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    let on_settings = |h: &Harness<'_, Option<LauncherUi>>| h.query_by_label("News and source code for every Redrob app").is_some();
    // Settings is a page: no Done button, one primary at most (Website is secondary now).
    open_settings(&mut h);
    assert!(on_settings(&h));
    assert!(h.query_by_label("Done").is_none());
    h.get_by_label("Quit Redrob Launcher");
    // The top bar's back button.
    h.get_by_label("Back to apps").click();
    h.run_steps(4);
    assert!(!on_settings(&h));
    h.get_by_label("Update to 0.5.0");
    assert!(h.query_by_label("Back to apps").is_none(), "Apps is where back leads; it has no back button");
    // Escape.
    open_settings(&mut h);
    assert!(on_settings(&h));
    h.key_press(egui::Key::Escape);
    h.run_steps(4);
    assert!(!on_settings(&h));
    h.get_by_label("Update to 0.5.0");
}

#[test]
fn escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    let on_settings = |h: &Harness<'_, Option<LauncherUi>>| h.query_by_label("News and source code for every Redrob app").is_some();
    // Keyboard focus on an ordinary button (as after Tab) doesn't swallow Escape.
    open_settings(&mut h);
    h.get_by_label("Quit Redrob Launcher").focus();
    h.run_steps(2);
    assert!(h.ctx.memory(|m| m.focused().is_some()), "the button holds keyboard focus");
    h.key_press(egui::Key::Escape);
    h.run_steps(4);
    assert!(!on_settings(&h), "Escape from a focused button returns to Apps");
    // An open list takes Escape for itself: it closes, and the page stays.
    open_settings(&mut h);
    h.get_by_value("Automatically").click();
    h.run_steps(4);
    assert!(egui::Popup::is_any_open(&h.ctx), "the check-for-updates list is open");
    h.key_press(egui::Key::Escape);
    h.run_steps(4);
    assert!(on_settings(&h), "Escape closed the list, not the page");
}

#[test]
fn the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme() {
    for width in [1180.0, 600.0] {
        let mut h = harness_sized(width);
        h.run_steps(6);
        let options: Vec<egui::Rect> = h.query_all(egui_kittest::kittest::by().role(egui::accesskit::Role::RadioButton)).map(|n| n.rect()).collect();
        assert_eq!(options.len(), 3, "{width}: System, Light, Dark");
        for r in &options {
            assert!(r.width() >= 32.0 && r.height() >= 32.0, "{width}: a theme option's target is only {r:?}");
        }
        // Neighbouring targets never overlap.
        for (a, b) in options.iter().zip(options.iter().skip(1)) {
            assert!(a.intersect(*b).area() <= 0.5, "{width}: {a:?} overlaps {b:?}");
        }
        h.get_by_label("Light").click();
        h.run_steps(3);
        assert_eq!(h.state().as_ref().unwrap().settings.theme, redrob_launcher_ui_egui::theme::ThemeChoice::Light, "{width}");
    }
}

/// The default harness at `width` x 900.
fn harness_sized(width: f32) -> Harness<'static, Option<LauncherUi>> {
    let mut backend = Some(Fake { events: events(), sent: Rc::new(RefCell::new(Vec::new())), manage: true, clock: Rc::new(Cell::new(NOW)) });
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    Harness::builder().with_size(egui::vec2(width, 900.0)).build_ui_state(
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
    )
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

#[test]
fn the_account_menu_holds_settings_quit_and_the_version() {
    for width in [1180.0, 600.0] {
        let mut h = harness_sized(width);
        h.run_steps(6);
        // No account backend: one row reading Settings, no Settings row of its own.
        assert_eq!(h.query_all_by_label("Settings").count(), 1, "{width}");
        let row = h.get_by_label("Settings").rect();
        h.get_by_label("Settings").click();
        h.run_steps(4);
        snapshot(&mut h, &format!("account-menu-no-account-{width}"));
        let version = h.get_by_label(&format!("Redrob Launcher {}", env!("CARGO_PKG_VERSION"))).rect();
        let quit = h.get_by_label("Quit Redrob Launcher").rect();
        let settings = h.get_all_by_label("Settings").map(|n| n.rect()).find(|r| r.bottom() <= row.top() + 0.5).unwrap();
        // Above the row, in order: Settings, Quit, the version.
        assert!(settings.top() < quit.top() && quit.top() < version.top() && version.bottom() <= row.top() + 0.5, "{width}");
        assert!(h.query_by_label("Log in or sign up").is_none(), "{width}: no account item without an account backend");
        // Escape closes it.
        h.key_press(egui::Key::Escape);
        h.run_steps(4);
        assert!(h.query_by_label("Quit Redrob Launcher").is_none(), "{width}");
    }
    let mut h = harness_sized(1180.0);
    h.run_steps(6);
    open_settings(&mut h);
    h.get_by_label("News and source code for every Redrob app");
}

#[test]
fn the_logo_opens_console_redrob_ai() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    assert_eq!(click_opens(&mut h, "Go to console.redrob.ai").as_deref(), Some("https://console.redrob.ai"));
}

#[test]
fn typing_in_the_search_on_another_page_shows_the_apps() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    let on_settings = |h: &Harness<'_, Option<LauncherUi>>| h.query_by_label("News and source code for every Redrob app").is_some();
    open_settings(&mut h);
    assert!(on_settings(&h));
    // The search stays in the title bar on every page.
    let field = h
        .query_all(egui_kittest::kittest::by().role(egui::accesskit::Role::TextInput).label("Search apps"))
        .find(|n| n.rect().bottom() <= 48.0)
        .unwrap();
    field.focus();
    field.type_text("canvas");
    h.run_steps(4);
    assert!(!on_settings(&h), "typing switched to the apps");
    h.get_by_label("Redrob Canvas");
    assert!(h.query_by_label("Redrob Shape").is_none(), "and the grid shows what it finds");
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
fn the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it() {
    let (mut h, _) = harness(egui::Theme::Light, true);
    h.run_steps(6);
    // The cards (each carries its status as its name) and the refresh above them share the page's
    // right edge, 24 px in: the scroll bar sits in that padding, at the window's edge.
    let right = h.query_all_by_label_contains(": Available to install").map(|n| n.rect().right()).fold(0.0, f32::max);
    let refresh = h.get_by_label("Check for updates").rect().right();
    // The harness draws inside an 8 px margin of its own, so its "window" edge is at 1172.
    assert!((right - (1180.0 - 8.0 - 24.0)).abs() <= 1.0, "cards end at {right}");
    assert!((refresh - right).abs() <= 1.0, "refresh {refresh} vs cards {right}");
    // Hovering the page shows the thumb (for the snapshot: it is painted, not a node).
    h.hover_at(egui::pos2(700.0, 500.0));
    h.run_steps(6);
    snapshot(&mut h, "grid-scroll-hover-light");
}

#[test]
fn the_launcher_update_banner_is_one_slim_line() {
    let mut ev = events();
    ev.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    let (mut h, _) = harness_with(ev, egui::Theme::Light, true, 1.0);
    h.run_steps(6);
    // The prompt asks first; "Later" leaves the banner.
    h.get_by_label("Later").click();
    h.run_steps(4);
    snapshot(&mut h, "launcher-update-banner-light");
    h.get_by_label_contains("Redrob Launcher 0.2.0 is available");
    // One line: its controls are Sm (32), so the bar (12 / 6 padding) is 44 px. Under it come the
    // 12 px gap and the filter row's own 12 px inset (its 40 px row around the 30 px cells, plus
    // the pill's padding), so the cells start 68 px under the bar's top: 44 + 12 + 12. A bar even
    // 2 px taller fails (before this, a 66 px bar and a 14 px gap put the cells 92 px down).
    let dismiss = h.get_by_label("Dismiss").rect();
    let action = ["Update and restart", "Download"]
        .iter()
        .flat_map(|l| h.query_all_by_label(l).map(|n| n.rect()).collect::<Vec<_>>())
        .find(|r| (r.center().y - dismiss.center().y).abs() < 1.0)
        .unwrap();
    assert_eq!((dismiss.height(), action.height()), (32.0, 32.0));
    // The filter row's "All" cell: the first thing under the banner.
    let filters = h
        .query_all_by_label_contains("All")
        .map(|n| n.rect())
        .filter(|r| r.top() > dismiss.bottom())
        .min_by(|a, b| a.top().total_cmp(&b.top()))
        .unwrap();
    let bar_top = dismiss.top() - 6.0;
    assert!(filters.top() - bar_top <= 44.0 + 12.0 + 12.0 + 1.0, "banner from {bar_top}, filters at {filters:?}");
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
fn launcher_update_prompts_to_relaunch_with_updated_version() {
    let mut ev = events();
    ev.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    let (mut h, _) = harness_with(ev, egui::Theme::Dark, true, 1.0);
    h.run_steps(6);
    // Dialog pops up on launch with the update prompt
    h.get_by_label_contains("Redrob Launcher 0.2.0 is available");
    h.get_by_label("A new version is available. Would you like to relaunch with the updated version?");
    h.get_by_label("What's new");
    snapshot(&mut h, "launcher-update-dialog-dark");
    // Clicking "Later" dismisses the modal dialog
    h.get_by_label("Later").click();
    h.run_steps(3);
    assert!(h.query_by_label("A new version is available. Would you like to relaunch with the updated version?").is_none());
    // Toolbar update banner is now visible with Update and restart
    h.get_by_label("Update and restart");
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
    assert!(!commands.contains(&Command::UpdateLauncher), "launcher should not be auto-updated when auto_update is false");
}

#[test]
fn settings_name_the_services_behind_what_is_on_screen() {
    let status = |extra: Vec<Event>| {
        let mut evs = events();
        evs.extend(extra);
        let sent = Rc::new(RefCell::new(Vec::new()));
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
        // No footer on the Apps page any more.
        assert!(h.query_by_label_contains("Checked ").is_none(), "the Apps page has no footer");
        open_settings(&mut h);
        let label =
            h.get_by_label_contains("Checked ").accesskit_node().label().or_else(|| h.get_by_label_contains("Checked ").accesskit_node().value());
        label.unwrap_or_default()
    };
    assert_eq!(status(Vec::new()), "Checked 3 min ago · using Redrob API");
    let from_github =
        vec![Event::Latest { app: "redrob-code".into(), latest: LatestRelease::Found(release("redrob-code", "0.3.1", NOW)), from: Origin::GitHub }];
    assert_eq!(status(from_github), "Checked 3 min ago · using Redrob API and GitHub");
}

#[test]
fn web_build_offers_downloads() {
    let (mut h, _) = harness(egui::Theme::Dark, false);
    h.run_steps(8);
    snapshot(&mut h, "web-grid-dark");
    assert!(h.query_all_by_label("Download for macOS").count() >= 7);
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
fn a_native_title_bar_puts_the_chrome_in_a_row_over_the_page() {
    // NoHost: the OS draws the title bar (or there is none, on the web).
    let (mut h, sent) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    snapshot(&mut h, "grid-native-dark");
    assert!(h.query_by_label("Minimize").is_none());
    // No title bar of ours: Update, the logo and the search sit in a 48 px row over the page, left to right.
    let search = search_field_rect(&h);
    let logo = h.get_by_label("Go to console.redrob.ai").rect();
    let update = h.get_by_label("Update all (2)").rect();
    for r in [search, logo, update] {
        assert!(r.bottom() <= 48.0, "the chrome row is 48 px: {r:?}");
    }
    assert!(update.right() <= logo.left() && logo.right() <= search.left(), "{update:?} {logo:?} {search:?}");
    // The Apps page has no top bar of its own: no "Apps" title over the grid, refresh at the filter row's end.
    assert_eq!(h.query_all_by_label("Apps").count(), 1, "only the sidebar's row");
    let check = h.get_by_label("Check for updates").rect();
    let filters = h.get_by_label_contains("Not installed ").rect();
    assert!(check.top() > 48.0 && (check.center().y - filters.center().y).abs() < 4.0, "{check:?} vs {filters:?}");
    assert!(check.left() > filters.right(), "refresh at the right end of the filter row");
    sent.borrow_mut().clear();
    h.get_by_label("Check for updates").click();
    h.run_steps(2);
    assert!(sent.borrow().contains(&Command::CheckForUpdates { force: true }));
}

#[test]
fn the_macos_title_bar_keeps_the_traffic_lights_clear() {
    struct MacHost;
    impl Host for MacHost {
        fn title_bar(&self) -> redrob_launcher_ui_egui::TitleBar {
            redrob_launcher_ui_egui::TitleBar::MacOs
        }
    }
    for width in [600.0, 960.0, 1180.0] {
        let mut backend = Some(Fake { events: events(), sent: Rc::new(RefCell::new(Vec::new())), manage: true, clock: Rc::new(Cell::new(NOW)) });
        let mut h = Harness::builder().with_size(egui::vec2(width, 900.0)).build_ui_state(
            move |ui, state: &mut Option<LauncherUi>| {
                if state.is_none()
                    && let Some(b) = backend.take()
                {
                    *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
                }
                if let Some(l) = state.as_mut() {
                    l.logic(&ui.ctx().clone(), &mut MacHost);
                    l.ui(ui, &mut MacHost);
                }
            },
            None,
        );
        h.run_steps(6);
        if width == 1180.0 {
            snapshot(&mut h, "grid-title-bar-macos-dark");
        }
        // The OS draws the traffic lights: no window buttons of ours.
        assert!(h.query_by_label("Minimize").is_none(), "{width}");
        let search = search_field_rect(&h);
        let logo = h.get_by_label("Go to console.redrob.ai").rect();
        let update = h.get_by_label("Update all (2)").rect();
        // Left to right: Update, the logo, the search, all in the 48 px bar and inside the window.
        assert!(update.right() <= logo.left() && logo.right() <= search.left() && search.right() <= width, "{width}: {update:?} {logo:?} {search:?}");
        for r in [search, logo, update] {
            assert!(r.bottom() <= 48.0, "{width}: {r:?}");
        }
        // The traffic lights' 72 px stay clear, with room to drag the window beside them.
        assert!(update.left() >= 72.0 + 24.0, "{width}: {update:?}");
    }
}

#[test]
fn the_grid_shows_every_app_name_whole_at_the_default_window() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    // Each card's name is one unwrapped line (a wrapped wordmark would be two lines tall) and
    // lies inside the window, beside its icon.
    let mut seen = 0;
    for app in redrob_launcher_core::SUITE {
        for node in h.query_all_by_label(app.name) {
            let r = node.rect();
            if r.top() < 48.0 || r.height() > 30.0 && r.width() > 300.0 {
                continue; // not a card's name (e.g. a whole card or the title bar)
            }
            assert!(r.height() <= 30.0, "{}: wrapped onto a second line: {r:?}", app.name);
            assert!(r.right() <= 1180.0 - 24.0, "{}: runs past the content edge: {r:?}", app.name);
            seen += 1;
        }
    }
    assert!(seen >= 5, "the first section's names were checked ({seen})");
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
    // room to drag left of Update.
    for width in [600.0, 960.0] {
        let mut h = frameless(width);
        h.run_steps(6);
        snapshot(&mut h, &format!("grid-title-bar-dark-{width}"));
        let minimize = h.get_by_label("Minimize").rect();
        let search = search_field_rect(&h);
        let update = h.get_by_label("Update all (2)").rect();
        assert!(search.right() <= minimize.left() && search.width() >= 159.5, "{width}: {search:?}");
        assert!(update.left() >= 24.0, "{width}: {update:?}");
    }
    let mut h = frameless(1180.0);
    h.run_steps(6);
    snapshot(&mut h, "grid-title-bar-dark");
    // The custom bar, right to left: window buttons, search, the group logo, Update.
    let minimize = h.get_by_label("Minimize").rect();
    let search = search_field_rect(&h);
    let logo = h.get_by_label("Go to console.redrob.ai").rect();
    let update = h.get_by_label("Update all (2)").rect();
    assert!(update.right() <= logo.left() && logo.right() <= search.left() && search.right() <= minimize.left());
    // One row, inside the caption buttons' band (whose target starts under the 5 px resize edge).
    for r in [search, logo, update] {
        assert!(r.top() >= minimize.top() - 5.0 && r.bottom() <= minimize.bottom(), "{r:?} vs {minimize:?}");
    }
    // Check for updates moved down to the filter row.
    assert!(h.get_by_label("Check for updates").rect().top() > 48.0);
    assert!(h.get_all_by_label("Apps").all(|n| n.rect().top() >= 48.0), "the sidebar and page start under the title bar");
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

#[test]
fn settings_say_when_github_can_be_asked_again_and_the_platform() {
    for width in [600.0, 1180.0] {
        let mut evs = events();
        evs.push(Event::CheckFinished { at: NOW - 180, source: Source::GitHubApi, rate_limited_until: Some(NOW + 1800) });
        let mut backend = Some(Fake { events: evs, sent: Rc::new(RefCell::new(Vec::new())), manage: true, clock: Rc::new(Cell::new(NOW)) });
        let mut h = Harness::builder().with_size(egui::vec2(width, 900.0)).build_ui_state(
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
        // The Apps page says none of it: the footer is gone.
        for gone in ["in 30 min", "Checked ", "Installing for"] {
            assert!(h.query_by_label_contains(gone).is_none(), "{width}: {gone} is still on the Apps page");
        }
        open_settings(&mut h);
        snapshot(&mut h, &format!("settings-rate-limited-{width}"));
        // Whole, wrapped if it must, and inside the window.
        let notice = h.get_by_label_contains("full details return in 30 min").rect();
        assert!(notice.right() <= width, "{width}: notice {notice:?}");
        let checked = h.get_by_label_contains("Checked ").rect();
        let platform = h.get_by_label_contains("Installing for").rect();
        // Read top to bottom: the check, the limit, the platform.
        assert!(checked.bottom() <= notice.top() && notice.bottom() <= platform.top(), "{width}: {checked:?} {notice:?} {platform:?}");
    }
}
