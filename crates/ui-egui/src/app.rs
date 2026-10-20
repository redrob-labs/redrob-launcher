// ADDED-LINE-RATIONALE c65f81c3ee035116:1: Gives `app` direct access to `crate / settings / Filter` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE bd791e931f0556db:1: Supports `Page` while preserving the nearby constraint: The shared Redrob Code engine: running or not, who uses it, start and stop.
// ADDED-LINE-RATIONALE 2f45a214ed3fe916:1: Supports `Page` while preserving the nearby constraint: Signing in to Redrob, and which apps use it.
// ADDED-LINE-RATIONALE 1f4cfc3b5846fdae:1: Keeps `Settings` in `Page`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 9219fc67cca7bea4:1: Delegates `show_engine`'s `set_page / Page / Engine` work to `self.set_page` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 618f2d2c65b8e37f:1: Supports `ui` while preserving the nearby constraint: page's own check below it would look as if nothing had been open.
// ADDED-LINE-RATIONALE 3aba3e1f3061631e:1: Supports `ui` while preserving the nearby constraint: search, logo and Update sit in a 48 px row over the main area instead.
// ADDED-LINE-RATIONALE 0ebf8d368d5f24e2:1: Freezes `if` for `ui` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cff5291e2bf08ce6:1: Delegates `ui`'s `egui / Panel / top` work to `egui::Panel::top` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0f266cb8cf4ed299:1: Delegates `ui`'s `frame / egui / Frame` work to `frame` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 910f79f861c5bd6e:1: Delegates `ui`'s `exact_size / crate / views` work to `exact_size` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE aa55012480b72796:1: Delegates `ui`'s `resizable` work to `resizable` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ba158783caff6453:1: Delegates `ui`'s `show / ui / ui` work to `show` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 144e68ae02fb4e12:31: Bounds the `ui` implementation here so `show / ui / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5e8776d9f15ca3ac:1: Freezes `inset` for `ui` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8594a272ec2e6603:1: Delegates `ui`'s `egui / Panel / left` work to `egui::Panel::left` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3ec015161a96f38d:1: Delegates `ui`'s `frame / egui / Frame` work to `frame` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e73210a336213360:1: Delegates `ui`'s `exact_size / Self / sidebar_width` work to `exact_size` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 42b984a00830490d:1: Delegates `ui`'s `show_separator_line` work to `show_separator_line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5cdaa950c4cbb719:1: Delegates `ui`'s `show / ui / ui` work to `show` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7d7017c4d30812a6:1: Freezes `if` for `ui` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 63f00d1c6154cc55:1: Delegates `ui`'s `egui / Panel / top` work to `egui::Panel::top` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9b27dd3171ac4d96:1: Delegates `ui`'s `frame / egui / Frame` work to `frame` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 910f79f861c5bd6e:2: Delegates `ui`'s `exact_size / crate / views` work to `exact_size` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE aa55012480b72796:2: Delegates `ui`'s `resizable` work to `resizable` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ba158783caff6453:2: Delegates `ui`'s `show / ui / ui` work to `show` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 144e68ae02fb4e12:32: Bounds the `ui` implementation here so `show / ui / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 22968439fb5fdfa7:1: Supports `ui` while preserving the nearby constraint: The Apps page has no top bar: its actions are in the chrome and its filter row.
// ADDED-LINE-RATIONALE 3c3c79864c15a24a:1: Delegates `ui`'s `egui / Panel / top` work to `egui::Panel::top` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9b27dd3171ac4d96:2: Delegates `ui`'s `frame / egui / Frame` work to `frame` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 28c4ef078517d488:1: Delegates `ui`'s `exact_size / crate / shell` work to `exact_size` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE aa55012480b72796:3: Delegates `ui`'s `resizable` work to `resizable` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 84e531e0438fc432:1: Delegates `ui`'s `show / ui / ui` work to `show` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 144e68ae02fb4e12:33: Bounds the `ui` implementation here so `show / ui / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 812c7814a091aba9:1: Supports `ui` while preserving the nearby constraint: 24 px is inside its scroll area, so the bar hugs the edge like Cowork's.
// ADDED-LINE-RATIONALE 41ba864cd7d993ac:1: Delegates `ui`'s `frame / egui / Frame` work to `frame` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b6c314ea7ef199b9:1: Delegates `ui`'s `show / ui / ui` work to `show` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ecd4ec216af1a58a:1: Freezes `Page` for `ui` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2b501ee6dcdee398:1: Freezes `Page` for `ui` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e5bc0f4d92293ce0:1: Delegates `ui`'s `crate / widgets / page_scroll` work to `crate::widgets::page_scroll` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 67eb662455877620:1: Supports `ui` while preserving the nearby constraint: A reading measure, like Cowork's content column.
// ADDED-LINE-RATIONALE 671594c0ef8e2046:1: Prevents `ui` from applying its side effect outside the `page` state.
// ADDED-LINE-RATIONALE 7ed59b35662f2e52:1: Freezes `Page` for `ui` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE dac5185a5f962204:1: Freezes `Page` for `ui` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 29532d02deae638d:1: Freezes `_` for `ui` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 42380c443e330c60:1: Supports `ui` while preserving the nearby constraint: Room under the last control, so nothing ends against the window's edge.
// ADDED-LINE-RATIONALE e4684424052696fe:1: Makes `ui / add_space` an explicit part of `ui` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 3028d712dcab3272:14: Bounds the `ui` implementation here so `Page / Apps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ff9d6d38366878ca:1: Freezes `Page` for `ui` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c5bb8d6702e3966f:1: Supports `ui` while preserving the nearby constraint: Above the scroll: the page's right padding, as inside it.
// ADDED-LINE-RATIONALE b8b712bab277c9a7:1: Delegates `ui`'s `inner_margin / egui / Margin` work to `inner_margin` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e997a09e9e89c7c0:1: Delegates `ui`'s `show / ui / ui` work to `show` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4f8a3c3333334dd4:1: Freezes `ids` for `ui` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cd9030d0dda13239:1: Delegates `ui`'s `crate / widgets / page_scroll` work to `crate::widgets::page_scroll` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 487b0d306273dda4:1: Prevents `ui` from applying its side effect outside the `ids / is_empty` state.
// ADDED-LINE-RATIONALE 03b5fc20524e9586:1: Delegates `ui`'s `empty_state / ui` work to `self.empty_state` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1ac169fcabb9dd16:1: Makes `empty_state / ui` an explicit part of `ui` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 2579ddb0a48d8c41:1: Delegates `ui`'s `grid / ui / ids` work to `self.grid` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9c8ce6e59d9a77d2:2: Bounds the `ui` implementation here so `grid / ui / ids` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 00feba57b64204f2:1: Delegates `ui`'s `ui / add_space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e4684424052696fe:2: Makes `ui / add_space` an explicit part of `ui` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 3028d712dcab3272:15: Bounds the `ui` implementation here so `ui / add_space` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 110aca60be984262:1: Supports `ui` while preserving the nearby constraint: focused control (a button reached by Tab) doesn't use Escape, so it doesn't block it.
// ADDED-LINE-RATIONALE 9977361a71f79782:1: Freezes `if` for `ui` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a15361cc03460d61:1: Delegates `ui`'s `dialog / is_none` work to `self.dialog.is_none` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 048179d09c635347:1: Makes `alert_open` an explicit part of `ui` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 0bcdff6246bace53:1: Makes `escape_taken` an explicit part of `ui` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE fa9108db29d5b35c:1: Delegates `ui`'s `egui / Popup / is_any_open` work to `egui::Popup::is_any_open` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 37717d187572c68a:1: Delegates `ui`'s `ctx / text_edit_focused` work to `ctx.text_edit_focused` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 94ea3fdf00338c2a:1: Delegates `ui`'s `ctx / input / i` work to `ctx.input` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 144e62ae02fb43e0:3: Bounds the `ui` implementation here so `ctx / input / i` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c5a4b7297ae84656:1: Delegates `ui`'s `set_page / Page / Apps` work to `self.set_page` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 144e68ae02fb4e12:34: Bounds the `ui` implementation here so `set_page / Page / Apps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0d5930e8336d404d:1: Delegates `ui`'s `dialogs / ctx` work to `self.dialogs` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6398f45536624fc5:1: Delegates `visible_ids`'s `crate / shelves / page_order` work to `crate::shelves::page_order` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5305014942aaadfb:1: Delegates `visible_ids`'s `into_iter` work to `into_iter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fc162c376b85f75c:1: Delegates `visible_ids`'s `filter_map / id / entry` work to `filter_map` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 25e8995e6ca58a94:1: Delegates `visible_ids`'s `filter / e / shown` work to `filter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fbe710ab6d05568d:1: Delegates `visible_ids`'s `filter / e` work to `filter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 11ff6a1b1dbe93e6:1: Delegates `visible_ids`'s `map / e / e` work to `map` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f75768ecf368a8f0:1: Delegates `visible_ids`'s `collect` work to `collect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9d65ce5383a7867c:1: Supports `shown` while preserving the nearby constraint: unless installed ([`crate::shelves::HIDDEN_UNLESS_INSTALLED`]).
// ADDED-LINE-RATIONALE cae898fef5b0b172:1: Delegates `shown`'s `crate / shelves / shown` work to `crate::shelves::shown` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 584baf5263686ce2:25: Bounds the `shown` implementation here so `crate / shelves / shown` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 25e8995e6ca58a94:2: Delegates `count`'s `filter / e / shown` work to `filter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 823b5441801f8670:1: Gives `draw_toasts` direct access to `crate / glyphs / Icon` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE b81f09b54e4e71bf:1: Freezes `W` for `draw_toasts` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE af9fd5850abfe01a:1: Supports `draw_toasts` while preserving the nearby constraint: 24 px in from the window's corner (the page's own padding); there is no footer to clear.
// ADDED-LINE-RATIONALE c3dfd9641ad75eb5:1: Delegates `draw_toasts`'s `ui / set_width / W` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a59680e9b1b9fe0d:1: Freezes `let` for `draw_toasts` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ad9b9a2add6849d6:1: Delegates `draw_toasts`'s `fill / t / raised` work to `fill` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 59f57dab5da58b51:1: Delegates `draw_toasts`'s `stroke / t / hairline` work to `stroke` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8f6a73a00433545c:1: Delegates `draw_toasts`'s `corner_radius / egui / CornerRadius` work to `corner_radius` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9176eda2966799be:1: Delegates `draw_toasts`'s `shadow / theme / shadow_lg` work to `shadow` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 298dde8a29bc66e6:1: Delegates `draw_toasts`'s `inner_margin` work to `inner_margin` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 290b38629cd64c55:1: Delegates `draw_toasts`'s `ui / set_width / W` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 391c59b37a9915d0:1: Delegates `draw_toasts`'s `ui / horizontal_top / ui` work to `ui.horizontal_top` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7c4c7788c49590ae:1: Freezes `ui` for `draw_toasts` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b2f3f38cc755fb56:1: Freezes `let` for `draw_toasts` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0d62390cad1eac0c:1: Delegates `draw_toasts`'s `crate / glyphs / paint` work to `crate::glyphs::paint` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b4e601f0c5636b12:1: Freezes `text_w` for `draw_toasts` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 96bb64abc31545e4:1: Delegates `draw_toasts`'s `ui / allocate_ui / egui` work to `ui.allocate_ui` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0da1966e6f65da9d:1: Delegates `draw_toasts`'s `ui / set_width / text_w` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bd18fb50265e8d64:1: Delegates `draw_toasts`'s `ui / add` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 735b002c77491302:1: Delegates `draw_toasts`'s `egui / Label / new` work to `egui::Label::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE cee71c11939e1f00:1: Keeps `wrap` in `draw_toasts`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE f84983a2d7ae2f37:1: Bounds the `draw_toasts` implementation here so `wrap` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 36e651cab70b2dde:1: Makes `crate / widgets / icon_button` an explicit part of `draw_toasts` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 1dedd1deb35c05f2:1: Delegates `draw_toasts`'s `crate / widgets / icon_button` work to `crate::widgets::icon_button` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9de4532acd79df81:1: Makes `crate / widgets / icon_button` an explicit part of `draw_toasts` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE a9236aa65502c519:1: Makes `inner` an explicit part of `draw_toasts` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE d92218ed0969eefe:1: Prevents `draw_toasts` from applying its side effect outside the `r / inner / r` state.
// ADDED-LINE-RATIONALE 2eed0a7f4b4210c7:1: Delegates `draw_toasts`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
//! [`LauncherUi`]: state, event handling and the page layout.

