//! The Redrob apps the launcher knows about.
//!
//! The list is data so a later version can fetch it from an API; [`SUITE`] is the built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`), and each `like` (the kind
//! of app) a row of its own.

use serde::{Deserialize, Serialize};

/// Which shelf an app sits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppGroup {
    /// Apps that make things: video, 3D, CAD, graphics, design.
    Create,
    /// Apps for everyday work: the browser, coding, documents, data, files.
    Work,
}

impl AppGroup {
    pub fn label(self) -> &'static str {
        match self {
            Self::Create => "Create",
            Self::Work => "Work",
        }
    }
}

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`redrob-cad`).
    pub id: &'static str,
    /// Display name (`Redrob CAD`).
    pub name: &'static str,
    /// What it does for you, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    /// What kind of app it is, in one or two words (`CAD`, `Video editing`).
    pub like: &'static str,
    pub group: AppGroup,
    /// The product's mark colour (sRGB) from the design system, for its tile and number tab
    /// only. Products without a mark of their own carry Redrob Blue.
    pub color: [u8; 3],
    /// Accent text on a light background. Product colour is never text (design system,
    /// `20-color.md`), so this is Redrob Blue's text role for every app.
    pub ink_light: [u8; 3],
    /// Accent text on a dark background (Redrob Blue's dark text role).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Reverse-DNS id: macOS bundle id and Linux desktop-file id.
    pub bundle: &'static str,
    /// Older names its release assets may carry.
    pub former_ids: &'static [&'static str],
}

impl CraftApp {
    /// The kind of app, as shown under its name.
    pub fn inspired_by(&self) -> String {
        self.like.to_string()
    }

