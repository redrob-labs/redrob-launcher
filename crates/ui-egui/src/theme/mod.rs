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
}

/// Mixes `c` into the background at `alpha` (0â€“1): tinted washes.
pub fn tint(c: Color32, alpha: f32) -> Color32 {
    let a = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a)
}

pub fn rgb(c: [u8; 3]) -> Color32 {
    Color32::from_rgb(c[0], c[1], c[2])
}

/// The shadow under menus, dialogs and toasts.
pub fn shadow(t: &Tokens) -> egui::epaint::Shadow {
    egui::epaint::Shadow { offset: [0, 8], blur: 24, spread: 0, color: Color32::from_black_alpha(if t.dark { 140 } else { 36 }) }
}

fn visuals(t: &Tokens) -> Visuals {
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.bg;
    v.window_fill = t.bg;
    v.extreme_bg_color = t.sunken;
    v.text_edit_bg_color = Some(t.sunken);
    v.faint_bg_color = t.sunken;
    v.code_bg_color = t.sunken;
    v.window_stroke = Stroke::new(1.0, t.line_strong);
    v.window_corner_radius = CornerRadius::ZERO;
    v.menu_corner_radius = CornerRadius::ZERO;
    v.window_shadow = shadow(t);
    v.popup_shadow = shadow(t);
    v.override_text_color = Some(t.ink);
    v.hyperlink_color = t.accent_ink;
    v.selection.bg_fill = t.accent;
    v.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    v.warn_fg_color = t.warn;
    v.error_fg_color = t.bad;
    for (w, fill, stroke, fg) in [
        (&mut v.widgets.noninteractive, t.bg, t.line, t.muted),
        (&mut v.widgets.inactive, t.bg, t.line, t.ink),
        (&mut v.widgets.hovered, t.sunken, t.line_strong, t.ink_strong),
        (&mut v.widgets.active, t.sunken, t.line_strong, t.ink_strong),
        (&mut v.widgets.open, t.sunken, t.line_strong, t.ink_strong),
    ] {
        w.bg_fill = fill;
        w.weak_bg_fill = fill;
        w.bg_stroke = Stroke::new(1.0, stroke);
        w.corner_radius = CornerRadius::ZERO;
        w.fg_stroke = Stroke::new(1.0, fg);
        w.expansion = 0.0;
    }
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
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        style.spacing.interact_size.y = CONTROL_H;
        style.spacing.menu_margin = egui::Margin::same(4);
        style.spacing.window_margin = egui::Margin::same(4);
        style.text_styles = [
            (TextStyle::Heading, FontId::new(24.0, display())),
            (TextStyle::Body, FontId::new(14.0, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(13.5, medium())),
            (TextStyle::Small, FontId::new(12.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(13.0, FontFamily::Monospace)),
        ]
        .into();
    });
}

/// The Korean system face of the Redrob design system (Wanted Sans Regular, SIL OFL 1.1; licence
/// beside the file). Pretendard carries no Hangul, so Korean falls to this face.
pub const KOREAN_FONT: &[u8] = include_bytes!("../../../assets/fonts/WantedSans-Regular.ttf");

/// Korean system text is set at 96% of the Latin size (design system, `35-korean.md`).
const KOREAN_SCALE: f32 = 0.96;

/// Height of buttons, inputs and filter cells (the site's medium button).
pub const CONTROL_H: f32 = 34.0;

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
    const REGULAR: &[u8] = include_bytes!("../../../assets/fonts/Pretendard-Regular.ttf");
    const MEDIUM: &[u8] = include_bytes!("../../../assets/fonts/Pretendard-Medium.ttf");
    const SEMIBOLD: &[u8] = include_bytes!("../../../assets/fonts/Pretendard-SemiBold.ttf");
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
