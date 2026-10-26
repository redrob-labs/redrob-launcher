//! `redrob-launcher://` links (and the older `redrob://`): web pages (Redrob Chat) open the
//! launcher on an app.
//!
//! The launcher's own scheme is `redrob-launcher`. `redrob` is still read, so links written before
//! it keep working, but it is not one only the launcher claims: Redrob Browser 0.2 to 0.4 registered
//! it as its own launch scheme on Windows, and that claim outlives an update. Pages link with
//! `redrob-launcher://` first.
//!
//! - `redrob://open/<app>` starts the installed app, or shows it with its Install button when it
//!   isn't installed;
//! - `redrob://install/<app>` shows the app and starts its install.
//!
//! `<app>` is the catalog id with or without its `redrob-` prefix (`cad`, `redrob-cad`). The link
//! comes from a web page, so it is hostile: it is parsed strictly, nothing from it reaches a
//! shell or a path, and anything this module doesn't recognise is `None` (the launcher then just
//! opens its home).
//!
//! `open` links may carry two query parameters, which reach the started app as arguments
//! ([`OpenArgs::to_args`]): `file=<dfl_ + 32 hex>` (a Redrob Design cloud file, `--open-file`) and
//! `link=<token>` (a view-link token, at most [`MAX_TOKEN`] of `[A-Za-z0-9._~-]` after
//! percent-decoding, `--open-link`). A value that doesn't fit is dropped, as is any other
//! parameter; the link itself still opens the app.
//!
//! `open/code?engine=1` doesn't start Redrob Code's terminal: it starts the AI engine with desk on
//! (what Redrob Chat needs to reach this PC) and shows the AI engine page.

use crate::catalog::{CraftApp, SUITE};

/// The URL scheme the launcher registers with the OS, and the one its links are written in.
pub const SCHEME: &str = "redrob-launcher";
/// The scheme links used before [`SCHEME`]; still read and still registered.
pub const LEGACY_SCHEME: &str = "redrob";
/// Every scheme the launcher reads and registers, its own first.
pub const SCHEMES: [&str; 2] = [SCHEME, LEGACY_SCHEME];
/// Longer links are refused outright. Room for a `link` token of [`MAX_TOKEN`] percent-encoded.
pub const MAX_LEN: usize = 2048;
/// The longest `link` token passed on, decoded.
pub const MAX_TOKEN: usize = 512;
/// The prefix every catalog id carries.
const ID_PREFIX: &str = "redrob-";
/// The prefix of a Redrob Design cloud file id.
const FILE_PREFIX: &str = "dfl_";

/// What an `open` link hands the app it starts. Every value has been checked against its
/// allowlist: it is plain ASCII with no separators, so it is safe as a single argument.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OpenArgs {
    /// `file=`: a cloud file id, `dfl_` and 32 lowercase hex digits.
    pub file: Option<String>,
    /// `link=`: a view-link token.
    pub link: Option<String>,
    /// `engine=1` on `open/code`: start the AI engine with desk on and show its page, instead of
    /// starting Redrob Code. Never an argument.
    pub engine: bool,
}

impl OpenArgs {
    /// The arguments the app is started with: `--open-file <id>`, `--open-link <token>`.
    pub fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if let Some(file) = &self.file {
            args.extend(["--open-file".to_string(), file.clone()]);
        }
        if let Some(link) = &self.link {
            args.extend(["--open-link".to_string(), link.clone()]);
        }
        args
    }

    /// Reads the allowlisted parameters from a query string (without its `?`).
    fn from_query(query: &str) -> Self {
        let mut args = Self::default();
        for pair in query.split('&') {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            let Some(value) = percent_decode(value) else { continue };
            match key {
                "file" if args.file.is_none() && is_file_id(&value) => args.file = Some(value),
                "link" if args.link.is_none() && is_token(&value) => args.link = Some(value),
                "engine" if value == "1" => args.engine = true,
                _ => {}
            }
        }
        args
    }
}

/// `dfl_` and 32 lowercase hex digits.
fn is_file_id(value: &str) -> bool {
    value.strip_prefix(FILE_PREFIX).is_some_and(|hex| hex.len() == 32 && hex.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)))
}

/// 1 to [`MAX_TOKEN`] of `[A-Za-z0-9._~-]`.
fn is_token(value: &str) -> bool {
    !value.is_empty() && value.len() <= MAX_TOKEN && value.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'~' | b'-'))
}

/// Decodes `%XX` escapes; `None` for a broken escape or bytes that aren't ASCII.
fn percent_decode(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = value.get(i + 1..i + 3)?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    if !out.is_ascii() {
        return None;
    }
    String::from_utf8(out).ok()
}

/// What a `redrob://` link asks for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeepLink {
    /// Start the app if it's installed, with these arguments; otherwise show it with its Install
    /// button.
    Open(&'static CraftApp, OpenArgs),
    /// Show the app and install it (or update it).
    Install(&'static CraftApp),
}

impl DeepLink {
    /// The app the link is about.
    pub fn app(&self) -> &'static CraftApp {
        match self {
            Self::Open(app, _) | Self::Install(app) => app,
        }
    }

    /// The link's canonical spelling (`redrob-launcher://open/cad`). The query is left out: it may carry a
    /// view-link token, and this spelling is what gets logged.
    pub fn to_url(&self) -> String {
        let (action, app) = match self {
            Self::Open(app, _) => ("open", app),
            Self::Install(app) => ("install", app),
        };
        let short = app.id.strip_prefix(ID_PREFIX).unwrap_or(app.id);
        format!("{SCHEME}://{action}/{short}")
    }
}

