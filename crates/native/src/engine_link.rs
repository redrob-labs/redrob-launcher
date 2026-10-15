//! Making a launcher-installed Redrob Code the engine every app finds.
//!
//! The engine client (`redrob-engine-client`, used by the launcher and every Rust app) and the
//! engine's own installer agree on one place: `~/.redrob/bin/redrob`. The launcher unpacks the
//! Redrob Code tarball under its own apps folder (`.../redrob-launcher/apps/redrob-code/bin/redrob`),
//! where nothing looks. Without this link the launcher's install -- and its update -- would leave
//! `~/.redrob/bin/redrob` on whatever version the installer last put there, and the daemon would
//! keep starting the old engine. So after a tree install the launcher points `~/.redrob/bin/redrob`
//! at the binary it installed (a symlink, replaced atomically), and on removal takes the link away
//! again when it still points into the launcher's tree.

use std::path::{Path, PathBuf};

/// The engine binary's name inside the Redrob Code release tree.
const ENGINE_BINARY: &str = "redrob";

/// `~/.redrob/bin/redrob`: where the engine client and the engine's installer look.
pub fn engine_path() -> Option<PathBuf> {
    Some(dirs::home_dir()?.join(".redrob").join("bin").join(ENGINE_BINARY))
}

/// The engine binary beside a tree install's `bin/<app>` (`bin/redrob-code` -> `bin/redrob`).
pub fn engine_in_tree(app_bin: &Path) -> Option<PathBuf> {
    let bin = app_bin.parent()?.join(ENGINE_BINARY);
    bin.is_file().then_some(bin)
}

/// Points `link` (normally [`engine_path`]) at `target`, replacing whatever was there. A regular
/// file from the engine's own installer is replaced too: once the launcher installs the engine,
/// the launcher keeps it current. Returns the link written, for the install record.
#[cfg(unix)]
pub fn expose(link: &Path, target: &Path) -> Result<PathBuf, String> {
    let dir = link.parent().ok_or("the engine link has no folder")?;
    std::fs::create_dir_all(dir).map_err(|e| format!("couldn't create {} ({e})", dir.display()))?;
    let staged = dir.join(format!(".{ENGINE_BINARY}.launcher-link"));
    let _ = std::fs::remove_file(&staged);
    std::os::unix::fs::symlink(target, &staged).map_err(|e| format!("couldn't link {} ({e})", staged.display()))?;
    std::fs::rename(&staged, link).map_err(|e| {
        let _ = std::fs::remove_file(&staged);
        format!("couldn't replace {} ({e})", link.display())
    })?;
    Ok(link.to_path_buf())
}

#[cfg(not(unix))]
pub fn expose(_link: &Path, _target: &Path) -> Result<PathBuf, String> {
    Err("the engine link is only written on Unix".into())
}

/// Removes `link` when it is a symlink into `tree` (the launcher's own); leaves anything else.
pub fn withdraw(link: &Path, tree: &Path) {
    let Ok(target) = std::fs::read_link(link) else { return };
    if target.starts_with(tree) {
        let _ = std::fs::remove_file(link);
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn the_link_points_at_the_installed_engine_and_replaces_an_installer_copy() {
        let dir = tempfile::tempdir().unwrap();
        let tree = dir.path().join("apps/redrob-code");
        std::fs::create_dir_all(tree.join("bin")).unwrap();
        std::fs::write(tree.join("bin/redrob-code"), "cli").unwrap();
        std::fs::write(tree.join("bin/redrob"), "engine 0.6.5").unwrap();
        let link = dir.path().join(".redrob/bin/redrob");
        std::fs::create_dir_all(link.parent().unwrap()).unwrap();
        std::fs::write(&link, "engine 0.6.4 from the installer").unwrap();

        let engine = engine_in_tree(&tree.join("bin/redrob-code")).unwrap();
        assert_eq!(engine, tree.join("bin/redrob"));
        assert_eq!(expose(&link, &engine).unwrap(), link);
        assert_eq!(std::fs::read_link(&link).unwrap(), engine);
        assert_eq!(std::fs::read_to_string(&link).unwrap(), "engine 0.6.5");

        // A second install (an update) swaps the link in place.
        let tree2 = dir.path().join("apps2/redrob-code");
        std::fs::create_dir_all(tree2.join("bin")).unwrap();
        std::fs::write(tree2.join("bin/redrob"), "engine 0.6.6").unwrap();
        expose(&link, &tree2.join("bin/redrob")).unwrap();
        assert_eq!(std::fs::read_to_string(&link).unwrap(), "engine 0.6.6");
        assert!(!link.parent().unwrap().join(".redrob.launcher-link").exists());

        // Removal takes only the launcher's own link away.
        withdraw(&link, &tree);
        assert!(link.exists(), "a link into another tree is not ours");
        withdraw(&link, &tree2);
        assert!(!link.exists());
    }

    #[test]
    fn a_tree_without_an_engine_binary_is_not_linked() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("bin")).unwrap();
        std::fs::write(dir.path().join("bin/redrob-cad"), "x").unwrap();
        assert_eq!(engine_in_tree(&dir.path().join("bin/redrob-cad")), None);
    }

    #[test]
    fn withdraw_leaves_a_real_file_alone() {
        let dir = tempfile::tempdir().unwrap();
        let link = dir.path().join("redrob");
        std::fs::write(&link, "installer copy").unwrap();
        withdraw(&link, dir.path());
        assert!(link.exists());
    }
}
