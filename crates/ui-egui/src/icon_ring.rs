//! The violet ring an app tile's icon takes on hover: a 2 px gradient border hugging the icon,
//! from the Design violet in its top-left corner through a blue-violet into Redrob Blue in its
//! bottom-right. Like the hero's gradient it is computed once into a small texture.

use egui::{Color32, ColorImage, CornerRadius, Rect, TextureHandle, TextureOptions, Ui};

/// How far the ring reaches outside the icon, in points.
pub(crate) const RING_W: f32 = 2.0;
/// The app icons' own corner radius at the tile's 36 px (their squircle is about 22 % of the side).
pub(crate) const ICON_RADIUS: f32 = 8.0;
/// The ring texture's side, in pixels (no fine detail, so linear filtering keeps it smooth).
const SIDE: usize = 64;
/// The ring's colour stops from the top-left corner (0) to the bottom-right (1), sRGB: the Design
/// violet, a blue-violet, Redrob Blue.
const STOPS: [(f32, [u8; 3]); 3] = [(0.0, [0xc1, 0x62, 0xf4]), (0.5, [0x7a, 0x3c, 0xf5]), (1.0, [0x2b, 0x52, 0xff])];

/// Paints the hover ring round `icon` at strength `k`: the gradient on a rounded rect
/// [`RING_W`] larger than the icon, then the tile's own fill over the icon's area, so only a ring
/// shows once the icon is painted on top.
///
/// * `ui` - the tile's Ui; its painter (and clip) draw the ring.
/// * `icon` - where the icon is painted, lift included.
/// * `fill` - the tile's fill, laid inside the ring so the icon's transparent corners show it.
/// * `k` - the hover's progress, 0 (nothing drawn) to 1 (the full ring).
pub(crate) fn paint(ui: &Ui, icon: Rect, fill: Color32, k: f32) {
    if k <= 0.0 {
        return;
    }
    let outer = CornerRadius::same((ICON_RADIUS + RING_W).round() as u8);
    egui::Image::new((texture(ui.ctx()).id(), icon.expand(RING_W).size()))
        .corner_radius(outer)
        .tint(Color32::WHITE.gamma_multiply(k))
        .paint_at(ui, icon.expand(RING_W));
    ui.painter().rect_filled(icon, CornerRadius::same(ICON_RADIUS as u8), fill.gamma_multiply(k));
}

/// The ring's gradient texture, made on first use and kept in the context's memory.
///
/// * `ctx` - the egui context the texture belongs to.
fn texture(ctx: &egui::Context) -> TextureHandle {
    let id = egui::Id::new("redrob-icon-ring");
    if let Some(handle) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) {
        return handle;
    }
    let handle = ctx.load_texture("redrob-icon-ring", image(), TextureOptions::LINEAR);
    ctx.data_mut(|d| d.insert_temp(id, handle.clone()));
    handle
}

/// The ring's gradient as pixels: [`STOPS`] along the top-left to bottom-right diagonal.
fn image() -> ColorImage {
    let mut pixels = Vec::with_capacity(SIDE * SIDE);
    for y in 0..SIDE {
        for x in 0..SIDE {
            let t = (x + y) as f32 / (2 * (SIDE - 1)) as f32;
            pixels.push(along(t));
        }
    }
    ColorImage::new([SIDE, SIDE], pixels)
}

/// The colour at `t` (0..1) along [`STOPS`].
///
/// * `t` - the position along the diagonal; clamped to 0..1.
fn along(t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let mut prev = STOPS[0];
    for stop in STOPS {
        if t <= stop.0 {
            let k = (t - prev.0) / (stop.0 - prev.0).max(f32::EPSILON);
            let ch = |i: usize| (prev.1[i] as f32 + (stop.1[i] as f32 - prev.1[i] as f32) * k).round() as u8;
            return Color32::from_rgb(ch(0), ch(1), ch(2));
        }
        prev = stop;
    }
    Color32::from_rgb(prev.1[0], prev.1[1], prev.1[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ring runs violet (top-left) into Redrob Blue (bottom-right).
    #[test]
    fn ring_runs_violet_to_blue() {
        let img = image();
        let first = img.pixels[0];
        let last = img.pixels[SIDE * SIDE - 1];
        assert_eq!((first.r(), first.g(), first.b()), (0xc1, 0x62, 0xf4));
        assert_eq!((last.r(), last.g(), last.b()), (0x2b, 0x52, 0xff));
    }
}
