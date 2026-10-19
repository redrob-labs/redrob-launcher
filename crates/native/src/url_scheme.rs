//! Makes this launcher the handler of `redrob://` links (`deep_link`, `docs/deep-links.md`).
//!
//! The packages declare the scheme themselves (the deb/rpm desktop entry's `MimeType`, the macOS
//! bundle's `CFBundleURLTypes`, the Windows setup's `HKCU\Software\Classes\redrob`). What they
//! can't cover, the launcher does on every start, cheaply and only when something changed:
//!
//! - Linux: a tarball install has no desktop entry, so the launcher writes one to
//!   `~/.local/share/applications` (the AppImage's `AppRun` writes its own); either way it then
//!   makes that entry the user's default `x-scheme-handler/redrob` with `xdg-mime`.
//! - Windows: the portable zip has no setup, and the setup's entry goes stale when the folder
//!   moves, so the launcher writes the same per-user keys (no admin rights) pointing at itself.
//! - macOS: nothing to do; Launch Services reads the bundle's `Info.plist`.
//!
//! Development builds never register, so `cargo run` doesn't take links away from an installed
//! launcher. Every failure is logged and otherwise ignored: links are a convenience.

/// Registers the scheme in the background (it may run `xdg-mime`).
pub fn register() {
    if cfg!(debug_assertions) {
        return;
    }
    std::thread::spawn(|| {
        if let Err(e) = register_now() {
            log::warn!("couldn't register the redrob: link handler: {e}");
        }
    });
}

#[cfg(target_os = "macos")]
fn register_now() -> Result<(), String> {
    Ok(())
}

/// The desktop entry's file name, as the packages install it.
#[cfg(all(unix, not(target_os = "macos")))]
const DESKTOP_FILE: &str = "ai.redrob.launcher.desktop";
#[cfg(all(unix, not(target_os = "macos")))]
const MIME: &str = "x-scheme-handler/redrob";

#[cfg(all(unix, not(target_os = "macos")))]
fn register_now() -> Result<(), String> {
    use std::path::Path;
    use std::process::{Command, Stdio};

    let packaged = ["/usr/share/applications", "/usr/local/share/applications"].iter().any(|d| Path::new(d).join(DESKTOP_FILE).exists());
    // A tarball install: no package and no AppImage (whose AppRun writes the entry) put one in.
    if !packaged && std::env::var_os("APPIMAGE").is_none() {
        let apps = dirs::data_dir().ok_or("no data folder")?.join("applications");
        let exe = std::env::current_exe().map_err(|e| format!("couldn't find this program ({e})"))?;
        let exe = exe.to_str().ok_or("this program's path isn't UTF-8")?;
        // Exec= is parsed by the desktop: like AppRun, skip a path that would need escaping.
        if exe.contains(['"', '`', '$', '\\', '%', '\n', '\r']) {
            return Err(format!("this program's path can't go in a desktop entry: {exe}"));
        }
        let entry = desktop_entry(exe);
        let path = apps.join(DESKTOP_FILE);
        if std::fs::read_to_string(&path).ok().as_deref() != Some(entry.as_str()) {
            crate::paths::write_atomic(&path, entry.as_bytes())?;
            let _ = Command::new("update-desktop-database").arg(&apps).stdout(Stdio::null()).stderr(Stdio::null()).status();
        }
    }
    let current = Command::new("xdg-mime").args(["query", "default", MIME]).stderr(Stdio::null()).output();
    let current = match current {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        // No xdg-utils: the desktop database (and the package's MimeType) is all there is.
        Err(_) => return Ok(()),
    };
    if current != DESKTOP_FILE {
        let status = Command::new("xdg-mime").args(["default", DESKTOP_FILE, MIME]).stdout(Stdio::null()).stderr(Stdio::null()).status();
        match status {
            Ok(s) if s.success() => log::info!("registered as the redrob: link handler"),
            Ok(s) => return Err(format!("xdg-mime default exited with {s}")),
            Err(e) => return Err(format!("couldn't run xdg-mime ({e})")),
        }
    }
    Ok(())
}

/// The tarball install's desktop entry: the packaged one, pointing at `exe`.
#[cfg(all(unix, not(target_os = "macos")))]
fn desktop_entry(exe: &str) -> String {
    format!(
        "[Desktop Entry]\nType=Application\nName=Redrob Launcher\nGenericName=App Launcher\nComment=Install, update and launch the Redrob apps\n\
         Exec=\"{exe}\" %u\nIcon=ai.redrob.launcher\nTerminal=false\nStartupNotify=true\nStartupWMClass=redrob-launcher\n\
         Categories=Utility;PackageManager;\nMimeType={MIME};\nKeywords=redrob;launcher;installer;updater;apps;\n"
    )
}

/// `HKCU\Software\Classes\redrob`, as the Windows setup writes it (`installer/windows`).
#[cfg(windows)]
const CLASS_KEY: &str = r"Software\Classes\redrob";

#[cfg(windows)]
fn register_now() -> Result<(), String> {
    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;

    let exe = std::env::current_exe().map_err(|e| format!("couldn't find this program ({e})"))?;
    let exe = exe.display().to_string();
    let command = format!("\"{exe}\" \"%1\"");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let current: Option<String> = hkcu.open_subkey(format!(r"{CLASS_KEY}\shell\open\command")).and_then(|k| k.get_value("")).ok();
    if current.as_deref() == Some(command.as_str()) {
        return Ok(());
    }
    let err = |e: std::io::Error| format!("couldn't write {CLASS_KEY} ({e})");
    let (class, _) = hkcu.create_subkey(CLASS_KEY).map_err(err)?;
    class.set_value("", &"URL:Redrob Launcher").map_err(err)?;
    class.set_value("URL Protocol", &"").map_err(err)?;
    let (icon, _) = hkcu.create_subkey(format!(r"{CLASS_KEY}\DefaultIcon")).map_err(err)?;
    icon.set_value("", &format!("\"{exe}\",0")).map_err(err)?;
    let (open, _) = hkcu.create_subkey(format!(r"{CLASS_KEY}\shell\open\command")).map_err(err)?;
    open.set_value("", &command).map_err(err)?;
    log::info!("registered as the redrob: link handler");
    Ok(())
}

#[cfg(not(any(unix, windows)))]
fn register_now() -> Result<(), String> {
    Ok(())
}
