//! The design system's line icons, drawn with egui shapes so no font has to carry them.
//!
//! Every icon is laid out on the DS's 24-unit grid (`src/icons` in `@redrob-labs/ui`) and drawn
//! with its 2-unit stroke, scaled to the size asked for (never thinner than 1 px). Strokes end
//! square and corners are sharp, the DS's "butt caps, miter joins": egui joins polylines with
//! mitres and does not round line ends.

use std::f32::consts::{PI, TAU};

use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, pos2, vec2};

/// A drawn icon. The full DS set is kept even where a variant has no caller yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub(crate) enum Icon {
    /// A gear: Settings.
    Settings,
    /// A circling arrow: check again.
    Refresh,
    /// A magnifier.
    Search,
    /// A chevron pointing down: a list opens below.
    ChevronDown,
    /// A chevron pointing left: back.
    ChevronLeft,
    /// Three dots: more actions.
    More,
    /// A cross: close, dismiss, cancel.
    Close,
    /// Two bars.
    Pause,
    /// A triangle.
    Play,
    /// A tick.
    Check,
    /// An arrow into a tray.
    Download,
    /// An arrow leaving a box: opens in the browser.
    ExternalLink,
    /// Four squares: the apps.
    Apps,
    /// A chip: local models.
    Models,
    /// A bolt: the engine.
    Engine,
    /// A person: the account.
    Account,
    /// The light appearance.
    Sun,
    /// The dark appearance.
    Moon,
    /// A screen: match the system.
    Monitor,
}

/// Maps the 24-unit grid onto a square.
struct Grid {
    min: Pos2,
    unit: f32,
    stroke: Stroke,
    color: Color32,
}

impl Grid {
    fn p(&self, x: f32, y: f32) -> Pos2 {
        self.min + vec2(x, y) * self.unit
    }

    fn line(&self, out: &mut Vec<Shape>, pts: &[(f32, f32)]) {
        out.push(Shape::line(pts.iter().map(|&(x, y)| self.p(x, y)).collect(), self.stroke));
    }

    fn closed(&self, out: &mut Vec<Shape>, pts: &[(f32, f32)]) {
        out.push(Shape::closed_line(pts.iter().map(|&(x, y)| self.p(x, y)).collect(), self.stroke));
    }

    fn rect(&self, out: &mut Vec<Shape>, (x0, y0): (f32, f32), (x1, y1): (f32, f32)) {
        self.closed(out, &[(x0, y0), (x1, y0), (x1, y1), (x0, y1)]);
    }

    fn fill(&self, out: &mut Vec<Shape>, pts: &[(f32, f32)]) {
        out.push(Shape::convex_polygon(pts.iter().map(|&(x, y)| self.p(x, y)).collect(), self.color, Stroke::NONE));
    }

    fn circle(&self, out: &mut Vec<Shape>, (x, y): (f32, f32), r: f32) {
        out.push(Shape::circle_stroke(self.p(x, y), r * self.unit, self.stroke));
    }

    fn dot(&self, out: &mut Vec<Shape>, (x, y): (f32, f32), r: f32) {
        out.push(Shape::circle_filled(self.p(x, y), r * self.unit, self.color));
    }

    /// Points along a circle from angle `a0` sweeping `sweep` radians (y down, so positive
    /// sweeps clockwise on screen).
    fn arc(&self, (cx, cy): (f32, f32), r: f32, a0: f32, sweep: f32) -> Vec<(f32, f32)> {
        let n = ((sweep.abs() / TAU) * 48.0).ceil().max(2.0) as usize;
        (0..=n)
            .map(|i| {
                let a = a0 + sweep * i as f32 / n as f32;
                (cx + r * a.cos(), cy + r * a.sin())
            })
            .collect()
    }
}

/// The signed sweep from `from` to `to` that passes through `through` (`want` true) or avoids it.
fn sweep_via(from: f32, to: f32, through: f32, want: bool) -> f32 {
    let ccw = (to - from).rem_euclid(TAU);
    let contains = (through - from).rem_euclid(TAU) < ccw;
    if contains == want { ccw } else { ccw - TAU }
}

/// Paints `icon` filling the square centred in `rect`, in `color`.
pub(crate) fn paint(painter: &Painter, rect: Rect, icon: Icon, color: Color32) {
    painter.extend(shapes(rect, icon, color));
}

