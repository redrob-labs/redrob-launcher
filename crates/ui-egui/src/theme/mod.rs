//! Colours, type and shapes from the Redrob design system: its colour roles (`@redrob-labs/ui`
//! `tokens.json`, generated into `generated.rs` by `cargo xtask tokens`),
//! Pretendard for Latin and Wanted Sans for Korean, hairline rules. Widgets read
//! [`Tokens::get`]; nothing hard-codes a colour.

use std::sync::Arc;

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, FontTweak, Stroke, TextStyle, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeChoice {
    /// Follow the system's light or dark setting.
    #[default]
    System,
    Dark,
    Light,
}

impl ThemeChoice {
    pub const ALL: [Self; 3] = [Self::System, Self::Dark, Self::Light];

    /// The English key translated at draw time into the current UI language
    /// (the shell sets it per frame via `i18n::set_current`). Same-crate lookup,
    /// pure and wasm-safe; no new dependency. Call sites stay unchanged.
    pub fn label(self) -> &'static str {
        match self {
            Self::System => crate::i18n::t("Match system"),
            Self::Dark => crate::i18n::t("Dark"),
            Self::Light => crate::i18n::t("Light"),
        }
    }
}

/// Colour tokens for one appearance: the website's (`globals.css`), by the same names.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub dark: bool,
    /// Page and card surface.
    pub bg: Color32,
    /// Badges, wells and row hovers.
    pub sunken: Color32,
    /// Body text.
    pub ink: Color32,
    /// Headings.
    pub ink_strong: Color32,
    /// Secondary text.
    pub muted: Color32,
    /// Labels and tertiary text.
    pub faint: Color32,
    /// Hairlines.
    pub line: Color32,
    /// Outlined buttons and stronger rules.
    pub line_strong: Color32,
    /// The brand blue, for surfaces.
    pub accent: Color32,
    /// The brand blue, for text and icons.
    pub accent_ink: Color32,
    /// Primary buttons and hover inversion.
    pub invert_bg: Color32,
    pub invert_fg: Color32,
    pub good: Color32,
    pub warn: Color32,
    pub bad: Color32,
}

const fn hex(rgb: u32) -> Color32 {
    let [_, r, g, b] = rgb.to_be_bytes();
    Color32::from_rgb(r, g, b)
}

/// The colour values, generated from the design-system package by `cargo xtask tokens`.
pub mod generated;

/// The design-system package release the colours come from (`@redrob-labs/ui x.y.z`).
pub const TOKENS_SOURCE: &str = generated::SOURCE;

macro_rules! from_generated {
    ($m:ident, $dark:expr) => {
        Self {
            dark: $dark,
            bg: hex(generated::$m::BG),
            sunken: hex(generated::$m::SUNKEN),
            ink: hex(generated::$m::INK),
            ink_strong: hex(generated::$m::INK_STRONG),
            muted: hex(generated::$m::MUTED),
            faint: hex(generated::$m::FAINT),
            line: hex(generated::$m::LINE),
            line_strong: hex(generated::$m::LINE_STRONG),
            accent: hex(generated::$m::ACCENT),
            accent_ink: hex(generated::$m::ACCENT_INK),
            invert_bg: hex(generated::$m::INVERT_BG),
            invert_fg: hex(generated::$m::INVERT_FG),
            good: hex(generated::$m::GOOD),
            warn: hex(generated::$m::WARN),
            bad: hex(generated::$m::BAD),
        }
    };
}

impl Tokens {
    // The values are the Redrob design system's (`@redrob-labs/ui`, `tokens.json`), bound to these
    // roles by xtask/src/tokens.rs and written to `generated.rs`. Nothing here is typed by hand.
    pub const DARK: Self = from_generated!(dark, true);

    pub const LIGHT: Self = from_generated!(light, false);

    pub fn get(ctx: &egui::Context) -> Self {
        if ctx.theme() == egui::Theme::Dark { Self::DARK } else { Self::LIGHT }
    }

    pub fn hairline(&self) -> Stroke {
        Stroke::new(1.0, self.line)
    }

    /// An app's text colour (`--app-ink`): readable on this appearance's background.
    pub fn app_ink(&self, app: &redrob_launcher_core::CraftApp) -> Color32 {
        rgb(if self.dark { app.ink_dark } else { app.ink_light })
    }

    // Roles the design system has but the generated set does not carry, derived from the roles
    // above (no colour is typed by hand). Each names the DS token it stands for and how close it is.

