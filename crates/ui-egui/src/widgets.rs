//! Small building blocks in the Redrob design system's style: labels, square buttons, badges and
//! hairline cell groups.
//!
//! Upper-case text is drawn upper case but announced to screen readers as written.

use std::sync::Arc;

use egui::{Color32, CornerRadius, Galley, Rect, Response, RichText, Sense, Stroke, Ui, vec2};

use crate::glyphs::Icon;
use crate::model::Tone;
use crate::theme::{self, Tokens, tint};

pub fn tone_color(t: &Tokens, tone: Tone) -> Color32 {
    match tone {
        Tone::Good => t.good,
        Tone::Warn => t.warn,
        Tone::Info => t.accent_ink,
        Tone::Neutral => t.muted,
        Tone::Faint => t.faint,
    }
}

/// `text` laid out as written in `family`, `spacing` points between letters. The design system
/// never sets labels in capitals, so the text is not upper-cased.
pub fn caps(ui: &Ui, text: &str, size: f32, family: egui::FontFamily, spacing: f32, color: Color32) -> Arc<Galley> {
    let format = egui::TextFormat { font_id: egui::FontId::new(size, family), color, extra_letter_spacing: spacing, ..Default::default() };
    ui.painter().layout_job(egui::text::LayoutJob::single_section(text.to_owned(), format))
}

/// A label: the design system's meta line (12 px Pretendard 500, sentence case).
pub fn hud_galley(ui: &Ui, text: &str, color: Color32) -> Arc<Galley> {
    caps(ui, text, theme::HUD_SIZE, theme::mono(), theme::HUD_SPACING, color)
}

/// Paints `galley` at the left of `rect`, centred vertically.
fn paint_left(ui: &Ui, rect: Rect, galley: Arc<Galley>, color: Color32) {
    let pos = egui::pos2(rect.left(), rect.center().y - galley.size().y / 2.0);
    ui.painter().galley_with_override_text_color(pos, galley, color);
}

/// Paints `galley` centred in `rect`.
fn paint_centred(ui: &Ui, rect: Rect, galley: Arc<Galley>, color: Color32) {
    ui.painter().galley_with_override_text_color(rect.center() - galley.size() / 2.0, galley, color);
}

/// A mono label.
pub fn hud(ui: &mut Ui, text: &str, color: Color32) -> Response {
    let galley = hud_galley(ui, text, color);
    let (rect, resp) = ui.allocate_exact_size(galley.size(), Sense::hover());
    paint_left(ui, rect, galley, color);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
    resp
}

/// A mono label cut to one line with "…" where it is wider than `max_width`; the full text shows
/// on hover and is what screen readers read.
pub fn hud_elided(ui: &mut Ui, text: &str, color: Color32, max_width: f32) -> Response {
    let format = egui::TextFormat {
        font_id: egui::FontId::new(theme::HUD_SIZE, theme::mono()),
        color,
        extra_letter_spacing: theme::HUD_SPACING,
        ..Default::default()
    };
    let mut job = egui::text::LayoutJob::single_section(text.to_owned(), format);
    job.wrap = egui::text::TextWrapping::truncate_at_width(max_width.min(ui.available_width()).max(0.0));
    let galley = ui.painter().layout_job(job);
    let elided = galley.elided;
    let (rect, resp) = ui.allocate_exact_size(galley.size(), Sense::hover());
    paint_left(ui, rect, galley, color);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
    if elided { resp.on_hover_text(text) } else { resp }
}

/// A DS meta line over a section: the brand's 2 x 12 tick raked at 40 degrees in Redrob Blue,
/// then the label in 12 px medium ink-secondary, sentence case.
pub fn meta_line(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = theme::space::S2;
        let (r, _) = ui.allocate_exact_size(vec2(8.0, 12.0), Sense::hover());
        let lean = (40.0_f32).to_radians().tan() * 6.0;
        let c = r.center();
        let pts = vec![
            egui::pos2(c.x - 1.0 - lean, c.y + 6.0),
            egui::pos2(c.x + 1.0 - lean, c.y + 6.0),
            egui::pos2(c.x + 1.0 + lean, c.y - 6.0),
            egui::pos2(c.x - 1.0 + lean, c.y - 6.0),
        ];
        ui.painter().add(egui::Shape::convex_polygon(pts, t.accent, Stroke::NONE));
        hud(ui, label, t.muted);
    });
}

/// The DS Loader inline (three bars raked at 40 degrees passing from border-strong to `active`
/// in turn), `size` tall: the launcher's only "working" indicator; no spinning ring. With motion
/// off (egui `animation_time` 0, the shell's reduced-motion setting) the bars stand still,
/// halfway lit, and ask for no repaints.
pub fn loader_inline(ui: &mut Ui, size: f32, active: Color32) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    paint_loader(ui, rect, active, t.line_strong);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::ProgressIndicator, true, tl!("Working…")));
    resp
}

/// Paints the DS Loader into `rect`, animated unless motion is off.
fn paint_loader(ui: &Ui, rect: Rect, active: Color32, rest: Color32) {
    if theme::motion::duration(ui.ctx(), 1.0) == 0.0 {
        let still = theme::mix(rest, active, 0.5);
        crate::glyphs::loader(ui.painter(), rect, still, still, 0.0);
    } else {
        crate::glyphs::loader(ui.painter(), rect, rest, active, ui.input(|i| i.time));
        ui.ctx().request_repaint();
    }
}

/// A DS Card on a page: surface-raised, a border-subtle hairline, radius 12, padding 20.
pub fn card_frame(t: &Tokens) -> egui::Frame {
    egui::Frame::new()
        .fill(t.raised())
        .stroke(t.hairline())
        .corner_radius(CornerRadius::same(theme::radius::LG))
        .inner_margin(egui::Margin::same(theme::space::S5 as i8))
}

