// ADDED-LINE-RATIONALE c98826a5304ceb8e:1: Supports `caption` while preserving the nearby constraint: ! egui's own hover and click.
// ADDED-LINE-RATIONALE 2c78d20fc5b2ab7f:1: Gives `caption` direct access to `std / sync / Mutex` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE dd72b6f1778979b0:1: Supports `caption` while preserving the nearby constraint: top-left, as `[left, top, right, bottom]`.
// ADDED-LINE-RATIONALE 84370bf594ca08cd:1: Freezes `static` for `caption` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c4cd00611c2cbfe4:1: Freezes `static` for `caption` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7467b85af249bb86:1: Supports `caption` while preserving the nearby constraint: What the OS reports about the pointer over Maximize.
// ADDED-LINE-RATIONALE eddafb7f0391e6d0:1: Supports `OsState` while preserving the nearby constraint: [derive(Clone, Copy, Debug, Default, PartialEq, Eq)].
// ADDED-LINE-RATIONALE 2ba07e4820f2c210:1: Supports `OsState` while preserving the nearby constraint: The pointer is over the button.
// ADDED-LINE-RATIONALE fcaf3169f56487f3:1: Supports `OsState` while preserving the nearby constraint: The primary button went down on it and hasn't come up.
// ADDED-LINE-RATIONALE 7b1ff0053ddc5131:1: Bounds the `OsState` implementation here so `pressed / bool` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b36e8b90deac8930:1: Supports `publish_maximize` while preserving the nearby constraint: Records where Maximize is this frame (its hit area, in points).
// ADDED-LINE-RATIONALE e00fa2149b5fb30e:1: Delegates `publish_maximize`'s `publish / ctx / pixels_per_point` work to `publish` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7b1ff0053ddc5131:2: Bounds the `publish_maximize` implementation here so `publish / ctx / pixels_per_point` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b3b38d87439a3821:1: Isolates `publish` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE e4654390b3313d1f:1: Freezes `physical` for `publish` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f9b9f7f31d90303b:1: Delegates `publish`'s `MAXIMIZE / lock / unwrap_or_else` work to `MAXIMIZE.lock` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7b1ff0053ddc5131:3: Supports `publish` while preserving the nearby constraint: MAXIMIZE.lock().unwrap_or_else(PoisonError::into_inner) = Some(physical);.
// ADDED-LINE-RATIONALE fcecc01abf0da2d2:1: Supports `maximize_rect` while preserving the nearby constraint: (`[left, top, right, bottom]`); `None` until a custom title bar draws it.
// ADDED-LINE-RATIONALE c29b3f01bd2fcb81:1: Delegates `maximize_rect`'s `MAXIMIZE / lock / unwrap_or_else` work to `MAXIMIZE.lock` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7b1ff0053ddc5131:4: Supports `maximize_rect` while preserving the nearby constraint: MAXIMIZE.lock().unwrap_or_else(PoisonError::into_inner).
// ADDED-LINE-RATIONALE d8b85ce00c4dc29b:1: Supports `on_maximize` while preserving the nearby constraint: Whether a physical client point is on Maximize.
// ADDED-LINE-RATIONALE 142107091548fc82:1: Freezes `maximize_rect` for `on_maximize` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7b1ff0053ddc5131:5: Bounds the `on_maximize` implementation here so `maximize_rect / is_some_and / l` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e97b2f1e4a9603bf:1: Supports `os_state` while preserving the nearby constraint: The OS's last report.
// ADDED-LINE-RATIONALE b9d31a2a7862ea91:1: Delegates `os_state`'s `OsState / hovered / HOVERED` work to `HOVERED.load` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7b1ff0053ddc5131:6: Bounds the `os_state` implementation here so `OsState / hovered / HOVERED` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e0563e970dd26fe9:1: Supports `set_os_state` while preserving the nearby constraint: Sets the OS's report. Returns whether it changed, so the host knows to repaint.
// ADDED-LINE-RATIONALE 6bbae827dc165d81:1: Freezes `was` for `set_os_state` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a320abc32a49fbd5:1: Delegates `set_os_state`'s `HOVERED / store / state` work to `HOVERED.store` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 42ab10124fe7ea19:1: Delegates `set_os_state`'s `PRESSED / store / state` work to `PRESSED.store` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0eb83a4e6cfa45d2:1: Freezes `was` for `set_os_state` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7b1ff0053ddc5131:7: Bounds the `set_os_state` implementation here so `was / state` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3ae795c18d5e481e:1: Supports `reset` while preserving the nearby constraint: Forgets the button and the OS's report, when its window goes.
// ADDED-LINE-RATIONALE b1b32e08e071785d:1: Delegates `reset`'s `MAXIMIZE / lock / unwrap_or_else` work to `MAXIMIZE.lock` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 453ad8b063f95e96:1: Supports `reset` while preserving the nearby constraint: MAXIMIZE.lock().unwrap_or_else(PoisonError::into_inner) = None;.
// ADDED-LINE-RATIONALE 7b1ff0053ddc5131:8: Bounds the `reset` implementation here so `set_os_state / OsState / default` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 87ba444777f38760:1: Declares the `cfg / test` contract on `reset` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE c4ce4aaf2ea3be52:1: Supports `tests` while preserving the nearby constraint: [cfg(test)].
// ADDED-LINE-RATIONALE 2bc5a405121bead9:1: Gives `tests` direct access to `super` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 986e6326699f88bf:1: Declares the `test` contract on `tests` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 63925abb95f4dd51:1: Supports `hit_area_is_in_physical_pixels_and_half_open` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 0d63b49f4b7c016b:1: Delegates `hit_area_is_in_physical_pixels_and_half_open`'s `publish / egui / Rect` work to `publish` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 92fd59b1be89b5c0:1: Makes `hit_area_is_in_physical_pixels_and_half_open` fail visibly when `assert_eq / maximize_rect` violates the behavior this path promises.
// ADDED-LINE-RATIONALE eb0a7d316f7d57c7:1: Makes `hit_area_is_in_physical_pixels_and_half_open` fail visibly when `assert / on_maximize` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 396244f07081a70c:1: Makes `hit_area_is_in_physical_pixels_and_half_open` fail visibly when `assert / on_maximize` violates the behavior this path promises.
// ADDED-LINE-RATIONALE d4b5522dd66601e0:1: Makes `hit_area_is_in_physical_pixels_and_half_open` fail visibly when `assert / on_maximize / the` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 1d15498d9ed2543b:1: Makes `hit_area_is_in_physical_pixels_and_half_open` fail visibly when `assert / on_maximize / the` violates the behavior this path promises.
// ADDED-LINE-RATIONALE eb28361483a2eef6:1: Makes `hit_area_is_in_physical_pixels_and_half_open` fail visibly when `assert / set_os_state / OsState` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 5c4f8aea812a6ae6:1: Makes `hit_area_is_in_physical_pixels_and_half_open` fail visibly when `assert / set_os_state / OsState` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 03ed513186d3255f:1: Delegates `hit_area_is_in_physical_pixels_and_half_open`'s `reset` work to `reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5ebd5389f76d5402:1: Makes `hit_area_is_in_physical_pixels_and_half_open` fail visibly when `assert_eq / maximize_rect / os_state` violates the behavior this path promises.
// ADDED-LINE-RATIONALE f5ccd5386cbf0e11:1: Bounds the `hit_area_is_in_physical_pixels_and_half_open` implementation here so `assert_eq / maximize_rect / os_state` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7b1ff0053ddc5131:9: Bounds the `hit_area_is_in_physical_pixels_and_half_open` implementation here so `assert_eq / maximize_rect / os_state` cannot leak into the adjacent declaration or control path.
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