use std::time::Duration;

use redrob_launcher_core::deep_link::DeepLink;
use redrob_launcher_core::protocol::{
    Backend, BackendOptions, CANCELLED, Capabilities, Command, Event, INSTALL_CANCELLED, JobPhase, LatestRelease, PAUSED, Source,
};
use redrob_launcher_core::release::Release;
use redrob_launcher_core::status::AppStatus;
use redrob_launcher_core::{Platform, SUITE};

use crate::i18n::set_current;
use crate::model::AppEntry;
use crate::settings::{Filter, Settings};
use crate::theme::{self, Tokens};

/// Who draws the window's title bar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TitleBar {
    /// Nothing to draw: the OS draws one, or there's no window (the web build).
    #[default]
    Native,
    /// The window has no OS title bar: the header is the title bar (drag it to move the window,
    /// double-click to maximize), with minimize, maximize and close at its right end, and the
    /// window's edges resize it.
    Custom,
    /// macOS: the header runs under a transparent title bar, with the traffic lights at its left.
    MacOs,
}

/// What only the shell can do. Every method has a do-nothing default (the web build).
pub trait Host {
    /// Who draws the title bar.
    fn title_bar(&self) -> TitleBar {
        TitleBar::Native
    }
    /// Shows a system notification.
    fn notify(&mut self, _title: &str, _body: &str) {}
    /// A tray (menu bar) icon exists, so closing the window can keep the launcher running.
    fn has_tray(&self) -> bool {
        false
    }
    /// The OS has desktops (macOS Spaces) the window can be shown on, so the "show on the current
    /// desktop" setting applies.
    fn has_desktops(&self) -> bool {
        false
    }
    /// `Some(enabled)` where starting at login is supported.
    fn login_item(&self) -> Option<bool> {
        None
    }
    fn set_login_item(&mut self, _on: bool) -> Result<(), String> {
        Err("not supported here".into())
    }
    /// Quit the launcher for real (not just hide to the tray).
    fn quit(&mut self) {}
    /// The language every Redrob app should draw in changed (the launcher's own language, or a
    /// per-app override). The desktop shell publishes it in the shared language file.
    fn language_changed(&mut self, _choice: &LanguageChoice) {}
}

