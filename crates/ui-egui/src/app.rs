//! [`LauncherUi`]: state, event handling and the page layout.

use std::time::Duration;

use redrob_launcher_core::protocol::{
    Backend, BackendOptions, CANCELLED, Capabilities, Command, Event, INSTALL_CANCELLED, JobPhase, LatestRelease, PAUSED, Source,
};
use redrob_launcher_core::release::Release;
use redrob_launcher_core::status::AppStatus;
use redrob_launcher_core::{Platform, SUITE};

use crate::i18n::set_current;
use crate::model::AppEntry;
use crate::settings::{Filter, Settings, SortKey, ViewMode};
use crate::theme::{self, Tokens};

/// Who draws the window's title bar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TitleBar {
    /// Nothing to draw: the OS draws one, or there's no window (the web build).
    #[default]
    Native,
    /// The window has no OS title bar: the header is the title bar (drag it to move the window,
    /// double-click to maximize), with minimize, maximize and close at its right end, and the
    /// window's edges resize it.
    Custom,
    /// macOS: the header runs under a transparent title bar, with the traffic lights at its left.
    MacOs,
}

/// What only the shell can do. Every method has a do-nothing default (the web build).
pub trait Host {
    /// Who draws the title bar.
    fn title_bar(&self) -> TitleBar {
        TitleBar::Native
    }
    /// Shows a system notification.
    fn notify(&mut self, _title: &str, _body: &str) {}
    /// A tray (menu bar) icon exists, so closing the window can keep the launcher running.
    fn has_tray(&self) -> bool {
        false
    }
    /// The OS has desktops (macOS Spaces) the window can be shown on, so the "show on the current
    /// desktop" setting applies.
    fn has_desktops(&self) -> bool {
        false
    }
    /// `Some(enabled)` where starting at login is supported.
    fn login_item(&self) -> Option<bool> {
        None
    }
    fn set_login_item(&mut self, _on: bool) -> Result<(), String> {
        Err("not supported here".into())
    }
    /// Quit the launcher for real (not just hide to the tray).
    fn quit(&mut self) {}
    /// The language every Redrob app should draw in changed (the launcher's own language, or a
    /// per-app override). The desktop shell publishes it in the shared language file.
    fn language_changed(&mut self, _choice: &LanguageChoice) {}
}

/// The language the launcher publishes for the other apps: what it draws in itself (resolved,
/// never `auto`), whether a person chose it, and per-app overrides (app id to code).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LanguageChoice {
    pub language: String,
    pub chosen: bool,
    pub apps: std::collections::BTreeMap<String, String>,
}

/// Which page the main area shows: one per sidebar row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Page {
    Apps,
    /// The models the apps can use on this computer.
    Models,
    /// The shared Redrob Code engine: running or not, who uses it, start and stop.
    Engine,
    /// Signing in to Redrob, and which apps use it.
    Account,
    Settings,
}

