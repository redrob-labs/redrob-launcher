//! Finding Redrob apps that are already installed, however they got there.
//!
//! - macOS: `<App>.app` in `/Applications` or `~/Applications`, read from its `Info.plist`.
//! - Windows: the Windows Installer's uninstall entries (`Redrob`), and portable
//!   copies the launcher unpacked.
//! - Linux: AppImages in `~/Applications` (where the launcher and AppImageLauncher put them),
//!   `~/AppImages` (Gear Lever) and `~/.local/bin`, Debian/RPM packages, and tarball installs.

use redrob_launcher_core::{CraftApp, InstallSource, Installed, version};

use crate::manifest::Manifest;

/// One look at the system, shared by every app's detection (package lists are read once).
#[derive(Default)]
pub struct Snapshot {
    #[allow(dead_code)]
    packages: Vec<crate::packages::Package>,
}

impl Snapshot {
    pub fn take() -> Self {
        #[cfg(target_os = "linux")]
        {
            Self { packages: linux::packages() }
        }
        #[cfg(not(target_os = "linux"))]
        {
            Self::default()
        }
    }
}

/// What's installed of `app`, best source first.
pub fn detect(app: &CraftApp, manifest: &Manifest, snapshot: &Snapshot) -> Option<Installed> {
    let _ = snapshot;
    if let Some(found) = from_manifest(app, manifest) {
        return Some(found);
    }
    #[cfg(target_os = "macos")]
    return macos::detect(app);
    #[cfg(windows)]
    return windows::detect(app);
    #[cfg(target_os = "linux")]
    return linux::detect(app, snapshot);
    #[allow(unreachable_code)]
    None
}

fn from_manifest(app: &CraftApp, manifest: &Manifest) -> Option<Installed> {
    let rec = manifest.apps.get(app.id)?;
    if !rec.path.exists() {
        return None;
    }
    // On macOS the bundle itself is the truth (the app may have updated itself).
    #[cfg(target_os = "macos")]
    if let Some(found) = macos::read_bundle(app, &rec.path) {
        return Some(found);
    }
    Some(Installed {
        version: version::parse(&rec.version),
        source: InstallSource::Launcher,
        location: rec.path.display().to_string(),
        package: None,
    })
}

/// The exe names an app's Windows builds use, likeliest first. Most are `<id>.exe`; the Electron
/// apps name theirs from their packaging config (Redrob Office's `executableName` is `redrob`,
/// Redrob Cowork keeps the product name, `Redrob Cowork.exe`); Redrob Code's installer puts
/// `redrob.exe` (and a `redrob-code.exe` copy) in `%LOCALAPPDATA%\Redrob\bin`; Redrob Browser is
/// Chromium, `chrome.exe`. The product name (`Redrob Design.exe`) is what older Tauri builds and
/// electron-builder's default produce, so it is always a candidate.
pub(crate) fn windows_exe_names(app: &CraftApp) -> Vec<String> {
    let own = format!("{}.exe", app.id);
    let named = format!("{}.exe", app.name);
    let mut names = match app.id {
        "redrob-code" => vec!["redrob.exe".to_string(), own],
        "redrob-office" => vec!["redrob.exe".to_string(), own],
        "redrob-browser" => vec!["chrome.exe".to_string(), own],
        _ => vec![own],
    };
    names.push(named);
    names
}

/// Whether a Settings > Apps `DisplayName` is `app`'s. Our own setups, the WiX MSIs and Tauri's
/// installers write the product name as is; electron-builder's NSIS installers append the version
/// (`Redrob Office 0.10.9`), which is why Office and Cowork showed an Install button while
/// installed (0.4.0); Redrob Browser's entry is Chromium's, called just "Redrob". Case aside.
#[cfg_attr(not(windows), allow(dead_code))]
pub(crate) fn display_name_matches(app: &CraftApp, name: &str) -> bool {
    let name = name.trim();
    if name.eq_ignore_ascii_case(app.name) || (app.id == "redrob-browser" && name.eq_ignore_ascii_case("Redrob")) {
        return true;
    }
    let Some(rest) = name.get(..app.name.len()).filter(|head| head.eq_ignore_ascii_case(app.name)).and(name.get(app.name.len()..)) else {
        return false;
    };
    // `<name> <version>`: a space, then something that starts like a version number.
    rest.strip_prefix(' ').is_some_and(|v| v.trim_start_matches('v').starts_with(|c: char| c.is_ascii_digit()))
}