/// The language the launcher publishes for the other apps: what it draws in itself (resolved,
/// never `auto`), whether a person chose it, and per-app overrides (app id to code).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LanguageChoice {
    pub language: String,
    pub chosen: bool,
    pub apps: std::collections::BTreeMap<String, String>,
}

/// Which page the main area shows: one per sidebar row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Page {
    Apps,
    /// The models the apps can use on this computer.
    Models,
    /// The shared Redrob Code engine: running or not, who uses it, start and stop.
    Engine,
    /// Signing in to Redrob, and which apps use it.
    Account,
    Settings,
}

pub(crate) struct Toast {
    pub text: String,
    pub error: bool,
    pub until: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Dialog {
    Notes(String),
    ConfirmUninstall(String),
    LauncherUpdate(redrob_launcher_core::Release),
}

pub struct LauncherUi {
    pub(crate) backend: Box<dyn Backend>,
    pub settings: Settings,
    pub(crate) entries: Vec<AppEntry>,
    pub(crate) caps: Capabilities,
    pub(crate) platform: Option<Platform>,
    pub(crate) checking: bool,
    pub(crate) last_source: Option<Source>,
    pub(crate) rate_limited_until: Option<i64>,
    pub(crate) launcher_update: Option<Release>,
    /// Keeping the shared Redrob Code engine current (stop, install, start again).
    pub(crate) engine_update: crate::engine_update::Flow,
    /// The launcher's own update, while it downloads and installs.
    pub(crate) launcher_job: Option<redrob_launcher_core::protocol::JobPhase>,
    /// The new launcher is in place: quit so it can start.
    restart_requested: bool,
    /// News from the release feed, shown inline until dismissed.
    pub(crate) announcements: Vec<redrob_launcher_core::feed::Notice>,
    /// Alerts from the feed, shown as popups until acknowledged.
    pub(crate) alerts: Vec<redrob_launcher_core::feed::Notice>,
    pub(crate) search: String,
    pub(crate) toasts: Vec<Toast>,
    pub(crate) dialog: Option<Dialog>,
    pub(crate) page: Page,
    /// Which tab of the Settings page is open.
    pub(crate) settings_tab: crate::channels::SettingsTab,
    /// Settings > Channels, as edited.
    pub(crate) channels: crate::channels::ChannelsPanel,
    started: bool,
    applied_theme: Option<theme::ThemeChoice>,
    /// Fonts registered with `set_fonts` only exist from the next frame on; drawing with the named
    /// weights before then would abort.
    fonts_ready: bool,
    /// Whether the installed fonts include the system's Chinese font. Added the first time Chinese
    /// is the language in use, then kept for the session.
    cjk_fonts: bool,
    sent_options: Option<BackendOptions>,
    /// Unix seconds of the last look at what's installed (startup counts).
    last_detect: i64,
    /// Whether the window had focus last frame, to re-check when the user comes back to it.
    was_focused: bool,
    /// Versions of apps or the launcher already queued for auto-update during this session,
    /// preventing repeated install loops if an update fails or is in progress.
    pub(crate) auto_update_attempted: std::collections::BTreeSet<(String, String)>,
    /// What was last handed to [`Host::language_changed`], so it is called on a change only.
    published_language: Option<LanguageChoice>,
    /// A `redrob://` link waiting until the backend knows enough about its app to act on it.
    pending_link: Option<DeepLink>,
    /// The app whose Install button takes the keyboard focus the next time it is drawn (a
    /// `redrob://open` link for an app that isn't installed).
    pub(crate) focus_install: Option<&'static str>,
}

/// The language to draw in: the preference, or English where it needs a Chinese font the system
/// doesn't have (Settings says so), rather than boxes for every character. The only place the
/// Chinese font gets loaded: when Chinese is selected, or Auto resolves to it.
pub(crate) fn effective_lang(pref: &str) -> crate::i18n::Lang {
    let lang = crate::i18n::Lang::from_pref(pref);
    if lang.needs_cjk_font() && !theme::cjk_font_available() { crate::i18n::Lang::EN } else { lang }
}

/// How often to look again at what's installed, to notice apps installed or removed outside
/// the launcher. A detection pass lists packages and reads a few files: cheap, but not free.
const RESCAN_EVERY: i64 = 5 * 60;
/// Coming back to the window re-checks, but not more often than this.
const RESCAN_ON_FOCUS_AFTER: i64 = 30;

impl LauncherUi {
    pub fn new(ctx: &egui::Context, backend: Box<dyn Backend>, settings: Settings) -> Self {
        let cjk_fonts = effective_lang(&settings.language).needs_cjk_font();
        theme::install_fonts(ctx, cjk_fonts);
        egui_extras::install_image_loaders(ctx);
        let caps = backend.capabilities();
        let platform = backend.platform();
        Self {
            backend,
            settings,
            entries: SUITE.iter().map(AppEntry::new).collect(),
            caps,
            platform,
            checking: false,
            last_source: None,
            rate_limited_until: None,
            launcher_update: None,
            engine_update: crate::engine_update::Flow::default(),
            launcher_job: None,
            restart_requested: false,
            announcements: Vec::new(),
            alerts: Vec::new(),
            search: String::new(),
            toasts: Vec::new(),
            dialog: None,
            settings_tab: crate::channels::SettingsTab::default(),
            channels: crate::channels::ChannelsPanel::default(),
            started: false,
            applied_theme: None,
            fonts_ready: false,
            cjk_fonts,
            sent_options: None,
            last_detect: 0,
            was_focused: true,
            auto_update_attempted: std::collections::BTreeSet::new(),
            published_language: None,
            pending_link: None,
            focus_install: None,
            page: Page::Apps,
        }
    }