pub(crate) struct Toast {
    pub text: String,
    pub error: bool,
    pub until: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Dialog {
    Notes(String),
    ConfirmUninstall(String),
    LauncherUpdate(redrob_launcher_core::Release),
}

pub struct LauncherUi {
    pub(crate) backend: Box<dyn Backend>,
    pub settings: Settings,
    pub(crate) entries: Vec<AppEntry>,
    pub(crate) caps: Capabilities,
    pub(crate) platform: Option<Platform>,
    pub(crate) checking: bool,
    pub(crate) last_source: Option<Source>,
    pub(crate) rate_limited_until: Option<i64>,
    pub(crate) launcher_update: Option<Release>,
    /// Keeping the shared Redrob Code engine current (stop, install, start again).
    pub(crate) engine_update: crate::engine_update::Flow,
    /// The launcher's own update, while it downloads and installs.
    pub(crate) launcher_job: Option<redrob_launcher_core::protocol::JobPhase>,
    /// The new launcher is in place: quit so it can start.
    restart_requested: bool,
    /// News from the release feed, shown inline until dismissed.
    pub(crate) announcements: Vec<redrob_launcher_core::feed::Notice>,
    /// Alerts from the feed, shown as popups until acknowledged.
    pub(crate) alerts: Vec<redrob_launcher_core::feed::Notice>,
    pub(crate) search: String,
    pub(crate) toasts: Vec<Toast>,
    pub(crate) dialog: Option<Dialog>,
    pub(crate) page: Page,
    /// Which tab of the Settings page is open.
    pub(crate) settings_tab: crate::channels::SettingsTab,
    /// Settings > Channels, as edited.
    pub(crate) channels: crate::channels::ChannelsPanel,
    started: bool,
    applied_theme: Option<theme::ThemeChoice>,
    /// Fonts registered with `set_fonts` only exist from the next frame on; drawing with the named
    /// weights before then would abort.
    fonts_ready: bool,
    /// Whether the installed fonts include the system's Chinese font. Added the first time Chinese
    /// is the language in use, then kept for the session.
    cjk_fonts: bool,
    sent_options: Option<BackendOptions>,
    /// Unix seconds of the last look at what's installed (startup counts).
    last_detect: i64,
    /// Whether the window had focus last frame, to re-check when the user comes back to it.
    was_focused: bool,
    /// Versions of apps or the launcher already queued for auto-update during this session,
    /// preventing repeated install loops if an update fails or is in progress.
    pub(crate) auto_update_attempted: std::collections::BTreeSet<(String, String)>,
    /// What was last handed to [`Host::language_changed`], so it is called on a change only.
    published_language: Option<LanguageChoice>,
}

/// The language to draw in: the preference, or English where it needs a Chinese font the system
/// doesn't have (Settings says so), rather than boxes for every character. The only place the
/// Chinese font gets loaded: when Chinese is selected, or Auto resolves to it.
pub(crate) fn effective_lang(pref: &str) -> crate::i18n::Lang {
    let lang = crate::i18n::Lang::from_pref(pref);
    if lang.needs_cjk_font() && !theme::cjk_font_available() { crate::i18n::Lang::EN } else { lang }
}

/// How often to look again at what's installed, to notice apps installed or removed outside
/// the launcher. A detection pass lists packages and reads a few files: cheap, but not free.
const RESCAN_EVERY: i64 = 5 * 60;
/// Coming back to the window re-checks, but not more often than this.
const RESCAN_ON_FOCUS_AFTER: i64 = 30;

impl LauncherUi {
    pub fn new(ctx: &egui::Context, backend: Box<dyn Backend>, settings: Settings) -> Self {
        let cjk_fonts = effective_lang(&settings.language).needs_cjk_font();
        theme::install_fonts(ctx, cjk_fonts);
        egui_extras::install_image_loaders(ctx);
        let caps = backend.capabilities();
        let platform = backend.platform();
        Self {
            backend,
            settings,
            entries: SUITE.iter().map(AppEntry::new).collect(),
            caps,
            platform,
            checking: false,
            last_source: None,
            rate_limited_until: None,
            launcher_update: None,
            engine_update: crate::engine_update::Flow::default(),
            launcher_job: None,
            restart_requested: false,
            announcements: Vec::new(),
            alerts: Vec::new(),
            search: String::new(),
            toasts: Vec::new(),
            dialog: None,
            settings_tab: crate::channels::SettingsTab::default(),
            channels: crate::channels::ChannelsPanel::default(),
            started: false,
            applied_theme: None,
            fonts_ready: false,
            cjk_fonts,
            sent_options: None,
            last_detect: 0,
            was_focused: true,
            auto_update_attempted: std::collections::BTreeSet::new(),
            published_language: None,
            page: Page::Apps,
        }
    }

    /// Moves the UI to another window's context: fonts, image loaders and the theme belong to a
    /// context, and toasts are timed on its clock.
    pub fn attach(&mut self, ctx: &egui::Context) {
        theme::install_fonts(ctx, self.cjk_fonts);
        egui_extras::install_image_loaders(ctx);
        self.fonts_ready = false;
        self.applied_theme = None;
        self.toasts.clear();
    }

    /// Installed apps with an update waiting (the tray shows the count).
    pub fn update_count(&self) -> usize {
        self.entries.iter().filter(|e| e.status(self.platform, self.caps.manage_apps) == AppStatus::UpdateAvailable).count()
    }

