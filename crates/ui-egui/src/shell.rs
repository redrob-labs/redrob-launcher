// ADDED-LINE-RATIONALE 8b161a64c4db0161:1: Supports `shell` while preserving the nearby constraint: ! continuous base ground so no grey seam shows between them.
// ADDED-LINE-RATIONALE 4f18276cb8a9a205:1: Gives `shell` direct access to `crate / app / Host` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE cc0a253ff812f1b5:1: Gives `shell` direct access to `crate / glyphs / Icon` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 0820e3fbdf404a3e:1: Gives `shell` direct access to `crate / settings / Filter` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 222cd0c3d42951bc:1: Gives `shell` direct access to `crate / theme / Tokens` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 7e2804da884f4bd6:1: Gives `shell` direct access to `crate / widgets / NavMeta` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 0bee38041b9211ff:1: Supports `shell` while preserving the nearby constraint: The sidebar card's width (Redrob Cowork's defaults to 260, the DS AppShell's to 232).
// ADDED-LINE-RATIONALE 58bb2dc7f9953f03:1: Supports `shell` while preserving the nearby constraint: The card folded to its icons in a narrow window (DS AppShell folded width).
// ADDED-LINE-RATIONALE 450223cc472f128f:1: Supports `shell` while preserving the nearby constraint: Below this window width the sidebar folds to [`RAIL_W`].
// ADDED-LINE-RATIONALE 84e5c343d2ce21ae:1: Supports `shell` while preserving the nearby constraint: The gap round the card: Cowork's `margin: 8px`.
// ADDED-LINE-RATIONALE 205c76343aa664a7:1: Supports `shell` while preserving the nearby constraint: Height of a page's top bar: Cowork's shell top bar, as tall as the title bar.
// ADDED-LINE-RATIONALE 51ff69651381d8df:1: Makes `LauncherUi` an explicit part of `shell` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE b838c05a934385bb:1: Supports `set_page` while preserving the nearby constraint: engine's files again.
// ADDED-LINE-RATIONALE 5f8a402beb298790:1: Freezes `if` for `set_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6ac95eaa8fd77d75:1: Delegates `set_page`'s `channels / reset` work to `self.channels.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7975f38702e50c15:1: Bounds the `set_page` implementation here so `channels / reset` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3afc6d2e182ada8e:1: Freezes `self` for `set_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0c336d9af511e4f5:1: Bounds the `set_page` implementation here so `page / page` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0e1cf002a525e7c4:1: Supports `rail` while preserving the nearby constraint: Is the window too narrow for the full sidebar?.
// ADDED-LINE-RATIONALE 9fca4890cd1a30c6:1: Delegates `rail`'s `ctx / content_rect / width` work to `ctx.content_rect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0c336d9af511e4f5:2: Bounds the `rail` implementation here so `ctx / content_rect / width` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 43544a2921589fa0:1: Supports `sidebar_width` while preserving the nearby constraint: The sidebar's width, with the gap on both sides of the card.
// ADDED-LINE-RATIONALE 60ad5dc662f15d59:1: Delegates `sidebar_width`'s `Self / rail / ctx` work to `Self::rail` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0c336d9af511e4f5:3: Bounds the `sidebar_width` implementation here so `Self / rail / ctx` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 30216bb65b48575a:1: Supports `sidebar` while preserving the nearby constraint: holds Settings) at its foot. Fills `ui`'s rect (the panel's, inside its 8 px margin).
// ADDED-LINE-RATIONALE fa1d7503d0104454:1: Freezes `t` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 94031d637d8e8242:1: Freezes `rail` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3359a3084ccd35bb:1: Freezes `card` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 369b87e375afbc48:1: Freezes `radius` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3e12f3f0e32ba4c9:1: Freezes `p` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 950871180dfe72f1:1: Supports `sidebar` while preserving the nearby constraint: Cowork: a negative-spread drop under the card, its edge, and a 1 px top highlight.
// ADDED-LINE-RATIONALE 77b8ea4a94f6b7e0:1: Delegates `sidebar`'s `p / rect / card` work to `p.rect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 33a8500922fdcd29:1: Freezes `p` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ed87f72a0169c04b:1: Freezes `inner` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0010018ef6b28476:1: Freezes `child` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 96684e2303dd7490:1: Delegates `sidebar`'s `child / set_clip_rect / card` work to `child.set_clip_rect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7aa95ebd76ce043b:1: Freezes `ui` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ac7758d3974f5058:1: Freezes `ui` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 47040f1f88695afd:1: Supports `sidebar` while preserving the nearby constraint: tracked in -0.01em (Redrob Cowork's wordmark text). The folded rail keeps the row's room, empty.
// ADDED-LINE-RATIONALE a48031c879de4e2c:1: Freezes `name` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9ed90b9f07290c02:1: Prevents `sidebar` from applying its side effect outside the `rail` state.
// ADDED-LINE-RATIONALE 05d51c7b2d272582:1: Delegates `sidebar`'s `ui / allocate_exact_size / vec2` work to `ui.allocate_exact_size` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4429d8a076e67c83:1: Makes `ui / allocate_exact_size / vec2` an explicit part of `sidebar` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE ead515b074e01cfd:1: Delegates `sidebar`'s `ui / horizontal / ui` work to `ui.horizontal` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c642d4cc5edf2dc0:1: Delegates `sidebar`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c1a6a368c346f307:1: Delegates `sidebar`'s `ui / add` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4a3b5aad1ea45728:1: Delegates `sidebar`'s `egui / Label / new` work to `egui::Label::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE afb490cf3b963afd:1: Delegates `sidebar`'s `RichText / new` work to `RichText::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5c33676ceae91536:1: Delegates `sidebar`'s `size / theme / text` work to `size` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1ea571e2b1b8b306:1: Delegates `sidebar`'s `family / theme / semibold` work to `family` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fd6fc76daabb3ac8:1: Delegates `sidebar`'s `color / t / ink` work to `color` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9e02d7480c8eb946:1: Keeps `extra_letter_spacing / theme / text` in `sidebar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 3416d9a861fd6f39:1: Bounds the `sidebar` implementation here so `extra_letter_spacing / theme / text` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 96ea03696feec202:1: Delegates `sidebar`'s `selectable` work to `selectable` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0d532c235a9347a7:1: Keeps `truncate` in `sidebar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 412f368181df7546:1: Bounds the `sidebar` implementation here so `truncate` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 866caf0ed1f1b85d:1: Makes `truncate` an explicit part of `sidebar` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 7975f38702e50c15:2: Bounds the `sidebar` implementation here so `ui / add_space / theme` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3d87de1f82ce54fe:1: Delegates `sidebar`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 584f71c3162457aa:1: Supports `sidebar` while preserving the nearby constraint: The pages.
// ADDED-LINE-RATIONALE e3540eae60ddb27e:1: Prevents `sidebar` from applying its side effect outside the `widgets / nav_row` state.
// ADDED-LINE-RATIONALE cb81e00ef9030280:1: Keeps `ui` in `sidebar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 7a84911cb640955d:1: Keeps `Icon / Apps` in `sidebar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE bc70550345b3cf09:1: Keeps `tl / All / apps` in `sidebar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 146e1d3cdb99360a:1: Keeps `tl / Every / Redrob` in `sidebar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 3a5e38e057b7a70c:1: Keeps `NavMeta / Count / updates` in `sidebar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 257ac77a6d4e0228:1: Freezes `self` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE be0bf84e03987cb6:1: Keeps `rail` in `sidebar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 79761f8702e556d9:1: Bounds the `sidebar` implementation here so `rail` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ac433433e4c8628a:1: Delegates `sidebar`'s `clicked` work to `clicked` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7975ed8702e501e3:1: Bounds the `sidebar` implementation here so `clicked` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c6438606a7d6e573:1: Delegates `sidebar`'s `set_page / Page / Apps` work to `self.set_page` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7975f38702e50c15:3: Bounds the `sidebar` implementation here so `set_page / Page / Apps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3c0741ac5bbc6eac:1: Delegates `sidebar`'s `models_nav / ui / rail` work to `self.models_nav` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7ea2b1860d63f728:1: Delegates `sidebar`'s `engine_nav / ui / rail` work to `self.engine_nav` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 556514817200339b:1: Supports `sidebar` while preserving the nearby constraint: The foot, from the bottom up: the account row (its menu holds Settings), a hairline, the theme.
// ADDED-LINE-RATIONALE 53e9c3f0a3a41e0f:1: Delegates `sidebar`'s `ui / with_layout / egui` work to `ui.with_layout` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dc4f67a49c909ded:1: Delegates `sidebar`'s `ui / add_space / INSET` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 81f34fb121b5b547:1: Delegates `sidebar`'s `account_menu / ui / rail` work to `self.account_menu` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4bc19d1e95acb241:1: Delegates `sidebar`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4734f663423af0c5:1: Freezes `let` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ae754cdc89df9c83:1: Delegates `sidebar`'s `ui / painter / hline` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 32dba21e87e11da0:1: Delegates `sidebar`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 130be6cba82d0e76:1: Freezes `choice` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE efb605e136f52fa8:1: Freezes `changed` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bf7051c6f76cfc7b:1: Delegates `sidebar`'s `widgets / theme_switch / ui` work to `widgets::theme_switch` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6b4ded78ec791aa3:1: Makes `widgets / theme_switch / ui` an explicit part of `sidebar` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 059ed695ae677d1d:1: Delegates `sidebar`'s `ui / horizontal / ui` work to `ui.horizontal` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6001b973f8a8a501:1: Delegates `sidebar`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ca7052c7365c2646:1: Delegates `sidebar`'s `widgets / theme_switch / ui` work to `widgets::theme_switch` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 424ccc8182d231b4:1: Makes `widgets / theme_switch / ui` an explicit part of `sidebar` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 9132024ef981c8ec:1: Makes `inner` an explicit part of `sidebar` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE f11e613a4bcf9dca:1: Bounds the `sidebar` implementation here so `inner` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 1707ef4843a07ffc:1: Prevents `sidebar` from applying its side effect outside the `changed` state.
// ADDED-LINE-RATIONALE 7052526a712f116f:1: Freezes `self` for `sidebar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 75412b26f9b36235:1: Bounds the `sidebar` implementation here so `settings / theme / choice` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 597680faa8ffeabd:1: Makes `settings / theme / choice` an explicit part of `sidebar` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 0c336d9af511e4f5:4: Bounds the `sidebar` implementation here so `A / page / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5d28f279e59a9f46:1: Supports `page_top_bar` while preserving the nearby constraint: search and Update live in the title bar, its refresh at the end of its filter row.
// ADDED-LINE-RATIONALE fa1d7503d0104454:2: Freezes `t` for `page_top_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE afa30691a5d59128:1: Freezes `let` for `page_top_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d8bb6d1bc3f734ba:1: Freezes `Page` for `page_top_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 74aff5250cdedeb7:1: Freezes `Page` for `page_top_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 02ec92fe2b5c04ab:1: Freezes `Page` for `page_top_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4ed6895ef6bf4895:1: Freezes `Page` for `page_top_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8804964efd89eaaf:1: Freezes `Page` for `page_top_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4878fc69eb33b22a:1: Bounds the `page_top_bar` implementation here so `Page / Settings / tl` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e32a50c58ffa52d6:1: Delegates `page_top_bar`'s `ui / horizontal_centered / ui` work to `ui.horizontal_centered` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3a65af4a5f9024e0:1: Freezes `ui` for `page_top_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f871ddc5df431c1b:1: Supports `page_top_bar` while preserving the nearby constraint: pulls its first ghost button the same way); its hover fill spills into the margin.
// ADDED-LINE-RATIONALE 260b14573692fb85:1: Prevents `page_top_bar` from applying its side effect outside the `widgets / icon_button / ui` state.
// ADDED-LINE-RATIONALE b77b310b529451d3:1: Delegates `page_top_bar`'s `set_page / Page / Apps` work to `self.set_page` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 75412b26f9b36235:2: Bounds the `page_top_bar` implementation here so `set_page / Page / Apps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 1b1170f776920bb0:1: Supports `page_top_bar` while preserving the nearby constraint: DS AppShell top bar: title 15 / 600, meta 12 ink-muted.
// ADDED-LINE-RATIONALE 550e4ec27098f546:1: Freezes `room` for `page_top_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9988bfd599cf9ae2:1: Prevents `page_top_bar` from applying its side effect outside the `meta / is_empty / room` state.
// ADDED-LINE-RATIONALE d5356d07011697b4:1: Delegates `page_top_bar`'s `widgets / hud_elided / ui` work to `widgets::hud_elided` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 75412b26f9b36235:3: Bounds the `page_top_bar` implementation here so `widgets / hud_elided / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 597680faa8ffeabd:2: Makes `widgets / hud_elided / ui` an explicit part of `page_top_bar` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 0c336d9af511e4f5:5: Bounds the `page_top_bar` implementation here so `widgets / hud_elided / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6d759c8ebece9dd5:1: Bounds the `page_top_bar` implementation here so `widgets / hud_elided / ui` cannot leak into the adjacent declaration or control path.
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