    /// Moves the UI to another window's context: fonts, image loaders and the theme belong to a
    /// context, and toasts are timed on its clock.
    pub fn attach(&mut self, ctx: &egui::Context) {
        theme::install_fonts(ctx, self.cjk_fonts);
        egui_extras::install_image_loaders(ctx);
        self.fonts_ready = false;
        self.applied_theme = None;
        self.toasts.clear();
    }

    /// Installed apps with an update waiting (the tray shows the count).
    pub fn update_count(&self) -> usize {
        self.entries.iter().filter(|e| e.status(self.platform, self.caps.manage_apps) == AppStatus::UpdateAvailable).count()
    }

    /// Something is downloading or installing, the launcher's own update included: quitting now
    /// would cut it short.
    pub fn busy(&self) -> bool {
        self.launcher_job.is_some() || self.entries.iter().any(|e| e.job.is_some())
    }

    /// Looks for updates now (the tray's "Check for Updates").
    /// Open the engine page (the tray's "AI engine" item).
    pub fn show_engine(&mut self) {
        if let Some(engine) = self.backend.engine() {
            engine.refresh();
        }
        self.set_page(Page::Engine);
    }

    pub fn check_now(&mut self) {
        self.auto_update_attempted.clear();
        self.backend.send(Command::CheckForUpdates { force: true });
        self.backend.send(Command::Detect);
        self.last_detect = self.backend.now();
    }

    /// Acts on a `redrob://` link (`deep_link`): shows its app on the apps page, then starts it
    /// or its install as soon as the backend knows whether it is installed and what's released.
    pub fn open_link(&mut self, link: DeepLink) {
        let app = link.app();
        log::info!("link: {}", link.to_url());
        self.page = Page::Apps;
        self.dialog = None;
        self.settings.filter = Filter::All;
        self.search = app.name.to_string();
        self.focus_install = None;
        self.pending_link = Some(link);
        self.resolve_link();
    }

    /// Carries out the pending link once its app's state is known; until then it waits.
    fn resolve_link(&mut self) {
        let Some(link) = self.pending_link.clone() else { return };
        let id = link.app().id;
        let Some(entry) = self.entry(id) else {
            self.pending_link = None;
            return;
        };
        // The web build, or the app is busy already: showing it is all there is to do.
        if !self.caps.manage_apps || entry.job.is_some() {
            self.pending_link = None;
            return;
        }
        let status = entry.status(self.platform, true);
        let command = match link {
            DeepLink::Open(..) if !entry.detected => return,
            DeepLink::Open(_, args) if entry.installed.is_some() => Some(Command::Launch { app: id.to_string(), args: args.to_args() }),
            DeepLink::Open(..) => {
                self.focus_install = Some(id);
                None
            }
            DeepLink::Install(_) => match status {
                AppStatus::Checking => return,
                AppStatus::Available | AppStatus::UpdateAvailable => Some(Command::Install { app: id.to_string() }),
                // Up to date, no release, offline: the card says so.
                _ => None,
            },
        };
        self.pending_link = None;
        if let Some(command) = command {
            self.send(command);
        }
    }

    pub(crate) fn send(&mut self, command: Command) {
        self.backend.send(command);
    }

