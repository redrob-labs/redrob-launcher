//! The header, toolbar and the grid of cards, laid out like the apps page on
//! the Redrob design system: hairline cells, quiet labels and square controls.

use egui::{Color32, CornerRadius, FontId, Rect, RichText, Sense, Stroke, Ui, vec2};
use redrob_launcher_core::platform::pick_asset;
use redrob_launcher_core::protocol::{Command, JobPhase};
use redrob_launcher_core::status::AppStatus;
use redrob_launcher_core::time;

use crate::app::{Dialog, LauncherUi, Page, TitleBar};
use crate::glyphs::Icon;
use crate::i18n::{current, fmt, t, tn, tr_ctx};
use crate::model::{self, AppEntry};
use crate::settings::Filter;
use crate::theme::{self, Tokens};
use crate::widgets::{self, Cell, Kind};

/// Translate a relative timestamp ("just now", "5 min ago", …) into the current language.
pub(crate) fn ago_text(secs: i64, now: i64) -> String {
    match time::ago_kind(secs, now) {
        time::TimeAgo::JustNow => t("just now").to_string(),
        time::TimeAgo::Minutes(n) => tn(n, "{n} min ago", "{n} min ago"),
        time::TimeAgo::Hours(n) => tn(n, "{n} h ago", "{n} h ago"),
        time::TimeAgo::Yesterday => t("yesterday").to_string(),
        time::TimeAgo::Days(n) => tn(n, "{n} day ago", "{n} days ago"),
        time::TimeAgo::Date(secs) => {
            let (y, m, d) = time::civil_from_unix(secs);
            let month = tr_ctx(current(), "month", time::month_short_en(m));
            fmt(tl!("{month} {day}, {year}"), &[("month", month), ("day", &d.to_string()), ("year", &y.to_string())])
        }
    }
}

/// Translate an absolute date ("Oct 8, 2026" → "8 окт. 2026") into the current language.
pub(crate) fn date_text(secs: i64) -> String {
    let (y, m, d) = time::civil_from_unix(secs);
    let month = tr_ctx(current(), "month", time::month_short_en(m));
    fmt(tl!("{month} {day}, {year}"), &[("month", month), ("day", &d.to_string()), ("year", &y.to_string())])
}

