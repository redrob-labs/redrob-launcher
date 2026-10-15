//! The shared language file every Redrob app reads (`docs/shared-language.md`).
//!
//! The launcher is the one writer. It writes the language the person chose in the installer (the
//! Windows setup wizard asks on its first page and records the answer under HKCU) or in the
//! launcher's own settings, plus any per-app overrides, to `language.json` next to the shared
//! `account.json`. An app reads it at startup and draws in that language; no file means "follow
//! the system".

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::account::shared_dir;

/// The shared language file's name, in [`shared_dir`].
pub const LANGUAGE_FILE: &str = "language.json";

/// What the launcher publishes for the other apps.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SharedLanguage {
    /// The language for every Redrob app, as a lowercase code from the launcher's registry
    /// (`en`, `ko`, ...).
    pub language: String,
    /// `true` when a person chose it (installer or settings); `false` when it is what the
    /// launcher worked out from the system, so an app with its own detection may prefer that.
    pub chosen: bool,
    /// Per-app overrides, app id (`redrob-canvas`) to language code. An app looks itself up here
    /// first, then falls back to `language`.
    pub apps: BTreeMap<String, String>,
}

/// Where the file is: `REDROB_SHARED_DIR`, or the platform's shared Redrob directory.
pub fn path() -> Option<PathBuf> {
    shared_dir().map(|d| d.join(LANGUAGE_FILE))
}

pub fn load(path: &Path) -> Option<SharedLanguage> {
    serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

/// Writes the file through a temporary file renamed over the old one, so a reader never sees half
/// of it. Unchanged content is not rewritten.
pub fn save(path: &Path, lang: &SharedLanguage) -> std::io::Result<()> {
    if load(path).as_ref() == Some(lang) {
        return Ok(());
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_string_pretty(lang).unwrap_or_default())?;
    std::fs::rename(&tmp, path)
}

/// The language the installer recorded, if it asked. Windows: the setup wizard's first page
/// writes `HKCU\Software\Redrob\Redrob Launcher\Language` (`en`, `ko`). The macOS disk image and
/// the Linux packages have no wizard, so there the answer is `None` and the launcher follows the
/// system until the person picks a language in its settings.
#[cfg(windows)]
pub fn installer_language() -> Option<String> {
    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;
    let key = RegKey::predef(HKEY_CURRENT_USER).open_subkey(r"Software\Redrob\Redrob Launcher").ok()?;
    let code: String = key.get_value("Language").ok()?;
    let code = code.trim().to_ascii_lowercase();
    if code.is_empty() { None } else { Some(code) }
}

#[cfg(not(windows))]
pub fn installer_language() -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_and_skips_unchanged_writes() {
        let dir = std::env::temp_dir().join(format!("redrob-lang-{}", std::process::id()));
        let path = dir.join("language.json");
        let lang = SharedLanguage { language: "ko".into(), chosen: true, apps: [("redrob-canvas".to_string(), "en".to_string())].into() };
        save(&path, &lang).unwrap();
        let first = std::fs::metadata(&path).unwrap().modified().unwrap();
        assert_eq!(load(&path), Some(lang.clone()));
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("\"language\": \"ko\"") && text.contains("\"chosen\": true") && text.contains("\"redrob-canvas\": \"en\""), "{text}");
        std::thread::sleep(std::time::Duration::from_millis(20));
        save(&path, &lang).unwrap();
        assert_eq!(std::fs::metadata(&path).unwrap().modified().unwrap(), first, "unchanged content was rewritten");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_fields_default() {
        let l: SharedLanguage = serde_json::from_str(r#"{"language":"en"}"#).unwrap();
        assert_eq!(l, SharedLanguage { language: "en".into(), chosen: false, apps: BTreeMap::new() });
    }
}