/// A full-width hairline between rows inside a card, with `gap` above and below.
pub fn row_rule(ui: &mut Ui, gap: f32) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(gap);
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().hline(r.x_range(), r.center().y, t.hairline());
    ui.add_space(gap);
}

/// The DS Alert, brand tone: surface-brand-subtle with a blue-2 edge, radius 8, padding 16 / 12.
/// For things that stay until they are dealt with (an update, an announcement).
pub fn alert_frame(t: &Tokens) -> egui::Frame {
    egui::Frame::new()
        .fill(t.brand_subtle())
        .stroke(Stroke::new(1.0, t.alert_edge()))
        .corner_radius(CornerRadius::same(theme::radius::MD))
        .inner_margin(egui::Margin::symmetric(16, 12))
}

/// [`alert_frame`] for a one-line alert (the launcher's own update): padding 12 / 6, so with a
/// 32 px control the bar is 44 px tall (was about 66).
pub fn alert_frame_dense(t: &Tokens) -> egui::Frame {
    alert_frame(t).inner_margin(egui::Margin::symmetric(12, 6))
}

/// How far in from the window's edges a custom title bar's window resizes ([`window_edges`]).
/// The title bar's own controls leave this band alone.
const RESIZE_EDGE: f32 = 5.0;

/// Custom title bars: dragging the header's empty space moves the window, and double-clicking it
/// maximizes or restores it. Call before drawing the header, so its controls stay on top.
pub fn title_bar_drag(ui: &mut Ui) {
    let r = ui.max_rect();
    let rect = Rect::from_min_max(r.min + vec2(RESIZE_EDGE, RESIZE_EDGE), r.max - vec2(RESIZE_EDGE, 0.0));
    let resp = ui.interact(rect, ui.id().with("title-bar"), Sense::click_and_drag());
    if resp.drag_started_by(egui::PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
    if resp.double_clicked() {
        let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
    }
}

/// A window button in a custom title bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WindowButton {
    Minimize,
    Maximize,
    Restore,
    Close,
}

/// Width of a caption button: Windows' own (Redrob Cowork keeps the native 46 px ones).
const CAPTION_W: f32 = 46.0;

/// Minimize, maximize (or restore) and close, `height` tall, for a right-to-left layout: close
/// ends up at the right.
pub fn window_buttons(ui: &mut Ui, height: f32) {
    let ctx = ui.ctx().clone();
    let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    if window_button(ui, WindowButton::Close, height).clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
    let size = if maximized { WindowButton::Restore } else { WindowButton::Maximize };
    if window_button(ui, size, height).clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
    }
    if window_button(ui, WindowButton::Minimize, height).clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
    }
}

/// One caption button, drawn like Redrob Cowork's (Windows' native ones over its 48 px bar): a
/// 10 px glyph, idle in ink-faint (Cowork's caption symbols are Gray 5, a step lighter than the
/// secondary ink); hover fills 8% of ink and the glyph takes ink; close's hover is status-danger
/// with white (was a hard-coded #c42b1c).
fn window_button(ui: &mut Ui, button: WindowButton, height: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(CAPTION_W, height), Sense::hover());
    // Clickable below the resize band along the top (and, for close, the right edge).
    let right = if button == WindowButton::Close { RESIZE_EDGE } else { 0.0 };
    let hit = Rect::from_min_max(rect.min + vec2(0.0, RESIZE_EDGE), rect.max - vec2(right, 0.0));
    let resp = ui.interact(hit, ui.id().with(("window-button", button as u8)), Sense::click());
    let hovered = resp.hovered() || resp.is_pointer_button_down_on();
    let k = ui.ctx().animate_bool_with_time(resp.id.with("hover"), hovered, theme::motion::duration(ui.ctx(), theme::motion::FAST));
    let (fill, ink) = match button {
        WindowButton::Close => (t.bad.gamma_multiply(k), theme::mix(t.faint, t.on_brand(), k)),
        _ => (tint(t.ink, 0.08 * k), theme::mix(t.faint, t.ink, k)),
    };
    let p = ui.painter();
    if k > 0.0 {
        p.rect_filled(rect, CornerRadius::ZERO, fill);
    }
    let (label, icon) = match button {
        WindowButton::Minimize => (tl!("Minimize"), Icon::Minimize),
        WindowButton::Maximize => (tl!("Maximize"), Icon::Maximize),
        WindowButton::Restore => (tl!("Restore"), Icon::Restore),
        WindowButton::Close => (tl!("Close"), Icon::CaptionClose),
    };
    // 10 px, like Windows' own caption glyphs; the icon grid draws them with a 1 px line.
    crate::glyphs::paint(p, Rect::from_center_size(rect.center(), vec2(10.0, 10.0)), icon, ink);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    tip(resp, label)
}

/// Custom title bars: a hairline round the window, and resizing it from its edges and corners.
pub fn window_edges(ctx: &egui::Context) {
    use egui::CursorIcon as C;
    use egui::viewport::ResizeDirection as D;
    let (maximized, fullscreen) = ctx.input(|i| (i.viewport().maximized.unwrap_or(false), i.viewport().fullscreen.unwrap_or(false)));
    if maximized || fullscreen {
        return;
    }
    let t = Tokens::get(ctx);
    let r = ctx.content_rect();
    ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("window-border"))).rect_stroke(
        r,
        CornerRadius::ZERO,
        t.hairline(),
        egui::StrokeKind::Inside,
    );
    let Some(p) = ctx.input(|i| i.pointer.hover_pos()).filter(|p| r.contains(*p)) else { return };
    let near = |d: f32| d < RESIZE_EDGE;
    // A corner takes a little of each edge next to it.
    let corner = |d: f32| d < 3.0 * RESIZE_EDGE;
    let (left, right, top, bottom) = (p.x - r.left(), r.right() - p.x, p.y - r.top(), r.bottom() - p.y);
    let hit = if (near(top) && corner(left)) || (near(left) && corner(top)) {
        Some((D::NorthWest, C::ResizeNorthWest))
    } else if (near(top) && corner(right)) || (near(right) && corner(top)) {
        Some((D::NorthEast, C::ResizeNorthEast))
    } else if (near(bottom) && corner(left)) || (near(left) && corner(bottom)) {
        Some((D::SouthWest, C::ResizeSouthWest))
    } else if (near(bottom) && corner(right)) || (near(right) && corner(bottom)) {
        Some((D::SouthEast, C::ResizeSouthEast))
    } else if near(top) {
        Some((D::North, C::ResizeNorth))
    } else if near(bottom) {
        Some((D::South, C::ResizeSouth))
    } else if near(left) {
        Some((D::West, C::ResizeWest))
    } else if near(right) {
        Some((D::East, C::ResizeEast))
    } else {
        None
    };
    if let Some((direction, cursor)) = hit {
        ctx.set_cursor_icon(cursor);
        if ctx.input(|i| i.pointer.primary_pressed()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(direction));
        }
    }
}

