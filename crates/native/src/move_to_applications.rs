//! macOS: offer to move the launcher into Applications when it runs from somewhere else.
//!
//! A copy opened straight from its disk image (`/Volumes/...`), or from Downloads (where macOS
//! runs it from a read-only "translocated" path), can't update itself and is gone once the image
//! is ejected. Like Slack, Discord and Chrome, the launcher asks once per start, copies itself into
//! /Applications (or ~/Applications), opens that copy and quits.

use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::process;

/// Asks, and moves the launcher if the person agrees. `true` means the copy in Applications was
/// opened and this process should exit now.
pub fn offer() -> bool {
    let Some(bundle) = running_bundle() else { return false };
    if !needs_move(&bundle, dirs::home_dir().as_deref()) {
        return false;
    }
    if !ask() {
        return false;
    }
    match move_bundle(&bundle) {
        Ok(dest) => {
            log::info!("moved the launcher to {}", dest.display());
            if let Some(volume) = volume_of(&bundle) {
                eject_after_exit(&volume);
            }
            true
        }
        Err(e) => {
            log::warn!("couldn't move the launcher to Applications: {e}");
            false
        }
    }
}

fn running_bundle() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let exe = exe.canonicalize().unwrap_or(exe);
    exe.ancestors().find(|p| p.extension().is_some_and(|e| e == "app")).map(Path::to_path_buf)
}

/// Whether `bundle` runs from a place it shouldn't stay: a disk image, a translocated path or
/// Downloads. Anywhere else (Applications, ~/Applications, a folder someone chose) is left alone.
fn needs_move(bundle: &Path, home: Option<&Path>) -> bool {
    let s = bundle.to_string_lossy();
    s.starts_with("/Volumes/")
        || s.contains("/AppTranslocation/")
        || home.is_some_and(|h| bundle.starts_with(h.join("Downloads")))
}

/// The mounted volume a bundle lives on (`/Volumes/Redrob Launcher 0.12.2`).
fn volume_of(bundle: &Path) -> Option<PathBuf> {
    let mut parts = bundle.components();
    let (_, volumes, name) = (parts.next()?, parts.next()?, parts.next()?);
    (volumes.as_os_str() == "Volumes").then(|| Path::new("/Volumes").join(name.as_os_str()))
}

/// The system's first preferred language is Korean.
fn korean() -> bool {
    process::capture("/usr/bin/defaults", &["read", "-g", "AppleLanguages"], Duration::from_secs(5))
        .and_then(|out| out.lines().map(|l| l.trim().trim_matches(|c| c == '"' || c == ',').to_string()).find(|l| l.chars().next().is_some_and(char::is_alphabetic)))
        .is_some_and(|first| first.starts_with("ko"))
}

/// A system dialog with two buttons; `true` when the person picks "Move".
fn ask() -> bool {
    let (message, keep, move_) = if korean() {
        (
            "Redrob Launcher를 응용 프로그램 폴더로 옮길까요?\n\n지금 위치에서는 업데이트가 안 되고, 디스크 이미지를 꺼내면 런처도 사라져요.",
            "그대로 두기",
            "응용 프로그램으로 옮기기",
        )
    } else {
        (
            "Move Redrob Launcher to the Applications folder?\n\nFrom here it can't update itself, and it goes away when the disk image is ejected.",
            "Don't Move",
            "Move to Applications",
        )
    };
    // Fixed strings with no quotes or backslashes, so plain AppleScript string literals are safe.
    let script = format!(
        "display dialog \"{message}\" with title \"Redrob Launcher\" buttons {{\"{keep}\", \"{move_}\"}} default button 2 with icon note"
    );
    process::capture("/usr/bin/osascript", &["-e", &script], Duration::from_secs(600))
        .is_some_and(|out| out.contains(&format!("button returned:{move_}")))
}

/// Copies the bundle into Applications (replacing an older copy there), then opens that copy.
fn move_bundle(bundle: &Path) -> Result<PathBuf, String> {
    let name = bundle.file_name().ok_or("the launcher has no bundle name")?;
    let dir = crate::install::macos::applications_dir(None)?;
    let dest = dir.join(name);
    let staged = dir.join(format!(".{}.partial", name.to_string_lossy()));
    let _ = std::fs::remove_dir_all(&staged);
    let (Some(src), Some(staged_s), Some(dest_s)) = (bundle.to_str(), staged.to_str(), dest.to_str()) else {
        return Err("unusual characters in a path".into());
    };
    // Same flags as an app install: the disk image's Finder metadata would fail a strict
    // signature check on the copy.
    process::run("/usr/bin/ditto", &["--norsrc", "--noextattr", "--noacl", src, staged_s], Duration::from_secs(600))
        .map_err(|e| format!("couldn't copy the app: {e}"))?;
    if let Err(e) = crate::install::swap_into_place(&staged, &dest) {
        let _ = std::fs::remove_dir_all(&staged);
        return Err(e);
    }
    // -n: a new instance, since this one still holds the single-instance lock until it exits.
    process::run("/usr/bin/open", &["-n", dest_s], Duration::from_secs(30)).map_err(|e| format!("couldn't open the moved copy: {e}"))?;
    Ok(dest)
}

/// Ejects the disk image once this process (running from it) has exited.
fn eject_after_exit(volume: &Path) {
    let mut cmd = std::process::Command::new("/bin/sh");
    cmd.args(["-c", "sleep 3; /usr/bin/hdiutil detach -quiet \"$1\"", "sh"]).arg(volume);
    if let Err(e) = process::spawn_detached(cmd) {
        log::warn!("couldn't eject {}: {e}", volume.display());
    }
}