    /// Something is downloading or installing, the launcher's own update included: quitting now
    /// would cut it short.
    pub fn busy(&self) -> bool {
        self.launcher_job.is_some() || self.entries.iter().any(|e| e.job.is_some())
    }

    /// Looks for updates now (the tray's "Check for Updates").
    /// Open the engine page (the tray's "Redrob Code engine" item).
    pub fn show_engine(&mut self) {
        if let Some(engine) = self.backend.engine() {
            engine.refresh();
        }
        self.set_page(Page::Engine);
    }

    pub fn check_now(&mut self) {
        self.auto_update_attempted.clear();
        self.backend.send(Command::CheckForUpdates { force: true });
        self.backend.send(Command::Detect);
        self.last_detect = self.backend.now();
    }

    pub(crate) fn send(&mut self, command: Command) {
        self.backend.send(command);
    }

    pub(crate) fn toast(&mut self, ctx: &egui::Context, text: impl Into<String>, error: bool) {
        let now = ctx.input(|i| i.time);
        self.toasts.push(Toast { text: text.into(), error, until: now + if error { 8.0 } else { 4.5 } });
        if self.toasts.len() > 4 {
            self.toasts.remove(0);
        }
    }

    pub(crate) fn entry(&self, id: &str) -> Option<&AppEntry> {
        self.entries.iter().find(|e| e.app.id == id)
    }

    fn entry_mut(&mut self, id: &str) -> Option<&mut AppEntry> {
        self.entries.iter_mut().find(|e| e.app.id == id)
    }

    /// Non-drawing work: runs every frame and also while the window is hidden in the tray.
    pub fn logic(&mut self, ctx: &egui::Context, host: &mut dyn Host) {
        set_current(crate::i18n::Lang::from_pref(&self.settings.language));
        let options = BackendOptions { kinds: self.settings.preferred_kinds.clone(), prereleases: self.settings.prereleases };
        if self.sent_options.as_ref() != Some(&options) {
            self.backend.send(Command::Configure(options.clone()));
            self.sent_options = Some(options);
        }
        if !self.started {
            self.started = true;
            self.backend.send(Command::CheckForUpdates { force: false });
        }
        let now = self.backend.now();
        if self.last_detect == 0 {
            // The backend looks once at startup by itself.
            self.last_detect = now;
        }
        let focused = ctx.input(|i| i.viewport().focused).unwrap_or(true);
        let came_back = focused && !self.was_focused && now - self.last_detect >= RESCAN_ON_FOCUS_AFTER;
        self.was_focused = focused;
        if came_back || now - self.last_detect >= RESCAN_EVERY {
            self.last_detect = now;
            self.backend.send(Command::Detect);
        }
        if let Some(every) = self.settings.check_every.seconds()
            && !self.checking
            && self.settings.last_check.is_none_or(|t| now - t >= every)
        {
            self.backend.send(Command::CheckForUpdates { force: false });
        }
        let mut changed = false;
        for event in self.backend.poll() {
            changed |= self.handle(ctx, event);
        }
        if changed {
            self.announce_updates(host);
            if self.settings.auto_update {
                self.trigger_auto_updates();
            }
        }
        self.engine_update_tick();
        if self.restart_requested {
            // The helper is waiting for this process to exit before it starts the new launcher.
            self.restart_requested = false;
            host.quit();
        }
        // Wake up now and then for scheduled checks, even in the tray.
        ctx.request_repaint_after(Duration::from_secs(60));
    }