/// Whether a file name is an AppImage of the app with asset stems `stems`, and the version its
/// name gives. Matches only the names a release AppImage ends up with: the release's own
/// (`redrob-canvas-0.6.0-linux-x86_64.AppImage`) and the bare app name Gear Lever renames it to
/// (`redrob-canvas.appimage`, version unknown). The extension's case doesn't matter.
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub(crate) fn appimage_version(name: &str, stems: &[&str]) -> Option<Option<String>> {
    let split = name.len().checked_sub(".appimage".len())?;
    let (base, ext) = (name.get(..split)?, name.get(split..)?);
    if !ext.eq_ignore_ascii_case(".appimage") {
        return None;
    }
    if stems.iter().any(|s| base.eq_ignore_ascii_case(s)) {
        return Some(None);
    }
    let parsed = redrob_launcher_core::platform::parse_asset_name(&format!("{base}.AppImage"), stems)?;
    (parsed.kind == redrob_launcher_core::PackageKind::AppImage && parsed.token.starts_with("linux-")).then_some(Some(parsed.version))
}

#[cfg(target_os = "macos")]
pub mod macos {
    use std::path::{Path, PathBuf};

    use super::*;

    /// Where a `.app` might be: the system and the user's Applications folders.
    pub fn candidates(app: &CraftApp) -> Vec<PathBuf> {
        let bundle = format!("{}.app", app.name);
        let mut out = vec![Path::new("/Applications").join(&bundle)];
        if let Some(home) = dirs::home_dir() {
            out.push(home.join("Applications").join(&bundle));
        }
        out
    }

    pub fn detect(app: &CraftApp) -> Option<Installed> {
        candidates(app).iter().find_map(|p| read_bundle(app, p))
    }

    /// Reads a bundle's `Info.plist`; `None` unless it's really this app.
    pub fn read_bundle(app: &CraftApp, path: &Path) -> Option<Installed> {
        let plist = plist::Value::from_file(path.join("Contents/Info.plist")).ok()?;
        let dict = plist.as_dictionary()?;
        if dict.get("CFBundleIdentifier")?.as_string()? != app.mac_bundle_id() {
            return None;
        }
        // `<App>Version` carries the full version including a pre-release suffix;
        // CFBundleShortVersionString only has the numbers.
        let full_key = format!("{}Version", app.name);
        let text =
            [full_key.as_str(), "CFBundleShortVersionString", "CFBundleVersion"].iter().find_map(|k| dict.get(k).and_then(plist::Value::as_string));
        Some(Installed { version: text.and_then(version::parse), source: InstallSource::MacApp, location: path.display().to_string(), package: None })
    }
}

#[cfg(windows)]
pub mod windows {
    use std::path::{Path, PathBuf};

