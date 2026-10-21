//! The Apps page's hero banner: a vivid, Apple Music-style gradient card that introduces the Redrob
//! product suite, at the top of the Apps page above the filter row, on every filter.
//!
//! The gradient falls from Apple Music's red in the top-right corner through magenta and violet
//! into Redrob Blue in the bottom-left, where the white copy sits. It is computed once into a small
//! texture and stretched over the banner (it has no fine detail, so linear filtering keeps it
//! smooth). Nothing on it moves: the design system's motion law keeps type and marks still.

use egui::{Color32, ColorImage, CornerRadius, Rect, RichText, Sense, TextureHandle, TextureOptions, Ui, vec2};

use crate::app::LauncherUi;
use crate::theme;

/// The banner's height at its roomiest; a narrow window lets the copy wrap and grow it.
const HERO_MIN_H: f32 = 168.0;
/// Padding inside the banner (DS space-8).
const HERO_PAD: f32 = 32.0;
/// The copy's widest measure, so the title stays on one or two lines.
const HERO_COPY_W: f32 = 480.0;
/// The suite's icons on the banner's right, in a 3-across grid.
const HERO_ICON: f32 = 40.0;
/// The gap between those icons.
const HERO_ICON_GAP: f32 = 12.0;
/// How many icons sit on one row of that grid.
const HERO_ICON_COLS: usize = 3;
/// Below this banner width the icon grid is left out and the copy takes the whole banner.
const HERO_ICONS_MIN_W: f32 = 640.0;
/// The gradient texture's width: a 4:1 strip, matching the banner's usual proportions.
const GRADIENT_W: usize = 512;
/// The gradient texture's height, a quarter of [`GRADIENT_W`] for that 4:1 strip.
const GRADIENT_H: usize = 128;

/// The gradient's colour stops from the top-right corner (0) to the bottom-left (1), as
/// (position 0..1, sRGB): Apple Music's red, a magenta, a blue-violet, then Redrob Blue (`blue-6`).
const STOPS: [(f32, [u8; 3]); 4] = [(0.0, [0xfa, 0x2d, 0x48]), (0.3, [0xe0, 0x2a, 0x8c]), (0.62, [0x7a, 0x3c, 0xf5]), (1.0, [0x2b, 0x52, 0xff])];

/// A soft round light over the gradient: centre (in 0..1 of the banner, may sit outside it), radius
/// (in banner heights), colour and strength. Together they give the gradient Apple Music's
/// mesh-like depth.
struct Glow {
    /// Where the light is brightest, as fractions of the banner's width and height.
    center: (f32, f32),
    /// How far it reaches, in banner heights.
    radius: f32,
    /// Its colour (sRGB).
    color: [u8; 3],
    /// How much of its colour it lays over the gradient at its centre (0..1).
    strength: f32,
}

/// A warm pink light in the top-right, where the red starts, and a deep blue shade in the
/// bottom-left under the copy (it keeps the white text readable).
const GLOWS: [Glow; 2] = [
    Glow { center: (0.98, -0.25), radius: 1.4, color: [0xff, 0x6a, 0x88], strength: 0.55 },
    Glow { center: (0.02, 1.25), radius: 1.5, color: [0x1a, 0x2c, 0xc8], strength: 0.6 },
];

impl LauncherUi {
    /// Draws the hero banner across the page's width: the suite's title and one line on what it
    /// does in the bottom-left corner in white, the suite's app icons (exactly as supplied) on the
    /// right when there is room.
    ///
    /// * `ui` - the Apps page's scroll content, top-down; the banner takes its full width.
    pub(crate) fn hero(&self, ui: &mut Ui) {
        let width = ui.available_width();
        let show_icons = width >= HERO_ICONS_MIN_W;
        let icons_w = if show_icons { HERO_ICON_COLS as f32 * (HERO_ICON + HERO_ICON_GAP) - HERO_ICON_GAP } else { 0.0 };
        let copy_w = (width - 2.0 * HERO_PAD - if show_icons { icons_w + HERO_PAD } else { 0.0 }).clamp(1.0, HERO_COPY_W);
        let title = hero_title(tl!("Work, create and build in one suite."));
        let body = RichText::new(tl!("Edit video, model in 3D, write code and run your office. Every app runs its AI through the same engine."))
            .size(theme::text::BODY)
            .color(Color32::from_white_alpha(225));
        // Measure the wrapped copy first, so a narrow window grows the banner instead of clipping it.
        let title_h = wrapped_height(ui, title.clone(), copy_w);
        let body_h = wrapped_height(ui, body.clone(), copy_w);
        let copy_h = title_h + theme::space::S2 + body_h;
        let height = (copy_h + 2.0 * HERO_PAD).max(HERO_MIN_H);
        let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());

        let radius = CornerRadius::same(theme::radius::XL);
        egui::Image::new((gradient(ui.ctx()).id(), rect.size())).corner_radius(radius).paint_at(ui, rect);

