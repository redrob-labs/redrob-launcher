//! The hand-off between the drawn Maximize button and a desktop host that tells the OS where it
//! is.
//!
//! Windows 11 opens Snap Layouts only over a window's own maximize button, which it finds by
//! asking the window (`WM_NCHITTEST`, answered `HTMAXBUTTON`). The launcher's caption buttons are
//! drawn by egui, so the desktop host (`apps/redrob-launcher/src/snap_layouts.rs`) reads the
//! button's place from here. Over that button the OS then sends the pointer to the window's frame
//! instead of to egui, so the host writes the hover and press it sees back here for the button to
//! draw.
//!
//! Hosts that never read it (the web, Linux, macOS, the tests) leave it idle, and the button uses
//! egui's own hover and click.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};

/// Where Maximize was last drawn: its hit area in physical pixels from the window's client
/// top-left, as `[left, top, right, bottom]`.
static MAXIMIZE: Mutex<Option<[f32; 4]>> = Mutex::new(None);
static HOVERED: AtomicBool = AtomicBool::new(false);
static PRESSED: AtomicBool = AtomicBool::new(false);

/// What the OS reports about the pointer over Maximize.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OsState {
    /// The pointer is over the button.
    pub hovered: bool,
    /// The primary button went down on it and hasn't come up.
    pub pressed: bool,
}

/// Records where Maximize is this frame (its hit area, in points).
pub(crate) fn publish_maximize(ctx: &egui::Context, hit: egui::Rect) {
    publish(ctx.pixels_per_point(), hit);
}

fn publish(ppp: f32, hit: egui::Rect) {
    let physical = [hit.left() * ppp, hit.top() * ppp, hit.right() * ppp, hit.bottom() * ppp];
    *MAXIMIZE.lock().unwrap_or_else(PoisonError::into_inner) = Some(physical);
}

/// Where Maximize was last drawn, in physical pixels from the client top-left
/// (`[left, top, right, bottom]`); `None` until a custom title bar draws it.
pub fn maximize_rect() -> Option<[f32; 4]> {
    *MAXIMIZE.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Whether a physical client point is on Maximize.
pub fn on_maximize(x: f32, y: f32) -> bool {
    maximize_rect().is_some_and(|[l, t, r, b]| x >= l && x < r && y >= t && y < b)
}

/// The OS's last report.
pub fn os_state() -> OsState {
    OsState { hovered: HOVERED.load(Ordering::Relaxed), pressed: PRESSED.load(Ordering::Relaxed) }
}

/// Sets the OS's report. Returns whether it changed, so the host knows to repaint.
pub fn set_os_state(state: OsState) -> bool {
    let was = os_state();
    HOVERED.store(state.hovered, Ordering::Relaxed);
    PRESSED.store(state.pressed, Ordering::Relaxed);
    was != state
}

/// Forgets the button and the OS's report, when its window goes.
pub fn reset() {
    *MAXIMIZE.lock().unwrap_or_else(PoisonError::into_inner) = None;
    set_os_state(OsState::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_area_is_in_physical_pixels_and_half_open() {
        publish(2.0, egui::Rect::from_min_max(egui::pos2(100.0, 5.0), egui::pos2(146.0, 48.0)));
        assert_eq!(maximize_rect(), Some([200.0, 10.0, 292.0, 96.0]));
        assert!(on_maximize(200.0, 10.0));
        assert!(on_maximize(291.5, 95.5));
        assert!(!on_maximize(292.0, 50.0), "the right edge belongs to the next button");
        assert!(!on_maximize(250.0, 9.0), "the resize band above stays the window's");
        assert!(set_os_state(OsState { hovered: true, pressed: false }));
        assert!(!set_os_state(OsState { hovered: true, pressed: false }), "an unchanged report asks for no repaint");
        reset();
        assert_eq!((maximize_rect(), os_state()), (None, OsState::default()));
    }
}
