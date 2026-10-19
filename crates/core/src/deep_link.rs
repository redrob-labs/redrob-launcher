//! `redrob://` links: web pages (Redrob Chat) open the launcher on an app.
//!
//! - `redrob://open/<app>` starts the installed app, or shows it with its Install button when it
//!   isn't installed;
//! - `redrob://install/<app>` shows the app and starts its install.
//!
//! `<app>` is the catalog id with or without its `redrob-` prefix (`cad`, `redrob-cad`). The link
//! comes from a web page, so it is hostile: it is parsed strictly, nothing from it reaches a
//! shell or a path, and anything this module doesn't recognise is `None` (the launcher then just
//! opens its home).

use crate::catalog::{CraftApp, SUITE};

/// The URL scheme the launcher registers with the OS.
pub const SCHEME: &str = "redrob";
/// Longer links are refused outright.
pub const MAX_LEN: usize = 256;
/// The prefix every catalog id carries.
const ID_PREFIX: &str = "redrob-";

/// What a `redrob://` link asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeepLink {
    /// Start the app if it's installed; otherwise show it with its Install button.
    Open(&'static CraftApp),
    /// Show the app and install it (or update it).
    Install(&'static CraftApp),
}

impl DeepLink {
    /// The app the link is about.
    pub fn app(&self) -> &'static CraftApp {
        match self {
            Self::Open(app) | Self::Install(app) => app,
        }
    }

    /// The link's canonical spelling (`redrob://open/cad`).
    pub fn to_url(&self) -> String {
        let (action, app) = match self {
            Self::Open(app) => ("open", app),
            Self::Install(app) => ("install", app),
        };
        let short = app.id.strip_prefix(ID_PREFIX).unwrap_or(app.id);
        format!("{SCHEME}://{action}/{short}")
    }
}

/// Whether a command-line argument is a `redrob:` link at all (valid or not): the launcher was
/// started by the OS for a link, so it shows itself even when the link means nothing.
pub fn is_link(arg: &str) -> bool {
    let arg = arg.trim();
    arg.len() > SCHEME.len()
        && arg.get(..SCHEME.len()).is_some_and(|s| s.eq_ignore_ascii_case(SCHEME))
        && arg.as_bytes().get(SCHEME.len()) == Some(&b':')
}

/// Reads a `redrob://<action>/<app>` link. `None` for anything else.
pub fn parse(url: &str) -> Option<DeepLink> {
    let url = url.trim();
    if url.len() > MAX_LEN || !url.is_ascii() || !is_link(url) {
        return None;
    }
    let rest = url.get(SCHEME.len() + 1..)?.strip_prefix("//")?;
    // The query and fragment carry nothing the launcher reads.
    let rest = rest.split(['?', '#']).next().unwrap_or_default();
    let (action, path) = rest.split_once('/')?;
    // One segment, with at most one trailing slash.
    let app = path.strip_suffix('/').unwrap_or(path);
    let app = find_app(app)?;
    if action.eq_ignore_ascii_case("open") {
        Some(DeepLink::Open(app))
    } else if action.eq_ignore_ascii_case("install") {
        Some(DeepLink::Install(app))
    } else {
        None
    }
}

/// The catalog app `name` (`cad` or `redrob-cad`, any case) names.
fn find_app(name: &str) -> Option<&'static CraftApp> {
    if name.is_empty() || name.len() > 64 || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-') {
        return None;
    }
    let name = name.to_ascii_lowercase();
    let id = if name.starts_with(ID_PREFIX) { name } else { format!("{ID_PREFIX}{name}") };
    SUITE.iter().find(|a| a.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app(id: &str) -> &'static CraftApp {
        CraftApp::by_id(id).unwrap()
    }

    #[test]
    fn open_and_install_name_an_app_by_its_short_or_full_id() {
        assert_eq!(parse("redrob://open/cad"), Some(DeepLink::Open(app("redrob-cad"))));
        assert_eq!(parse("redrob://open/redrob-cad"), Some(DeepLink::Open(app("redrob-cad"))));
        assert_eq!(parse("redrob://install/cowork"), Some(DeepLink::Install(app("redrob-cowork"))));
        assert_eq!(parse("redrob://install/redrob-query/"), Some(DeepLink::Install(app("redrob-query"))));
        for a in &SUITE {
            let short = a.id.strip_prefix("redrob-").unwrap();
            assert_eq!(parse(&format!("redrob://open/{short}")), Some(DeepLink::Open(a)), "{}", a.id);
            assert_eq!(parse(&DeepLink::Install(a).to_url()), Some(DeepLink::Install(a)), "{}", a.id);
        }
    }

    #[test]
    fn case_whitespace_query_and_fragment_do_not_matter() {
        assert_eq!(parse("  REDROB://Open/CAD  "), Some(DeepLink::Open(app("redrob-cad"))));
        assert_eq!(parse("redrob://install/code?from=chat"), Some(DeepLink::Install(app("redrob-code"))));
        assert_eq!(parse("redrob://open/office#top"), Some(DeepLink::Open(app("redrob-office"))));
    }

    #[test]
    fn anything_else_is_refused() {
        for url in [
            "",
            "redrob:",
            "redrob:open/cad",
            "redrob:/open/cad",
            "redrob://",
            "redrob://open",
            "redrob://open/",
            "redrob://open/nonexistent",
            "redrob://open/launcher",
            "redrob://open/redrob-launcher",
            "redrob://launch/cad",
            "redrob://uninstall/cad",
            "redrob://open/cad/extra",
            "redrob://open//cad",
            "redrob://open/../cad",
            "redrob://open/cad%20",
            "redrob://open/c%61d",
            "redrob://open/cad;rm",
            "redrob://open/ca d",
            "redrob://user@open/cad",
            "redrob://open:80/cad",
            "redrob://open/çad",
            "https://open/cad",
            "redrobx://open/cad",
            "file:///etc/passwd",
        ] {
            assert_eq!(parse(url), None, "{url}");
        }
        let long = format!("redrob://open/cad?{}", "x".repeat(MAX_LEN));
        assert_eq!(parse(&long), None);
    }

    #[test]
    fn a_link_is_recognised_even_when_it_means_nothing() {
        assert!(is_link("redrob://open/nonexistent"));
        assert!(is_link("Redrob:anything"));
        assert!(!is_link("redrob"));
        assert!(!is_link("--hidden"));
        assert!(!is_link("redrobx://open/cad"));
        assert!(!is_link("/home/me/redrob:/x"));
    }
}