/// The shapes that draw `icon` in the square centred in `rect`.
fn shapes(rect: Rect, icon: Icon, color: Color32) -> Vec<Shape> {
    let side = rect.width().min(rect.height());
    let unit = side / 24.0;
    let square = Rect::from_center_size(rect.center(), vec2(side, side));
    let g = Grid { min: square.min, unit, stroke: Stroke::new((2.0 * unit).max(1.0), color), color };
    let mut s = Vec::new();
    match icon {
        Icon::Settings => {
            // Eight square-cut teeth round a ring, a hole in the middle.
            let mut pts = Vec::with_capacity(32);
            for k in 0..8 {
                let base = k as f32 * PI / 4.0;
                let deg = PI / 180.0;
                for (r, da) in [(7.0, -16.5 * deg), (9.5, -9.0 * deg), (9.5, 9.0 * deg), (7.0, 16.5 * deg)] {
                    pts.push((12.0 + r * (base + da).cos(), 12.0 + r * (base + da).sin()));
                }
            }
            g.closed(&mut s, &pts);
            g.circle(&mut s, (12.0, 12.0), 3.0);
        }
        Icon::Refresh => {
            // Most of a circle, open at the top right, with an arrowhead where it ends.
            let (a0, sweep) = (-PI / 4.0 + 0.5, TAU - 1.0);
            let pts = g.arc((12.0, 12.0), 7.5, a0, sweep);
            g.line(&mut s, &pts);
            let end = a0 + sweep;
            let (ex, ey) = (12.0 + 7.5 * end.cos(), 12.0 + 7.5 * end.sin());
            // The tangent at the end, in the direction of travel.
            let (tx, ty) = (-end.sin(), end.cos());
            let head = 3.5;
            let back = |turn: f32| {
                let (c, sn) = (turn.cos(), turn.sin());
                (ex - head * (tx * c - ty * sn), ey - head * (tx * sn + ty * c))
            };
            g.line(&mut s, &[back(PI / 4.0), (ex, ey), back(-PI / 4.0)]);
        }
        Icon::Search => {
            g.circle(&mut s, (10.5, 10.5), 6.5);
            g.line(&mut s, &[(15.2, 15.2), (20.0, 20.0)]);
        }
        Icon::ChevronDown => g.line(&mut s, &[(6.0, 9.0), (12.0, 15.0), (18.0, 9.0)]),
        Icon::ChevronLeft => g.line(&mut s, &[(15.0, 6.0), (9.0, 12.0), (15.0, 18.0)]),
        Icon::More => {
            for x in [5.5, 12.0, 18.5] {
                g.dot(&mut s, (x, 12.0), 1.75);
            }
        }
        Icon::Close => {
            g.line(&mut s, &[(6.0, 6.0), (18.0, 18.0)]);
            g.line(&mut s, &[(18.0, 6.0), (6.0, 18.0)]);
        }
        Icon::Pause => {
            g.fill(&mut s, &[(7.0, 5.0), (10.0, 5.0), (10.0, 19.0), (7.0, 19.0)]);
            g.fill(&mut s, &[(14.0, 5.0), (17.0, 5.0), (17.0, 19.0), (14.0, 19.0)]);
        }
        Icon::Play => g.fill(&mut s, &[(8.0, 5.0), (19.0, 12.0), (8.0, 19.0)]),
        Icon::Check => g.line(&mut s, &[(5.0, 12.5), (10.0, 17.5), (19.0, 7.0)]),
        Icon::Download => {
            g.line(&mut s, &[(12.0, 4.0), (12.0, 15.0)]);
            g.line(&mut s, &[(7.0, 10.0), (12.0, 15.0), (17.0, 10.0)]);
            g.line(&mut s, &[(5.0, 20.0), (19.0, 20.0)]);
        }
        Icon::ExternalLink => {
            g.line(&mut s, &[(18.0, 13.0), (18.0, 19.0), (5.0, 19.0), (5.0, 6.0), (11.0, 6.0)]);
            g.line(&mut s, &[(10.0, 14.0), (19.0, 5.0)]);
            g.line(&mut s, &[(13.0, 5.0), (19.0, 5.0), (19.0, 11.0)]);
        }
        Icon::Apps => {
            for (x, y) in [(4.0, 4.0), (14.0, 4.0), (4.0, 14.0), (14.0, 14.0)] {
                g.rect(&mut s, (x, y), (x + 6.0, y + 6.0));
            }
        }
        Icon::Models => {
            g.rect(&mut s, (7.0, 7.0), (17.0, 17.0));
            for v in [10.0, 14.0] {
                g.line(&mut s, &[(v, 3.0), (v, 7.0)]);
                g.line(&mut s, &[(v, 17.0), (v, 21.0)]);
                g.line(&mut s, &[(3.0, v), (7.0, v)]);
                g.line(&mut s, &[(17.0, v), (21.0, v)]);
            }
        }
        Icon::Engine => g.closed(&mut s, &[(13.0, 3.0), (5.0, 14.0), (11.0, 14.0), (10.0, 21.0), (19.0, 10.0), (13.0, 10.0)]),
        Icon::Account => {
            g.circle(&mut s, (12.0, 8.0), 4.0);
            let shoulders = g.arc((12.0, 21.5), 7.5, PI, PI);
            g.line(&mut s, &shoulders);
        }
        Icon::Sun => {
            g.circle(&mut s, (12.0, 12.0), 4.0);
            for k in 0..8 {
                let a = k as f32 * PI / 4.0;
                g.line(&mut s, &[(12.0 + 7.0 * a.cos(), 12.0 + 7.0 * a.sin()), (12.0 + 9.5 * a.cos(), 12.0 + 9.5 * a.sin())]);
            }
        }
        Icon::Moon => {
            // A disc with a second one bitten out of its top right.
            let (c1, r1, c2, r2) = ((12.0_f32, 12.0_f32), 8.0_f32, (17.0_f32, 8.0_f32), 7.0_f32);
            let (dx, dy) = (c2.0 - c1.0, c2.1 - c1.1);
            let d = (dx * dx + dy * dy).sqrt();
            let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * d);
            let h = (r1 * r1 - a * a).max(0.0).sqrt();
            let (px, py) = (c1.0 + a * dx / d, c1.1 + a * dy / d);
            let i1 = (px - h * dy / d, py + h * dx / d);
            let i2 = (px + h * dy / d, py - h * dx / d);
            let ang = |c: (f32, f32), p: (f32, f32)| (p.1 - c.1).atan2(p.0 - c.0);
            let towards_c2 = dy.atan2(dx);
            let (b1, b2) = (ang(c1, i1), ang(c1, i2));
            let mut pts = g.arc(c1, r1, b1, sweep_via(b1, b2, towards_c2, false));
            let (e2, e1) = (ang(c2, i2), ang(c2, i1));
            let back = g.arc(c2, r2, e2, sweep_via(e2, e1, towards_c2 + PI, true));
            pts.extend(back.into_iter().skip(1));
            pts.pop();
            g.closed(&mut s, &pts);
        }
        Icon::Monitor => {
            g.rect(&mut s, (3.0, 4.0), (21.0, 16.0));
            g.line(&mut s, &[(12.0, 16.0), (12.0, 20.0)]);
            g.line(&mut s, &[(8.0, 20.0), (16.0, 20.0)]);
        }
    }
    s
}

