//! macOS only: `redrob://` links (`deep_link`). macOS doesn't start the launcher with the link as
//! an argument; Launch Services sends the running (or just started) launcher a `kAEGetURL` Apple
//! Event, which winit doesn't surface. This module installs the Apple Event handler and passes
//! each link's URL to `on_link`.
//!
//! The one module in the workspace that needs `unsafe`: an Objective-C class for the handler and
//! two messages to `NSAppleEventManager`. Each block says why it is sound. The URL itself is only
//! read as a string and parsed by `deep_link::parse`, like a link from the command line.

#![allow(unsafe_code)]

use std::cell::OnceCell;

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject};
use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, sel};
use objc2_foundation::{NSObjectProtocol, NSString};

/// `kInternetEventClass` and `kAEGetURL`: both `'GURL'`.
const GURL: u32 = u32::from_be_bytes(*b"GURL");
/// `keyDirectObject`: `'----'`, the event's URL.
const KEY_DIRECT_OBJECT: u32 = u32::from_be_bytes(*b"----");

struct Ivars {
    on_link: Box<dyn Fn(String)>,
}

define_class!(
    // SAFETY: NSObject has no subclassing requirements, and `UrlHandler` doesn't implement Drop.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "RedrobLauncherUrlHandler"]
    #[ivars = Ivars]
    struct UrlHandler;

    impl UrlHandler {
        // SAFETY: the signature is `-[handleGetURL:(NSAppleEventDescriptor *)event
        // withReplyEvent:(NSAppleEventDescriptor *)reply]`, which NSAppleEventManager calls with
        // two non-null descriptors.
        #[unsafe(method(handleGetURL:withReplyEvent:))]
        fn handle_get_url(&self, event: &AnyObject, _reply: &AnyObject) {
            // SAFETY: `event` is an NSAppleEventDescriptor; `paramDescriptorForKeyword:` takes an
            // AEKeyword (UInt32) and returns a descriptor or nil.
            let direct: Option<Retained<AnyObject>> = unsafe { msg_send![event, paramDescriptorForKeyword: KEY_DIRECT_OBJECT] };
            let Some(direct) = direct else { return };
            // SAFETY: `stringValue` of an NSAppleEventDescriptor returns an NSString or nil.
            let url: Option<Retained<NSString>> = unsafe { msg_send![&*direct, stringValue] };
            if let Some(url) = url {
                (self.ivars().on_link)(url.to_string());
            }
        }
    }

    unsafe impl NSObjectProtocol for UrlHandler {}
);

impl UrlHandler {
    fn new(mtm: MainThreadMarker, on_link: Box<dyn Fn(String)>) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(Ivars { on_link });
        // SAFETY: NSObject's designated initialiser, on a freshly allocated object.
        unsafe { msg_send![super(this), init] }
    }
}

thread_local! {
    /// The handler: NSAppleEventManager doesn't retain it, so it lives as long as the main thread.
    static HANDLER: OnceCell<Retained<UrlHandler>> = const { OnceCell::new() };
}

/// Installs the handler (the first call's `on_link` is kept; later calls install it again, in case
/// AppKit put its own in place while launching). Must run on the main thread; elsewhere it does
/// nothing. Call it before the event loop starts and again once the first window exists, so the
/// link the launcher was started for arrives too.
pub fn install(on_link: impl Fn(String) + 'static) {
    let Some(mtm) = MainThreadMarker::new() else { return };
    HANDLER.with(|cell| {
        let handler = cell.get_or_init(|| UrlHandler::new(mtm, Box::new(on_link)));
        let Some(manager) = shared_manager() else { return };
        let handler: &AnyObject = handler;
        // SAFETY: `handler` implements `handleGetURL:withReplyEvent:` with the signature
        // NSAppleEventManager calls, and stays alive in `HANDLER`; the event class and id are
        // AEEventClass and AEEventID (UInt32).
        let () = unsafe {
            msg_send![&*manager, setEventHandler: handler, andSelector: sel!(handleGetURL:withReplyEvent:), forEventClass: GURL, andEventID: GURL]
        };
    });
}

fn shared_manager() -> Option<Retained<AnyObject>> {
    let class = objc2::runtime::AnyClass::get(c"NSAppleEventManager")?;
    // SAFETY: `+[NSAppleEventManager sharedAppleEventManager]` takes nothing and returns the
    // shared manager.
    unsafe { msg_send![class, sharedAppleEventManager] }
}
