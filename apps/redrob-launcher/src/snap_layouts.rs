//! Windows 11 Snap Layouts on the drawn Maximize button.
//!
//! Windows opens Snap Layouts (the flyout of window layouts) when the pointer rests on a window's
//! maximize button. It finds that button by sending the window `WM_NCHITTEST` and reading back
//! `HTMAXBUTTON`, which winit never answers: its frameless windows only report the caption and
//! the resize edges. So this module subclasses the launcher's window and answers `HTMAXBUTTON`
//! over the place the UI last drew Maximize ([`redrob_launcher_ui_egui::caption`]). That is
//! Redrob Cowork's behaviour, where Electron's caption buttons do the same.
//!
//! Over `HTMAXBUTTON` the pointer is in the frame, so egui no longer sees it there. The frame's
//! messages are relayed back instead: `WM_NCMOUSEMOVE` / `WM_NCMOUSELEAVE` give the hover, and
//! `WM_NCLBUTTONDOWN` / `WM_NCLBUTTONUP` the press and the click, which maximizes or restores the
//! window. Those button messages stop here: passed on, the default handling would run the classic
//! caption-button press loop. Everything else goes to winit untouched.
//!
//! This is the workspace's one use of `unsafe` (`AGENTS.md`): Win32 calls on the launcher's own
//! window, from its own UI thread.

#![allow(unsafe_code)]

use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::{Mutex, PoisonError};

use redrob_launcher_ui_egui::caption::{self, OsState};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::ScreenToClient;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{TME_LEAVE, TME_NONCLIENT, TRACKMOUSEEVENT, TrackMouseEvent};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, GWLP_WNDPROC, HTMAXBUTTON, IsZoomed, SW_MAXIMIZE, SW_RESTORE, SetWindowLongPtrW, ShowWindow, WM_MOUSEMOVE, WM_NCDESTROY,
    WM_NCHITTEST, WM_NCLBUTTONDBLCLK, WM_NCLBUTTONDOWN, WM_NCLBUTTONUP, WM_NCMOUSELEAVE, WM_NCMOUSEMOVE, WNDPROC,
};

/// The subclassed window, as an integer (`HWND` is a pointer, so not `Sync`); 0 when none.
static WINDOW: AtomicIsize = AtomicIsize::new(0);
/// The window's own procedure (winit's), called for everything this one doesn't take.
static PREVIOUS: AtomicIsize = AtomicIsize::new(0);
/// Repaints the window when the OS's report on Maximize changes.
static CONTEXT: Mutex<Option<egui::Context>> = Mutex::new(None);

/// Subclasses `hwnd`, the launcher's window. A window opened later (after one closed to the tray)
/// replaces the last: the old one is gone by then.
pub fn install(hwnd: isize, ctx: egui::Context) {
    if hwnd == 0 || WINDOW.load(Ordering::Relaxed) == hwnd {
        return;
    }
    *CONTEXT.lock().unwrap_or_else(PoisonError::into_inner) = Some(ctx);
    caption::reset();
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
        WM_NCMOUSEMOVE => {
            if on_max {
                // Ask for WM_NCMOUSELEAVE, which says when the pointer leaves the frame area.
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
        }
        WM_NCMOUSELEAVE | WM_MOUSEMOVE => report(OsState::default()),
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