    use winreg::RegKey;
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY};

    use super::*;

    const UNINSTALL: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall";

    /// Whether a Settings > Apps `Publisher` is one of ours. Our setups say "Redrob"; the MSIs
    /// (WiX) the company that signs them, "McKinley Rice, Inc."; electron-builder writes the
    /// package author (Redrob Office: "Redrob", Redrob Cowork: "Redrob Cowork"); Tauri's NSIS
    /// and WiX write the identifier's second segment, "redrob". Matched by prefix, case aside, so
    /// a spelling change ("McKinley Rice Inc.") doesn't hide an installed app behind an Install
    /// button again (Film in 0.3.0, Office / Cowork in 0.4.0).
    fn ours(publisher: &str) -> bool {
        let p = publisher.trim().to_ascii_lowercase();
        p.starts_with("redrob") || p.starts_with("mckinley rice")
    }

    /// An install registered in Settings > Apps: a Windows Installer product (`product_code` is its
    /// `{GUID}`) or an app's own setup (`product_code` is its key name, `uninstall_string` its
    /// uninstaller command).
    pub struct MsiInstall {
        pub product_code: String,
        pub version: Option<redrob_launcher_core::version::Version>,
        pub exe: Option<PathBuf>,
        pub uninstall_string: Option<String>,
    }

    /// The hives and views Settings > Apps entries live in: per-machine installs in the 64-bit and
    /// 32-bit views of HKLM (WiX MSIs, electron-builder's `perMachine`), per-user installs in
    /// HKCU (our setups, electron-builder's default, Tauri's `currentUser` NSIS mode).
    const HIVES: [(winreg::HKEY, u32); 4] =
        [(HKEY_LOCAL_MACHINE, KEY_WOW64_64KEY), (HKEY_LOCAL_MACHINE, KEY_WOW64_32KEY), (HKEY_CURRENT_USER, 0), (HKEY_CURRENT_USER, KEY_WOW64_32KEY)];

    /// Every Settings > Apps entry for `app`, newest first.
    pub fn msi_installs(app: &CraftApp) -> Vec<MsiInstall> {
        let mut out = Vec::new();
        for (hive, flags) in HIVES {
            let Ok(root) = RegKey::predef(hive).open_subkey_with_flags(UNINSTALL, KEY_READ | flags) else { continue };
            for code in root.enum_keys().flatten() {
                let Ok(key) = root.open_subkey_with_flags(&code, KEY_READ | flags) else { continue };
                let name: String = key.get_value("DisplayName").unwrap_or_default();
                let publisher: String = key.get_value("Publisher").unwrap_or_default();
                if !is_display_name(app, &name) || !ours(&publisher) || out.iter().any(|m: &MsiInstall| m.product_code == code) {
                    continue;
                }
                let version = key.get_value::<String, _>("DisplayVersion").ok().and_then(|v| version::parse(&v));
                let text = |k: &str| key.get_value::<String, _>(k).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
                // Where the files are: `InstallLocation` (quoted by Tauri), else the uninstaller's
                // folder (electron-builder records the folder elsewhere but puts `Uninstall <app>.exe`
                // in it), and `DisplayIcon` (`<exe>,0`) names the exe itself.
                let uninstall_string = text("UninstallString");
                let location = text("InstallLocation")
                    .map(|l| PathBuf::from(l.trim_matches('"')))
                    .or_else(|| uninstall_string.as_deref().and_then(command_path).and_then(|p| p.parent().map(Path::to_path_buf)));
                let icon = text("DisplayIcon").and_then(|i| icon_exe(app, &i));
                let exe = icon.or_else(|| exe_path(app, location));
                out.push(MsiInstall { product_code: code, version, exe, uninstall_string });
            }
        }
        out.sort_by(|a, b| match (&b.version, &a.version) {
            (Some(x), Some(y)) => version::compare(x, y),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });
        out
    }

    /// Whether a Settings > Apps `DisplayName` is this app's. Our own setups, the MSIs and Tauri
    /// use the product name as is; electron-builder appends the version ("Redrob Office 0.10.9");
    /// Redrob Browser's entry is Chromium's, called just "Redrob". Case aside.
    fn is_display_name(app: &CraftApp, name: &str) -> bool {
        super::display_name_matches(app, name)
    }

    /// The program a command line starts: its first token, quoted or not.
    fn command_path(command: &str) -> Option<PathBuf> {
        let c = command.trim();
        let path = if let Some(rest) = c.strip_prefix('"') { rest.split('"').next()? } else { c.split_whitespace().next()? };
        (!path.is_empty()).then(|| PathBuf::from(path))
    }

    /// The exe a `DisplayIcon` value points at (`"C:\...\redrob.exe",0`), when it is this app's exe
    /// and is there.
    fn icon_exe(app: &CraftApp, icon: &str) -> Option<PathBuf> {
        let mut s = icon.trim();
        if let Some(rest) = s.strip_prefix('"') {
            s = rest.split('"').next()?;
        } else if let Some((path, _index)) = s.rsplit_once(',') {
            s = path;
        }
        let p = PathBuf::from(s.trim());
        let name = p.file_name()?.to_str()?;
        let ours = windows_exe_names(app).iter().any(|n| n.eq_ignore_ascii_case(name));
        (ours && p.is_file()).then_some(p)
    }

    /// Folders an app is known to install into besides the recorded one and Program Files:
    /// `%LOCALAPPDATA%\Programs\<name>` (electron-builder's assisted per-user installer: Office,
    /// Cowork), `%LOCALAPPDATA%\<name>` (Tauri's `currentUser` NSIS), our own setups'
    /// folders (`%LOCALAPPDATA%\Redrob\<product>`), and the launcher's own apps folder.
    fn known_dirs(app: &CraftApp) -> Vec<PathBuf> {
        let local = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
        let home = std::env::var_os("USERPROFILE").map(PathBuf::from);
        let mut dirs: Vec<PathBuf> = match app.id {
            "redrob-code" => [local.as_ref().map(|l| l.join(r"Redrob\bin")), home.map(|h| h.join(r".redrob\bin"))].into_iter().flatten().collect(),
            "redrob-browser" => local.as_ref().map(|l| l.join(r"Redrob\Application")).into_iter().collect(),
            _ => Vec::new(),
        };
        if let Some(l) = &local {
            let short = app.name.strip_prefix("Redrob ").unwrap_or(app.name);
            dirs.push(l.join("Programs").join(app.name));
            dirs.push(l.join(app.name));
            dirs.push(l.join("Redrob").join(short));
            dirs.push(l.join("Redrob Launcher").join("apps").join(app.id));
        }
        dirs
    }

    /// The app's exe: from `App Paths`, the install folder, its known folders, or Program Files.
    fn exe_path(app: &CraftApp, location: Option<PathBuf>) -> Option<PathBuf> {
        let names = windows_exe_names(app);
        for exe in &names {
            let app_paths = format!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\{exe}");
            for (hive, flags) in HIVES {
                if let Ok(key) = RegKey::predef(hive).open_subkey_with_flags(&app_paths, KEY_READ | flags)
                    && let Ok(path) = key.get_value::<String, _>("")
                {
                    let p = PathBuf::from(path.trim_matches('"'));
                    // App Paths for a shared name (chrome.exe, redrob.exe) may be another app's: it
                    // must be in one of this app's folders.
                    let ours = *exe == format!("{}.exe", app.id) || known_dirs(app).iter().any(|d| p.starts_with(d));
                    if ours && p.is_file() {
                        return Some(p);
                    }
                }
            }
        }
        let mut dirs: Vec<PathBuf> = location.into_iter().collect();
        dirs.extend(known_dirs(app));
        for var in ["ProgramW6432", "ProgramFiles", "ProgramFiles(x86)"] {
            if let Some(pf) = std::env::var_os(var) {
                dirs.push(PathBuf::from(pf).join(app.name));
            }
        }
        // At the folder's top (Film's MSI) or under bin\ (Canvas's setup lays its zip out that way).
        dirs.into_iter()
            .flat_map(|d| names.iter().flat_map(move |exe| [d.join(exe), d.join("bin").join(exe)]).collect::<Vec<_>>())
            .find(|p| p.is_file())
    }

    /// The exe to start for an installed app whose recorded location is stale, a folder, or a
    /// name without a path: looked up again the way detection does.
    pub fn resolve_exe(app: &CraftApp, location: &Path) -> Option<PathBuf> {
        if location.is_file() {
            return Some(location.to_path_buf());
        }
        let dir = location.is_dir().then(|| location.to_path_buf());
        msi_installs(app).into_iter().find_map(|m| m.exe.filter(|e| e.is_file())).or_else(|| exe_path(app, dir))
    }

    pub fn detect(app: &CraftApp) -> Option<Installed> {
        if let Some(m) = msi_installs(app).into_iter().next() {
            return Some(Installed {
                version: m.version,
                source: InstallSource::Msi,
                location: m.exe.map_or_else(|| app.name.to_string(), |p| p.display().to_string()),
                package: Some(m.product_code),
            });
        }
        // No Settings > Apps entry, but the exe is in one of the app's known folders (Redrob Code put
        // there by its install script, for one): installed, version unknown, not ours to remove.
        let exe = exe_path(app, None)?;
        Some(Installed { version: None, source: InstallSource::Binary, location: exe.display().to_string(), package: None })
    }
}