/// What the main button of an app does.
enum Primary {
    Install,
    /// The download is already here; only the install step is left. Carries the button's label.
    Retry(String),
    Update(String),
    Open,
    Download(String),
    Unavailable(&'static str),
}

/// Height of the header bar (the site's nav).
pub(crate) const HEADER_H: f32 = 48.0;

/// Three tiles across the main area beside the sidebar at the default 1180 px window (four once
/// it is wider).
const CARD_MIN_W: f32 = 240.0;
/// The gap between a tile's name and its description.
const CARD_LINE_GAP: f32 = 2.0;
/// The tallest thing on a tile's actions row (the main button, a progress bar or the … button).
const CARD_ACTION_H: f32 = theme::control::MD;
/// The space between tiles (DS card gap in a dense grid: space-4).
const CARD_GAP: f32 = 16.0;
/// DS card padding, dense (space-4).
const CARD_PAD: f32 = 16.0;
/// The app icon on a tile (48 before #83, then 40).
const CARD_ICON: f32 = 36.0;
/// The icon buttons beside an app's main action (pause, cancel, More actions): DS IconButton sm.
pub(crate) const ACTION_ICON: f32 = theme::control::SM;
/// Room macOS's traffic lights take at the left of the header.
const TRAFFIC_LIGHTS_W: f32 = 72.0;
/// Where the group logo leads (Redrob Cowork's title bar does the same).
const CONSOLE_URL: &str = "https://console.redrob.ai";
/// The search field's width: Cowork's `clamp(260px, 29vw, 416px)`, and the least it shrinks to in a
/// narrow window.
const SEARCH_MIN_W: f32 = 160.0;
const SEARCH_W: std::ops::RangeInclusive<f32> = 260.0..=416.0;
/// Empty title bar left of the controls, so the window can always be dragged.
const DRAG_MIN_W: f32 = 24.0;
/// Debug builds only: set to a number (`REDROB_LAUNCHER_SAMPLE_UPDATES=2`) to preview the filter
/// row's Update all button with that many sample updates, even when nothing needs updating.
const SAMPLE_UPDATES_ENV: &str = "REDROB_LAUNCHER_SAMPLE_UPDATES";

/// How many sample updates [`SAMPLE_UPDATES_ENV`] asks for, read once. Always 0 in release builds,
/// so the preview can never ship.
fn sample_updates() -> usize {
    #[cfg(debug_assertions)]
    {
        static SAMPLE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
        *SAMPLE.get_or_init(|| std::env::var(SAMPLE_UPDATES_ENV).ok().and_then(|v| v.trim().parse().ok()).unwrap_or(0))
    }
    #[cfg(not(debug_assertions))]
    {
        0
    }
}

/// The update-check status lines Settings shows (see [`LauncherUi::update_status`]).
pub(crate) struct UpdateStatus {
    /// "Checked 3 min ago · using Redrob API", "Checking for updates…" or "Not checked yet".
    pub checked: String,
    /// GitHub's rate-limit notice, only while the limit applies.
    pub rate_limit: Option<String>,
    /// "Installing for Windows x64" and friends.
    pub platform: String,
}

/// A run of cards, under an optional meta-line title.
type Section = (Option<&'static str>, Vec<&'static str>);

impl LauncherUi {
    /// The title bar (Redrob Cowork's): with a custom or macOS title bar the whole bar drags the
    /// window outside its controls. At its right end, right to left: the window buttons (custom
    /// bar), the search and the Redrob group logo (console.redrob.ai). With
    /// `TitleBar::Native` (the OS's bar, or the web) the same cluster is a 48 px row over the page.
    pub(crate) fn header(&mut self, ui: &mut Ui, bar: TitleBar) {
        let h = HEADER_H;
        if bar != TitleBar::Native {
            widgets::title_bar_drag(ui);
        }
        let bar_w = ui.max_rect().width();
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            if bar == TitleBar::MacOs {
                ui.add_space(TRAFFIC_LIGHTS_W);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                match bar {
                    TitleBar::Custom => {
                        widgets::window_buttons(ui, h);
                        ui.add_space(theme::space::S2);
                    }
                    TitleBar::MacOs => ui.add_space(theme::space::S4),
                    TitleBar::Native => {}
                }
                self.chrome_cluster(ui, bar_w);
            });
        });
    }

    /// Search and the group logo, right to left. The search shrinks first in a narrow window (never
    /// under [`SEARCH_MIN_W`]), leaving the bar room to drag. Update all lives in the filter row
    /// ([`Self::toolbar`]), so the chrome carries no action button.
    fn chrome_cluster(&mut self, ui: &mut Ui, bar_w: f32) {
        let gap = theme::space::S2;
        let room = ui.available_width() - theme::control::SM - gap - DRAG_MIN_W;
        let search_w = (bar_w * 0.29).clamp(*SEARCH_W.start(), *SEARCH_W.end()).min(room).max(SEARCH_MIN_W);
        // Typing here from another page shows the apps it finds.
        if widgets::search_field(ui, &mut self.search, tl!("Search apps"), search_w, "apps").changed() && self.page != Page::Apps {
            self.set_page(Page::Apps);
        }
        ui.add_space(gap);
        if widgets::logo_button(ui, crate::icons::redrob_mark(), tl!("Go to console.redrob.ai")).clicked() {
            ui.ctx().open_url(egui::OpenUrl::new_tab(CONSOLE_URL));
        }
    }

    /// The filter row's update button, drawn just left of Check for updates: "Update 1 app" or
    /// "Update all (N)", only when something can be updated. Debug builds add
    /// [`sample_updates`] to the count so the button can be previewed with nothing to update.
    ///
    /// * `ui` - the filter row's right-to-left layout; the button lands left of what came before.
    fn update_all_button(&mut self, ui: &mut Ui) {
        let real = if self.caps.manage_apps { self.count(Filter::Updates) } else { 0 };
        let sample = sample_updates();
        let updates = real + sample;
        if updates == 0 {
            return;
        }
        let label = if updates == 1 { tl!("Update 1 app").to_string() } else { fmt(tl!("Update all ({count})"), &[("count", &updates.to_string())]) };
        // The row's own 8 px item spacing separates it from Refresh.
        // Tinted, not filled: a quiet action beside the Refresh icon, the same 32 px tall.
        let resp = widgets::button_sized(ui, &label, Kind::Tinted, widgets::Size::Sm, 0.0);
        // Debug-only preview text, never shipped, so it is left untranslated.
        let resp = if sample > 0 { resp.on_hover_text(format!("Preview: {sample} sample update(s) from {SAMPLE_UPDATES_ENV}")) } else { resp };
        if resp.clicked() {
            // Sample updates have no app behind them: only real ones are installed.
            self.update_all();
        }
    }

    /// Updates every installed app that has an update and isn't busy.
    fn update_all(&mut self) {
        let ids: Vec<&str> = self
            .entries
            .iter()
            .filter(|e| e.status(self.platform, true) == AppStatus::UpdateAvailable && e.job.is_none())
            .map(|e| e.app.id)
            .collect();
        for id in ids {
            self.send(Command::Install { app: id.to_string() });
        }
    }

    pub(crate) fn toolbar(&mut self, ui: &mut Ui) {
        let t = Tokens::get(ui.ctx());
        if self.dialog.is_none()
            && let Some(r) = self.launcher_update.clone()
        {
            widgets::alert_frame_dense(&t).show(ui, |ui| {
                // A row is at least interact_size tall: 32 here (the Sm buttons), not the page's 40.
                ui.spacing_mut().interact_size.y = theme::control::SM;
                ui.horizontal(|ui| {
                    // A one-line DS Alert: a 16 px icon in the tone's ink, then the title, all on a
                    // 32 px row (the buttons and the dismiss are Sm).
                    let (icon, _) = ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
                    crate::glyphs::paint(ui.painter(), icon, Icon::Download, t.accent_ink);
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(fmt(tl!("Redrob Launcher {version} is available."), &[("version", &r.version_label())]))
                            .family(theme::semibold())
                            .color(t.ink_strong),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if let Some(phase) = self.launcher_job {
                            let (fraction, caption) = match phase {
                                JobPhase::Downloading { done, total } if total > 0 => (
                                    Some(done as f32 / total as f32),
                                    fmt(
                                        tl!("Downloading {done} of {total}"),
                                        &[("done", &widgets::human_bytes(done)), ("total", &widgets::human_bytes(total))],
                                    ),
                                ),
                                JobPhase::Downloading { .. } | JobPhase::Reconnecting { .. } => (None, tl!("Downloading…").to_string()),
                                JobPhase::Verifying => (None, tl!("Verifying…").to_string()),
                                JobPhase::Waiting => (None, tl!("Waiting for other installs…").to_string()),
                                JobPhase::Installing | JobPhase::Removing => (None, tl!("Installing, then restarting…").to_string()),
                            };
                            // Another app's install can hang on a password prompt: the wait can be called off.
                            if phase == JobPhase::Waiting
                                && widgets::icon_button(ui, crate::glyphs::Icon::Close, tl!("Cancel update"), theme::control::SM)
                                    .on_hover_text(tl!("Stop waiting; the update can be started again later"))
                                    .clicked()
                            {
                                self.send(Command::Cancel { app: redrob_launcher_core::catalog::LAUNCHER_ID.to_string() });
                            }
                            widgets::progress(ui, fraction, &caption, t.accent, 220.0);
                            return;
                        }
                        if widgets::icon_button(ui, crate::glyphs::Icon::Close, tl!("Dismiss"), theme::control::SM).clicked() {
                            self.launcher_update = None;
                        }
                        if self.caps.self_update {
                            if widgets::button_sized(ui, tl!("Update and restart"), Kind::Primary, widgets::Size::Sm, 0.0)
                                .on_hover_text(tl!("Downloads the new version, replaces this one and opens it again"))
                                .clicked()
                            {
                                self.send(Command::UpdateLauncher);
                            }
                        } else {
                            let url = self.platform.and_then(|p| launcher_download(&r, p)).unwrap_or_else(|| r.html_url.clone());
                            if widgets::button_sized(ui, tl!("Download"), Kind::Primary, widgets::Size::Sm, 0.0).clicked() {
                                ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                            }
                        }
                        ui.add_space(8.0);
                        widgets::hud_link(ui, tl!("What's new"), &r.html_url);
                    });
                });
            });
            ui.add_space(theme::space::S3);
        }
        self.announcements(ui);
        ui.horizontal(|ui| {
            let filters: Vec<(Filter, Cell<'_>)> = [
                (Filter::All, tl!("All"), tl!("Every Redrob app"), None),
                // The tab names a group ("installed ones"); the status pill says "Installed".
                (Filter::Installed, tr_ctx(current(), "filter", "Installed"), tl!("Apps on this computer"), None),
                (Filter::Updates, tl!("Updates"), tl!("Installed apps with a newer version"), Some(t.warn)),
                (Filter::NotInstalled, tl!("Not installed"), tl!("Apps you can install"), None),
            ]
            .into_iter()
            .filter(|(f, ..)| self.caps.manage_apps || *f == Filter::All)
            .map(|(f, label, tip, highlight)| (f, Cell { label, tip, count: Some((self.count(f), highlight)) }))
            .collect();
            widgets::cells(ui, &mut self.settings.filter, &filters);
            // Check for updates at the row's right end (where the sort was), Update all left of it.
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if widgets::icon_button_busy(ui, Icon::Refresh, tl!("Check for updates"), theme::control::SM, self.checking).clicked() {
                    self.check_now();
                }
                self.update_all_button(ui);
            });
        });
    }

    /// The DS EmptyState for a filter or search that matches nothing: left aligned, a 48 px
    /// brand-subtle disc with the search glyph, a title, what to relax, and the way back.
    pub(crate) fn empty_state(&mut self, ui: &mut Ui) {
        let t = Tokens::get(ui.ctx());
        let query = self.search.trim().to_string();
        ui.add_space(theme::space::S12);
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = theme::space::S3;
            let (disc, _) = ui.allocate_exact_size(vec2(48.0, 48.0), Sense::hover());
            ui.painter().circle_filled(disc.center(), 24.0, t.brand_subtle());
            crate::glyphs::paint(ui.painter(), Rect::from_center_size(disc.center(), vec2(22.0, 22.0)), Icon::Search, t.accent_ink);
            ui.label(widgets::heading(tl!("No apps match"), theme::text::TITLE_3, t.ink));
            let text = if query.is_empty() {
                tl!("No app is in this group right now. Show every app instead.").to_string()
            } else {
                fmt(tl!("Nothing matches “{query}”. Try a shorter or different word."), &[("query", &query)])
            };
            ui.scope(|ui| {
                ui.set_max_width(420.0);
                ui.add(egui::Label::new(RichText::new(text).size(theme::text::BODY_SM).color(t.muted)).wrap());
            });
            ui.horizontal(|ui| {
                if !query.is_empty() && widgets::button(ui, tl!("Clear search"), Kind::Secondary, 0.0).clicked() {
                    self.search.clear();
                }
                if self.settings.filter != Filter::All && widgets::button(ui, tl!("Show all apps"), Kind::Secondary, 0.0).clicked() {
                    self.settings.filter = Filter::All;
                }
            });
        });
    }

    /// News from the release feed (and app renames), until the user dismisses each.
    fn announcements(&mut self, ui: &mut Ui) {
        let t = Tokens::get(ui.ctx());
        let shown: Vec<redrob_launcher_core::feed::Notice> =
            self.announcements.iter().filter(|n| !self.settings.dismissed_notices.contains(&n.id)).take(3).cloned().collect();
        for notice in shown {
            widgets::alert_frame(&t).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        if widgets::icon_button(ui, crate::glyphs::Icon::Close, tl!("Dismiss"), theme::control::SM).clicked() {
                            self.settings.dismissed_notices.insert(notice.id.clone());
                        }
                        if let Some(link) = &notice.link {
                            widgets::hud_link(ui, tl!("Learn more"), link);
                        }
                        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                            ui.spacing_mut().item_spacing.y = 2.0;
                            if !notice.title.is_empty() {
                                ui.label(RichText::new(&notice.title).family(theme::semibold()).color(t.ink_strong));
                            }
                            if !notice.body.is_empty() {
                                ui.add(egui::Label::new(RichText::new(&notice.body).color(t.muted)).wrap());
                            }
                        });
                    });
                });
            });
            ui.add_space(8.0);
        }
    }

    /// What the footer used to say, now shown in Settings > Updates: when the catalog was last
    /// checked and from where, GitHub's rate-limit notice while it applies, and the platform apps
    /// are installed (or downloaded) for.
    pub(crate) fn update_status(&self) -> UpdateStatus {
        let now = self.backend.now();
        let checked = if self.checking {
            tl!("Checking for updates…").to_string()
        } else if let Some(at) = self.settings.last_check {
            let ago = ago_text(at, now);
            match self.sources_label() {
                Some(using) => fmt(tl!("Checked {ago} · using {sources}"), &[("ago", &ago), ("sources", &using)]),
                None => fmt(tl!("Checked {ago}"), &[("ago", &ago)]),
            }
        } else {
            tl!("Not checked yet").to_string()
        };
        let rate_limit = self.rate_limited_until.filter(|u| *u > now).map(|until| {
            let min = ((until - now) / 60).max(1).to_string();
            fmt(tl!("GitHub's hourly limit was reached; full details return in {min} min"), &[("min", &min)])
        });
        let platform = match self.platform {
            Some(p) if self.caps.manage_apps => fmt(tl!("Installing for {platform}"), &[("platform", &p.label())]),
            Some(p) => fmt(tl!("Downloads for {platform}"), &[("platform", &p.label())]),
            None => tl!("No downloads for this system").into(),
        };
        UpdateStatus { checked, rate_limit, platform }
    }

    // ---- grid ----------------------------------------------------------------------------

    pub(crate) fn grid(&mut self, ui: &mut Ui, ids: &[&'static str]) {
        let width = ui.available_width();
        // DS ConnectorCard tiles, 16 px apart (no shared hairlines).
        let cols = (((width + CARD_GAP) / (CARD_MIN_W + CARD_GAP)).floor() as usize).max(1);
        let card_w = (width - (cols as f32 - 1.0) * CARD_GAP) / cols as f32;
        // Every app, unsearched: on its shelf, in page order. One voice across the three headings.
        let grouped = self.settings.filter == Filter::All && self.search.trim().is_empty();
        let sections: Vec<Section> = if grouped {
            use crate::shelves::{Shelf, shelf_of};
            Shelf::ALL
                .iter()
                .map(|shelf| {
                    let title = match shelf {
                        Shelf::Engine => tl!("For your AI engine"),
                        Shelf::Work => tl!("For your everyday work"),
                        Shelf::Creative => tl!("For your creative work"),
                    };
                    let list = ids.iter().copied().filter(|id| self.entry(id).is_some_and(|e| shelf_of(e.app) == *shelf)).collect();
                    (Some(title), list)
                })
                .collect()
        } else {
            vec![(None, ids.to_vec())]
        };
        let t = Tokens::get(ui.ctx());
        // Every tile is as tall as the one whose description wraps to the most lines.
        let card_h = card_height(ui, ids, card_w);
        for (title, list) in sections {
            if list.is_empty() {
                continue;
            }
            if let Some(title) = title {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    widgets::shelf_heading(ui, title);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        widgets::hud(ui, &tn(list.len() as u64, "{n} app", "{n} apps"), t.faint);
                    });
                });
                ui.add_space(theme::space::S3);
            }
            let rows = list.len().div_ceil(cols);
            let height = rows as f32 * (card_h + CARD_GAP) - CARD_GAP;
            let (area, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
            for (i, id) in list.iter().enumerate() {
                let (row, col) = (i / cols, i % cols);
                let min = area.min + vec2(col as f32 * (card_w + CARD_GAP), row as f32 * (card_h + CARD_GAP));
                self.card(ui, id, Rect::from_min_size(min, vec2(card_w, card_h)));
            }
            ui.add_space(theme::space::S6);
        }
    }

    /// One app as a DS ConnectorCard: raised, radius 12, a hairline that strengthens with a
    /// shadow-md on hover (140 ms); the icon exactly as supplied (no shadow, border or tint) beside
    /// the name and its full short description; the version; the actions. No status chip and no links: the
    /// button says what can be done, and every card exposes its menu through both the named …
    /// button and right-click. The product's colour is only its icon.
    fn card(&mut self, ui: &mut Ui, id: &str, rect: Rect) {
        let Some(entry) = self.entry(id).cloned() else { return };
        let t = Tokens::get(ui.ctx());
        let app = entry.app;
        let status = entry.status(self.platform, self.caps.manage_apps);
        // The card itself, under its controls (they are added after it, so they win the pointer):
        // right-click opens the app's menu. With no chip on screen, the card is named for screen
        // readers by its app and status ("Redrob Film: Available to install"), and a failed check
        // explains itself on hover.
        let card = ui.interact(rect, ui.id().with(("card", id)), Sense::click());
        let (status_text, _) = model::entry_status_text(&entry, &status, self.caps.manage_apps);
        let card_name = fmt(tl!("{name}: {status}"), &[("name", app.name), ("status", &status_text)]);
        card.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Other, true, card_name.clone()));
        let card = match entry.failure() {
            Some(f) => card.on_hover_text(fmt(tl!("Couldn't check for updates: {f}"), &[("f", f)])),
            None => card,
        };
        card.context_menu(|ui| {
            ui.set_min_width(200.0);
            // DS menu items are about 36 px.
            ui.spacing_mut().interact_size.y = 36.0;
            self.menu_items(ui, &entry, &status);
        });
        let hovered = ui.rect_contains_pointer(rect);
        let k = ui.ctx().animate_bool_with_time(ui.id().with(("card-hover", id)), hovered, theme::motion::duration(ui.ctx(), theme::motion::FAST));
        let radius = CornerRadius::same(theme::radius::LG);
        let painter = ui.painter_at(rect.expand(24.0));
        if k > 0.0 {
            let mut shadow = theme::shadow_md(&t);
            shadow.color = shadow.color.gamma_multiply(k);
            painter.add(shadow.as_shape(rect, radius));
        }
        painter.rect(rect, radius, card_fill(&t), Stroke::new(1.0, theme::mix(t.line, t.line_strong, k)), egui::StrokeKind::Inside);

        let inner = rect.shrink(CARD_PAD);
        // A detached child: the card's contents never move the next card along.
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner).layout(egui::Layout::top_down(egui::Align::Min)));
        child.set_clip_rect(rect.intersect(ui.clip_rect()));
        let ui = &mut child;
        let (name_font, desc_font, _) = card_fonts();
        let text_w = (inner.width() - CARD_ICON - theme::space::S3).max(1.0);
        ui.horizontal_top(|ui| {
            // The icon sits at the top of the text column, so a long description grows downward.
            let (slot, icon_response) = ui.allocate_exact_size(vec2(CARD_ICON, CARD_ICON), Sense::hover());
            icon_response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Image, true, fmt(tl!("{name} icon"), &[("name", app.name)])));
            if let Some(icon) = crate::icons::app(app.id) {
                egui::Image::new(icon).paint_at(ui, slot);
            }
            ui.add_space(theme::space::S3);
            ui.allocate_ui_with_layout(vec2(text_w, 0.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.spacing_mut().item_spacing.y = CARD_LINE_GAP;
                // The name's row is as tall as its text, not as a button.
                ui.spacing_mut().interact_size.y = 0.0;
                widgets::wordmark(ui, app, name_font.size);
                // The short description in full: it wraps instead of being cut off.
                ui.add(egui::Label::new(RichText::new(tr_ctx(current(), app.id, app.tagline)).font(desc_font).color(t.muted)).wrap());
            });
        });
        ui.add_space(theme::space::S3);
        // The version alone, in the lightest ink, with no release date. A narrow tile cuts it short;
        // egui then shows the whole line on hover.
        ui.add(egui::Label::new(RichText::new(version_line(&entry, &status)).font(card_fonts().2).color(t.faint)).truncate());
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            ui.horizontal(|ui| self.actions(ui, &entry, &status, inner.width()));
        });
    }

    // ---- actions -------------------------------------------------------------------------

    fn primary_action(&self, entry: &AppEntry, status: &AppStatus) -> Primary {
        if !self.caps.manage_apps {
            let platform = self.platform;
            let url = entry.release().zip(platform).and_then(|(r, p)| {
                let stems: Vec<&str> = entry.app.stems().collect();
                pick_asset(r, &stems, p, p.download_kinds()).map(|(a, _)| a.url.clone())
            });
            return match (status, url) {
                (AppStatus::NoRelease, _) => Primary::Unavailable(tl!("Coming soon")),
                (AppStatus::Checking, _) => Primary::Unavailable(tl!("Checking…")),
                (_, Some(url)) => Primary::Download(url),
                (_, None) => Primary::Unavailable(tl!("Not available")),
            };
        }
        match status {
            AppStatus::Available if entry.ready => Primary::Retry(tl!("Retry install").into()),
            AppStatus::UpdateAvailable if entry.ready => {
                Primary::Retry(fmt(tl!("Retry update to {version}"), &[("version", &entry.latest_label().unwrap_or_default())]))
            }
            AppStatus::Available => Primary::Install,
            AppStatus::UpdateAvailable => Primary::Update(entry.latest_label().unwrap_or_default()),
            AppStatus::UpToDate | AppStatus::Installed => Primary::Open,
            AppStatus::Checking => Primary::Unavailable(tl!("Checking…")),
            AppStatus::NoRelease => Primary::Unavailable(tl!("Coming soon")),
            AppStatus::Unavailable => Primary::Unavailable(if entry.failure().is_some() { tl!("Offline") } else { tl!("Not available") }),
        }
    }

    /// The main button (or progress) and the named … menu within `width`.
    fn actions(&mut self, ui: &mut Ui, entry: &AppEntry, status: &AppStatus, width: f32) {
        let t = Tokens::get(ui.ctx());
        let id = entry.app.id.to_string();
        let gap = ui.spacing().item_spacing.x;
        // The named menu is always reachable by keyboard, including while an install job replaces
        // the primary action; right-click remains a pointer shortcut to the same commands.
        let main_w = (width - ACTION_ICON - gap).max(120.0);
        if let Some(phase) = entry.job {
            let downloading = matches!(phase, JobPhase::Downloading { .. } | JobPhase::Reconnecting { .. });
            // Reserve the exact number of trailing controls so the progress caption yields space
            // before Pause, Cancel, or More can overlap at compact card widths.
            let trailing_controls = if downloading {
                3.0
            } else if phase == JobPhase::Waiting {
                2.0
            } else {
                1.0
            };
            let bar_w = (width - trailing_controls * ACTION_ICON - trailing_controls * gap).max(72.0);
            let short = bar_w < 200.0;
            let amount = |done: u64, total: u64| {
                if total > 0 {
                    fmt(tl!("{done} of {total}"), &[("done", &widgets::human_bytes(done)), ("total", &widgets::human_bytes(total))])
                } else {
                    widgets::human_bytes(done)
                }
            };
            let (fraction, caption, color) = match phase {
                JobPhase::Downloading { done, total } => {
                    let f = (total > 0).then(|| done as f32 / total as f32);
                    let pct = f.map(|f| format!(" · {:.0}%", f * 100.0)).unwrap_or_default();
                    let caption = if bar_w < 140.0 {
                        match f {
                            Some(f) => {
                                fmt(tl!("{done} of {total}"), &[("done", &format!("{:.0}%", f * 100.0)), ("total", &widgets::human_bytes(total))])
                            }
                            None => widgets::human_bytes(done),
                        }
                    } else if short {
                        format!("{}{pct}", amount(done, total))
                    } else {
                        format!("{} {}{pct}", tl!("Downloading"), amount(done, total))
                    };
                    (f, caption, t.accent)
                }
                JobPhase::Reconnecting { done, total } => {
                    let f = (total > 0).then(|| done as f32 / total as f32);
                    (f, if short { tl!("Reconnecting…").into() } else { format!("{} {}", tl!("Reconnecting…"), amount(done, total)) }, t.warn)
                }
                JobPhase::Verifying => (None, tl!("Verifying checksum…").into(), t.accent),
                JobPhase::Waiting => (None, tl!("Waiting for another app…").into(), t.accent),
                JobPhase::Installing => (None, tl!("Installing…").into(), t.accent),
                JobPhase::Removing => (None, tl!("Removing…").into(), t.accent),
            };
            // A job waiting for the installer hasn't started it yet, so it can still be called off.
            let waiting = phase == JobPhase::Waiting;
            widgets::progress(ui, fraction, &caption, color, bar_w);
            if downloading && widgets::icon_button(ui, crate::glyphs::Icon::Pause, tl!("Pause download"), ACTION_ICON).clicked() {
                self.send(Command::Pause { app: id.clone() });
            }
            if (downloading || waiting)
                && widgets::icon_button(ui, crate::glyphs::Icon::Close, if waiting { tl!("Cancel") } else { tl!("Cancel download") }, ACTION_ICON)
                    .clicked()
            {
                self.send(Command::Cancel { app: id });
            }
            self.menu(ui, entry, status);
            return;
        }
        if let Some(p) = entry.paused
            && self.caps.manage_apps
        {
            let pct = if p.total > 0 { format!(" · {:.0}%", p.done as f32 / p.total as f32 * 100.0) } else { String::new() };
            // Resume yields space to both Discard and the keyboard-reachable More menu.
            let resume_w = (width - 2.0 * (ACTION_ICON + gap)).max(90.0);
            // Narrow: just tl!("Resume"); the tooltip says how far it got.
            let label = match resume_w {
                w if w < 150.0 => tl!("Resume").to_string(),
                w if w < 220.0 => format!("{}{pct}", tl!("Resume")),
                _ => format!("{}{pct}", tl!("Resume download")),
            };
            let resp = widgets::button(ui, &label, Kind::Primary, resume_w);
            let tip = if p.total > 0 {
                fmt(tl!("{done} of {total} downloaded"), &[("done", &widgets::human_bytes(p.done)), ("total", &widgets::human_bytes(p.total))])
            } else {
                fmt(tl!("{done} downloaded"), &[("done", &widgets::human_bytes(p.done))])
            };
            if resp.on_hover_text(tip).clicked() {
                self.send(Command::Install { app: id.clone() });
            }
            if widgets::icon_button(ui, crate::glyphs::Icon::Close, tl!("Discard download"), ACTION_ICON).clicked() {
                self.send(Command::Cancel { app: id.clone() });
            }
            self.menu(ui, entry, status);
            return;
        }
        match self.primary_action(entry, status) {
            Primary::Install => {
                let resp = widgets::button(ui, tl!("Install"), Kind::Primary, main_w);
                // Opened by a `redrob://open` link for an app that isn't installed yet.
                if self.focus_install == Some(entry.app.id) {
                    self.focus_install = None;
                    resp.request_focus();
                    resp.scroll_to_me(None);
                }
                if resp.clicked() {
                    self.send(Command::Install { app: id.clone() });
                }
            }
            Primary::Retry(label) => {
                let tip = tl!("The download is still here. It's checked again, then installed.");
                if widgets::button(ui, &label, Kind::Primary, main_w).on_hover_text(tip).clicked() {
                    self.send(Command::Install { app: id.clone() });
                }
            }
            Primary::Update(v) => {
                if widgets::button(ui, &fmt(tl!("Update to {v}"), &[("v", &v)]), Kind::Primary, main_w).clicked() {
                    self.send(Command::Install { app: id.clone() });
                }
            }
            Primary::Open => {
                if widgets::button(ui, tl!("Open"), Kind::Secondary, main_w).clicked() {
                    self.send(Command::Launch { app: id.clone(), args: Vec::new() });
                }
            }
            Primary::Download(url) => {
                let label = match self.platform {
                    Some(p) => fmt(tl!("Download for {os}"), &[("os", p.os.label())]),
                    None => tl!("Download").into(),
                };
                if widgets::button(ui, &label, Kind::Primary, main_w).clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                }
            }
            Primary::Unavailable(label) => {
                ui.add_enabled_ui(false, |ui| widgets::button(ui, label, Kind::Secondary, main_w));
            }
        }
        self.menu(ui, entry, status);
    }

    /// The … button and its menu.
    fn menu(&mut self, ui: &mut Ui, entry: &AppEntry, status: &AppStatus) {
        widgets::more_menu(ui, |ui| self.menu_items(ui, entry, status));
    }

    /// An app's menu: from the … button or a right-click on its card.
    fn menu_items(&mut self, ui: &mut Ui, entry: &AppEntry, status: &AppStatus) {
        let id = entry.app.id.to_string();
        let manage = self.caps.manage_apps;
        let reveal = self.caps.reveal_label;
        {
            if entry.release().is_some() && ui.button(tl!("Release notes")).clicked() {
                self.dialog = Some(Dialog::Notes(id.clone()));
                ui.close();
            }
            // Only where Install would download: an up-to-date app would be reinstalled.
            let can_download_again =
                manage && entry.ready && entry.paused.is_none() && matches!(status, AppStatus::Available | AppStatus::UpdateAvailable);
            if can_download_again && ui.button(tl!("Download again")).clicked() {
                self.send(Command::Cancel { app: id.clone() });
                self.send(Command::Install { app: id.clone() });
                ui.close();
            }
            if manage && status.is_installed() {
                if ui.button(tl!("Open")).clicked() {
                    self.send(Command::Launch { app: id.clone(), args: Vec::new() });
                    ui.close();
                }
                if ui.button(t(reveal)).clicked() {
                    self.send(Command::Reveal { app: id.clone() });
                    ui.close();
                }
                if *status == AppStatus::UpToDate && ui.button(tl!("Reinstall")).clicked() {
                    self.send(Command::Install { app: id.clone() });
                    ui.close();
                }
            }
            if !manage && let (Some(r), Some(p)) = (entry.release(), self.platform) {
                let stems: Vec<&str> = entry.app.stems().collect();
                let options: Vec<(String, String)> = p
                    .download_kinds()
                    .iter()
                    .filter_map(|k| {
                        pick_asset(r, &stems, p, &[*k])
                            .map(|(a, _)| (format!("{} · {}", crate::i18n::t(k.label()), widgets::human_bytes(a.size)), a.url.clone()))
                    })
                    .collect();
                if !options.is_empty() {
                    ui.separator();
                    for (label, url) in options {
                        if ui.button(label).clicked() {
                            ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                            ui.close();
                        }
                    }
                }
            }
            ui.separator();
            let link = |ui: &mut Ui, label: &str, url: String| {
                if ui.button(label).clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                    ui.close();
                }
            };
            if let Some(site) = entry.app.website() {
                link(ui, tl!("Website"), site);
            }
            link(ui, tl!("GitHub repository"), entry.app.repo_url());
            link(ui, tl!("All releases"), entry.app.releases_url());
            // DS Menu: the destructive action last, in status-danger.
            if manage && status.is_installed() {
                ui.separator();
                if ui.button(RichText::new(tl!("Uninstall…")).color(Tokens::get(ui.ctx()).bad)).clicked() {
                    self.dialog = Some(Dialog::ConfirmUninstall(id.clone()));
                    ui.close();
                }
            }
        }
    }
}

