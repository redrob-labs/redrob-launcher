//! Windows 11 Snap Layouts on the drawn Maximize button.
//!
//! Windows opens Snap Layouts (the flyout of window layouts) when the pointer rests on a window's
//! maximize button. Two things decide whether a custom-drawn button counts:
//!
//! - The window answers `WM_NCHITTEST` with `HTMAXBUTTON` over it. winit never does: its
//!   frameless windows only report the caption and the resize edges. So this module subclasses the
//!   launcher's window and answers `HTMAXBUTTON` over the place the UI last drew Maximize
//!   ([`redrob_launcher_ui_egui::caption`]).
//! - The window has no system-drawn caption buttons. winit keeps `WS_SYSMENU`, so Windows believes
//!   the window still has its own (invisible) minimize, maximize and close in the default caption
//!   and attaches the flyout to those, ignoring the drawn one. Measured with `GetTitleBarInfo`:
//!   with `WS_SYSMENU` the launcher's maximize reads as present; without it, as invisible, like
//!   Redrob Cowork's (Electron's frameless windows have no `WS_SYSMENU`). So the style is removed,
//!   and kept off when winit rewrites the style (`WM_STYLECHANGING`).
//!
//! Over `HTMAXBUTTON` the pointer is in the frame, so egui no longer sees it there. The frame's
//! messages are relayed back instead: `WM_NCMOUSEMOVE` / `WM_NCMOUSELEAVE` give the hover, and
//! `WM_NCLBUTTONDOWN` / `WM_NCLBUTTONUP` the press and the click, which maximizes or restores the
//! window. Those messages stop here when they are about Maximize: passed on, the default handling
//! would hot-track or run the press loop of a classic caption button. Everything else goes to
//! winit untouched.
//!
//! This is the workspace's one use of `unsafe` (`AGENTS.md`): Win32 calls on the launcher's own
//! window, from its own UI thread.

#![allow(unsafe_code)]

use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::{Mutex, PoisonError};

use redrob_launcher_ui_egui::caption::{self, OsState};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::ScreenToClient;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{TME_LEAVE, TME_NONCLIENT, TRACKMOUSEEVENT, TrackMouseEvent};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, GWL_STYLE, GWLP_WNDPROC, GetWindowLongPtrW, HTMAXBUTTON, IsZoomed, STYLESTRUCT, SW_MAXIMIZE, SW_RESTORE, SWP_FRAMECHANGED,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SetWindowLongPtrW, SetWindowPos, ShowWindow, WM_MOUSEMOVE, WM_NCDESTROY, WM_NCHITTEST,
    WM_NCLBUTTONDBLCLK, WM_NCLBUTTONDOWN, WM_NCLBUTTONUP, WM_NCMOUSELEAVE, WM_NCMOUSEMOVE, WM_STYLECHANGING, WNDPROC, WS_SYSMENU,
};

/// The subclassed window, as an integer (`HWND` is a pointer, so not `Sync`); 0 when none.
static WINDOW: AtomicIsize = AtomicIsize::new(0);
/// The window's own procedure (winit's), called for everything this one doesn't take.
static PREVIOUS: AtomicIsize = AtomicIsize::new(0);
/// Whether a `WM_NCMOUSELEAVE` request is outstanding (asked once per stay, as WezTerm does).
static TRACKING: AtomicBool = AtomicBool::new(false);
/// Repaints the window when the OS's report on Maximize changes.
static CONTEXT: Mutex<Option<egui::Context>> = Mutex::new(None);

/// Subclasses `hwnd`, the launcher's window, and removes its system caption buttons. A window
/// opened later (after one closed to the tray) replaces the last: the old one is gone by then.
pub fn install(hwnd: isize, ctx: egui::Context) {
    if hwnd == 0 || WINDOW.load(Ordering::Relaxed) == hwnd {
        return;
    }
    *CONTEXT.lock().unwrap_or_else(PoisonError::into_inner) = Some(ctx);
    caption::reset();
    TRACKING.store(false, Ordering::Relaxed);
    let window = hwnd as HWND;
    let procedure: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT = procedure;
    // SAFETY: `hwnd` is the live window eframe just created on this thread, and `procedure` has
    // the window-procedure signature. The previous procedure is stored before the window can
    // reach the new one, which needs this thread's message loop.
    let previous = unsafe { SetWindowLongPtrW(window, GWLP_WNDPROC, procedure as usize as isize) };
    if previous == 0 {
        log::warn!("Snap Layouts: couldn't subclass the window");
        return;
    }
    PREVIOUS.store(previous, Ordering::Relaxed);
    WINDOW.store(hwnd, Ordering::Relaxed);
    // SAFETY: style calls on the same live window; SWP_FRAMECHANGED makes Windows re-read the
    // style without moving, sizing, reordering or activating it.
    unsafe {
        let style = GetWindowLongPtrW(window, GWL_STYLE);
        SetWindowLongPtrW(window, GWL_STYLE, style & !(WS_SYSMENU as isize));
        SetWindowPos(window, std::ptr::null_mut(), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED);
    }
}