/// A mono link with an ↗: muted, ink on hover. Opens `url` in the browser.
pub fn hud_link(ui: &mut Ui, text: &str, url: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let galley = hud_galley(ui, &format!("{text} ↗"), t.muted);
    let (rect, resp) = ui.allocate_exact_size(galley.size(), Sense::click());
    paint_left(ui, rect, galley, if resp.hovered() { t.ink_strong } else { t.muted });
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Link, true, text));
    let resp = resp.on_hover_text(url).on_hover_cursor(egui::CursorIcon::PointingHand);
    if resp.clicked() {
        ui.ctx().open_url(egui::OpenUrl::new_tab(url));
    }
    resp
}

/// The DS Badge (subtle tone, md): a 24 px pill, 12 px semibold text in `color` on a light wash
/// of it, no extra letter spacing. Never clickable; state is a word, not only a colour.
pub fn badge(ui: &mut Ui, text: &str, color: Color32) -> Response {
    badge_sized(ui, text, color, Size::Md)
}

/// [`badge`] at the DS's sm (20 px, 11 px text) or md (24 px, 12 px text) size.
pub fn badge_sized(ui: &mut Ui, text: &str, color: Color32, size: Size) -> Response {
    let t = Tokens::get(ui.ctx());
    let (h, pad, font) =
        if size == Size::Sm { (20.0, theme::space::S2, theme::text::CAPTION) } else { (24.0, theme::space::S3 - 2.0, theme::text::META) };
    let galley = caps(ui, text, font, theme::semibold(), 0.0, color);
    let (rect, resp) = ui.allocate_exact_size(vec2(galley.size().x + 2.0 * pad, h), Sense::hover());
    ui.painter().rect_filled(rect, CornerRadius::same(theme::radius::PILL), t.subtle(color));
    paint_centred(ui, rect, galley, color);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
    resp
}

/// Which of the design system's button variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// `action-primary` blue with white text; `action-primary-hover` on hover. One per view.
    Primary,
    /// The base surface outlined in `border-strong`; sunken on hover.
    Secondary,
    /// `status-danger` with white text; a little darker on hover.
    Danger,
    /// No fill, `ink-brand` text; `surface-brand-subtle` on hover.
    Ghost,
    /// `surface-brand-subtle` with `ink-brand` text, a touch more blue on hover: a quiet action
    /// that still reads as one, for chrome (Cowork's title bar carries no filled button).
    Tinted,
}

/// The design system's control sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    /// 32 tall, 12 px padding, 13 px label.
    Sm,
    /// 40 tall, 16 px padding, 14 px label: the default.
    #[default]
    Md,
    /// 48 tall, 20 px padding, 15 px label.
    Lg,
}

impl Size {
    /// (height, side padding, label size).
    pub fn metrics(self) -> (f32, f32, f32) {
        match self {
            Self::Sm => (theme::control::SM, theme::space::S3, 13.0),
            Self::Md => (theme::control::MD, theme::space::S4, 14.0),
            Self::Lg => (theme::control::LG, theme::space::S5, 15.0),
        }
    }
}

/// DS disabled controls: the whole control at 45%.
const DISABLED_ALPHA: f32 = 0.45;

/// The DS focus ring: 2 px of `focus-ring`, 2 px outside the control, following its radius.
/// Drawn only when focus arrived from the keyboard (CSS `:focus-visible`): a click draws none.
pub fn focus_ring(ui: &Ui, resp: &Response, radius: u8) {
    let id = resp.id.with("focus-visible");
    if resp.gained_focus() {
        let by_pointer = ui.input(|i| i.pointer.any_pressed() || i.pointer.any_down());
        ui.data_mut(|d| d.insert_temp(id, !by_pointer));
    }
    let keyboard = ui.data(|d| d.get_temp::<bool>(id)).unwrap_or(false);
    if resp.has_focus() && keyboard {
        let t = Tokens::get(ui.ctx());
        let r = CornerRadius::same(radius.saturating_add(3));
        ui.painter().rect_stroke(resp.rect.expand(3.0), r, Stroke::new(2.0, t.focus()), egui::StrokeKind::Middle);
    }
}

/// A tooltip in the DS's style: a `redrob-black` bubble with 12 px white text (inverted in dark),
/// radius 6, `shadow-md`, 8 px from its control. egui's own tooltips share the menus' frame, so
/// our controls draw theirs with this.
pub fn tip(resp: Response, text: &str) -> Response {
    let t = Tokens::get(&resp.ctx);
    let (fill, ink) = t.tooltip();
    let mut tooltip = egui::Tooltip::for_enabled(&resp).gap(theme::space::S2);
    tooltip.popup = tooltip.popup.frame(
        egui::Frame::new()
            .fill(fill)
            .corner_radius(CornerRadius::same(theme::radius::SM))
            .inner_margin(egui::Margin::symmetric(12, 6))
            .shadow(theme::shadow_md(&t)),
    );
    tooltip.show(|ui| {
        ui.add(egui::Label::new(RichText::new(text).size(theme::text::META).color(ink)).wrap_mode(egui::TextWrapMode::Extend));
    });
    resp
}

