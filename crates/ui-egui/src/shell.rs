//! The window's shell, laid out like Redrob Cowork: a floating sidebar card on the left (the
//! pages, the theme, Settings and the account) and a 48 px top bar over each page, on one
//! continuous base ground so no grey seam shows between them.

use egui::{Rect, RichText, Sense, Stroke, Ui, vec2};
use redrob_launcher_core::protocol::Command;
use redrob_launcher_core::status::AppStatus;

use crate::app::{LauncherUi, Page, TitleBar};
use crate::glyphs::Icon;
use crate::i18n::{fmt, tn};
use crate::settings::{Filter, ViewMode};
use crate::theme::{self, Tokens};
use crate::widgets::{self, Cell, Kind, NavMeta};

/// The sidebar card's width (Redrob Cowork's defaults to 260, the DS AppShell's to 232).
pub(crate) const SIDEBAR_W: f32 = 248.0;
/// The card folded to its icons in a narrow window (DS AppShell folded width).
pub(crate) const RAIL_W: f32 = 64.0;
/// Below this window width the sidebar folds to [`RAIL_W`].
pub(crate) const FOLD_BELOW: f32 = 980.0;
/// The gap round the card: Cowork's `margin: 8px`.
pub(crate) const INSET: f32 = 8.0;
/// Height of a page's top bar: Cowork's shell top bar, as tall as the title bar.
pub(crate) const PAGE_TOP_H: f32 = 48.0;

impl LauncherUi {
    /// Shows `page`. Leaving Settings forgets its Channels drafts, so the next visit reads the
    /// engine's files again.
    pub(crate) fn set_page(&mut self, page: Page) {
        if self.page == Page::Settings && page != Page::Settings {
            self.channels.reset();
        }
        self.page = page;
    }

    /// Is the window too narrow for the full sidebar?
    pub(crate) fn rail(ctx: &egui::Context) -> bool {
        ctx.content_rect().width() < FOLD_BELOW
    }

    /// The sidebar's width, with the gap on both sides of the card.
    pub(crate) fn sidebar_width(ctx: &egui::Context) -> f32 {
        (if Self::rail(ctx) { RAIL_W } else { SIDEBAR_W }) + 2.0 * INSET
    }