/// Which of [`SCHEMES`] `arg` is written in, by its length; `None` for anything else.
fn scheme_len(arg: &str) -> Option<usize> {
    SCHEMES
        .iter()
        .map(|s| (s, s.len()))
        .find(|&(s, n)| arg.len() > n && arg.get(..n).is_some_and(|head| head.eq_ignore_ascii_case(s)) && arg.as_bytes().get(n) == Some(&b':'))
        .map(|(_, n)| n)
}

/// Whether a command-line argument is a launcher link at all (valid or not): the launcher was
/// started by the OS for a link, so it shows itself even when the link means nothing.
pub fn is_link(arg: &str) -> bool {
    scheme_len(arg.trim()).is_some()
}

/// Reads a `redrob://<action>/<app>` link. `None` for anything else.
pub fn parse(url: &str) -> Option<DeepLink> {
    let url = url.trim();
    if url.len() > MAX_LEN || !url.is_ascii() {
        return None;
    }
    let rest = url.get(scheme_len(url)? + 1..)?.strip_prefix("//")?;
    // The fragment carries nothing the launcher reads; the query only `open`'s allowlisted args.
    let rest = rest.split('#').next().unwrap_or_default();
    let (rest, query) = rest.split_once('?').unwrap_or((rest, ""));
    let (action, path) = rest.split_once('/')?;
    // One segment, with at most one trailing slash.
    let app = path.strip_suffix('/').unwrap_or(path);
    let app = find_app(app)?;
    if action.eq_ignore_ascii_case("open") {
        Some(DeepLink::Open(app, OpenArgs::from_query(query)))
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

    fn open(id: &str) -> DeepLink {
        DeepLink::Open(app(id), OpenArgs::default())
    }

    #[test]
    fn open_and_install_name_an_app_by_its_short_or_full_id() {
        assert_eq!(parse("redrob://open/cad"), Some(open("redrob-cad")));
        assert_eq!(parse("redrob://open/redrob-cad"), Some(open("redrob-cad")));
        assert_eq!(parse("redrob://install/cowork"), Some(DeepLink::Install(app("redrob-cowork"))));
        assert_eq!(parse("redrob://install/redrob-office/"), Some(DeepLink::Install(app("redrob-office"))));
        for a in &SUITE {
            let short = a.id.strip_prefix("redrob-").unwrap();
            assert_eq!(parse(&format!("redrob://open/{short}")), Some(DeepLink::Open(a, OpenArgs::default())), "{}", a.id);
            assert_eq!(parse(&DeepLink::Install(a).to_url()), Some(DeepLink::Install(a)), "{}", a.id);
        }
    }

    #[test]
    fn case_whitespace_query_and_fragment_do_not_matter() {
        assert_eq!(parse("  REDROB://Open/CAD  "), Some(open("redrob-cad")));
        assert_eq!(parse("redrob://install/code?from=chat"), Some(DeepLink::Install(app("redrob-code"))));
        assert_eq!(parse("redrob://open/office#top"), Some(open("redrob-office")));
        assert_eq!(parse("redrob://open/office?from=chat"), Some(open("redrob-office")));
    }

    #[test]
    fn open_passes_on_only_allowlisted_file_and_link() {
        let file = format!("dfl_{}", "0123456789abcdef".repeat(2));
        let Some(DeepLink::Open(_, args)) = parse(&format!("redrob://open/design?file={file}&link=a%2Eb_c-d~e&x=1")) else { panic!() };
        assert_eq!(args.file.as_deref(), Some(file.as_str()));
        assert_eq!(args.link.as_deref(), Some("a.b_c-d~e"));
        assert_eq!(args.to_args(), ["--open-file", file.as_str(), "--open-link", "a.b_c-d~e"]);
        for query in [
            "file=dfl_0123",
            "file=DFL_0123456789abcdef0123456789abcdef",
            "file=dfl_0123456789ABCDEF0123456789abcdef",
            "file=dfl_0123456789abcdef0123456789abcdeg",
            "file=../dfl_0123456789abcdef0123456789abcdef",
            "link=",
            "link=a%20b",
            "link=a%2",
            "link=a/b",
            "link=a%23b",
            "link=--x;rm",
            "link=%C3%A7",
        ] {
            let Some(DeepLink::Open(_, args)) = parse(&format!("redrob://open/design?{query}")) else { panic!("{query}") };
            assert_eq!(args, OpenArgs::default(), "{query}");
        }
        let long = format!("redrob://open/design?link={}", "a".repeat(MAX_TOKEN + 1));
        assert_eq!(parse(&long), Some(open("redrob-design")));
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
    fn the_launcher_scheme_reads_like_the_old_one() {
        assert_eq!(parse("redrob-launcher://open/cad"), Some(open("redrob-cad")));
        assert_eq!(parse("REDROB-LAUNCHER://install/cowork"), Some(DeepLink::Install(app("redrob-cowork"))));
        assert_eq!(DeepLink::Install(app("redrob-cad")).to_url(), "redrob-launcher://install/cad");
        assert!(is_link("redrob-launcher://open/nonexistent"));
        assert_eq!(parse("redrob-launcherx://open/cad"), None);
        assert_eq!(parse("redrob-launcher:open/cad"), None);
    }

    #[test]
    fn engine_on_open_code_is_a_flag_never_an_argument() {
        for url in ["redrob-launcher://open/code?engine=1", "redrob://open/code?engine=1&x=2"] {
            let Some(DeepLink::Open(a, args)) = parse(url) else { panic!("{url}") };
            assert_eq!(a.id, "redrob-code");
            assert!(args.engine, "{url}");
            assert!(args.to_args().is_empty(), "{url}");
        }
        for query in ["engine=0", "engine=", "engine=true", "engine=1%20"] {
            let Some(DeepLink::Open(_, args)) = parse(&format!("redrob-launcher://open/code?{query}")) else { panic!("{query}") };
            assert!(!args.engine, "{query}");
        }
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