/// A DS button, medium size, at least `width` wide.
pub fn button(ui: &mut Ui, text: &str, kind: Kind, width: f32) -> Response {
    button_sized(ui, text, kind, Size::Md, width)
}

/// A DS button: radius 8, a 600-weight label, `size` tall, at least `width` wide.
pub fn button_sized(ui: &mut Ui, text: &str, kind: Kind, size: Size, width: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    let (height, pad, label) = size.metrics();
    let galley = caps(ui, text, label, theme::mono_bold(), theme::BUTTON_SPACING, t.ink);
    let (rect, resp) = ui.allocate_exact_size(vec2((galley.size().x + 2.0 * pad).max(width), height), Sense::click());
    let enabled = ui.is_enabled();
    let hovered = (resp.hovered() || resp.is_pointer_button_down_on()) && enabled;
    let (rest, hover, stroke, color) = match kind {
        Kind::Primary => (t.accent, t.primary_hover(), Stroke::NONE, t.on_brand()),
        Kind::Danger => (t.bad, t.bad_hover(), Stroke::NONE, t.on_brand()),
        Kind::Secondary => (t.bg, t.sunken, Stroke::new(1.0, t.line_strong), t.ink),
        Kind::Ghost => (Color32::TRANSPARENT, t.brand_subtle(), Stroke::NONE, t.accent_ink),
        Kind::Tinted => (t.brand_subtle(), theme::mix(t.brand_subtle(), t.accent, 0.14), Stroke::NONE, t.accent_ink),
    };
    // The fill changes over the DS's 140 ms standard state change.
    let k = ui.ctx().animate_bool_with_time(resp.id.with("hover"), hovered, theme::motion::duration(ui.ctx(), theme::motion::FAST));
    let k = theme::motion::Easing::Standard.apply(k);
    let fill = if kind == Kind::Ghost { hover.gamma_multiply(k) } else { theme::mix(rest, hover, k) };
    let fade = |c: Color32| if enabled { c } else { c.gamma_multiply(DISABLED_ALPHA) };
    let radius = CornerRadius::same(theme::radius::MD);
    ui.painter().rect(rect, radius, fade(fill), Stroke::new(stroke.width, fade(stroke.color)), egui::StrokeKind::Inside);
    paint_centred(ui, rect, galley, fade(color));
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, text));
    focus_ring(ui, &resp, theme::radius::MD);
    if enabled { resp.on_hover_cursor(egui::CursorIcon::PointingHand) } else { resp }
}

/// The width [`button_sized`] gives `text` at `size` (with no minimum width).
pub fn button_width(ui: &Ui, text: &str, size: Size) -> f32 {
    let (_, pad, label) = size.metrics();
    caps(ui, text, label, theme::mono_bold(), theme::BUTTON_SPACING, Tokens::get(ui.ctx()).ink).size().x + 2.0 * pad
}

/// The DS search input (sm): 32 tall, radius 8, a 1 px border-subtle on the base ground that
/// strengthens on hover and while typing, the search glyph (16 px, ink-muted) at the left and the
/// hint in ink-muted 13 px. One per window: its text field keeps the id `id_salt`, so focus
/// survives a change of page. Returns the text field's response.
pub fn search_field(ui: &mut Ui, text: &mut String, hint: &str, width: f32, id_salt: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    const GLYPH: f32 = 16.0;
    const PAD: f32 = 10.0;
    let (rect, bg) = ui.allocate_exact_size(vec2(width, theme::control::SM), Sense::click());
    let id = egui::Id::new(("search-field", id_salt));
    let strong = ui.memory(|m| m.has_focus(id)) || ui.rect_contains_pointer(rect);
    // Under the text field: drawn first.
    let p = ui.painter();
    p.rect(
        rect,
        CornerRadius::same(theme::radius::MD),
        t.bg,
        Stroke::new(1.0, if strong { t.line_strong } else { t.line }),
        egui::StrokeKind::Inside,
    );
    let glyph = Rect::from_center_size(egui::pos2(rect.left() + PAD + GLYPH / 2.0, rect.center().y), vec2(GLYPH, GLYPH));
    crate::glyphs::paint(p, glyph, Icon::Search, t.muted);
    let field = Rect::from_min_max(egui::pos2(glyph.right() + theme::space::S2, rect.top()), egui::pos2(rect.right() - PAD, rect.bottom()));
    // A child, not `ui.put`: putting would move a right-to-left parent's cursor back to the field.
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(field).layout(egui::Layout::left_to_right(egui::Align::Center)));
    let resp = child.add(
        egui::TextEdit::singleline(text)
            .id(id)
            .frame(egui::Frame::NONE)
            .hint_text(RichText::new(hint).color(t.muted))
            .font(egui::FontId::proportional(theme::text::BODY_SM))
            .text_color(t.ink)
            .vertical_align(egui::Align::Center)
            .margin(vec2(0.0, 0.0))
            .desired_width(field.width()),
    );
    // The hint is only a placeholder to screen readers; the field's name is the hint's words.
    ui.ctx().accesskit_node_builder(resp.id, |node| node.set_label(hint));
    // The glyph and padding focus the field too.
    if bg.clicked() {
        ui.memory_mut(|m| m.request_focus(id));
    }
    resp.on_hover_cursor(egui::CursorIcon::Text)
}