    /// The floating sidebar card: brand row, the pages, then the theme, Settings and the account
    /// at its foot. Fills `ui`'s rect (the panel's, inside its 8 px margin).
    pub(crate) fn sidebar(&mut self, ui: &mut Ui) {
        let t = Tokens::get(ui.ctx());
        let rail = Self::rail(ui.ctx());
        let card = ui.max_rect();
        let radius = egui::CornerRadius::same(theme::radius::XL);
        let p = ui.painter();
        // Cowork: a negative-spread drop under the card, its edge, and a 1 px top highlight.
        p.add(theme::shadow_floating(&t).as_shape(card.shrink(theme::FLOATING_SHADOW_INSET), radius));
        p.rect(card, radius, t.bg, Stroke::new(1.0, t.card_edge()), egui::StrokeKind::Inside);
        p.hline(card.left() + 16.0..=card.right() - 16.0, card.top() + 1.5, Stroke::new(1.0, t.card_highlight()));

        let inner = card.shrink2(vec2(INSET, 0.0));
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner).layout(egui::Layout::top_down(egui::Align::Min)));
        child.set_clip_rect(card.intersect(ui.clip_rect()));
        let ui = &mut child;
        ui.spacing_mut().item_spacing = vec2(0.0, 1.0);

        // Brand row (DS AppShell: padding 20 / 16 / 16): the lockup as supplied, then the product.
        ui.add_space(theme::space::S5);
        if rail {
            let (cell, _) = ui.allocate_exact_size(vec2(ui.available_width(), 28.0), Sense::hover());
            egui::Image::new(crate::icons::redrob_mark())
                .alt_text(tl!("Redrob Launcher"))
                .paint_at(ui, Rect::from_center_size(cell.center(), vec2(28.0, 28.0)));
        } else {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = theme::space::S2;
                ui.add_space(theme::space::S2);
                let logo = vec2(18.0 * crate::icons::REDROB_LOGO_ASPECT, 18.0);
                let (cell, _) = ui.allocate_exact_size(vec2(logo.x, 22.0), Sense::hover());
                egui::Image::new(crate::icons::redrob_logo(t.dark)).alt_text(tl!("Redrob")).paint_at(ui, Rect::from_center_size(cell.center(), logo));
                ui.label(RichText::new(tl!("Launcher")).size(theme::text::CONTROL).family(theme::semibold()).color(t.ink));
            });
        }
        ui.add_space(theme::space::S4);

        // The pages.
        let updates = if self.caps.manage_apps { self.count(Filter::Updates) } else { 0 };
        if widgets::nav_row(ui, Icon::Apps, tl!("Apps"), tl!("The Redrob apps"), NavMeta::Count(updates, t.warn), self.page == Page::Apps, rail)
            .clicked()
        {
            self.set_page(Page::Apps);
        }
        self.models_nav(ui, rail);
        self.engine_nav(ui, rail);

        // The foot, from the bottom up: account, a hairline, Settings, the theme.
        let align = if rail { egui::Align::Center } else { egui::Align::Min };
        ui.with_layout(egui::Layout::bottom_up(align), |ui| {
            ui.add_space(INSET);
            if self.account_nav(ui, rail) {
                ui.add_space(theme::space::S1);
                let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
                ui.painter().hline(r.x_range(), r.center().y, t.hairline());
                ui.add_space(theme::space::S1);
            }
            if widgets::nav_row(
                ui,
                Icon::Settings,
                tl!("Settings"),
                tl!("Language, updates, appearance"),
                NavMeta::None,
                self.page == Page::Settings,
                rail,
            )
            .clicked()
            {
                self.set_page(Page::Settings);
            }
            ui.add_space(theme::space::S2);
            let mut choice = self.settings.theme;
            let changed = if rail {
                widgets::theme_switch(ui, &mut choice, true)
            } else {
                ui.horizontal(|ui| {
                    ui.add_space(theme::space::S1);
                    widgets::theme_switch(ui, &mut choice, false)
                })
                .inner
            };
            if changed {
                self.settings.theme = choice;
            }
        });
    }

    /// The page's top bar: its title and a meta line, and the page's own actions at the right.
    /// With no title bar of ours (`TitleBar::Native`), Check for updates sits here at the right.
    pub(crate) fn page_top_bar(&mut self, ui: &mut Ui, bar: TitleBar) {
        let t = Tokens::get(ui.ctx());
        let (title, meta): (&str, String) = match self.page {
            Page::Apps => (tl!("Apps"), tn(self.count(Filter::All) as u64, "{n} app", "{n} apps")),
            Page::Models => (tl!("Models"), String::new()),
            Page::Engine => (tl!("Redrob Code engine"), tl!("One engine on this computer; every Redrob app runs its AI through it").to_string()),
            Page::Account => (tl!("Redrob account"), tl!("One sign-in for every Redrob app on this computer").to_string()),
            Page::Settings => (tl!("Settings"), format!("Redrob Launcher {}", env!("CARGO_PKG_VERSION"))),
        };
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = theme::space::S2;
            // DS AppShell top bar: title 15 / 600, meta 12 ink-muted.
            ui.label(RichText::new(title).size(theme::text::BODY).family(theme::semibold()).color(t.ink));
            let actions_w = if self.page == Page::Apps { 520.0 } else { 0.0 } + if bar == TitleBar::Native { 40.0 } else { 0.0 };
            let room = (ui.available_width() - actions_w).max(0.0);
            if !meta.is_empty() && room > 40.0 {
                widgets::hud_elided(ui, &meta, t.faint, room);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if bar == TitleBar::Native
                    && widgets::icon_button_busy(ui, Icon::Refresh, tl!("Check for updates"), theme::control::SM, self.checking).clicked()
                {
                    self.check_now();
                }
                if self.page == Page::Apps {
                    self.apps_actions(ui);
                }
            });
        });
    }

    /// The Apps page's actions: update everything, search, and grid or table.
    fn apps_actions(&mut self, ui: &mut Ui) {
        let updates = self.count(Filter::Updates);
        if self.caps.manage_apps && updates > 0 {
            let label =
                if updates == 1 { tl!("Update 1 app").to_string() } else { fmt(tl!("Update all ({count})"), &[("count", &updates.to_string())]) };
            if widgets::button_sized(ui, &label, Kind::Primary, widgets::Size::Sm, 0.0).clicked() {
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
        }
        let search_w = (ui.available_width() - 260.0).clamp(120.0, 220.0);
        ui.add(
            egui::TextEdit::singleline(&mut self.search)
                .hint_text(tl!("Search apps"))
                .desired_width(search_w)
                .min_size(vec2(0.0, theme::control::SM))
                .vertical_align(egui::Align::Center)
                .margin(vec2(10.0, 0.0)),
        );
        widgets::cells(
            ui,
            &mut self.settings.view,
            &[
                (ViewMode::Grid, Cell { label: tl!("Grid"), tip: tl!("Show apps as cards"), count: None }),
                (ViewMode::Table, Cell { label: tl!("Table"), tip: tl!("Show apps as a table"), count: None }),
            ],
        );
    }
}