    pub(crate) fn toast(&mut self, ctx: &egui::Context, text: impl Into<String>, error: bool) {
        let now = ctx.input(|i| i.time);
        self.toasts.push(Toast { text: text.into(), error, until: now + if error { 8.0 } else { 4.5 } });
        if self.toasts.len() > 4 {
            self.toasts.remove(0);
        }
    }

    pub(crate) fn entry(&self, id: &str) -> Option<&AppEntry> {
        self.entries.iter().find(|e| e.app.id == id)
    }

    fn entry_mut(&mut self, id: &str) -> Option<&mut AppEntry> {
        self.entries.iter_mut().find(|e| e.app.id == id)
    }

    /// Non-drawing work: runs every frame and also while the window is hidden in the tray.
    pub fn logic(&mut self, ctx: &egui::Context, host: &mut dyn Host) {
        set_current(crate::i18n::Lang::from_pref(&self.settings.language));
        let options = BackendOptions { kinds: self.settings.preferred_kinds.clone(), prereleases: self.settings.prereleases };
        if self.sent_options.as_ref() != Some(&options) {
            self.backend.send(Command::Configure(options.clone()));
            self.sent_options = Some(options);
        }
        if !self.started {
            self.started = true;
            self.backend.send(Command::CheckForUpdates { force: false });
        }
        let now = self.backend.now();
        if self.last_detect == 0 {
            // The backend looks once at startup by itself.
            self.last_detect = now;
        }
        let focused = ctx.input(|i| i.viewport().focused).unwrap_or(true);
        let came_back = focused && !self.was_focused && now - self.last_detect >= RESCAN_ON_FOCUS_AFTER;
        self.was_focused = focused;
        if came_back || now - self.last_detect >= RESCAN_EVERY {
            self.last_detect = now;
            self.backend.send(Command::Detect);
        }
        if let Some(every) = self.settings.check_every.seconds()
            && !self.checking
            && self.settings.last_check.is_none_or(|t| now - t >= every)
        {
            self.backend.send(Command::CheckForUpdates { force: false });
        }
        let mut changed = false;
        for event in self.backend.poll() {
            changed |= self.handle(ctx, event);
        }
        if changed {
            self.announce_updates(host);
            if self.settings.auto_update {
                self.trigger_auto_updates();
            }
        }
        self.engine_update_tick();
        self.resolve_link();
        if self.restart_requested {
            // The helper is waiting for this process to exit before it starts the new launcher.
            self.restart_requested = false;
            host.quit();
        }
        // Wake up now and then for scheduled checks, even in the tray.
        ctx.request_repaint_after(Duration::from_secs(60));
    }

    /// Returns whether app state changed.
    fn handle(&mut self, ctx: &egui::Context, event: Event) -> bool {
        match event {
            Event::Latest { app, latest, from } => {
                if let Some(e) = self.entry_mut(&app) {
                    // A failed check keeps the release we already know about (and where it came from).
                    if !(matches!(latest, LatestRelease::Failed(_)) && matches!(e.latest, Some(LatestRelease::Found(_)))) {
                        e.latest = Some(latest);
                        e.origin = Some(from);
                    }
                }
                true
            }
            Event::Detected { app, installed } => {
                if let Some(e) = self.entry_mut(&app) {
                    e.installed = installed;
                    e.detected = true;
                }
                true
            }
            // The product's icon as its latest release ships it; a repaint shows it.
            Event::Icon { app, png } => crate::icons::set_fetched(&app, png),
            Event::Progress { app, phase } => {
                if app == redrob_launcher_core::catalog::LAUNCHER_ID {
                    self.launcher_job = Some(phase);
                    if matches!(self.dialog, Some(Dialog::LauncherUpdate(_))) {
                        self.dialog = None;
                    }
                } else if let Some(e) = self.entry_mut(&app) {
                    e.job = Some(phase);
                }
                false
            }
            Event::LauncherRestarting => {
                self.restart_requested = true;
                false
            }
            Event::Notices { announcements, alerts } => {
                self.announcements = announcements;
                self.alerts = alerts;
                false
            }
            Event::Paused { app, download } => {
                if let Some(e) = self.entry_mut(&app) {
                    e.paused = download;
                }
                false
            }
            Event::ReadyToInstall { app, ready } => {
                if let Some(e) = self.entry_mut(&app) {
                    e.ready = ready;
                }
                false
            }
            Event::JobFinished { app, result } => {
                let launcher = app == redrob_launcher_core::catalog::LAUNCHER_ID;
                // What the job was doing when it ended says what a cancellation called off.
                let last = if launcher { self.launcher_job.take() } else { self.entry(&app).and_then(|e| e.job) };
                if let Some(e) = self.entry_mut(&app) {
                    e.job = None;
                }
                let kept = self.entry(&app).is_some_and(|e| e.ready);
                if app == crate::engine_update::ENGINE_APP
                    && self.engine_update.job_finished(result.is_ok())
                    && let Some(engine) = self.backend.engine()
                {
                    // The daemon was up before the update: bring it back on the new binary.
                    engine.start();
                }
                match result {
                    Ok(msg) => self.toast(ctx, msg, false),
                    // The paused card says it all.
                    Err(e) if e == PAUSED => {}
                    Err(e) if e == CANCELLED && launcher => self.toast(ctx, tl!("Update cancelled"), false),
                    // An install's own cancellations end as INSTALL_CANCELLED, so past the
                    // download this is a removal called off while it waited for the installer.
                    Err(e) if e == CANCELLED && matches!(last, Some(JobPhase::Waiting | JobPhase::Removing)) => {
                        self.toast(ctx, tl!("Uninstall cancelled"), false);
                    }
                    Err(e) if e == CANCELLED => self.toast(ctx, tl!("Download cancelled"), false),
                    Err(e) if e == INSTALL_CANCELLED && kept => {
                        self.toast(ctx, tl!("Install cancelled. The download is kept for the next try."), false);
                    }
                    Err(e) if e == INSTALL_CANCELLED => self.toast(ctx, tl!("Install cancelled"), false),
                    Err(e) => self.toast(ctx, e, true),
                }
                true
            }
            Event::LauncherUpdate(r) => {
                let fresh = self.launcher_update.as_ref().is_none_or(|cur| cur.version != r.version);
                self.launcher_update = Some(r.clone());
                // Ask once per new version, unless automatic updates will install it anyway.
                let automatic = self.settings.auto_update && self.caps.self_update;
                if fresh && !automatic && self.dialog.is_none() && self.launcher_job.is_none() {
                    self.dialog = Some(Dialog::LauncherUpdate(r));
                }
                fresh
            }
            Event::CheckStarted => {
                self.checking = true;
                false
            }
            Event::CheckFinished { at, source, rate_limited_until } => {
                self.checking = false;
                self.settings.last_check = Some(at);
                self.last_source = Some(source);
                self.rate_limited_until = rate_limited_until;
                true
            }
        }
    }