    /// `surface-raised` (#f8f9fb / #141719): halfway from the base to the sunken surface
    /// (#f7f8f9 / #131519).
    pub fn raised(&self) -> Color32 {
        mix(self.bg, self.sunken, 0.5)
    }

    /// `action-primary-hover` (#1733d5 / #507fff): light is exactly `ink-brand`; dark mixes the
    /// brand blue 40% toward `ink-brand` (#4f75ff). Not `accent_ink` in dark, which is #8aafff.
    pub fn primary_hover(&self) -> Color32 {
        if self.dark { mix(self.accent, self.accent_ink, 0.4) } else { self.accent_ink }
    }

    /// `surface-brand-subtle` (#eef4ff / #030c34): the brand blue 8% (dark 12%) over the base
    /// (#eef1ff / #0f1429). Ghost-button hover, focus halo, selected rows.
    pub fn brand_subtle(&self) -> Color32 {
        mix(self.bg, self.accent, if self.dark { 0.12 } else { 0.08 })
    }

    /// `selection` background (#d9e6ff / #09209c): the brand blue 16% (dark 55%) over the base
    /// (#dde3ff / #1c3292).
    pub fn selection(&self) -> Color32 {
        mix(self.bg, self.accent, if self.dark { 0.55 } else { 0.16 })
    }

    /// `focus-ring` (#2b52ff / #8aafff): exactly `action-primary` light, `ink-brand` dark.
    pub fn focus(&self) -> Color32 {
        if self.dark { self.accent_ink } else { self.accent }
    }

    /// `ink-on-brand` (#ffffff in both appearances): text and glyphs on a brand, danger or
    /// product-colour fill, and the switch thumb.
    pub fn on_brand(&self) -> Color32 {
        Color32::WHITE
    }

    /// The tooltip bubble (DS Tooltip: `redrob-black` with white text; dark inverts to
    /// #f8f9fb with #0a0b0c): (fill, text).
    pub fn tooltip(&self) -> (Color32, Color32) {
        (if self.dark { self.ink } else { self.invert_bg }, self.invert_fg)
    }

    /// Danger button hover (DS: `status-danger` at `brightness(0.92)`).
    pub fn bad_hover(&self) -> Color32 {
        scale(self.bad, 0.92)
    }

    /// A subtle badge's fill for a status colour (DS Badge `subtle`: the accent family's step 1
    /// light / step 5 dark), approximated as the colour 12% (dark 18%) over the base.
    pub fn subtle(&self, c: Color32) -> Color32 {
        mix(self.bg, c, if self.dark { 0.18 } else { 0.12 })
    }
}

/// `a` moved toward `b` by `f` (0-1), opaque: a colour between two roles.
pub fn mix(a: Color32, b: Color32, f: f32) -> Color32 {
    let f = f.clamp(0.0, 1.0);
    let ch = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * f).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgb(ch(a.r(), b.r()), ch(a.g(), b.g()), ch(a.b(), b.b()))
}

/// `c` with its red, green and blue scaled by `k` (CSS `brightness(k)`), alpha kept.
pub fn scale(c: Color32, k: f32) -> Color32 {
    let ch = |x: u8| (x as f32 * k).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgba_premultiplied(ch(c.r()), ch(c.g()), ch(c.b()), c.a())
}

/// Corner radii (DS `radius-*`), in points.
pub mod radius {
    /// `radius-xs`: checkbox box, chart containers.
    pub const XS: u8 = 4;
    /// `radius-sm` / `field-radius`: text fields, nav rows, menu items, tooltips.
    pub const SM: u8 = 6;
    /// `radius-md` / `control-radius`: buttons, icon buttons, menus, toasts.
    pub const MD: u8 = 8;
    /// `radius-lg` / `card-radius`: cards, tiles, select lists, table wraps.
    pub const LG: u8 = 12;
    /// `radius-xl`: modals, drawers, the floating sidebar card.
    pub const XL: u8 = 16;
    /// `radius-full`: pills (badges, switches, segmented controls, progress tracks).
    pub const PILL: u8 = 255;
}

/// The space scale (DS `space-*`), in points.
pub mod space {
    /// `space-1`.
    pub const S1: f32 = 4.0;
    /// `space-2`.
    pub const S2: f32 = 8.0;
    /// `space-3`.
    pub const S3: f32 = 12.0;
    /// `space-4`.
    pub const S4: f32 = 16.0;
    /// `space-5`; also `card-padding`.
    pub const S5: f32 = 20.0;
    /// `space-6`; also `section-gap` and the work area's padding.
    pub const S6: f32 = 24.0;
    /// `space-8`.
    pub const S8: f32 = 32.0;
    /// `space-12`.
    pub const S12: f32 = 48.0;
}

