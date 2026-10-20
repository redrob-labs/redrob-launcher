//! Windows 11 Snap Layouts on the drawn Maximize button.
//!
//! Windows opens Snap Layouts (the flyout of window layouts) when the pointer rests on a window's
//! maximize button. A custom-drawn button participates when the window answers `WM_NCHITTEST` with
//! `HTMAXBUTTON` over it. winit never does: its frameless windows only report the caption and the
//! resize edges. This module therefore subclasses the launcher's window and answers `HTMAXBUTTON`
//! over the place the UI last drew Maximize ([`redrob_launcher_ui_egui::caption`]). It also removes
//! `WS_SYSMENU`: otherwise Windows associates the flyout with winit's invisible standard caption
//! buttons instead of the drawn one. Because that style normally supplies Alt+Space behavior, the
//! subclass explicitly displays and dispatches the standard system menu from the keyboard.
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
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::ScreenToClient;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{TME_LEAVE, TME_NONCLIENT, TRACKMOUSEEVENT, TrackMouseEvent, VK_SPACE};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, EnableMenuItem, GWL_STYLE, GWLP_WNDPROC, GetSystemMenu, GetWindowRect, HTMAXBUTTON, IsIconic, IsZoomed, MF_BYCOMMAND,
    MF_ENABLED, MF_GRAYED, SC_CLOSE, SC_MAXIMIZE, SC_MINIMIZE, SC_MOVE, SC_RESTORE, SC_SIZE, STYLESTRUCT, SW_MAXIMIZE, SW_RESTORE, SWP_FRAMECHANGED,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SendMessageW, SetForegroundWindow, SetWindowPos, ShowWindow, TPM_RETURNCMD,
    TPM_RIGHTBUTTON, TrackPopupMenu, WM_MOUSEMOVE, WM_NCDESTROY, WM_NCHITTEST, WM_NCLBUTTONDBLCLK, WM_NCLBUTTONDOWN, WM_NCLBUTTONUP, WM_NCMOUSELEAVE,
    WM_NCMOUSEMOVE, WM_STYLECHANGING, WM_SYSCOMMAND, WM_SYSKEYDOWN, WNDPROC, WS_MAXIMIZEBOX, WS_MINIMIZEBOX, WS_SYSMENU, WS_THICKFRAME,
};
#[cfg(target_pointer_width = "64")]
use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW};
#[cfg(target_pointer_width = "32")]
use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowLongW, SetWindowLongW};

/// The subclassed window as a pointer-sized integer (`HWND` is not `Sync`); using `isize` keeps
/// the bridge lossless on both 32- and 64-bit Windows. Zero means no window.
static WINDOW: AtomicIsize = AtomicIsize::new(0);
/// The window's own procedure (winit's), called for everything this one doesn't take.
static PREVIOUS: AtomicIsize = AtomicIsize::new(0);
/// Whether a `WM_NCMOUSELEAVE` request is outstanding (asked once per stay, as WezTerm does).
static TRACKING: AtomicBool = AtomicBool::new(false);
/// Repaints the window when the OS's report on Maximize changes.
static CONTEXT: Mutex<Option<egui::Context>> = Mutex::new(None);

/// Pointer-width Win32 accessors. The `Ptr` symbols do not exist in windows-sys on 32-bit Windows,
/// where the `W` functions carry the same 32-bit pointer bits.
#[cfg(target_pointer_width = "64")]
unsafe fn get_window_long(hwnd: HWND, index: i32) -> isize {
    // SAFETY: the caller supplies a live window and a valid GWL/GWLP index.
    unsafe { GetWindowLongPtrW(hwnd, index) }
}

#[cfg(target_pointer_width = "32")]
unsafe fn get_window_long(hwnd: HWND, index: i32) -> isize {
    // SAFETY: the caller supplies a live window and a valid GWL/GWLP index; i32 and isize have the
    // same width on this target.
    unsafe { GetWindowLongW(hwnd, index) as isize }
}

#[cfg(target_pointer_width = "64")]
unsafe fn set_window_long(hwnd: HWND, index: i32, value: isize) -> isize {
    // SAFETY: the caller supplies a live window, valid GWL/GWLP index, and compatible value.
    unsafe { SetWindowLongPtrW(hwnd, index, value) }
}

#[cfg(target_pointer_width = "32")]
unsafe fn set_window_long(hwnd: HWND, index: i32, value: isize) -> isize {
    // SAFETY: the caller supplies a live window, valid GWL/GWLP index, and compatible value; i32
    // and isize have the same width on this target, so procedure pointers are not truncated.
    unsafe { SetWindowLongW(hwnd, index, value as i32) as isize }
}

