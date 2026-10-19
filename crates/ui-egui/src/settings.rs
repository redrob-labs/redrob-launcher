//! The user's choices, saved between runs by the shell.

use std::collections::BTreeMap;

use redrob_launcher_core::PackageKind;
use serde::{Deserialize, Serialize};

use crate::i18n::t;
use crate::theme::ThemeChoice;

/// The grid or the table. Kept so saved settings still load: the launcher shows the grid only and
/// ignores it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewMode {
    #[default]
    Grid,
    Table,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    #[default]
    All,
    Installed,
    Updates,
    NotInstalled,
}

/// The order of the apps. Kept so saved settings still load: the grid is always in suite order and
/// ignores it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortKey {
    /// Suite order: classic apps, then new.
    #[default]
    Suite,
    Name,
    /// The product an app is inspired by.
    InspiredBy,
    Status,
    /// Installed version.
    Installed,
    /// Latest released version.
    Latest,
    /// Release date.
    Released,
}

/// How often to look for updates in the background.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckEvery {
    Hour,
    FourHours,
    Day,
    Never,
}

impl CheckEvery {
    /// The choices offered. `FourHours` and `Day` are kept so saved settings still load; they now
    /// mean "automatically" too.
    pub const ALL: [Self; 2] = [Self::Hour, Self::Never];

    pub fn label(self) -> String {
        match self {
            Self::Hour | Self::FourHours | Self::Day => t("Automatically").to_string(),
            Self::Never => t("Only when I ask").to_string(),
        }
    }

    /// How often to look. Checks ask the release feed (one cheap request; GitHub only as a
    /// fallback), so automatic checks run every five minutes.
    pub fn seconds(self) -> Option<i64> {
        match self {
            Self::Hour | Self::FourHours | Self::Day => Some(5 * 60),
            Self::Never => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Kept for compatibility, ignored: the grid is the only view.
    pub view: ViewMode,
    pub filter: Filter,
    /// Kept for compatibility, ignored: the grid is always in suite order.
    pub sort: SortKey,
    /// Kept for compatibility, ignored.
    pub sort_descending: bool,
    pub theme: ThemeChoice,
    /// Interface language: `auto` (follow the system) or a language code from the registry
    /// (`en`, `ru`, `zh-CN`, `de`).
    pub language: String,
    /// Per-app interface language overrides, app id to language code; an app not listed follows
    /// `language`. Published for the apps in the shared language file.
    #[serde(default)]
    pub app_languages: BTreeMap<String, String>,
    /// Closing the window keeps the launcher in the tray (menu bar) instead of quitting.
    pub close_to_tray: bool,
    /// macOS: showing the window brings it to the current desktop (Space) instead of switching to
    /// the desktop it was on.
    pub show_on_current_desktop: bool,
    pub check_every: CheckEvery,
    /// Automatically download and install updates when found.
    #[serde(default)]
    pub auto_update: bool,
    /// Keep the shared Redrob Code engine on its newest release by itself (when no app is using
    /// it; "Update now" in the engine dialog overrides). On by default: the engine is shared
    /// infrastructure, and a fix there is a fix for every app.
    #[serde(default = "default_true")]
    pub engine_auto_update: bool,
    /// Show a system notification when an installed app has an update.
    pub notify: bool,
    pub prereleases: bool,
    /// Show models that need PrismML's llama.cpp fork (ternary packs such as Bonsai 2). Off by
    /// default: the fork is a second runtime to download and keep current.
    #[serde(default)]
    pub prism_runtime: bool,
    /// Models page: hide what this computer cannot run.
    #[serde(default)]
    pub models_only_runnable: bool,
    /// Linux: the package format to prefer for new installs (empty: the system's own).
    pub preferred_kinds: Vec<PackageKind>,
    /// Announcements and alerts from the release feed the user dismissed (by notice id).
    pub dismissed_notices: std::collections::BTreeSet<String>,
    /// Updates already announced, so each version is announced once: app id → version.
    pub notified: BTreeMap<String, String>,
    /// Unix seconds of the last finished check.
    pub last_check: Option<i64>,
}

impl Settings {
    /// Machine-readable code for "follow the system language".
    pub const LANGUAGE_AUTO: &'static str = "auto";

    /// Display name for a language `code`: a language's name in itself, from the registry
    /// ([`LANGUAGES`](crate::i18n::LANGUAGES)). A language needing the system's Chinese font is
    /// named in English, with its own name added only once that font is loaded (labels never load
    /// it: choosing the language does); [`Settings::system_language_label`] for `auto` and
    /// unknown codes, like [`Lang::from_pref`](crate::i18n::Lang::from_pref).
    pub fn language_label(code: &str) -> String {
        match crate::i18n::Lang::from_code(code) {
            Some(lang) if lang.needs_cjk_font() && crate::theme::cjk_font_loaded() => format!("{} ({})", lang.english_name(), lang.name()),
            Some(lang) if lang.needs_cjk_font() => lang.english_name().to_string(),
            Some(lang) => lang.name().to_string(),
            None => Self::system_language_label(),
        }
    }

    /// "Match system (한국어)": the `auto` choice, naming the language the system resolves to so
    /// the person sees what "system" means on this computer.
    pub fn system_language_label() -> String {
        let system = crate::i18n::system_lang();
        crate::i18n::fmt(t("Match system ({language})"), &[("language", system.name())])
    }

    /// The launcher's language setting on its first run, from the language the Windows setup
    /// recorded. When that is the system language the person only accepted the setup's default,
    /// so the launcher keeps following the system (`auto`, published with `chosen: false`) and a
    /// later change of the system language reaches it. A different language was picked on
    /// purpose and is kept.
    pub fn language_from_installer(code: &str, system: crate::i18n::Lang) -> String {
        match crate::i18n::Lang::from_code(code) {
            Some(lang) if lang != system => lang.code().to_string(),
            _ => Self::LANGUAGE_AUTO.to_string(),
        }
    }
}

/// Whether this process was launched from an AppImage.
pub fn is_appimage() -> bool {
    #[cfg(target_os = "linux")]
    {
        std::env::var_os("APPIMAGE").is_some()
    }
    #[cfg(not(target_os = "linux"))]
    false
}

fn default_true() -> bool {
    true
}

/// Default package kinds preferred on Linux based on how the launcher itself was installed.
pub fn default_preferred_kinds() -> Vec<PackageKind> {
    if is_appimage() { vec![PackageKind::AppImage] } else { Vec::new() }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            view: ViewMode::Grid,
            filter: Filter::All,
            sort: SortKey::Suite,
            sort_descending: false,
            theme: ThemeChoice::System,
            language: "auto".to_string(),
            app_languages: BTreeMap::new(),
            close_to_tray: true,
            show_on_current_desktop: true,
            check_every: CheckEvery::Hour,
            auto_update: false,
            engine_auto_update: true,
            notify: true,
            prereleases: false,
            prism_runtime: false,
            models_only_runnable: false,
            preferred_kinds: default_preferred_kinds(),
            dismissed_notices: std::collections::BTreeSet::new(),
            notified: BTreeMap::new(),
            last_check: None,
        }
    }
}