/// "0.4.1 installed → 0.5.0 released", "Version 0.3.0" and friends: versions only, no release date
/// (the date is in Release notes).
fn version_line(e: &AppEntry, status: &AppStatus) -> String {
    match (status, e.installed_label(), e.latest_label()) {
        (AppStatus::UpdateAvailable, Some(i), Some(l)) => {
            fmt(tl!("{version} installed → {latest} released"), &[("version", i.as_str()), ("latest", l.as_str())])
        }
        (AppStatus::UpToDate, Some(i), _) => fmt(tl!("Version {version}"), &[("version", i.as_str())]),
        (_, Some(i), Some(l)) => fmt(tl!("{installed} installed · latest {latest}"), &[("installed", i.as_str()), ("latest", l.as_str())]),
        (_, Some(i), None) => fmt(tl!("Version {version} installed"), &[("version", i.as_str())]),
        (_, None, Some(l)) => fmt(tl!("Version {latest}"), &[("latest", l.as_str())]),
        (AppStatus::NoRelease, None, None) => tl!("No release published yet").to_string(),
        _ => " ".to_string(),
    }
}

/// The launcher's own download for this platform.
pub(crate) fn launcher_download(r: &redrob_launcher_core::Release, p: redrob_launcher_core::Platform) -> Option<String> {
    use redrob_launcher_core::PackageKind;
    let kinds: &[PackageKind] = match p.os {
        redrob_launcher_core::Os::Linux => &[PackageKind::AppImage, PackageKind::Deb],
        _ => p.default_kinds(),
    };
    pick_asset(r, &[redrob_launcher_core::catalog::LAUNCHER_ID], p, kinds).map(|(a, _)| a.url.clone())
}

