//! Redrob Launcher's desktop backend.
//!
//! [`NativeBackend`] runs every slow thing (HTTP, scanning disks, installers) on worker threads
//! and reports back through [`Event`](redrob_launcher_core::protocol::Event)s, so the UI never
//! waits on it. Release information comes from a [`source::ReleaseSource`]: GitHub today, the
//! Redrob catalog API later.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

pub mod account;
pub mod app_icons;
pub mod backend;
pub mod channels;
pub mod desk;
pub mod desktop_entry;
pub mod detect;
pub mod device;
pub mod engine;
pub mod engine_link;
pub mod feed_throttle;
pub mod http;
pub mod install;
pub mod language;
pub mod log_file;
pub mod login;
pub mod manifest;
pub mod models;
#[cfg(target_os = "macos")]
pub mod move_to_applications;
pub mod packages;
pub mod paths;
pub mod process;
pub mod proxy;
pub mod queue;
pub mod runtime;
pub mod self_update;
pub mod single_instance;
pub mod source;
pub mod url_scheme;

pub use backend::NativeBackend;

/// Unix seconds now.
pub fn now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
}