    /// Returns whether app state changed.
    fn handle(&mut self, ctx: &egui::Context, event: Event) -> bool {
        match event {
            Event::Latest { app, latest, from } => {
                if let Some(e) = self.entry_mut(&app) {
                    // A failed check keeps the release we already know about (and where it came from).
                    if !(matches!(latest, LatestRelease::Failed(_)) && matches!(e.latest, Some(LatestRelease::Found(_)))) {
                        e.latest = Some(latest);
                        e.origin = Some(from);
                    }
                }
                true
            }
            Event::Detected { app, installed } => {
                if let Some(e) = self.entry_mut(&app) {
                    e.installed = installed;
                    e.detected = true;
                }
                true
            }
            // The product's icon as its latest release ships it; a repaint shows it.
            Event::Icon { app, png } => crate::icons::set_fetched(&app, png),
            Event::Progress { app, phase } => {
                if app == redrob_launcher_core::catalog::LAUNCHER_ID {
                    self.launcher_job = Some(phase);
                    if matches!(self.dialog, Some(Dialog::LauncherUpdate(_))) {
                        self.dialog = None;
                    }
                } else if let Some(e) = self.entry_mut(&app) {
                    e.job = Some(phase);
                }
                false
            }
            Event::LauncherRestarting => {
                self.restart_requested = true;
                false
            }
            Event::Notices { announcements, alerts } => {
                self.announcements = announcements;
                self.alerts = alerts;
                false
            }
            Event::Paused { app, download } => {
                if let Some(e) = self.entry_mut(&app) {
                    e.paused = download;
                }
                false
            }
            Event::ReadyToInstall { app, ready } => {
                if let Some(e) = self.entry_mut(&app) {
                    e.ready = ready;
                }
                false
            }
            Event::JobFinished { app, result } => {
                let launcher = app == redrob_launcher_core::catalog::LAUNCHER_ID;
                // What the job was doing when it ended says what a cancellation called off.
                let last = if launcher { self.launcher_job.take() } else { self.entry(&app).and_then(|e| e.job) };
                if let Some(e) = self.entry_mut(&app) {
                    e.job = None;
                }
                let kept = self.entry(&app).is_some_and(|e| e.ready);
                if app == crate::engine_update::ENGINE_APP
                    && self.engine_update.job_finished(result.is_ok())
                    && let Some(engine) = self.backend.engine()
                {
                    // The daemon was up before the update: bring it back on the new binary.
                    engine.start();
                }
                match result {
                    Ok(msg) => self.toast(ctx, msg, false),
                    // The paused card says it all.
                    Err(e) if e == PAUSED => {}
                    Err(e) if e == CANCELLED && launcher => self.toast(ctx, tl!("Update cancelled"), false),
                    // An install's own cancellations end as INSTALL_CANCELLED, so past the
                    // download this is a removal called off while it waited for the installer.
                    Err(e) if e == CANCELLED && matches!(last, Some(JobPhase::Waiting | JobPhase::Removing)) => {
                        self.toast(ctx, tl!("Uninstall cancelled"), false);
                    }
                    Err(e) if e == CANCELLED => self.toast(ctx, tl!("Download cancelled"), false),
                    Err(e) if e == INSTALL_CANCELLED && kept => {
                        self.toast(ctx, tl!("Install cancelled. The download is kept for the next try."), false);
                    }
                    Err(e) if e == INSTALL_CANCELLED => self.toast(ctx, tl!("Install cancelled"), false),
                    Err(e) => self.toast(ctx, e, true),
                }
                true
            }
            Event::LauncherUpdate(r) => {
                let fresh = self.launcher_update.as_ref().is_none_or(|cur| cur.version != r.version);
                self.launcher_update = Some(r.clone());
                // Ask once per new version, unless automatic updates will install it anyway.
                let automatic = self.settings.auto_update && self.caps.self_update;
                if fresh && !automatic && self.dialog.is_none() && self.launcher_job.is_none() {
                    self.dialog = Some(Dialog::LauncherUpdate(r));
                }
                fresh
            }
            Event::CheckStarted => {
                self.checking = true;
                false
            }
            Event::CheckFinished { at, source, rate_limited_until } => {
                self.checking = false;
                self.settings.last_check = Some(at);
                self.last_source = Some(source);
                self.rate_limited_until = rate_limited_until;
                true
            }
        }
    }