/// The Redrob group logo as a ghost button (Redrob Cowork's group mark): `image` exactly as
/// supplied, 28 px in a 32 px target with radius 8; hover tints the button behind it (8% of ink),
/// never the mark. `label` is the tooltip and the name screen readers read.
pub fn logo_button(ui: &mut Ui, image: egui::ImageSource<'static>, label: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(theme::control::SM, theme::control::SM), Sense::click());
    let hovered = resp.hovered() || resp.is_pointer_button_down_on();
    let k = ui.ctx().animate_bool_with_time(resp.id.with("hover"), hovered, theme::motion::duration(ui.ctx(), theme::motion::FAST));
    if k > 0.0 {
        ui.painter().rect_filled(rect, CornerRadius::same(theme::radius::MD), tint(t.ink, 0.08 * k));
    }
    egui::Image::new(image).alt_text(label).paint_at(ui, Rect::from_center_size(rect.center(), vec2(28.0, 28.0)));
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    focus_ring(ui, &resp, theme::radius::MD);
    tip(resp, label).on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A DS IconButton (ghost): a square `size` (32 or 40) with a drawn icon in `ink-secondary`,
/// sunken with `ink-primary` on hover, radius 8. `label` is mandatory: it is the tooltip and the
/// name screen readers read.
pub fn icon_button(ui: &mut Ui, icon: Icon, label: &str, size: f32) -> Response {
    icon_button_busy(ui, icon, label, size, false)
}

/// [`icon_button`] that, while `busy`, shows the DS loader instead of its icon and can't be clicked.
pub fn icon_button_busy(ui: &mut Ui, icon: Icon, label: &str, size: f32, busy: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), if busy { Sense::hover() } else { Sense::click() });
    let enabled = ui.is_enabled() && !busy;
    let hovered = (resp.hovered() || resp.is_pointer_button_down_on()) && enabled;
    let k = ui.ctx().animate_bool_with_time(resp.id.with("hover"), hovered, theme::motion::duration(ui.ctx(), theme::motion::FAST));
    let p = ui.painter();
    if k > 0.0 {
        p.rect_filled(rect, CornerRadius::same(theme::radius::MD), t.sunken.gamma_multiply(theme::motion::Easing::Standard.apply(k)));
    }
    // DS: glyphs 16 px in a 32 control, 20 px in a 40.
    let glyph = if size >= theme::control::MD { 20.0 } else { 16.0 };
    let glyph_rect = Rect::from_center_size(rect.center(), vec2(glyph, glyph));
    if busy {
        paint_loader(ui, Rect::from_center_size(rect.center(), vec2(14.0, 14.0)), t.accent, t.line_strong);
    } else {
        let ink = if hovered { t.ink } else { t.muted };
        crate::glyphs::paint(p, glyph_rect, icon, if ui.is_enabled() { ink } else { ink.gamma_multiply(DISABLED_ALPHA) });
    }
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, label));
    focus_ring(ui, &resp, theme::radius::MD);
    let resp = tip(resp, label);
    if enabled { resp.on_hover_cursor(egui::CursorIcon::PointingHand) } else { resp }
}

/// One option in a [`cells`] group.
pub struct Cell<'a> {
    pub label: &'a str,
    pub tip: &'a str,
    /// A count after the label, and the colour it takes when it isn't zero.
    pub count: Option<(usize, Option<Color32>)>,
}