#[cfg(target_os = "linux")]
pub mod linux {
    use std::path::{Path, PathBuf};
    use std::time::Duration;

    use super::*;
    use crate::packages::{self, Package};
    use crate::process::capture;

    /// Installed Redrob app packages from every package manager on this system (dpkg, rpm,
    /// pacman). Read once per detection pass.
    pub fn packages() -> Vec<Package> {
        let ours = |name: &str| {
            redrob_launcher_core::SUITE.iter().any(|a| {
                [InstallSource::Deb, InstallSource::Rpm, InstallSource::Pacman]
                    .iter()
                    .any(|m| crate::packages::candidate_names(a, *m).iter().any(|n| n == name))
            })
        };
        let t = Duration::from_secs(10);
        let mut out = Vec::new();
        let mut add = |manager, list: Vec<(String, String)>| {
            out.extend(list.into_iter().filter(|(n, _)| ours(n)).map(|(name, version)| Package { manager, name, version }));
        };
        if let Some(list) = capture("dpkg-query", &["-W", "-f", "${Package}|${db:Status-Status}|${Version}\n"], t) {
            add(InstallSource::Deb, packages::parse_dpkg(&list));
        }
        if let Some(list) = capture("rpm", &["-qa", "--qf", "%{NAME}|%{VERSION}\n"], t) {
            add(InstallSource::Rpm, packages::parse_rpm(&list));
        }
        if let Some(list) = capture("pacman", &["-Q"], t) {
            add(InstallSource::Pacman, packages::parse_pacman(&list));
        }
        out
    }