        // The copy, anchored to the banner's bottom-left padding. A detached child: laying it out
        // never moves the page's cursor back up into the banner.
        let copy = Rect::from_min_size(egui::pos2(rect.left() + HERO_PAD, rect.bottom() - HERO_PAD - copy_h), vec2(copy_w, copy_h));
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(copy).layout(egui::Layout::top_down(egui::Align::Min)));
        child.spacing_mut().item_spacing.y = theme::space::S2;
        child.add(egui::Label::new(title).wrap());
        child.add(egui::Label::new(body).wrap());

        if show_icons {
            // The suite's icons in page order, as a grid centred on the banner's right.
            let ids = crate::shelves::page_order();
            let rows = ids.len().div_ceil(HERO_ICON_COLS);
            let grid_h = rows as f32 * (HERO_ICON + HERO_ICON_GAP) - HERO_ICON_GAP;
            let origin = egui::pos2(rect.right() - HERO_PAD - icons_w, rect.center().y - grid_h / 2.0);
            for (i, id) in ids.iter().enumerate() {
                let Some(icon) = crate::icons::app(id) else { continue };
                let (row, col) = (i / HERO_ICON_COLS, i % HERO_ICON_COLS);
                let min = origin + vec2(col as f32 * (HERO_ICON + HERO_ICON_GAP), row as f32 * (HERO_ICON + HERO_ICON_GAP));
                egui::Image::new(icon).paint_at(ui, Rect::from_min_size(min, vec2(HERO_ICON, HERO_ICON)));
            }
        }
    }
}

/// The banner's title: DS title-3 (21, 600) in the display face, white, tracked in like the
/// site's headings.
///
/// * `text` - the translated title.
fn hero_title(text: &str) -> RichText {
    crate::widgets::heading(text, theme::text::TITLE_3, Color32::WHITE)
}

/// The height `text` takes wrapped at `width`.
///
/// * `ui` - any Ui, for its fonts and style.
/// * `text` - the styled text to measure.
/// * `width` - the wrap width.
fn wrapped_height(ui: &Ui, text: RichText, width: f32) -> f32 {
    let job = egui::WidgetText::from(text).into_layout_job(ui.style(), egui::FontSelection::Default, egui::Align::Min);
    let mut job = std::sync::Arc::unwrap_or_clone(job);
    job.wrap.max_width = width;
    ui.fonts_mut(|f| f.layout_job(job)).size().y
}

/// Paints the hero's gradient over `rect` at strength `k`, clipped to `radius`: the hovered app
/// tile's edge uses it, so the page has one accent.
///
/// * `ui` - the Ui whose painter (and clip) draw it.
/// * `rect` - where the gradient is stretched.
/// * `radius` - the corners it is clipped to.
/// * `k` - its opacity, 0 (nothing) to 1 (full).
pub(crate) fn paint_gradient(ui: &Ui, rect: Rect, radius: CornerRadius, k: f32) {
    egui::Image::new((gradient(ui.ctx()).id(), rect.size())).corner_radius(radius).tint(Color32::WHITE.gamma_multiply(k)).paint_at(ui, rect);
}

/// The gradient texture, made on first use and kept in the context's memory.
///
/// * `ctx` - the egui context the texture belongs to.
fn gradient(ctx: &egui::Context) -> TextureHandle {
    let id = egui::Id::new("redrob-hero-gradient");
    if let Some(handle) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) {
        return handle;
    }
    let handle = ctx.load_texture("redrob-hero-gradient", gradient_image(), TextureOptions::LINEAR);
    ctx.data_mut(|d| d.insert_temp(id, handle.clone()));
    handle
}

/// The gradient as pixels: [`STOPS`] along a shallow diagonal from the top-right corner to the
/// bottom-left, then each of [`GLOWS`] laid over it.
fn gradient_image() -> ColorImage {
    let aspect = GRADIENT_W as f32 / GRADIENT_H as f32;
    let mut pixels = Vec::with_capacity(GRADIENT_W * GRADIENT_H);
    for y in 0..GRADIENT_H {
        for x in 0..GRADIENT_W {
            let (u, v) = ((x as f32 + 0.5) / GRADIENT_W as f32, (y as f32 + 0.5) / GRADIENT_H as f32);
            // 0 in the top-right corner, 1 in the bottom-left: mostly across, a little down.
            let mut c = along(0.7 * (1.0 - u) + 0.3 * v);
            for g in &GLOWS {
                // Distance in banner heights, so the glows stay round on the wide strip.
                let d = (((u - g.center.0) * aspect).powi(2) + (v - g.center.1).powi(2)).sqrt() / g.radius;
                let k = g.strength * (1.0 - d.clamp(0.0, 1.0)).powi(2);
                c = mix3(c, rgb_f(g.color), k);
            }
            pixels.push(Color32::from_rgb(to_u8(c[0]), to_u8(c[1]), to_u8(c[2])));
        }
    }
    ColorImage::new([GRADIENT_W, GRADIENT_H], pixels)
}

/// The colour at `t` (0..1) along [`STOPS`].
///
/// * `t` - how far along the stops to sample, 0 (first stop) to 1 (last).
fn along(t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    let mut prev = STOPS[0];
    for stop in STOPS {
        if t <= stop.0 {
            let span = (stop.0 - prev.0).max(f32::EPSILON);
            return mix3(rgb_f(prev.1), rgb_f(stop.1), (t - prev.0) / span);
        }
        prev = stop;
    }
    rgb_f(prev.1)
}

/// `a` moved toward `b` by `k` (0..1), per channel.
///
/// * `a` - the colour at `k` = 0.
/// * `b` - the colour at `k` = 1.
/// * `k` - how far from `a` to `b`, 0 to 1.
fn mix3(a: [f32; 3], b: [f32; 3], k: f32) -> [f32; 3] {
    [a[0] + (b[0] - a[0]) * k, a[1] + (b[1] - a[1]) * k, a[2] + (b[2] - a[2]) * k]
}

/// An sRGB triple as 0..1 floats.
///
/// * `c` - the three sRGB bytes.
fn rgb_f(c: [u8; 3]) -> [f32; 3] {
    [c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0]
}

/// A 0..1 channel back to a byte.
///
/// * `c` - the channel value, clamped to 0..1.
fn to_u8(c: f32) -> u8 {
    (c.clamp(0.0, 1.0) * 255.0).round() as u8
}