/// Mutually exclusive options as the DS segmented control: a sunken pill track (3 px inset)
/// holding 30 px pill segments, 13 px semibold; the chosen one raised with `shadow-sm`.
/// Returns whether the choice changed.
pub fn cells<T: PartialEq + Copy>(ui: &mut Ui, value: &mut T, options: &[(T, Cell<'_>)]) -> bool {
    let t = Tokens::get(ui.ctx());
    let mut changed = false;
    const PAD: f32 = 3.0;
    const SEG_H: f32 = 30.0;
    const SEG_PAD: f32 = 14.0;
    // In a right-to-left parent the row fills from the right: keep the options' order.
    let rtl = ui.layout().prefer_right_to_left();
    let ordered: Vec<&(T, Cell<'_>)> = if rtl { options.iter().rev().collect() } else { options.iter().collect() };
    let label_galley = |ui: &Ui, s: &str, c: Color32| caps(ui, s, theme::text::LABEL, theme::semibold(), 0.0, c);
    // Measure first: the track is drawn under the segments.
    let widths: Vec<f32> = ordered
        .iter()
        .map(|(_, cell)| {
            let label = label_galley(ui, cell.label, t.ink).size().x;
            let count = cell.count.map_or(0.0, |(n, _)| hud_galley(ui, &n.to_string(), t.faint).size().x + 6.0);
            label + count + 2.0 * SEG_PAD
        })
        .collect();
    let total = widths.iter().sum::<f32>() + 2.0 * PAD;
    let (track, _) = ui.allocate_exact_size(vec2(total, SEG_H + 2.0 * PAD), Sense::hover());
    let pill = CornerRadius::same(theme::radius::PILL);
    ui.painter().rect_filled(track, pill, t.sunken);
    let mut x = if rtl { track.right() - PAD } else { track.left() + PAD };
    for ((opt, cell), w) in ordered.into_iter().zip(widths) {
        let on = *value == *opt;
        let rect = if rtl {
            x -= w;
            Rect::from_min_size(egui::pos2(x, track.top() + PAD), vec2(w, SEG_H))
        } else {
            let r = Rect::from_min_size(egui::pos2(x, track.top() + PAD), vec2(w, SEG_H));
            x += w;
            r
        };
        let resp = ui.interact(rect, ui.id().with(("segment", cell.label)), Sense::click());
        let hovered = resp.hovered();
        let p = ui.painter();
        if on {
            p.add(theme::shadow_sm(&t).as_shape(rect, pill));
            p.rect_filled(rect, pill, t.raised());
        }
        let ink = if on || hovered { t.ink } else { t.muted };
        let label = label_galley(ui, cell.label, ink);
        let label_w = label.size().x;
        let count_w = cell.count.map_or(0.0, |(n, _)| hud_galley(ui, &n.to_string(), t.faint).size().x + 6.0);
        let text_rect = Rect::from_center_size(rect.center(), vec2(label_w + count_w, rect.height()));
        paint_left(ui, text_rect, label, ink);
        if let Some((n, highlight)) = cell.count {
            let c = highlight.filter(|_| n > 0).unwrap_or(t.faint);
            paint_left(ui, text_rect.with_min_x(text_rect.left() + label_w + 6.0), hud_galley(ui, &n.to_string(), c), c);
        }
        let name = match cell.count {
            Some((n, _)) => format!("{} {n}", cell.label),
            None => cell.label.to_string(),
        };
        resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, on, &name));
        focus_ring(ui, &resp, theme::radius::PILL);
        if resp.on_hover_text(cell.tip).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() && !on {
            *value = *opt;
            changed = true;
        }
    }
    changed
}

/// The DS Switch (md): a 40 x 24 pill, `border-strong` off and `action-primary` on, a white 20 px
/// thumb with `shadow-sm` that travels in 220 ms with the settle overshoot (instant when motion
/// is off), then `label`. It acts at once: for settings that apply immediately, never in a form
/// with Save. Returns the response, `changed()` when flipped.
pub fn switch(ui: &mut Ui, on: &mut bool, label: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let enabled = ui.is_enabled();
    let galley = ui.painter().layout(
        label.to_owned(),
        egui::FontId::new(theme::text::CONTROL, egui::FontFamily::Proportional),
        t.ink,
        ui.available_width() - 52.0,
    );
    let (track_w, track_h, thumb) = (40.0, 24.0, 20.0);
    let height = galley.size().y.max(track_h).max(theme::control::SM);
    let (rect, mut resp) = ui.allocate_exact_size(vec2(track_w + theme::space::S3 + galley.size().x, height), Sense::click());
    if resp.clicked() && enabled {
        *on = !*on;
        resp.mark_changed();
    }
    let k = ui.ctx().animate_bool_with_time(resp.id.with("switch"), *on, theme::motion::duration(ui.ctx(), theme::motion::BASE));
    let travel = theme::motion::Easing::Settle.apply(k);
    let track = Rect::from_min_size(egui::pos2(rect.left(), rect.center().y - track_h / 2.0), vec2(track_w, track_h));
    let fade = |c: Color32| if enabled { c } else { c.gamma_multiply(0.5) };
    let p = ui.painter();
    let pill = CornerRadius::same(theme::radius::PILL);
    p.rect_filled(track, pill, fade(theme::mix(t.line_strong, t.accent, k)));
    let inset = (track_h - thumb) / 2.0;
    let x = track.left() + inset + thumb / 2.0 + travel * (track_w - thumb - 2.0 * inset);
    let thumb_rect = Rect::from_center_size(egui::pos2(x, track.center().y), vec2(thumb, thumb));
    p.add(theme::shadow_sm(&t).as_shape(thumb_rect, pill));
    p.rect_filled(thumb_rect, pill, fade(t.on_brand()));
    let text_pos = egui::pos2(track.right() + theme::space::S3, rect.center().y - galley.size().y / 2.0);
    p.galley(text_pos, galley, fade(t.ink));
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, enabled, *on, label));
    focus_ring(ui, &resp, theme::radius::PILL);
    if enabled { resp.on_hover_cursor(egui::CursorIcon::PointingHand) } else { resp }
}

/// A progress bar (DS Progress): `caption` above an 8 px pill track on the sunken surface, the
/// bar in `color` (blue, warning when reconnecting). Without a fraction the bar is the DS's
/// indeterminate 40% slide (1.4 s, ease-in-out); with motion off, a full bar at 60%.
pub fn progress(ui: &mut Ui, fraction: Option<f32>, caption: &str, color: Color32, width: f32) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(width, theme::CONTROL_H), Sense::hover());
    let p = ui.painter();
    let pill = CornerRadius::same(theme::radius::PILL);
    let track = Rect::from_min_size(egui::pos2(rect.left(), rect.bottom() - 10.0), vec2(width, 8.0));
    p.rect_filled(track, pill, t.sunken);
    match fraction {
        Some(f) => {
            let w = track.width() * f.clamp(0.0, 1.0);
            if w > 0.0 {
                p.rect_filled(Rect::from_min_size(track.min, vec2(w.max(track.height()), track.height())), pill, color);
            }
        }
        None if theme::motion::duration(ui.ctx(), 1.0) == 0.0 => {
            p.rect_filled(track, pill, color.gamma_multiply(0.6));
        }
        None => {
            let s = (ui.input(|i| i.time) / 1.4).fract() as f32;
            // ease-in-out
            let e = if s < 0.5 { 2.0 * s * s } else { 1.0 - (-2.0 * s + 2.0).powi(2) / 2.0 };
            let left = track.left() + track.width() * (-0.4 + 1.4 * e);
            let seg = Rect::from_min_size(egui::pos2(left, track.top()), vec2(track.width() * 0.4, track.height())).intersect(track);
            if seg.width() > 0.0 {
                p.rect_filled(seg, pill, color);
            }
            ui.ctx().request_repaint();
        }
    }
    p.text(
        egui::pos2(rect.left(), rect.top() + 10.0),
        egui::Align2::LEFT_CENTER,
        caption,
        egui::FontId::new(theme::text::META, theme::medium()),
        t.muted,
    );
    let resp = ui.interact(rect, ui.id().with(("progress", caption)), Sense::hover());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::ProgressIndicator, true, caption));
}

pub fn human_bytes(n: u64) -> String {
    const MB: f64 = 1024.0 * 1024.0;
    let n = n as f64;
    if n >= 1024.0 * MB {
        format!("{:.1} GB", n / (1024.0 * MB))
    } else if n >= MB {
        format!("{:.0} MB", n / MB)
    } else {
        format!("{:.0} KB", (n / 1024.0).max(1.0))
    }
}