    /// Where a package put the app's binary: on `PATH`, else `/usr/bin/<id>`.
    fn binary_on_path(id: &str) -> String {
        std::env::var_os("PATH")
            .and_then(|path| std::env::split_paths(&path).map(|d| d.join(id)).find(|p| p.is_file()))
            .unwrap_or_else(|| PathBuf::from("/usr/bin").join(id))
            .display()
            .to_string()
    }

    /// `~/Applications`, where the launcher puts AppImages (the AppImageLauncher convention).
    pub fn appimage_dir() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join("Applications"))
    }

    /// Every folder searched for AppImages, the launcher's own first: `~/Applications`,
    /// `~/AppImages` (Gear Lever's default) and `~/.local/bin`.
    pub fn appimage_dirs() -> Vec<PathBuf> {
        let Some(home) = dirs::home_dir() else { return Vec::new() };
        vec![home.join("Applications"), home.join("AppImages"), home.join(".local").join("bin")]
    }

    /// AppImages of `app` in `dirs`, newest first (unknown versions last; earlier folders win
    /// ties).
    pub fn appimages_in(app: &CraftApp, dirs: &[PathBuf]) -> Vec<(PathBuf, Option<version::Version>)> {
        let stems: Vec<&str> = app.stems().collect();
        let mut out: Vec<(PathBuf, Option<version::Version>)> = Vec::new();
        for dir in dirs {
            let Ok(entries) = std::fs::read_dir(dir) else { continue };
            let mut found: Vec<(PathBuf, Option<version::Version>)> = entries
                .flatten()
                .filter_map(|e| {
                    let name = e.file_name().into_string().ok()?;
                    let version = appimage_version(&name, &stems)?;
                    let path = e.path();
                    path.is_file().then(|| (path, version.as_deref().and_then(version::parse)))
                })
                .collect();
            found.sort_by(|a, b| a.0.cmp(&b.0));
            out.extend(found);
        }
        // Stable sort: equal versions keep the folder order above.
        out.sort_by(|a, b| match (&b.1, &a.1) {
            (Some(x), Some(y)) => version::compare(x, y),
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (None, None) => std::cmp::Ordering::Equal,
        });
        out
    }

    /// AppImages of `app` in any of the usual folders, newest first.
    pub fn appimages(app: &CraftApp) -> Vec<(PathBuf, Option<version::Version>)> {
        appimages_in(app, &appimage_dirs())
    }

    /// AppImages of `app` in the launcher's own folder only (the ones an update replaces).
    pub fn launcher_appimages(app: &CraftApp) -> Vec<PathBuf> {
        appimage_dir().map(|d| appimages_in(app, &[d]).into_iter().map(|(p, _)| p).collect()).unwrap_or_default()
    }

    pub fn detect(app: &CraftApp, snapshot: &Snapshot) -> Option<Installed> {
        if let Some((path, version)) = appimages(app).into_iter().next() {
            return Some(Installed { version, source: InstallSource::AppImage, location: path.display().to_string(), package: None });
        }
        let order = [InstallSource::Deb, InstallSource::Rpm, InstallSource::Pacman];
        if let Some(p) = packages::find(app, &snapshot.packages, &order) {
            let location = binary_on_path(app.id);
            return Some(Installed { version: packages::version_of(p), source: p.manager, location, package: Some(p.name.clone()) });
        }
        detect_system_install(app)
    }

    /// When the package managers don't report the app (a query timed out or was locked, an
    /// unlisted package name, or a sandbox that can't see them) but its binary is there. It is
    /// reported as a system binary: the launcher can open it but leaves updating and removing it
    /// to whatever installed it, since it can't tell which package manager that was.
    fn detect_system_install(app: &CraftApp) -> Option<Installed> {
        let bin = find_binary(app.id)?;
        Some(Installed { version: binary_version(&bin, app.id), source: InstallSource::Binary, location: bin.display().to_string(), package: None })
    }

    fn find_binary(id: &str) -> Option<PathBuf> {
        let on_path = std::env::var_os("PATH").and_then(|path| std::env::split_paths(&path).map(|d| d.join(id)).find(|p| p.is_file()));
        let usual = ["/usr/bin", "/usr/local/bin"].into_iter().map(PathBuf::from).chain(dirs::home_dir().map(|h| h.join(".local/bin")));
        on_path.or_else(|| usual.map(|d| d.join(id)).find(|p| p.is_file()))
    }

    pub(crate) fn binary_version(bin: &Path, id: &str) -> Option<version::Version> {
        let out = crate::process::capture(bin.to_str()?, &["--version"], Duration::from_secs(5))?;
        let rest = out.trim().strip_prefix(id)?.trim_start();
        version::parse(rest.split_whitespace().next()?)
    }
}