    /// One notification per new version of an installed app.
    fn announce_updates(&mut self, host: &mut dyn Host) {
        let mut fresh = Vec::new();
        for e in &self.entries {
            if e.status(self.platform, self.caps.manage_apps) != AppStatus::UpdateAvailable {
                continue;
            }
            let Some(v) = e.latest_label() else { continue };
            if self.settings.notified.get(e.app.id) != Some(&v) {
                fresh.push((e.app.id, e.app.name, v));
            }
        }
        if fresh.is_empty() {
            return;
        }
        if self.settings.notify {
            let (title, body) = match fresh.as_slice() {
                [(_, name, v)] => {
                    let body: String = tl!("Open Redrob Launcher to update.").to_string();
                    (crate::i18n::fmt(tl!("{name} {v} is available"), &[("name", name), ("v", v)]), body)
                }
                many => {
                    let body: String = many.iter().map(|(_, n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(", ");
                    (crate::i18n::fmt(tl!("{count} updates are available"), &[("count", &many.len().to_string())]), body)
                }
            };
            host.notify(&title, &body);
        }
        for (id, _, version) in fresh {
            self.settings.notified.insert(id.to_string(), version);
        }
    }

    /// When auto-updates are enabled, automatically downloads and installs pending updates.
    pub(crate) fn trigger_auto_updates(&mut self) {
        if !self.caps.manage_apps || !self.settings.auto_update {
            return;
        }
        let to_update: Vec<(String, String)> = self
            .entries
            .iter()
            // The engine is a daemon apps are attached to; `engine_update_tick` updates it.
            .filter(|e| e.app.id != crate::engine_update::ENGINE_APP)
            .filter(|e| e.status(self.platform, true) == AppStatus::UpdateAvailable && e.job.is_none() && !e.ready)
            .filter_map(|e| {
                let v = e.latest_label()?;
                if self.auto_update_attempted.contains(&(e.app.id.to_string(), v.clone())) { None } else { Some((e.app.id.to_string(), v)) }
            })
            .collect();
        for (id, version) in to_update {
            log::info!("auto-update: queuing update for {id} to {version}");
            self.auto_update_attempted.insert((id.clone(), version));
            self.send(Command::Install { app: id });
        }
        if self.caps.self_update
            && let Some(r) = &self.launcher_update
            && self.launcher_job.is_none()
        {
            let id = redrob_launcher_core::catalog::LAUNCHER_ID.to_string();
            let v = r.version_label();
            if !self.auto_update_attempted.contains(&(id.clone(), v.clone())) {
                log::info!("auto-update: queuing launcher self-update to {v}");
                self.auto_update_attempted.insert((id, v));
                self.send(Command::UpdateLauncher);
            }
        }
    }

    /// One frame of keeping the shared engine current: see `engine_update`. Runs every frame; the
    /// engine state it reads is cached by the backend, so it costs nothing most frames.
    pub(crate) fn engine_update_tick(&mut self) {
        use crate::engine_update::{Action, ENGINE_APP, Facts};
        if !self.caps.manage_apps {
            return;
        }
        let Some(entry) = self.entry(ENGINE_APP) else { return };
        let status = entry.status(self.platform, true);
        let newer = (status == AppStatus::UpdateAvailable).then(|| entry.latest_label()).flatten();
        let job_running = entry.job.is_some();
        if self.engine_update.last_seen.as_ref() != Some(&(status.clone(), newer.clone())) {
            log::info!(
                "engine update: status {status:?}, installed {:?}, latest {:?}, step {:?}",
                entry.installed_label(),
                entry.latest_label(),
                self.engine_update.step
            );
            self.engine_update.last_seen = Some((status, newer.clone()));
        }
        if newer.is_none() {
            self.engine_update.settled();
        }
        if !(self.settings.engine_auto_update || self.engine_update.forced) && self.engine_update.step == crate::engine_update::Step::Idle {
            // Off: the dialog still shows the newer version and its "Update now".
            self.engine_update.waiting_on.clear();
            return;
        }
        let Some(engine) = self.backend.engine() else { return };
        let facts = Facts {
            attempted: newer.as_ref().is_some_and(|v| self.engine_update.attempted.contains(v)),
            newer,
            job_running,
            engine: engine.state(),
            forced: self.engine_update.forced,
        };
        let before = self.engine_update.waiting_on.clone();
        match self.engine_update.tick(&facts) {
            Action::None => {}
            Action::Wait { apps } => {
                if apps != before {
                    log::info!(
                        "engine update: {} is newer, waiting until no app is attached (now: {})",
                        facts.newer.as_deref().unwrap_or("?"),
                        apps.join(", ")
                    );
                }
            }
            Action::Stop => {
                log::info!("engine update: stopping the daemon before installing {}", facts.newer.as_deref().unwrap_or("?"));
                if let Some(engine) = self.backend.engine() {
                    engine.stop();
                }
            }
            Action::Install { restart } => {
                log::info!("engine update: installing {} (restart after: {restart})", facts.newer.as_deref().unwrap_or("?"));
                self.send(Command::Install { app: ENGINE_APP.to_string() });
            }
        }
    }

    /// The language to publish for the other apps: the one the launcher draws in, `chosen` unless
    /// the setting is `auto`, with the per-app overrides from the settings.
    pub fn language_choice(&self) -> LanguageChoice {
        LanguageChoice {
            language: effective_lang(&self.settings.language).code().to_string(),
            chosen: crate::i18n::Lang::from_code(&self.settings.language).is_some(),
            apps: self.settings.app_languages.clone(),
        }
    }

    /// Draws the launcher.
    pub fn ui(&mut self, ui: &mut egui::Ui, host: &mut dyn Host) {
        let ctx = ui.ctx().clone();
        // Whether something already open owns Escape this frame, read before drawing: a list or a
        // text field handles the key while it is drawn (closing, or giving up focus), so by the
        // page's own check below it would look as if nothing had been open.
        let escape_taken = egui::Popup::is_any_open(&ctx) || ctx.text_edit_focused();
        // Set the current language for this frame based on settings.
        let lang = effective_lang(&self.settings.language);
        set_current(lang);
        let choice = self.language_choice();
        if self.published_language.as_ref() != Some(&choice) {
            host.language_changed(&choice);
            self.published_language = Some(choice);
        }
        // `effective_lang` only picks Chinese once its font has loaded.
        if lang.needs_cjk_font() && !self.cjk_fonts {
            theme::install_fonts(&ctx, true);
            self.cjk_fonts = true;
            self.fonts_ready = false;
        }
        if !self.fonts_ready {
            self.fonts_ready = true;
            ctx.request_repaint();
            return;
        }
        if self.applied_theme != Some(self.settings.theme) {
            theme::apply(&ctx, self.settings.theme);
            self.applied_theme = Some(self.settings.theme);
        }
        let tokens = Tokens::get(&ctx);
        if self.entries.iter().any(|e| e.job.is_some()) || self.checking {
            ctx.request_repaint_after(Duration::from_millis(50));
        }

        // The title bar across the top (Redrob Cowork's 48 px bar on the sunken ground, its hairline
        // under it), then the sidebar card down the left; each page's top bar spans the main area
        // only, on the same base ground as the card's gap. There is no footer: what it said (the
        // last check, GitHub's limit, the platform) is in Settings > Updates.
        // A window the OS decorates (and the web build) has no title bar of ours: the title bar's
        // search, logo and Update sit in a 48 px row over the main area instead.
        let bar = host.title_bar();
        if bar != TitleBar::Native {
            egui::Panel::top("header")
                .frame(egui::Frame::new().fill(tokens.sunken))
                .exact_size(crate::views::HEADER_H)
                .resizable(false)
                .show(ui, |ui| self.header(ui, bar));
        }
        let inset = crate::shell::INSET as i8;
        egui::Panel::left("sidebar")
            .frame(egui::Frame::new().fill(tokens.bg).inner_margin(egui::Margin { left: inset, right: inset, top: inset, bottom: inset }))
            .exact_size(Self::sidebar_width(&ctx))
            .resizable(false)
            .show_separator_line(false)
            .show(ui, |ui| self.sidebar(ui, host));
        if bar == TitleBar::Native {
            egui::Panel::top("chrome")
                .frame(egui::Frame::new().fill(tokens.bg).inner_margin(egui::Margin::symmetric(24, 0)))
                .exact_size(crate::views::HEADER_H)
                .resizable(false)
                .show(ui, |ui| self.header(ui, bar));
        }
        // The Apps page has no top bar: its actions are in the chrome and its filter row.
        if self.page != Page::Apps {
            egui::Panel::top("page-top")
                .frame(egui::Frame::new().fill(tokens.bg).inner_margin(egui::Margin::symmetric(24, 0)))
                .exact_size(crate::shell::PAGE_TOP_H)
                .resizable(false)
                .show(ui, |ui| self.page_top_bar(ui));
        }
        // The right margin is only the scroll bar's gap from the window's edge: the page's own
        // 24 px is inside its scroll area, so the bar hugs the edge like Cowork's.
        let edge = crate::widgets::PAGE_SCROLL_EDGE as i8;
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(tokens.bg).inner_margin(egui::Margin { left: 24, right: edge, top: 20, bottom: 0 }))
            .show(ui, |ui| match self.page {
                Page::Models => self.models_page(ui),
                Page::Engine | Page::Account | Page::Settings => {
                    crate::widgets::page_scroll(ui, "page", |ui| {
                        // A reading measure, like Cowork's content column.
                        ui.set_max_width(720.0);
                        match self.page {
                            Page::Engine => self.engine_page(ui),
                            Page::Account => self.account_page(ui),
                            _ => self.settings_page(ui, host),
                        }
                        // Room under the last control, so nothing ends against the window's edge.
                        ui.add_space(32.0);
                    });
                }
                Page::Apps => {
                    // Above the scroll: the page's right padding, as inside it.
                    egui::Frame::new()
                        .inner_margin(egui::Margin { right: crate::widgets::PAGE_SCROLL_PAD as i8, ..egui::Margin::ZERO })
                        .show(ui, |ui| self.toolbar(ui));
                    ui.add_space(16.0);
                    let ids = self.visible_ids();
                    crate::widgets::page_scroll(ui, "apps", |ui| {
                        if ids.is_empty() {
                            self.empty_state(ui);
                        } else {
                            self.grid(ui, &ids);
                        }
                        ui.add_space(16.0);
                    });
                }
            });
        // Escape leaves a page for Apps, when nothing above the page (a dialog, the feed alert, a
        // menu or a list) or a text field being edited would take the key itself. An ordinary
        // focused control (a button reached by Tab) doesn't use Escape, so it doesn't block it.
        let alert_open = self.alerts.iter().any(|a| !self.settings.dismissed_notices.contains(&a.id));
        if self.page != Page::Apps
            && self.dialog.is_none()
            && !alert_open
            && !escape_taken
            && !egui::Popup::is_any_open(&ctx)
            && !ctx.text_edit_focused()
            && ctx.input(|i| i.key_pressed(egui::Key::Escape))
        {
            self.set_page(Page::Apps);
        }
        self.dialogs(&ctx);
        self.draw_toasts(&ctx);
        if host.title_bar() == TitleBar::Custom {
            crate::widgets::window_edges(&ctx);
        }
    }

    /// The ids to show, filtered, in page order ([`crate::shelves::page_order`], the grid's only
    /// order). Apps kept off the page ([`crate::shelves::shown`]) never appear.
    pub(crate) fn visible_ids(&self) -> Vec<&'static str> {
        let needle = self.search.trim().to_lowercase();
        crate::shelves::page_order()
            .into_iter()
            .filter_map(|id| self.entry(id))
            .filter(|e| self.shown(e))
            .filter(|e| {
                let s = e.status(self.platform, self.caps.manage_apps);
                let keep = match self.settings.filter {
                    Filter::All => true,
                    Filter::Installed => s.is_installed(),
                    Filter::Updates => s == AppStatus::UpdateAvailable,
                    Filter::NotInstalled => !s.is_installed(),
                };
                keep && (needle.is_empty()
                    || e.app.name.to_lowercase().contains(&needle)
                    || e.app.tagline.to_lowercase().contains(&needle)
                    || e.app.like.to_lowercase().contains(&needle))
            })
            .map(|e| e.app.id)
            .collect()
    }