/// The "more" icon button that opens an app's menu (DS Menu: 8 px radius list, min 200 wide).
pub fn more_menu<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) {
    let resp = icon_button(ui, Icon::More, tl!("More actions"), theme::control::SM);
    egui::Popup::menu(&resp).gap(theme::space::S1).show(|ui| {
        ui.set_min_width(200.0);
        // DS menu items are about 36 px.
        ui.spacing_mut().interact_size.y = 36.0;
        add_contents(ui)
    });
}

/// An app's name in the display face, in ink: product colour is never text (design system,
/// `20-color.md`).
pub fn wordmark(ui: &mut Ui, app: &redrob_launcher_core::CraftApp, size: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    let mut job = egui::text::LayoutJob::default();
    let format = |color| egui::TextFormat {
        font_id: egui::FontId::new(size, theme::display()),
        color,
        extra_letter_spacing: -0.03 * size,
        ..Default::default()
    };
    job.append(app.name, 0.0, format(t.ink_strong));
    ui.add(egui::Label::new(job).selectable(false))
}

/// A heading in the display face, tracked in like the site's (`tracking-[-0.02em]`).
pub fn heading(text: &str, size: f32, color: Color32) -> RichText {
    RichText::new(text).size(size).family(theme::display()).color(color).extra_letter_spacing(-0.02 * size)
}

/// What a sidebar row shows at its right end.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NavMeta {
    None,
    /// A count in this colour (DS nav meta: 12 px, right aligned); nothing when zero.
    Count(usize, Color32),
    /// A 7 px status dot (DS TaskStatus).
    Dot(Color32),
}

/// Height of a sidebar row (Redrob Cowork: 7 px of padding round a 24 px line).
pub const NAV_ROW_H: f32 = 38.0;

/// A sidebar row (DS AppShell nav item, Cowork's numbers): 16 px icon in ink-muted, 13 px label
/// in ink-secondary, radius 6; hover takes a step of the sunken surface; the current row is
/// sunken, semibold ink, its icon ink-brand. No accent bar. In the folded `rail` only the icon
/// shows, the label becomes its tooltip and a meta becomes a dot on the icon.
pub fn nav_row(ui: &mut Ui, icon: Icon, label: &str, hint: &str, meta: NavMeta, current: bool, rail: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), NAV_ROW_H), Sense::click());
    let hovered = resp.hovered();
    let p = ui.painter();
    let radius = CornerRadius::same(theme::radius::SM);
    if current {
        p.rect_filled(rect, radius, t.sunken);
    } else if hovered {
        p.rect_filled(rect, radius, t.hover_row());
    }
    let icon_ink = if current {
        t.accent_ink
    } else if hovered {
        t.ink
    } else {
        t.faint
    };
    let ink = if current || hovered { t.ink } else { t.muted };
    let pad = theme::space::S2;
    let icon_rect = if rail {
        Rect::from_center_size(rect.center(), vec2(16.0, 16.0))
    } else {
        Rect::from_min_size(egui::pos2(rect.left() + pad, rect.center().y - 8.0), vec2(16.0, 16.0))
    };
    crate::glyphs::paint(p, icon_rect, icon, icon_ink);
    if rail {
        let dot = match meta {
            NavMeta::Dot(c) => Some(c),
            NavMeta::Count(n, c) if n > 0 => Some(c),
            _ => None,
        };
        if let Some(c) = dot {
            p.circle_filled(icon_rect.right_top() + vec2(1.0, 1.0), 3.5, c);
        }
    } else {
        let mut right = rect.right() - pad;
        match meta {
            NavMeta::Count(n, c) if n > 0 => {
                let g = caps(ui, &n.to_string(), theme::text::META, theme::medium(), 0.0, c);
                let w = g.size().x;
                paint_left(ui, Rect::from_min_max(egui::pos2(right - w, rect.top()), egui::pos2(right, rect.bottom())), g, c);
                right -= w + pad;
            }
            NavMeta::Dot(c) => {
                p.circle_filled(egui::pos2(right - 3.5, rect.center().y), 3.5, c);
                right -= 7.0 + pad;
            }
            _ => {}
        }
        let text_left = icon_rect.right() + pad;
        let family = if current { theme::semibold() } else { egui::FontFamily::Proportional };
        let format = egui::TextFormat { font_id: egui::FontId::new(theme::text::LABEL, family), color: ink, ..Default::default() };
        let mut job = egui::text::LayoutJob::single_section(label.to_owned(), format);
        job.wrap = egui::text::TextWrapping::truncate_at_width((right - text_left).max(0.0));
        let g = ui.painter().layout_job(job);
        paint_left(ui, Rect::from_min_max(egui::pos2(text_left, rect.top()), rect.max), g, ink);
    }
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, current, label));
    focus_ring(ui, &resp, theme::radius::SM);
    let resp = if rail {
        tip(resp, label)
    } else if hint.is_empty() {
        resp
    } else {
        tip(resp, hint)
    };
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// The sidebar's account row (Redrob Cowork's): a 32 px avatar on the sunken surface (the
/// workspace's initial in ink, or a person), the name in 13 px semibold and a 12 px status line in
/// ink-faint under it. Screen readers read `name`.
pub fn account_row(ui: &mut Ui, name: &str, meta: &str, initial: Option<char>, current: bool, rail: bool, hint: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 48.0), Sense::click());
    let hovered = resp.hovered();
    let p = ui.painter();
    let radius = CornerRadius::same(theme::radius::SM);
    if current {
        p.rect_filled(rect, radius, t.sunken);
    } else if hovered {
        p.rect_filled(rect, radius, t.hover_row());
    }
    let size = 32.0;
    let centre = if rail { rect.center() } else { egui::pos2(rect.left() + theme::space::S2 + size / 2.0, rect.center().y) };
    p.circle(centre, size / 2.0, if current || hovered { t.bg } else { t.sunken }, t.hairline());
    match initial {
        Some(c) => {
            let s: String = c.to_uppercase().collect();
            p.text(centre, egui::Align2::CENTER_CENTER, s, egui::FontId::new(theme::text::LABEL, theme::semibold()), t.ink);
        }
        None => crate::glyphs::paint(p, Rect::from_center_size(centre, vec2(16.0, 16.0)), Icon::Account, t.muted),
    }
    if !rail {
        let left = centre.x + size / 2.0 + 10.0;
        let width = (rect.right() - theme::space::S2 - left).max(0.0);
        let line = |text: &str, size: f32, family: egui::FontFamily, color: Color32| {
            let format = egui::TextFormat { font_id: egui::FontId::new(size, family), color, ..Default::default() };
            let mut job = egui::text::LayoutJob::single_section(text.to_owned(), format);
            job.wrap = egui::text::TextWrapping::truncate_at_width(width);
            ui.painter().layout_job(job)
        };
        let top = line(name, theme::text::LABEL, theme::semibold(), t.ink);
        let bottom = line(meta, theme::text::META, egui::FontFamily::Proportional, t.faint);
        let total = top.size().y + bottom.size().y;
        let y = rect.center().y - total / 2.0;
        let top_h = top.size().y;
        ui.painter().galley(egui::pos2(left, y), top, t.ink);
        ui.painter().galley(egui::pos2(left, y + top_h), bottom, t.faint);
    }
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, current, name));
    focus_ring(ui, &resp, theme::radius::SM);
    let resp = if rail { tip(resp, name) } else { tip(resp, hint) };
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A DS Menu item, `width` wide: 36 px, radius 6, a 16 px icon in ink-muted (or its room, so the
/// labels line up) and a 13 px label in ink; hover takes the sunken step. A disabled item (the
/// version line) is ink-faint and can't be clicked.
pub fn menu_item(ui: &mut Ui, icon: Option<Icon>, label: &str, enabled: bool, width: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(width, 36.0), if enabled { Sense::click() } else { Sense::hover() });
    let hovered = enabled && (resp.hovered() || resp.is_pointer_button_down_on());
    let p = ui.painter();
    if hovered {
        p.rect_filled(rect, CornerRadius::same(theme::radius::SM), t.hover_row());
    }
    let pad = theme::space::S2;
    let icon_rect = Rect::from_min_size(egui::pos2(rect.left() + pad, rect.center().y - 8.0), vec2(16.0, 16.0));
    if let Some(icon) = icon {
        crate::glyphs::paint(p, icon_rect, icon, if hovered { t.ink } else { t.muted });
    }
    let ink = if enabled { t.ink } else { t.faint };
    let galley = caps(ui, label, theme::text::LABEL, egui::FontFamily::Proportional, 0.0, ink);
    paint_left(ui, Rect::from_min_max(egui::pos2(icon_rect.right() + pad, rect.top()), rect.max), galley, ink);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, label));
    if !enabled {
        return resp;
    }
    focus_ring(ui, &resp, theme::radius::SM);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// The DS ThemeSwitch: System, Light and Dark as three 28 px round icon options in a pill with a
