//! Product icons that follow the apps: a release's `redrob-launcher.json` may name a PNG asset of
//! that release as the app's icon (`docs/release-manifest.md`), and the launcher shows it in place
//! of the icon it was built with. Fetched with the release check, kept in the cache folder so the
//! next start shows it at once, replaced only when the bytes change.

use std::path::{Path, PathBuf};

use redrob_launcher_core::release::{MAX_ICON_BYTES, Release};
use redrob_launcher_core::{CraftApp, SUITE};

use crate::http::Http;

/// PNG's eight-byte signature.
const PNG_MAGIC: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];

fn icon_file(dir: &Path, app: &CraftApp) -> PathBuf {
    dir.join(format!("{}.png", app.id))
}

/// Whether `bytes` is a PNG the UI will accept: the signature, and not empty beyond it.
fn is_png(bytes: &[u8]) -> bool {
    bytes.len() > PNG_MAGIC.len() + 16 && bytes[..8] == PNG_MAGIC
}

/// The icons saved by earlier checks, by app id.
pub fn cached(dir: &Path) -> Vec<(String, Vec<u8>)> {
    SUITE
        .iter()
        .filter_map(|app| {
            let bytes = std::fs::read(icon_file(dir, app)).ok()?;
            is_png(&bytes).then(|| (app.id.to_string(), bytes))
        })
        .collect()
}

/// Brings `app`'s cached icon in line with `release`'s manifest. Returns the icon's bytes when the
/// cache now holds a different icon than before (new, or changed), `None` when nothing changed,
/// the release names no icon, or it couldn't be fetched (which is logged, not shown: the built-in
/// icon stands in).
pub fn sync(http: &Http, dir: &Path, app: &CraftApp, release: &Release) -> Option<Vec<u8>> {
    let url = release.manifest.as_ref()?.icon_url(release)?;
    let path = icon_file(dir, app);
    let bytes = match http.get_bytes(&url, MAX_ICON_BYTES) {
        Ok(Some(b)) if is_png(&b) => b,
        Ok(Some(_)) => {
            log::warn!("{}: {} isn't a PNG; icon ignored", app.id, url);
            return None;
        }
        Ok(None) => {
            log::warn!("{}: {} isn't there; icon ignored", app.id, url);
            return None;
        }
        Err(e) => {
            log::warn!("{}: icon: {e}", app.id);
            return None;
        }
    };
    if std::fs::read(&path).is_ok_and(|old| old == bytes) {
        return None;
    }
    if let Err(e) = std::fs::create_dir_all(dir).map_err(|e| e.to_string()).and_then(|()| crate::paths::write_atomic(&path, &bytes)) {
        log::warn!("{}: couldn't keep the icon: {e}", app.id);
    }
    Some(bytes)
}