/// Subclasses `hwnd`, the launcher's window, and suppresses its invisible system caption buttons.
/// A window opened later (after one closed to the tray) replaces the last: the old one is gone by
/// then.
pub fn install(hwnd: isize, ctx: egui::Context) {
    if hwnd == 0 {
        return;
    }
    if WINDOW.load(Ordering::Relaxed) == hwnd {
        *CONTEXT.lock().unwrap_or_else(PoisonError::into_inner) = Some(ctx);
        return;
    }

    let window = hwnd as HWND;
    let procedure: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT = procedure;
    // SAFETY: `hwnd` is the live window eframe just created on this thread, and `procedure` has
    // the window-procedure signature. This thread cannot dispatch another window message between
    // this call and storing the returned procedure below.
    let previous = unsafe { set_window_long(window, GWLP_WNDPROC, procedure as usize as isize) };
    if previous == 0 {
        log::warn!("Snap Layouts: couldn't subclass the window");
        return;
    }

    PREVIOUS.store(previous, Ordering::Relaxed);
    WINDOW.store(hwnd, Ordering::Relaxed);
    *CONTEXT.lock().unwrap_or_else(PoisonError::into_inner) = Some(ctx);
    caption::reset();
    TRACKING.store(false, Ordering::Relaxed);

    // SAFETY: style calls target the same live window. `SWP_FRAMECHANGED` makes Windows re-read the
    // style without moving, sizing, reordering, or activating it. Alt+Space is replaced below.
    unsafe {
        let style = get_window_long(window, GWL_STYLE);
        set_window_long(window, GWL_STYLE, style & !(WS_SYSMENU as isize));
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

/// Enables or grays one standard command in the window-owned system menu.
fn set_system_command_enabled(menu: windows_sys::Win32::UI::WindowsAndMessaging::HMENU, command: u32, enabled: bool) {
    // SAFETY: `menu` is the live menu returned by GetSystemMenu; these are standard command IDs.
    unsafe { EnableMenuItem(menu, command, MF_BYCOMMAND | if enabled { MF_ENABLED } else { MF_GRAYED }) };
}

/// Shows the standard Alt+Space menu that `WS_SYSMENU` would normally provide.
fn show_system_menu(hwnd: HWND) {
    // SAFETY: every handle used here belongs to the live window whose procedure is running. The
    // returned menu remains owned by the window, and TPM_RETURNCMD keeps dispatch under our control.
    unsafe {
        let menu = GetSystemMenu(hwnd, 0);
        if menu.is_null() {
            log::warn!("Snap Layouts: couldn't get the system menu");
            return;
        }

        let style = get_window_long(hwnd, GWL_STYLE);
        let minimized = IsIconic(hwnd) != 0;
        let maximized = IsZoomed(hwnd) != 0;
        let normal = !minimized && !maximized;
        // Mirror the default system menu: window state controls Restore/Move, while the remaining
        // caption-box styles still describe which operations winit allows after WS_SYSMENU is off.
        set_system_command_enabled(menu, SC_RESTORE, minimized || maximized);
        set_system_command_enabled(menu, SC_MOVE, normal);
        set_system_command_enabled(menu, SC_SIZE, normal && style & WS_THICKFRAME as isize != 0);
        set_system_command_enabled(menu, SC_MINIMIZE, !minimized && style & WS_MINIMIZEBOX as isize != 0);
        set_system_command_enabled(menu, SC_MAXIMIZE, !maximized && style & WS_MAXIMIZEBOX as isize != 0);
        set_system_command_enabled(menu, SC_CLOSE, true);

        let mut bounds = RECT::default();
        if GetWindowRect(hwnd, &mut bounds) == 0 {
            log::warn!("Snap Layouts: couldn't position the system menu");
            return;
        }

        // TrackPopupMenu requires its owner to be foreground for normal keyboard/click-away behavior.
        SetForegroundWindow(hwnd);
        let command = TrackPopupMenu(menu, TPM_RETURNCMD | TPM_RIGHTBUTTON, bounds.left, bounds.top, 0, hwnd, std::ptr::null());
        if command != 0 {
            // SAFETY: command is one of the menu's pointer-width-safe, predefined SC_* identifiers;
            // dispatching WM_SYSCOMMAND preserves the window procedure's normal command handling.
            SendMessageW(hwnd, WM_SYSCOMMAND, command as WPARAM, 0);
        }
    }
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
        WM_SYSKEYDOWN if wparam == VK_SPACE as WPARAM && lparam & (1 << 29) != 0 => {
            // Bit 30 marks key-repeat. Consume repeats too, but open only once per Alt+Space press.
            if lparam & (1 << 30) == 0 {
                show_system_menu(hwnd);
            }
            return 0;
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
            TRACKING.store(false, Ordering::Relaxed);
            caption::reset();
            *CONTEXT.lock().unwrap_or_else(PoisonError::into_inner) = None;
            if previous != 0 {
                // SAFETY: puts back the window's own procedure before it is destroyed.
                unsafe { set_window_long(hwnd, GWLP_WNDPROC, previous) };
            }
            return call_previous(previous, hwnd, msg, wparam, lparam);
        }
        _ => {}
    }
    call_previous(PREVIOUS.load(Ordering::Relaxed), hwnd, msg, wparam, lparam)
}

/// Hands a message to the window's own procedure.
fn call_previous(previous: isize, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // SAFETY: `previous` came from the pointer-width window-long wrapper for GWLP_WNDPROC on
    // this window, so it is its procedure (or 0, which CallWindowProcW treats as default handling).
    unsafe {
        let previous = std::mem::transmute::<isize, WNDPROC>(previous);
        CallWindowProcW(previous, hwnd, msg, wparam, lparam)
    }
}