/// hairline, the chosen one sunken with a hairline ring. Three options, never a two-way toggle.
/// Stacked when `vertical` (the folded sidebar). Returns whether the choice changed.
pub fn theme_switch(ui: &mut Ui, value: &mut theme::ThemeChoice, vertical: bool) -> bool {
    use theme::ThemeChoice;
    let t = Tokens::get(ui.ctx());
    // 28 px circles (DS ThemeSwitch), each with a larger target: 32 along the row (neighbours'
    // targets meet, never overlap) and 36 across it (Cowork's 36 px hit target).
    const OPT: f32 = 28.0;
    const PAD: f32 = 2.0;
    const GAP: f32 = 4.0;
    const HIT_ALONG: f32 = OPT + GAP;
    const HIT_ACROSS: f32 = 36.0;
    let options = [(ThemeChoice::System, Icon::Monitor), (ThemeChoice::Light, Icon::Sun), (ThemeChoice::Dark, Icon::Moon)];
    let long = 3.0 * OPT + 2.0 * GAP + 2.0 * PAD;
    let short = OPT + 2.0 * PAD;
    let (track, _) = ui.allocate_exact_size(if vertical { vec2(short, long) } else { vec2(long, short) }, Sense::hover());
    let pill = CornerRadius::same(theme::radius::PILL);
    ui.painter().rect(track, pill, t.bg, t.hairline(), egui::StrokeKind::Inside);
    let mut changed = false;
    for (i, (choice, icon)) in options.into_iter().enumerate() {
        let off = PAD + i as f32 * (OPT + GAP);
        let min = if vertical { track.min + vec2(PAD, off) } else { track.min + vec2(off, PAD) };
        let rect = Rect::from_min_size(min, vec2(OPT, OPT));
        let label = choice.label();
        let hit = Rect::from_center_size(rect.center(), if vertical { vec2(HIT_ACROSS, HIT_ALONG) } else { vec2(HIT_ALONG, HIT_ACROSS) });
        let resp = ui.interact(hit, ui.id().with(("theme-switch", i)), Sense::click());
        let on = *value == choice;
        let p = ui.painter();
        if on {
            p.rect(rect, pill, t.sunken, t.hairline(), egui::StrokeKind::Inside);
        }
        let ink = if on || resp.hovered() { t.ink } else { t.faint };
        crate::glyphs::paint(p, Rect::from_center_size(rect.center(), vec2(16.0, 16.0)), icon, ink);
        resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::RadioButton, true, on, label));
        focus_ring(ui, &resp, theme::radius::PILL);
        if tip(resp, label).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() && !on {
            *value = choice;
            changed = true;
        }
    }
    changed
}