/// Control heights (DS `control-h-*`): buttons, inputs, selects.
pub mod control {
    /// `control-h-sm`: small buttons and icon buttons.
    pub const SM: f32 = 32.0;
    /// `control-h-md`: the default.
    pub const MD: f32 = 40.0;
    /// `control-h-lg`: the one prominent action.
    pub const LG: f32 = 48.0;
}

/// Type sizes (DS text styles), in points. Weights come from the family: label/title in
/// [`semibold`], meta in [`medium`].
pub mod text {
    /// `body` (15/24, 400): prose.
    pub const BODY: f32 = 15.0;
    /// The size egui's Body text style keeps (DS controls, menus and selects are 14).
    pub const CONTROL: f32 = 14.0;
    /// `body-sm` (13/20, 400): dense and secondary text.
    pub const BODY_SM: f32 = 13.0;
    /// `label` (13/18, 600): form labels, buttons, tabs.
    pub const LABEL: f32 = 13.0;
    /// `meta` (12/18, 500): the line above a title; sentence case, never upper case.
    pub const META: f32 = 12.0;
    /// `caption` (11/16, 400): helper text.
    pub const CAPTION: f32 = 11.0;
    /// `title-4` (17/24, 600): card and dialog titles.
    pub const TITLE_4: f32 = 17.0;
    /// `title-3` (21/28, 600): modal, drawer and empty-state titles.
    pub const TITLE_3: f32 = 21.0;
    /// `title-2` (27/34, 700): major sections.
    pub const TITLE_2: f32 = 27.0;
}

/// Motion durations (DS `duration-*`), in seconds, and the DS easing curves.
pub mod motion {
    /// `duration-instant` (80 ms): hover and press feedback.
    pub const INSTANT: f32 = 0.080;
    /// `duration-fast` (140 ms): the default state change (colour, border, opacity).
    pub const FAST: f32 = 0.140;
    /// `duration-base` (220 ms): small surfaces in and out, the switch thumb, chevrons.
    pub const BASE: f32 = 0.220;
    /// `duration-slow` (320 ms): drawers, modals, expanding panels, progress width.
    pub const SLOW: f32 = 0.320;