/// The DS Loader: three bars raked at 40 degrees, each passing from `rest` to `active` in turn
/// (1080 ms, 110 ms apart). No spinning ring. `rect` is the loader's box; repaints while shown.
pub(crate) fn loader(painter: &Painter, rect: Rect, rest: Color32, active: Color32, time: f64) {
    let h = rect.height().min(rect.width());
    let w = (0.19 * h).max(2.0);
    let gap = 0.2 * h;
    let total = 3.0 * w + 2.0 * gap;
    let left = rect.center().x - total / 2.0;
    let base = rect.center().y + h / 2.0;
    // skewX(-40deg): the top leans right by tan(40) of the height.
    let lean = (40.0_f32).to_radians().tan();
    for (i, k) in [0.6_f32, 1.0, 0.6].into_iter().enumerate() {
        let bh = h * k;
        let bottom = base - (h - bh) / 2.0;
        let top = bottom - bh;
        let x = left + i as f32 * (w + gap);
        let phase = ((time - i as f64 * 0.110) / 1.080).rem_euclid(1.0) as f32;
        // Keyframes: rest at 0 and from 62%, active 16-40%.
        let on = match phase {
            p if p < 0.16 => p / 0.16,
            p if p < 0.40 => 1.0,
            p if p < 0.62 => 1.0 - (p - 0.40) / 0.22,
            _ => 0.0,
        };
        let c = crate::theme::mix(rest, active, on);
        let cy = rect.center().y;
        let dx = |y: f32| (cy - y) * lean;
        let pts = vec![pos2(x + dx(bottom), bottom), pos2(x + w + dx(bottom), bottom), pos2(x + w + dx(top), top), pos2(x + dx(top), top)];
        painter.add(Shape::convex_polygon(pts, c, Stroke::NONE));
    }
}