    /// One notification per new version of an installed app.
    fn announce_updates(&mut self, host: &mut dyn Host) {
        let mut fresh = Vec::new();
        for e in &self.entries {
            if e.status(self.platform, self.caps.manage_apps) != AppStatus::UpdateAvailable {
                continue;
            }
            let Some(v) = e.latest_label() else { continue };
            if self.settings.notified.get(e.app.id) != Some(&v) {
                fresh.push((e.app.id, e.app.name, v));
            }
        }
        if fresh.is_empty() {
            return;
        }
        if self.settings.notify {
            let (title, body) = match fresh.as_slice() {
                [(_, name, v)] => {
                    let body: String = tl!("Open Redrob Launcher to update.").to_string();
                    (crate::i18n::fmt(tl!("{name} {v} is available"), &[("name", name), ("v", v)]), body)
                }
                many => {
                    let body: String = many.iter().map(|(_, n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(", ");
                    (crate::i18n::fmt(tl!("{count} updates are available"), &[("count", &many.len().to_string())]), body)
                }
            };
            host.notify(&title, &body);
        }
        for (id, _, version) in fresh {
            self.settings.notified.insert(id.to_string(), version);
        }
    }

    /// When auto-updates are enabled, automatically downloads and installs pending updates.
    pub(crate) fn trigger_auto_updates(&mut self) {
        if !self.caps.manage_apps || !self.settings.auto_update {
            return;
        }
        let to_update: Vec<(String, String)> = self
            .entries
            .iter()
            // The engine is a daemon apps are attached to; `engine_update_tick` updates it.
            .filter(|e| e.app.id != crate::engine_update::ENGINE_APP)
            .filter(|e| e.status(self.platform, true) == AppStatus::UpdateAvailable && e.job.is_none() && !e.ready)
            .filter_map(|e| {
                let v = e.latest_label()?;
                if self.auto_update_attempted.contains(&(e.app.id.to_string(), v.clone())) { None } else { Some((e.app.id.to_string(), v)) }
            })
            .collect();
        for (id, version) in to_update {
            log::info!("auto-update: queuing update for {id} to {version}");
            self.auto_update_attempted.insert((id.clone(), version));
            self.send(Command::Install { app: id });
        }
        if self.caps.self_update
            && let Some(r) = &self.launcher_update
            && self.launcher_job.is_none()
        {
            let id = redrob_launcher_core::catalog::LAUNCHER_ID.to_string();
            let v = r.version_label();
            if !self.auto_update_attempted.contains(&(id.clone(), v.clone())) {
                log::info!("auto-update: queuing launcher self-update to {v}");
                self.auto_update_attempted.insert((id, v));
                self.send(Command::UpdateLauncher);
            }
        }
    }

    /// One frame of keeping the shared engine current: see `engine_update`. Runs every frame; the
    /// engine state it reads is cached by the backend, so it costs nothing most frames.
    pub(crate) fn engine_update_tick(&mut self) {
        use crate::engine_update::{Action, ENGINE_APP, Facts};
        if !self.caps.manage_apps {
            return;
        }
        let Some(entry) = self.entry(ENGINE_APP) else { return };
        let status = entry.status(self.platform, true);
        let newer = (status == AppStatus::UpdateAvailable).then(|| entry.latest_label()).flatten();
        let job_running = entry.job.is_some();
        if self.engine_update.last_seen.as_ref() != Some(&(status.clone(), newer.clone())) {
            log::info!(
                "engine update: status {status:?}, installed {:?}, latest {:?}, step {:?}",
                entry.installed_label(),
                entry.latest_label(),
                self.engine_update.step
            );
            self.engine_update.last_seen = Some((status, newer.clone()));
        }
        if newer.is_none() {
            self.engine_update.settled();
        }
        if !(self.settings.engine_auto_update || self.engine_update.forced) && self.engine_update.step == crate::engine_update::Step::Idle {
            // Off: the dialog still shows the newer version and its "Update now".
            self.engine_update.waiting_on.clear();
            return;
        }
        let Some(engine) = self.backend.engine() else { return };
        let facts = Facts {
            attempted: newer.as_ref().is_some_and(|v| self.engine_update.attempted.contains(v)),
            newer,
            job_running,
            engine: engine.state(),
            forced: self.engine_update.forced,
        };
        let before = self.engine_update.waiting_on.clone();
        match self.engine_update.tick(&facts) {
            Action::None => {}
            Action::Wait { apps } => {
                if apps != before {
                    log::info!(
                        "engine update: {} is newer, waiting until no app is attached (now: {})",
                        facts.newer.as_deref().unwrap_or("?"),
                        apps.join(", ")
                    );
                }
            }
            Action::Stop => {
                log::info!("engine update: stopping the daemon before installing {}", facts.newer.as_deref().unwrap_or("?"));
                if let Some(engine) = self.backend.engine() {
                    engine.stop();
                }
            }
            Action::Install { restart } => {
                log::info!("engine update: installing {} (restart after: {restart})", facts.newer.as_deref().unwrap_or("?"));
                self.send(Command::Install { app: ENGINE_APP.to_string() });
            }
        }
    }

    /// The language to publish for the other apps: the one the launcher draws in, `chosen` unless
    /// the setting is `auto`, with the per-app overrides from the settings.
    pub fn language_choice(&self) -> LanguageChoice {
        LanguageChoice {
            language: effective_lang(&self.settings.language).code().to_string(),
            chosen: crate::i18n::Lang::from_code(&self.settings.language).is_some(),
            apps: self.settings.app_languages.clone(),
        }
    }

    /// Draws the launcher.
    pub fn ui(&mut self, ui: &mut egui::Ui, host: &mut dyn Host) {
        let ctx = ui.ctx().clone();
        // Set the current language for this frame based on settings.
        let lang = effective_lang(&self.settings.language);
        set_current(lang);
        let choice = self.language_choice();
        if self.published_language.as_ref() != Some(&choice) {
            host.language_changed(&choice);
            self.published_language = Some(choice);
        }
        // `effective_lang` only picks Chinese once its font has loaded.
        if lang.needs_cjk_font() && !self.cjk_fonts {
            theme::install_fonts(&ctx, true);
            self.cjk_fonts = true;
            self.fonts_ready = false;
        }
        if !self.fonts_ready {
            self.fonts_ready = true;
            ctx.request_repaint();
            return;
        }
        if self.applied_theme != Some(self.settings.theme) {
            theme::apply(&ctx, self.settings.theme);
            self.applied_theme = Some(self.settings.theme);
        }
        let tokens = Tokens::get(&ctx);
        if self.entries.iter().any(|e| e.job.is_some()) || self.checking {
            ctx.request_repaint_after(Duration::from_millis(50));
        }

        // The title bar across the top (Redrob Cowork's 48 px bar on the sunken ground, its hairline
        // under it), then the sidebar card down the left; the footer and each page's top bar span
        // the main area only, on the same base ground as the card's gap.
        // A window the OS decorates (and the web build) has no chrome of ours to draw: no
        // header, and Check for updates moves into the page's top bar.
        let bar = host.title_bar();
        if bar != TitleBar::Native {
            egui::Panel::top("header")
                .frame(egui::Frame::new().fill(tokens.sunken))
                .exact_size(crate::views::HEADER_H)
                .resizable(false)
                .show(ui, |ui| self.header(ui, bar));
        }
        let inset = crate::shell::INSET as i8;
        egui::Panel::left("sidebar")
            .frame(egui::Frame::new().fill(tokens.bg).inner_margin(egui::Margin { left: inset, right: inset, top: inset, bottom: inset }))
            .exact_size(Self::sidebar_width(&ctx))
            .resizable(false)
            .show_separator_line(false)
            .show(ui, |ui| self.sidebar(ui));
        egui::Panel::bottom("footer")
            .frame(egui::Frame::new().fill(tokens.bg).inner_margin(egui::Margin::symmetric(24, 0)))
            .exact_size(crate::views::FOOTER_H)
            .resizable(false)
            .show(ui, |ui| self.footer(ui));
        egui::Panel::top("page-top")
            .frame(egui::Frame::new().fill(tokens.bg).inner_margin(egui::Margin::symmetric(24, 0)))
            .exact_size(crate::shell::PAGE_TOP_H)
            .resizable(false)
            .show(ui, |ui| self.page_top_bar(ui, bar));
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(tokens.bg).inner_margin(egui::Margin { left: 24, right: 24, top: 20, bottom: 0 }))
            .show(ui, |ui| match self.page {
                Page::Models => self.models_page(ui),
                Page::Engine | Page::Account | Page::Settings => {
                    egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                        // A reading measure, like Cowork's content column.
                        ui.set_max_width(720.0);
                        match self.page {
                            Page::Engine => self.engine_page(ui),
                            Page::Account => self.account_page(ui),
                            _ => self.settings_page(ui, host),
                        }
                        // Room under the last control, so nothing ends against the footer.
                        ui.add_space(32.0);
                    });
                }
                Page::Apps => {
                    self.toolbar(ui);
                    ui.add_space(16.0);
                    let ids = self.visible_ids();
                    // The table is wider than a narrow window: let it scroll sideways too.
                    let scroll = match self.settings.view {
                        ViewMode::Grid => egui::ScrollArea::vertical(),
                        ViewMode::Table => egui::ScrollArea::both(),
                    };
                    scroll.auto_shrink(false).show(ui, |ui| {
                        if ids.is_empty() {
                            self.empty_state(ui);
                        } else {
                            match self.settings.view {
                                ViewMode::Grid => self.grid(ui, &ids),
                                ViewMode::Table => self.table(ui, &ids),
                            }
                        }
                        ui.add_space(16.0);
                    });
                }
            });
        // Escape leaves a page for Apps, when nothing above the page (a dialog, the feed alert, a
        // menu or a list) would take the key itself.
        let alert_open = self.alerts.iter().any(|a| !self.settings.dismissed_notices.contains(&a.id));
        if self.page != Page::Apps
            && self.dialog.is_none()
            && !alert_open
            && !egui::Popup::is_any_open(&ctx)
            && ctx.memory(|m| m.focused().is_none())
            && ctx.input(|i| i.key_pressed(egui::Key::Escape))
        {
            self.set_page(Page::Apps);
        }
        self.dialogs(&ctx);
        self.draw_toasts(&ctx);
        if host.title_bar() == TitleBar::Custom {
            crate::widgets::window_edges(&ctx);
        }
    }