    /// The DS easing curves (CSS `cubic-bezier`).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Easing {
        /// `ease-standard` (0.2, 0, 0, 1): state changes.
        Standard,
        /// `ease-entrance` (0.05, 0.7, 0.1, 1): arrivals.
        Entrance,
        /// `ease-exit` (0.3, 0, 0.8, 0.15): departures, faster than arrivals.
        Exit,
        /// `ease-settle` (0.34, 1.4, 0.64, 1): a small overshoot for switches and chevrons.
        Settle,
    }

    impl Easing {
        /// The curve's control points (x1, y1, x2, y2).
        pub const fn points(self) -> [f32; 4] {
            match self {
                Self::Standard => [0.2, 0.0, 0.0, 1.0],
                Self::Entrance => [0.05, 0.7, 0.1, 1.0],
                Self::Exit => [0.3, 0.0, 0.8, 0.15],
                Self::Settle => [0.34, 1.4, 0.64, 1.0],
            }
        }

        /// Progress at `t` (0-1 of the duration): may leave 0-1 for [`Easing::Settle`].
        pub fn apply(self, t: f32) -> f32 {
            let [x1, y1, x2, y2] = self.points();
            cubic_bezier(x1, y1, x2, y2, t)
        }
    }

    /// CSS `cubic-bezier(x1, y1, x2, y2)` at time `x` (0-1): solves the curve's x for its
    /// parameter by bisection (x is monotonic for x1, x2 in 0-1), then returns its y.
    pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, x: f32) -> f32 {
        let x = x.clamp(0.0, 1.0);
        let at = |a: f32, b: f32, s: f32| {
            let r = 1.0 - s;
            3.0 * r * r * s * a + 3.0 * r * s * s * b + s * s * s
        };
        let (mut lo, mut hi) = (0.0_f32, 1.0_f32);
        for _ in 0..24 {
            let mid = (lo + hi) / 2.0;
            if at(x1, x2, mid) < x {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        at(y1, y2, (lo + hi) / 2.0)
    }

    /// `secs`, or 0 when motion is off (egui's `animation_time` set to 0, which is how the shell
    /// honours reduced motion).
    pub fn duration(ctx: &egui::Context, secs: f32) -> f32 {
        if ctx.global_style().animation_time <= 0.0 { 0.0 } else { secs }
    }
}

/// A DS shadow: `y` offset, `blur`, the colour's alpha light / dark (light shadows are
/// redrob-black, dark ones black).
fn ds_shadow(t: &Tokens, y: i8, blur: u8, light: f32, dark: f32) -> egui::epaint::Shadow {
    let color = if t.dark { tint(Color32::BLACK, dark) } else { tint(t.invert_bg, light) };
    egui::epaint::Shadow { offset: [0, y], blur, spread: 0, color }
}

/// `shadow-sm` (0 1px 2px, .08 / dark .6): resting inputs, the switch thumb, a selected segment.
pub fn shadow_sm(t: &Tokens) -> egui::epaint::Shadow {
    ds_shadow(t, 1, 2, 0.08, 0.6)
}

/// `shadow-md` (0 4px 12px, .10 / dark .7): menus, popovers, tooltips, a hovered card.
pub fn shadow_md(t: &Tokens) -> egui::epaint::Shadow {
    ds_shadow(t, 4, 12, 0.10, 0.7)
}

/// `shadow-lg` (0 16px 40px, .14 / dark .8): modals, drawers, toasts, select lists.
pub fn shadow_lg(t: &Tokens) -> egui::epaint::Shadow {
    ds_shadow(t, 16, 40, 0.14, 0.8)
}

/// Redrob Cowork's floating sidebar card shadow (0 12px 32px -16px, .30). egui shadows have no
/// negative spread, so draw it under the card's rect shrunk by [`FLOATING_SHADOW_INSET`].
pub fn shadow_floating(t: &Tokens) -> egui::epaint::Shadow {
    ds_shadow(t, 12, 32, 0.30, 0.6)
}

/// The negative spread of [`shadow_floating`]: shrink the card's rect by this before casting it.
pub const FLOATING_SHADOW_INSET: f32 = 16.0;

/// The shadow under dialogs and toasts: the DS's `shadow-lg`.
pub fn shadow(t: &Tokens) -> egui::epaint::Shadow {
    shadow_lg(t)
}

fn visuals(t: &Tokens) -> Visuals {
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.bg;
    v.window_fill = t.bg;
    v.extreme_bg_color = t.sunken;
    // DS text fields sit on the base surface with a strong outline (Field control).
    v.text_edit_bg_color = Some(t.bg);
    v.faint_bg_color = t.sunken;
    v.code_bg_color = t.sunken;
    v.window_stroke = Stroke::new(1.0, t.line);
    v.window_corner_radius = CornerRadius::same(radius::XL);
    v.menu_corner_radius = CornerRadius::same(radius::MD);
    v.window_shadow = shadow_lg(t);
    v.popup_shadow = shadow_md(t);
    v.override_text_color = Some(t.ink);
    v.hyperlink_color = t.accent_ink;
    v.selection.bg_fill = t.selection();
    v.selection.stroke = Stroke::new(1.0, t.ink);
    v.text_cursor.stroke = Stroke::new(2.0, t.focus());
    v.warn_fg_color = t.warn;
    v.error_fg_color = t.bad;
    for (w, fill, stroke, fg) in [
        (&mut v.widgets.noninteractive, t.bg, t.line, t.muted),
        (&mut v.widgets.inactive, t.bg, t.line_strong, t.ink),
        (&mut v.widgets.hovered, t.sunken, t.muted, t.ink_strong),
        // Pressed and focused fields: the brand blue outline (DS Field focus).
        (&mut v.widgets.active, t.sunken, t.accent, t.ink_strong),
        (&mut v.widgets.open, t.sunken, t.accent, t.ink_strong),
    ] {
        w.bg_fill = fill;
        w.weak_bg_fill = fill;
        w.bg_stroke = Stroke::new(1.0, stroke);
        w.corner_radius = CornerRadius::same(radius::MD);
        w.fg_stroke = Stroke::new(1.0, fg);
        w.expansion = 0.0;
    }
    // Labels and separators are not controls: hairlines, no radius to speak of.
    v.widgets.noninteractive.corner_radius = CornerRadius::same(radius::SM);
    v
}

/// Applies both appearances (egui switches between them with the theme preference).
pub fn apply(ctx: &egui::Context, choice: ThemeChoice) {
    ctx.set_visuals_of(egui::Theme::Dark, visuals(&Tokens::DARK));
    ctx.set_visuals_of(egui::Theme::Light, visuals(&Tokens::LIGHT));
    ctx.set_theme(match choice {
        ThemeChoice::System => egui::ThemePreference::System,
        ThemeChoice::Dark => egui::ThemePreference::Dark,
        ThemeChoice::Light => egui::ThemePreference::Light,
    });
    ctx.all_styles_mut(|style| {
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        // Between a checkbox (or radio button) and its label.
        style.spacing.icon_spacing = 10.0;
        // DS Button md pads 16 px either side; egui's padding also sizes ComboBoxes and menu rows,
        // so it stays at 12 and [`crate::widgets::button`] pads its own. The height comes from
        // `interact_size`.
        style.spacing.button_padding = egui::vec2(space::S3, 6.0);
        style.spacing.interact_size.y = CONTROL_H;
        // DS Menu list: 8 px of padding.
        style.spacing.menu_margin = egui::Margin::same(space::S2 as i8);
        style.spacing.window_margin = egui::Margin::same(space::S1 as i8);
        // Body stays 14 (DS controls, menus and selects), not the DS's 15 prose size: the
        // launcher's text is almost all control text, and 15 overflows the fixed table columns.
        style.text_styles = [
            (TextStyle::Heading, FontId::new(text::TITLE_3, display())),
            (TextStyle::Body, FontId::new(text::CONTROL, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(text::CONTROL, semibold())),
            (TextStyle::Small, FontId::new(text::META, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(text::BODY_SM, FontFamily::Monospace)),
        ]
        .into();
    });
}

/// The Korean system face of the Redrob design system (Wanted Sans Regular, SIL OFL 1.1; licence
/// beside the file). Pretendard carries no Hangul, so Korean falls to this face.
pub const KOREAN_FONT: &[u8] = include_bytes!("../../../../assets/fonts/WantedSans-Regular.ttf");

/// Korean system text is set at 96% of the Latin size (design system, `35-korean.md`).
const KOREAN_SCALE: f32 = 0.96;

/// Height of buttons, inputs and filter cells: the DS's default control (`control-h-md`).
pub const CONTROL_H: f32 = control::MD;

/// Labels: the design system's meta line, 12 px Pretendard 500, sentence case, no extra tracking.
pub const HUD_SIZE: f32 = 12.0;
pub const HUD_SPACING: f32 = 0.0;
/// Buttons: the design system's label, Pretendard 600, no extra tracking.
pub const BUTTON_SPACING: f32 = 0.0;

/// Pretendard 500.
pub fn medium() -> FontFamily {
    FontFamily::Name("medium".into())
}

/// Pretendard 600.
pub fn semibold() -> FontFamily {
    FontFamily::Name("semibold".into())
}

/// Headings and app names: Pretendard 600 (the design system's card and dialog title).
pub fn display() -> FontFamily {
    FontFamily::Name("display".into())
}

/// Labels and badges: Pretendard 500 (the design system's meta line). The system uses one
/// typeface; a monospace face is for code only.
pub fn mono() -> FontFamily {
    FontFamily::Name("mono".into())
}

/// Buttons and app number tabs: Pretendard 600 (the design system's label).
pub fn mono_bold() -> FontFamily {
    FontFamily::Name("mono-bold".into())
}

/// A font registered under `name`, and the family it sets.
type Face<'a> = (&'a str, &'static [u8], FontFamily);

/// The Redrob design system's Latin face (Pretendard, at the weights the UI uses) and its Korean
/// face (Wanted Sans) behind it in every family. Pretendard carries no Hangul, so each glyph
/// falls to the right face (design system `30-typography.md`). Files are the design system's,
/// byte for byte (SIL OFL 1.1, licences beside them).
pub fn install_fonts(ctx: &egui::Context, cjk: bool) {
    const REGULAR: &[u8] = include_bytes!("../../../../assets/fonts/Pretendard-Regular.ttf");
    const MEDIUM: &[u8] = include_bytes!("../../../../assets/fonts/Pretendard-Medium.ttf");
    const SEMIBOLD: &[u8] = include_bytes!("../../../../assets/fonts/Pretendard-SemiBold.ttf");
    let mut fonts = FontDefinitions::default();
    // egui's own fonts stay behind ours, for symbols and emoji ours don't have.
    let fallback: Vec<String> = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    fonts
        .font_data
        .insert("WantedSans".to_owned(), Arc::new(FontData::from_static(KOREAN_FONT).tweak(FontTweak { scale: KOREAN_SCALE, ..Default::default() })));
    let faces: [Face<'_>; 7] = [
        ("Pretendard", REGULAR, FontFamily::Proportional),
        ("Pretendard-Medium", MEDIUM, medium()),
        ("Pretendard-SemiBold", SEMIBOLD, semibold()),
        ("Pretendard-SemiBold", SEMIBOLD, display()),
        ("Pretendard", REGULAR, FontFamily::Monospace),
        ("Pretendard-Medium", MEDIUM, mono()),
        ("Pretendard-SemiBold", SEMIBOLD, mono_bold()),
    ];
    for (name, bytes, family) in faces {
        fonts.font_data.entry(name.to_owned()).or_insert_with(|| Arc::new(FontData::from_static(bytes)));
        let mut stack = vec![name.to_owned(), "WantedSans".to_owned()];
        stack.extend(fallback.iter().cloned());
        fonts.families.insert(family, stack);
    }
    // Chinese, behind everything else, from the system: only when asked for, as the font is large.
    if cjk && let Some(font) = cjk_font() {
        fonts.font_data.insert("CJK".to_owned(), font);
        for stack in fonts.families.values_mut() {
            stack.push("CJK".to_owned());
        }
    }
    ctx.set_fonts(fonts);
}

/// The system's Simplified Chinese font, read once on the first call (tens of MB). Desktop only:
/// the web build has none, so Chinese can't be drawn there.
pub fn cjk_font() -> Option<Arc<FontData>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        CJK_FONT.get_or_init(|| crate::cjk_font::load().map(Arc::new)).clone()
    }
    #[cfg(target_arch = "wasm32")]
    {
        None
    }
}

/// The system's Chinese font once [`cjk_font`] has looked for it.
#[cfg(not(target_arch = "wasm32"))]
static CJK_FONT: std::sync::OnceLock<Option<Arc<FontData>>> = std::sync::OnceLock::new();

/// Has the system's Chinese font already been loaded? Never loads it.
pub fn cjk_font_loaded() -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        CJK_FONT.get().is_some_and(Option::is_some)
    }
    #[cfg(target_arch = "wasm32")]
    {
        false
    }
}