#[cfg(all(test, target_os = "linux"))]
mod linux_tests {
    use super::*;

    #[test]
    fn ours_filter_matches_rpm_packages() {
        let ours = |name: &str| {
            redrob_launcher_core::SUITE.iter().any(|a| {
                [InstallSource::Deb, InstallSource::Rpm, InstallSource::Pacman]
                    .iter()
                    .any(|m| crate::packages::candidate_names(a, *m).iter().any(|n| n == name))
            })
        };
        assert!(ours("redrob-canvas"));
        assert!(!ours("unrelated-package"));
    }

    #[test]
    fn reads_binary_version_from_cli() {
        let photo = CraftApp::by_id("redrob-canvas").unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let bin = temp_dir.path().join("redrob-canvas");
        std::fs::write(&bin, "#!/bin/sh\necho 'redrob-canvas 0.6.0 (0c72d9542, 2026-10-10)'\n").unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&bin, std::fs::Permissions::from_mode(0o755)).unwrap();
        let ver = linux::binary_version(&bin, photo.id);
        assert_eq!(ver.unwrap().to_string(), "0.6.0");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_appimage_names_conservatively() {
        let pdf = ["redrob-office", "redrob-docs"];
        let v = |n: &str| appimage_version(n, &pdf);
        assert_eq!(v("redrob-office-0.3.0-linux-x86_64.AppImage"), Some(Some("0.3.0".into())));
        assert_eq!(v("redrob-docs-0.2.1-linux-aarch64.appimage"), Some(Some("0.2.1".into())));
        assert_eq!(v("redrob-office-0.3.0-rc.1-linux-x86_64.AppImage"), Some(Some("0.3.0-rc.1".into())));
        // Gear Lever's rename, and a plain rename by hand.
        assert_eq!(v("redrob-office.appimage"), Some(None));
        assert_eq!(v("Redrob-Office.AppImage"), Some(None));
        for other in [
            "redrob-office-0.3.0-linux-x86_64.AppImage.zsync",
            "redrob-office-0.3.0-linux-x86_64.deb",
            "redrob-office-0.3.0-windows-x64.AppImage",
            "redrob-office-cli-0.3.0-linux-x86_64.AppImage",
            "redrob-canvas-0.6.0-linux-x86_64.AppImage",
            "mypdfcraft.AppImage",
            "redrob-office-latest.AppImage",
            ".appimage",
            "x",
        ] {
            assert_eq!(v(other), None, "{other}");
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn finds_appimages_in_every_folder_newest_first() {
        let app = CraftApp::by_id("redrob-canvas").unwrap();
        let root = tempfile::tempdir().unwrap();
        let (apps, gear, bin) = (root.path().join("Applications"), root.path().join("AppImages"), root.path().join("bin"));
        for d in [&apps, &gear, &bin] {
            std::fs::create_dir_all(d).unwrap();
        }
        std::fs::write(apps.join("redrob-canvas-0.5.0-linux-x86_64.AppImage"), b"").unwrap();
        std::fs::write(apps.join("redrob-code-0.3.0-linux-x86_64.AppImage"), b"").unwrap();
        std::fs::write(gear.join("redrob-canvas.appimage"), b"").unwrap();
        std::fs::write(bin.join("redrob-canvas-0.6.0-linux-x86_64.AppImage"), b"").unwrap();
        std::fs::create_dir(bin.join("redrob-canvas.AppImage")).unwrap();
        let found = linux::appimages_in(app, &[apps.clone(), gear.clone(), bin.clone(), root.path().join("missing")]);
        let names: Vec<_> = found.iter().map(|(p, v)| (p.clone(), v.as_ref().map(ToString::to_string))).collect();
        assert_eq!(
            names,
            [
                (bin.join("redrob-canvas-0.6.0-linux-x86_64.AppImage"), Some("0.6.0".to_string())),
                (apps.join("redrob-canvas-0.5.0-linux-x86_64.AppImage"), Some("0.5.0".to_string())),
                (gear.join("redrob-canvas.appimage"), None),
            ]
        );
    }
}