    /// "Redrob API", "GitHub" or "Redrob API and GitHub": the services behind the releases on
    /// screen right now. `None` until something has loaded.
    pub(crate) fn sources_label(&self) -> Option<String> {
        use redrob_launcher_core::protocol::Origin;
        let mut api = false;
        let mut github = false;
        for e in &self.entries {
            if matches!(e.latest, Some(LatestRelease::Found(_) | LatestRelease::None)) {
                match e.origin {
                    Some(Origin::RedrobApi) => api = true,
                    Some(Origin::GitHub) => github = true,
                    None => {}
                }
            }
        }
        match (api, github) {
            (true, true) => Some(crate::i18n::fmt(tl!("{a} and {b}"), &[("a", Origin::RedrobApi.label()), ("b", Origin::GitHub.label())])),
            (true, false) => Some(Origin::RedrobApi.label().to_string()),
            (false, true) => Some(Origin::GitHub.label().to_string()),
            (false, false) => None,
        }
    }

    /// Whether an app is on the page (and in its counts): every app except those kept off it
    /// unless installed ([`crate::shelves::HIDDEN_UNLESS_INSTALLED`]).
    pub(crate) fn shown(&self, e: &AppEntry) -> bool {
        crate::shelves::shown(e.app.id, e.installed.is_some())
    }