    /// The ids to show, filtered and sorted.
    pub(crate) fn visible_ids(&self) -> Vec<&'static str> {
        let needle = self.search.trim().to_lowercase();
        let mut list: Vec<(usize, &AppEntry)> = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                let s = e.status(self.platform, self.caps.manage_apps);
                let keep = match self.settings.filter {
                    Filter::All => true,
                    Filter::Installed => s.is_installed(),
                    Filter::Updates => s == AppStatus::UpdateAvailable,
                    Filter::NotInstalled => !s.is_installed(),
                };
                keep && (needle.is_empty()
                    || e.app.name.to_lowercase().contains(&needle)
                    || e.app.tagline.to_lowercase().contains(&needle)
                    || e.app.like.to_lowercase().contains(&needle))
            })
            .collect();
        let platform = self.platform;
        let manage = self.caps.manage_apps;
        let desc = self.settings.sort_descending;
        let dir = |o: std::cmp::Ordering| if desc { o.reverse() } else { o };
        // Empty cells (not installed, no release) go last whichever way the column sorts.
        let opt = |a: Option<std::cmp::Ordering>, a_some: bool, b_some: bool| match (a_some, b_some) {
            (true, true) => dir(a.unwrap_or(std::cmp::Ordering::Equal)),
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (false, false) => std::cmp::Ordering::Equal,
        };
        let installed = |e: &AppEntry| e.installed.as_ref().and_then(|i| i.version.clone());
        let latest = |e: &AppEntry| e.release().and_then(|r| r.version.clone());
        list.sort_by(|(ia, a), (ib, b)| {
            let ord = match self.settings.sort {
                SortKey::Suite => dir(ia.cmp(ib)),
                SortKey::Name => dir(a.app.name.cmp(b.app.name)),
                SortKey::InspiredBy => dir(a.app.like.cmp(b.app.like)),
                SortKey::Status => {
                    dir(crate::model::status_rank(&a.status(platform, manage)).cmp(&crate::model::status_rank(&b.status(platform, manage))))
                }
                SortKey::Installed => {
                    let (x, y) = (installed(a), installed(b));
                    opt(x.as_ref().zip(y.as_ref()).map(|(x, y)| x.cmp_precedence(y)), x.is_some(), y.is_some())
                }
                SortKey::Latest => {
                    let (x, y) = (latest(a), latest(b));
                    opt(x.as_ref().zip(y.as_ref()).map(|(x, y)| x.cmp_precedence(y)), x.is_some(), y.is_some())
                }
                SortKey::Released => {
                    let at = |e: &AppEntry| e.release().and_then(|r| r.published_at);
                    let (x, y) = (at(a), at(b));
                    opt(x.zip(y).map(|(x, y)| x.cmp(&y)), x.is_some(), y.is_some())
                }
            };
            // Ties keep suite order.
            ord.then(ia.cmp(ib))
        });
        list.into_iter().map(|(_, e)| e.app.id).collect()
    }

    /// "Redrob API", "GitHub" or "Redrob API and GitHub": the services behind the releases on
    /// screen right now. `None` until something has loaded.
    pub(crate) fn sources_label(&self) -> Option<String> {
        use redrob_launcher_core::protocol::Origin;
        let mut api = false;
        let mut github = false;
        for e in &self.entries {
            if matches!(e.latest, Some(LatestRelease::Found(_) | LatestRelease::None)) {
                match e.origin {
                    Some(Origin::RedrobApi) => api = true,
                    Some(Origin::GitHub) => github = true,
                    None => {}
                }
            }
        }
        match (api, github) {
            (true, true) => Some(crate::i18n::fmt(tl!("{a} and {b}"), &[("a", Origin::RedrobApi.label()), ("b", Origin::GitHub.label())])),
            (true, false) => Some(Origin::RedrobApi.label().to_string()),
            (false, true) => Some(Origin::GitHub.label().to_string()),
            (false, false) => None,
        }
    }

    pub(crate) fn count(&self, filter: Filter) -> usize {
        self.entries
            .iter()
            .filter(|e| {
                let s = e.status(self.platform, self.caps.manage_apps);
                match filter {
                    Filter::All => true,
                    Filter::Installed => s.is_installed(),
                    Filter::Updates => s == AppStatus::UpdateAvailable,
                    Filter::NotInstalled => !s.is_installed(),
                }
            })
            .count()
    }

    /// DS Toasts: 360 wide, raised, radius 8, shadow-lg, a 20 px status icon in its colour, the
    /// text, and a close button. Clicking a toast also dismisses it.
    fn draw_toasts(&mut self, ctx: &egui::Context) {
        use crate::glyphs::Icon;
        let now = ctx.input(|i| i.time);
        self.toasts.retain(|t| t.until > now);
        if self.toasts.is_empty() {
            return;
        }
        ctx.request_repaint_after(Duration::from_millis(250));
        let t = Tokens::get(ctx);
        const W: f32 = 360.0;
        let anchor = egui::vec2(-24.0, -(crate::views::FOOTER_H + 16.0));
        egui::Area::new(egui::Id::new("toasts")).anchor(egui::Align2::RIGHT_BOTTOM, anchor).order(egui::Order::Foreground).show(ctx, |ui| {
            ui.set_width(W);
            let mut dismiss = None;
            for (i, toast) in self.toasts.iter().enumerate() {
                let (icon, tone) = if toast.error { (Icon::Close, t.bad) } else { (Icon::Check, t.good) };
                let r = egui::Frame::new()
                    .fill(t.raised())
                    .stroke(t.hairline())
                    .corner_radius(egui::CornerRadius::same(theme::radius::MD))
                    .shadow(theme::shadow_lg(&t))
                    .inner_margin(16)
                    .show(ui, |ui| {
                        ui.set_width(W - 32.0);
                        ui.horizontal_top(|ui| {
                            ui.spacing_mut().item_spacing.x = theme::space::S3;
                            let (r, _) = ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());
                            crate::glyphs::paint(ui.painter(), r, icon, tone);
                            let text_w = W - 32.0 - 20.0 - theme::control::SM - 2.0 * theme::space::S3;
                            ui.allocate_ui(egui::vec2(text_w, 0.0), |ui| {
                                ui.set_width(text_w);
                                ui.add(
                                    egui::Label::new(egui::RichText::new(crate::i18n::msg(&toast.text)).size(theme::text::BODY_SM).color(t.ink))
                                        .wrap(),
                                );
                            });
                            crate::widgets::icon_button(ui, Icon::Close, tl!("Dismiss notification"), theme::control::SM).clicked()
                        })
                        .inner
                    });
                if r.inner || r.response.interact(egui::Sense::click()).clicked() {
                    dismiss = Some(i);
                }
                ui.add_space(theme::space::S2);
            }
            if let Some(i) = dismiss {
                self.toasts.remove(i);
            }
        });
    }
}