/// Can Chinese be drawn? Loads the system font on the first call ([`cjk_font`]).
pub fn cjk_font_available() -> bool {
    cjk_font().is_some()
}

/// Mixes `c` into the background at `alpha` (0â€“1): tinted washes.
pub fn tint(c: Color32, alpha: f32) -> Color32 {
    let a = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a)
}

pub fn rgb(c: [u8; 3]) -> Color32 {
    Color32::from_rgb(c[0], c[1], c[2])
}

#[cfg(test)]
mod tests {
    use super::motion::Easing;
    use super::*;

    #[test]
    fn easing_curves_start_at_0_end_at_1_and_settle_overshoots() {
        for e in [Easing::Standard, Easing::Entrance, Easing::Exit, Easing::Settle] {
            assert!(e.apply(0.0).abs() < 1e-3, "{e:?}");
            assert!((e.apply(1.0) - 1.0).abs() < 1e-3, "{e:?}");
        }
        // Standard is fast out of the gate; exit starts slowly; settle passes 1 before it lands.
        assert!(Easing::Standard.apply(0.5) > 0.7);
        assert!(Easing::Exit.apply(0.3) < 0.1);
        assert!((0..=100).map(|i| Easing::Settle.apply(i as f32 / 100.0)).any(|y| y > 1.02));
    }

    #[test]
    fn derived_roles_are_close_to_the_design_system() {
        let near = |c: Color32, rgb: u32| {
            let [_, r, g, b] = rgb.to_be_bytes();
            c.r().abs_diff(r) <= 12 && c.g().abs_diff(g) <= 12 && c.b().abs_diff(b) <= 12
        };
        // action-primary-hover
        assert!(near(Tokens::LIGHT.primary_hover(), 0x1733d5));
        assert!(near(Tokens::DARK.primary_hover(), 0x507fff));
        // surface-raised
        assert!(near(Tokens::LIGHT.raised(), 0xf8f9fb));
        assert!(near(Tokens::DARK.raised(), 0x141719));
        // surface-brand-subtle
        assert!(near(Tokens::LIGHT.brand_subtle(), 0xeef4ff));
        // selection (light)
        assert!(near(Tokens::LIGHT.selection(), 0xd9e6ff));
    }
}