/// The height every tile takes at `card_w`, set by the app whose description wraps to the most
/// lines: padding, the icon-and-text block, the version line, the actions row, padding.
///
/// * `ui` - any Ui on the page, for font metrics.
/// * `ids` - the apps shown; every tile takes the tallest one's height so rows stay even.
/// * `card_w` - one tile's outer width.
fn card_height(ui: &Ui, ids: &[&'static str], card_w: f32) -> f32 {
    let (name, desc, meta) = card_fonts();
    let text_w = (card_w - 2.0 * CARD_PAD - CARD_ICON - theme::space::S3).max(1.0);
    let line = |font: &FontId| ui.fonts_mut(|f| f.layout_no_wrap(String::new(), font.clone(), Color32::PLACEHOLDER)).size().y;
    let desc_h = ids
        .iter()
        .filter_map(|id| redrob_launcher_core::CraftApp::by_id(id))
        .map(|app| {
            let text = tr_ctx(current(), app.id, app.tagline).to_string();
            ui.fonts_mut(|f| f.layout(text, desc.clone(), Color32::PLACEHOLDER, text_w)).size().y
        })
        .fold(line(&desc), f32::max);
    let block = (line(&name) + CARD_LINE_GAP + desc_h).max(CARD_ICON);
    (2.0 * CARD_PAD + block + theme::space::S3 + line(&meta) + theme::space::S4 + CARD_ACTION_H).ceil()
}

/// A tile's fill: white (`surface-base`, #ffffff) in light, where the hairline and the hover shadow
/// separate it from the page; in dark the raised surface stays, since a white tile would break
/// the dark appearance.
///
/// * `t` - the current appearance's tokens.
fn card_fill(t: &Tokens) -> Color32 {
    if t.dark { t.raised() } else { t.bg }
}

/// A tile's three type roles: (app name in the display face, description, version line).
fn card_fonts() -> (FontId, FontId, FontId) {
    (FontId::new(theme::text::BODY, theme::display()), FontId::proportional(theme::text::BODY_SM), FontId::proportional(theme::text::META))
}
