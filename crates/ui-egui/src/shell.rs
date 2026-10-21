//! The window's shell, laid out like Redrob Cowork: a floating sidebar card on the left (the
//! pages, the theme, Settings and the account) and a 48 px top bar over each page but Apps, on one
//! continuous base ground so no grey seam shows between them.

use egui::{RichText, Sense, Stroke, Ui, vec2};

use crate::app::{Host, LauncherUi, Page};
use crate::glyphs::Icon;
use crate::settings::Filter;
use crate::theme::{self, Tokens};
use crate::widgets::{self, NavMeta};

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

    /// The floating sidebar card: brand row, the pages, then the theme and the account menu (which
    /// holds Settings) at its foot. Fills `ui`'s rect (the panel's, inside its 8 px margin).
    pub(crate) fn sidebar(&mut self, ui: &mut Ui, host: &mut dyn Host) {
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

        // Brand row (DS AppShell: padding 20 / 16 / 16): the product's name as plain text, 15 / 600 ink,
        // tracked in -0.01em (Redrob Cowork's wordmark text). The folded rail keeps the row's room, empty.
        ui.add_space(theme::space::S5);
        let name = tl!("Redrob Launcher");
        if rail {
            ui.allocate_exact_size(vec2(ui.available_width(), 22.0), Sense::hover());
        } else {
            ui.horizontal(|ui| {
                ui.add_space(theme::space::S2);
                ui.add(
                    egui::Label::new(
                        RichText::new(name)
                            .size(theme::text::BODY)
                            .family(theme::semibold())
                            .color(t.ink)
                            .extra_letter_spacing(-0.01 * theme::text::BODY),
                    )
                    .selectable(false)
                    .truncate(),
                );
            });
        }
        ui.add_space(theme::space::S4);

        // The pages.
        let updates = if self.caps.manage_apps { self.count(Filter::Updates) } else { 0 };
        if widgets::nav_row(
            ui,
            Icon::Apps,
            tl!("All apps"),
            tl!("Every Redrob app, to install, update and open"),
            NavMeta::Count(updates, t.warn),
            self.page == Page::Apps,
            rail,
        )
        .clicked()
        {
            self.set_page(Page::Apps);
        }
        self.models_nav(ui, rail);
        self.engine_nav(ui, rail);

        // The foot, from the bottom up: the account row (its menu holds Settings), a hairline, the theme.
        let align = if rail { egui::Align::Center } else { egui::Align::Min };
        ui.with_layout(egui::Layout::bottom_up(align), |ui| {
            ui.add_space(INSET);
            self.account_menu(ui, rail, host);
            ui.add_space(theme::space::S1);
            let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
            ui.painter().hline(r.x_range(), r.center().y, t.hairline());
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

    /// A page's top bar: back to the apps, its title and a meta line. The Apps page has none: its
    /// search and Update live in the title bar, its refresh at the end of its filter row.
    pub(crate) fn page_top_bar(&mut self, ui: &mut Ui) {
        let t = Tokens::get(ui.ctx());
        let (title, meta): (&str, String) = match self.page {
            Page::Apps => return,
            Page::Models => (tl!("On-device AI"), tl!("AI models that run on this computer. What you give them stays here.").to_string()),
            Page::Engine => (tl!("AI engine"), tl!("Your Redrob apps use it to reach AI. It starts on its own when one needs it.").to_string()),
            Page::Account => (tl!("Redrob account"), tl!("One sign-in for every Redrob app on this computer").to_string()),
            Page::Settings => (tl!("Settings"), format!("Redrob Launcher {}", env!("CARGO_PKG_VERSION"))),
        };
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = theme::space::S2;
            // Every page leads back to the apps (Escape does too). The button is pulled left by its
            // own padding, so the chevron's ink sits on the content edge under it (DS ConnectorCard
            // pulls its first ghost button the same way); its hover fill spills into the margin.
            ui.add_space(-theme::space::S2);
            if widgets::icon_button(ui, Icon::ChevronLeft, tl!("Back to apps"), theme::control::SM).clicked() {
                self.set_page(Page::Apps);
            }
            // DS AppShell top bar: title 15 / 600, meta 12 ink-muted.
            ui.label(RichText::new(title).size(theme::text::BODY).family(theme::semibold()).color(t.ink));
            let room = ui.available_width().max(0.0);
            if !meta.is_empty() && room > 40.0 {
                widgets::hud_elided(ui, &meta, t.faint, room);
            }
        });
    }
}

/// Room kept free at the monitor's bottom edge for a taskbar or Dock egui can't see (the Windows
/// taskbar is 40–48 pt tall; a bottom Dock about as much).
const TASKBAR: f32 = 48.0;

/// Does a window of `outer` size fit on a `monitor` less the taskbar?
fn fits(outer: egui::Vec2, monitor: egui::Vec2) -> bool {
    outer.x <= monitor.x + 0.5 && outer.y <= monitor.y - TASKBAR + 0.5
}

/// Once per window, as soon as its monitor is known: a window bigger than the monitor's work area
/// is maximized into it. eframe only shrinks a saved size to the LARGEST monitor, so a window
/// closed on an external monitor reopened on a laptop larger than its screen, the title bar (which
/// moves and closes it) above the top edge. Egui doesn't expose the work area itself, only the
/// monitor's size, so the taskbar is a fixed margin. Returns true when it asked to maximize.
pub fn fit_window(ctx: &egui::Context) -> bool {
    let fitted = egui::Id::new("launcher-window-fitted");
    if ctx.data(|d| d.get_temp::<bool>(fitted)).is_some() {
        return false;
    }
    let content = ctx.content_rect();
    let (monitor, outer, maximized, fullscreen) = ctx.input(|i| {
        let v = i.viewport();
        (v.monitor_size, v.outer_rect, v.maximized, v.fullscreen)
    });
    let Some(monitor) = monitor else { return false };
    ctx.data_mut(|d| d.insert_temp(fitted, true));
    // macOS keeps new windows inside the screen's visible frame itself.
    if cfg!(target_os = "macos") {
        return false;
    }
    // Wayland never tells a window where it is, so there is no outer rect: the content is the
    // least the window takes up.
    let outer = outer.unwrap_or(content);
    if maximized == Some(true) || fullscreen == Some(true) || !monitor.is_finite() || !outer.is_finite() || fits(outer.size(), monitor) {
        return false;
    }
    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
    true
}