/// Reports a new state for Maximize and repaints if it changed.
fn report(state: OsState) {
    if caption::set_os_state(state)
        && let Some(ctx) = CONTEXT.lock().unwrap_or_else(PoisonError::into_inner).as_ref()
    {
        ctx.request_repaint();
    }
}

/// The screen point in a mouse message's `lparam`, in `hwnd`'s client pixels.
fn client_point(hwnd: HWND, lparam: LPARAM) -> (f32, f32) {
    // GET_X_LPARAM / GET_Y_LPARAM: signed, so a second monitor left of or above the first works.
    let mut p = POINT { x: i32::from((lparam & 0xFFFF) as u16 as i16), y: i32::from(((lparam >> 16) & 0xFFFF) as u16 as i16) };
    // SAFETY: `hwnd` is the window whose procedure is running, and `p` is a valid POINT.
    unsafe { ScreenToClient(hwnd, &mut p) };
    (p.x as f32, p.y as f32)
}

/// The launcher window's procedure while subclassed.
unsafe extern "system" fn procedure(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let on_max = wparam == HTMAXBUTTON as WPARAM;
    match msg {
        WM_NCHITTEST => {
            let (x, y) = client_point(hwnd, lparam);
            if caption::on_maximize(x, y) {
                return HTMAXBUTTON as LRESULT;
            }
        }
        // winit recomputes the whole style on some state changes (maximize, resizable, ...): keep
        // WS_SYSMENU off so the system caption buttons never come back.
        WM_STYLECHANGING if wparam == GWL_STYLE as isize as WPARAM && lparam != 0 => {
            // SAFETY: for WM_STYLECHANGING with GWL_STYLE, lparam points to a STYLESTRUCT the
            // sender owns for the duration of this call.
            unsafe { (*(lparam as *mut STYLESTRUCT)).styleNew &= !WS_SYSMENU };
        }
        WM_NCMOUSEMOVE => {
            // Ask once for WM_NCMOUSELEAVE (until it arrives), as WezTerm does.
            if !TRACKING.swap(true, Ordering::Relaxed) {
                let mut track = TRACKMOUSEEVENT {
                    cbSize: size_of::<TRACKMOUSEEVENT>() as u32,
                    dwFlags: TME_LEAVE | TME_NONCLIENT,
                    hwndTrack: hwnd,
                    dwHoverTime: 0,
                };
                // SAFETY: `track` is a valid TRACKMOUSEEVENT for this window.
                unsafe { TrackMouseEvent(&mut track) };
            }
            let pressed = on_max && caption::os_state().pressed;
            report(OsState { hovered: on_max, pressed });
            if on_max {
                return 0;
            }
        }
        WM_NCMOUSELEAVE => {
            TRACKING.store(false, Ordering::Relaxed);
            report(OsState::default());
        }
        WM_MOUSEMOVE => report(OsState::default()),
        WM_NCLBUTTONDOWN | WM_NCLBUTTONDBLCLK if on_max => {
            report(OsState { hovered: true, pressed: true });
            return 0;
        }
        WM_NCLBUTTONUP if on_max => {
            if caption::os_state().pressed {
                // SAFETY: `hwnd` is the window whose procedure is running.
                unsafe { ShowWindow(hwnd, if IsZoomed(hwnd) != 0 { SW_RESTORE } else { SW_MAXIMIZE }) };
            }
            report(OsState { hovered: true, pressed: false });
            return 0;
        }
        WM_NCDESTROY => {
            let previous = PREVIOUS.swap(0, Ordering::Relaxed);
            WINDOW.store(0, Ordering::Relaxed);
            caption::reset();
            // SAFETY: puts back the window's own procedure before it is destroyed.
            unsafe { SetWindowLongPtrW(hwnd, GWLP_WNDPROC, previous) };
            return call_previous(previous, hwnd, msg, wparam, lparam);
        }
        _ => {}
    }
    call_previous(PREVIOUS.load(Ordering::Relaxed), hwnd, msg, wparam, lparam)
}

/// Hands a message to the window's own procedure.
fn call_previous(previous: isize, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // SAFETY: `previous` came from SetWindowLongPtrW(GWLP_WNDPROC) on this window, so it is its
    // window procedure (or 0, which CallWindowProcW turns into the default handling).
    unsafe {
        let previous = std::mem::transmute::<isize, WNDPROC>(previous);
        CallWindowProcW(previous, hwnd, msg, wparam, lparam)
    }
}