    pub(crate) fn count(&self, filter: Filter) -> usize {
        self.entries
            .iter()
            .filter(|e| self.shown(e))
            .filter(|e| {
                let s = e.status(self.platform, self.caps.manage_apps);
                match filter {
                    Filter::All => true,
                    Filter::Installed => s.is_installed(),
                    Filter::Updates => s == AppStatus::UpdateAvailable,
                    Filter::NotInstalled => !s.is_installed(),
                }
            })
            .count()
    }

    /// DS Toasts: 360 wide, raised, radius 8, shadow-lg, a 20 px status icon in its colour, the
    /// text, and a close button. Clicking a toast also dismisses it.
    fn draw_toasts(&mut self, ctx: &egui::Context) {
        use crate::glyphs::Icon;
        let now = ctx.input(|i| i.time);
        self.toasts.retain(|t| t.until > now);
        if self.toasts.is_empty() {
            return;
        }
        ctx.request_repaint_after(Duration::from_millis(250));
        let t = Tokens::get(ctx);
        const W: f32 = 360.0;
        // 24 px in from the window's corner (the page's own padding); there is no footer to clear.
        let anchor = egui::vec2(-24.0, -24.0);
        egui::Area::new(egui::Id::new("toasts")).anchor(egui::Align2::RIGHT_BOTTOM, anchor).order(egui::Order::Foreground).show(ctx, |ui| {
            ui.set_width(W);
            let mut dismiss = None;
            for (i, toast) in self.toasts.iter().enumerate() {
                let (icon, tone) = if toast.error { (Icon::Close, t.bad) } else { (Icon::Check, t.good) };
                let r = egui::Frame::new()
                    .fill(t.raised())
                    .stroke(t.hairline())
                    .corner_radius(egui::CornerRadius::same(theme::radius::MD))
                    .shadow(theme::shadow_lg(&t))
                    .inner_margin(16)
                    .show(ui, |ui| {
                        ui.set_width(W - 32.0);
                        ui.horizontal_top(|ui| {
                            ui.spacing_mut().item_spacing.x = theme::space::S3;
                            let (r, _) = ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());
                            crate::glyphs::paint(ui.painter(), r, icon, tone);
                            let text_w = W - 32.0 - 20.0 - theme::control::SM - 2.0 * theme::space::S3;
                            ui.allocate_ui(egui::vec2(text_w, 0.0), |ui| {
                                ui.set_width(text_w);
                                ui.add(
                                    egui::Label::new(egui::RichText::new(crate::i18n::msg(&toast.text)).size(theme::text::BODY_SM).color(t.ink))
                                        .wrap(),
                                );
                            });
                            crate::widgets::icon_button(ui, Icon::Close, tl!("Dismiss notification"), theme::control::SM).clicked()
                        })
                        .inner
                    });
                if r.inner || r.response.interact(egui::Sense::click()).clicked() {
                    dismiss = Some(i);
                }
                ui.add_space(theme::space::S2);
            }
            if let Some(i) = dismiss {
                self.toasts.remove(i);
            }
        });
    }
}