    /// macOS bundle id and Linux desktop-file id.
    pub fn bundle_id(&self) -> String {
        self.bundle.to_string()
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's own web page. None yet: every app links to its repository instead.
    pub fn website(&self) -> Option<String> {
        None
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        SUITE.iter().find(|a| a.id == id)
    }
}

/// Redrob Blue (`blue-6`): the tile of every product without a mark of its own.
const BLUE: [u8; 3] = [0x2b, 0x52, 0xff];
/// Redrob Blue's text roles: `ink-brand` light (`blue-7`) and dark (`blue-4`).
const INK_LIGHT: [u8; 3] = [0x17, 0x33, 0xd5];
const INK_DARK: [u8; 3] = [0x8a, 0xaf, 0xff];

/// One app; every app lives at `redrob-labs/<id>`.
macro_rules! app {
    ($id:literal, $name:literal, $tagline:literal, $like:literal, $group:expr, $color:expr, $bundle:literal) => {
        CraftApp {
            id: $id,
            name: $name,
            tagline: $tagline,
            like: $like,
            group: $group,
            color: $color,
            ink_light: INK_LIGHT,
            ink_dark: INK_DARK,
            repo: concat!("redrob-labs/", $id),
            bundle: $bundle,
            former_ids: &[],
        }
    };
}

/// The Redrob apps, in display order: the ones that make things, then the ones for work. Only apps
/// with a 2026 app icon (`@redrob-labs/ui` `brand/app-icons/`) are listed.
///
/// Mark colours are the design system's `product-*` marks where a product has one (Browser, Code,
/// Design, Office); the rest carry Redrob Blue until the design system gives them a mark.
pub const SUITE: [CraftApp; 9] = [
    app!(
        "redrob-film",
        "Redrob Film",
        "Describe the edit and the motion graphics and let the agent make them",
        "Video editing",
        AppGroup::Create,
        BLUE,
        "ai.redrob.film"
    ),
    app!("redrob-shape", "Redrob Shape", "Shape and shade 3D models by prompt", "3D", AppGroup::Create, BLUE, "ai.redrob.shape"),
    app!("redrob-cad", "Redrob CAD", "Design, constrain and iterate on parts with an agent", "CAD", AppGroup::Create, BLUE, "ai.redrob.cad"),
    app!("redrob-canvas", "Redrob Canvas", "Paint and edit images with an agent", "Graphics", AppGroup::Create, BLUE, "ai.redrob.canvas"),
    app!(
        "redrob-design",
        "Redrob Design",
        "Open .fig and .pen files and design by prompt",
        "Design",
        AppGroup::Create,
        [0xc1, 0x62, 0xf4],
        "design.redrob.RedrobDesign"
    ),
    app!(
        "redrob-browser",
        "Redrob Browser",
        "A browser whose agent reads, acts on and automates the web",
        "Browser",
        AppGroup::Work,
        [0xf1, 0x4a, 0xa8],
        "ai.redrob.browser"
    ),
    app!("redrob-code", "Redrob Code", "The AI coding agent for your terminal", "Coding agent", AppGroup::Work, [0x02, 0xa2, 0x52], "ai.redrob.code"),
    app!("redrob-cowork", "Redrob Cowork", "Work with AI agents on your own files", "Agent workspace", AppGroup::Work, BLUE, "ai.redrob.cowork"),
    app!(
        "redrob-office",
        "Redrob Office",
        "Docs, Sheets, Slides, PDF and Hangul in one window",
        "Office suite",
        AppGroup::Work,
        [0xfe, 0x4d, 0x3e],
        "ai.redrob.office"
    ),
];

/// Redrob's website.
pub const WEBSITE_URL: &str = "https://redrob.ai";
/// Redrob on GitHub: every app's source.
pub const GITHUB_URL: &str = "https://github.com/redrob-labs";

/// Redrob Launcher itself, described like an app so it can update itself with the same machinery
/// (asset names `redrob-launcher-<ver>-…`, bundle id `ai.redrob.launcher`, `Redrob Launcher.app`).
/// Not part of [`SUITE`].
pub const LAUNCHER: CraftApp = CraftApp {
    id: "redrob-launcher",
    name: "Redrob Launcher",
    tagline: "Installs, updates and connects every Redrob app",
    like: "",
    group: AppGroup::Work,
    color: BLUE,
    ink_light: INK_LIGHT,
    ink_dark: INK_DARK,
    repo: "redrob-labs/redrob-launcher",
    bundle: "ai.redrob.launcher",
    former_ids: &[],
};

/// The launcher's own repo, checked so it can say when a newer launcher is out.
pub const LAUNCHER_REPO: &str = "redrob-labs/redrob-launcher";
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "redrob-launcher";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_ids_are_unique_and_name_their_repo() {
        let mut ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SUITE.len());
        for a in &SUITE {
            assert_eq!(a.repo, format!("redrob-labs/{}", a.id));
            assert!(a.name.starts_with("Redrob "), "{}", a.name);
            assert!(!a.like.is_empty() && !a.tagline.is_empty(), "{}", a.id);
            assert!(a.bundle.contains('.'), "{}", a.id);
        }
        assert!(SUITE.iter().any(|a| a.group == AppGroup::Create));
        assert!(SUITE.iter().any(|a| a.group == AppGroup::Work));
    }

    #[test]
    fn nothing_points_at_the_upstream_project() {
        for a in SUITE.iter().chain(std::iter::once(&LAUNCHER)) {
            for s in [a.repo, a.bundle, a.name] {
                assert!(!s.contains("storytold") && !s.contains("storyteller") && !s.contains("Craft"), "{s}");
            }
        }
        assert_eq!(LAUNCHER.repo, LAUNCHER_REPO);
        assert_eq!(LAUNCHER.id, LAUNCHER_ID);
    }

    #[test]
    fn product_text_is_never_the_product_colour() {
        for a in &SUITE {
            assert_eq!((a.ink_light, a.ink_dark), (INK_LIGHT, INK_DARK), "{}", a.id);
        }
        assert_eq!(CraftApp::by_id("redrob-cad").unwrap().bundle_id(), "ai.redrob.cad");
        assert_eq!(CraftApp::by_id("redrob-cad").unwrap().website(), None);
    }
}
