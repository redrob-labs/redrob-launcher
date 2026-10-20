// ADDED-LINE-RATIONALE 2d11e6ddcb934983:1: Gives `account` direct access to `redrob_launcher_core / account / AppLink` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 2bbee333f7b54c0a:1: Gives `account` direct access to `crate / app / Host` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 7599e90bb97f2a90:1: Gives `account` direct access to `crate / glyphs / Icon` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE d461e3c030dd9462:1: Prevents `header_account` from applying its side effect outside the `acct / workspace / acct` state.
// ADDED-LINE-RATIONALE 8f55ad1ee2075ace:1: Keeps `_ / ConnectState / Starting` in `header_account`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 0cc5e0185cd5f9ec:1: Keeps `w / _ / HeaderAccount` in `header_account`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE e7f8842604804db6:1: Keeps `_ / HeaderAccount / SignedOut` in `header_account`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE f9d5402e2c193ec2:1: Supports `account_menu` while preserving the nearby constraint: the one way into Settings. Escape or a click outside closes it.
// ADDED-LINE-RATIONALE d0696b3c6967b28a:1: Freezes `version` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e40ba8f4bdba9707:1: Freezes `let` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 82f21eef47b5fc7a:1: Freezes `HeaderAccount` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 03b3e93b355bc19f:1: Freezes `HeaderAccount` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fcb90fc93d99addb:1: Delegates `account_menu`'s `tl / Log / in` work to `tl` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6758e24404f18202:1: Bounds the `account_menu` implementation here so `tl / Log / in` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e50cc2c3c923da9e:1: Freezes `HeaderAccount` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 166fc5302808dd94:1: Delegates `account_menu`'s `tl / Log / in` work to `tl` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6758e24404f18202:2: Bounds the `account_menu` implementation here so `tl / Log / in` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 21c936ae03c767ce:1: Freezes `HeaderAccount` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2d7e3cf0a6f1e35b:1: Freezes `initial` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 11e77f57c1686150:1: Delegates `account_menu`'s `tl / Signed / in` work to `tl` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6758e24404f18202:3: Bounds the `account_menu` implementation here so `tl / Signed / in` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 175aee071b9f6919:1: Freezes `current` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f14bd62ddc13bb4b:1: Freezes `row` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 066362912824ffb3:1: Supports `account_menu` while preserving the nearby constraint: DS Menu: as wide as the row (at least 220), above it.
// ADDED-LINE-RATIONALE a141e38a5c9a8854:1: Freezes `quit_offered` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 722e05f1adb15b54:1: Freezes `go` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c256aaa852667056:1: Freezes `quit` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 260677123ad6237a:1: Delegates `account_menu`'s `egui / Popup / menu` work to `egui::Popup::menu` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a123c2755c62e228:1: Freezes `ui` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE aa16b2352ae08a07:1: Freezes `if` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 99f8efa0254e75a7:1: Prevents `account_menu` from applying its side effect outside the `widgets / menu_item / ui` state.
// ADDED-LINE-RATIONALE f0e97cd79577b108:1: Freezes `go` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 63cab4ace5f4e869:1: Delegates `account_menu`'s `ui / close` work to `ui.close` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7cf4ca7d6bc42bb2:1: Bounds the `account_menu` implementation here so `ui / close` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6ddbb05679f3a3cc:1: Delegates `account_menu`'s `ui / separator` work to `ui.separator` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6758e24404f18202:4: Bounds the `account_menu` implementation here so `ui / separator` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a7ce4abf04f26790:1: Prevents `account_menu` from applying its side effect outside the `widgets / menu_item / ui` state.
// ADDED-LINE-RATIONALE f144d6de5155c724:1: Freezes `go` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3e51002fddf67e79:1: Delegates `account_menu`'s `ui / close` work to `ui.close` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6758e24404f18202:5: Bounds the `account_menu` implementation here so `ui / close` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 662966a7a406c690:1: Prevents `account_menu` from applying its side effect outside the `quit_offered / widgets / menu_item` state.
// ADDED-LINE-RATIONALE 137c8d9e333b8c7e:1: Freezes `quit` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3e51002fddf67e79:2: Delegates `account_menu`'s `ui / close` work to `ui.close` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6758e24404f18202:6: Bounds the `account_menu` implementation here so `ui / close` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 86fb46fa7bcbfb5c:1: Delegates `account_menu`'s `ui / separator` work to `ui.separator` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0d0be27a91ca79ce:1: Delegates `account_menu`'s `widgets / menu_item / ui` work to `widgets::menu_item` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2ba63852bc41f78e:1: Makes `widgets / menu_item / ui` an explicit part of `account_menu` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 8bc8ab043953d5ad:1: Freezes `if` for `account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1c4c416701bb43ae:1: Delegates `account_menu`'s `set_page / page` work to `self.set_page` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e697a6b0147b2252:2: Bounds the `account_menu` implementation here so `set_page / page` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5dfd420bb2ffe8b2:1: Prevents `account_menu` from applying its side effect outside the `quit` state.
// ADDED-LINE-RATIONALE 954d7ef22af0a08a:1: Delegates `account_menu`'s `host / quit` work to `host.quit` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e85ca806f14aa257:1: Supports `account_page` while preserving the nearby constraint: The account page (its title is in the page's top bar).
// ADDED-LINE-RATIONALE 7a432bfe20cfadd9:1: Freezes `Some` for `account_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 29ec8bd25f0086b1:1: Delegates `account_page`'s `widgets / loader_inline / ui` work to `widgets::loader_inline` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0ec82adea8b5e85e:1: Prevents `account_page` from applying its side effect outside the `widgets / button_sized / ui` state.
// ADDED-LINE-RATIONALE 13ec8a2cc97b324d:1: Supports `account_page` while preserving the nearby constraint: The page scrolls as a whole; the apps are a card of rows on hairlines.
// ADDED-LINE-RATIONALE 1351053746ab35eb:1: Delegates `account_page`'s `ui / set_width / ui` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f0008e21d4b23582:1: Supports `account_page` while preserving the nearby constraint: Page order, and only the apps the Apps page shows.
// ADDED-LINE-RATIONALE 4cd401c77b2b7c3f:1: Delegates `account_page`'s `crate / shelves / page_order` work to `crate::shelves::page_order` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8bd9303cde99829c:1: Applies `account_page` consistently to every `i / app / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 8d51db430f0ce1cc:1: Prevents `account_page` from applying its side effect outside the `i` state.
// ADDED-LINE-RATIONALE 7f8ca8859ca8ea3f:1: Delegates `account_page`'s `widgets / row_rule / ui` work to `widgets::row_rule` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7cf4ca7d6bc42bb2:4: Bounds the `account_page` implementation here so `widgets / row_rule / ui` cannot leak into the adjacent declaration or control path.
//! The account page and the sidebar's account menu: sign this computer in to a Redrob workspace once, and see which apps can
//! use it. The backend does the work ([`AccountBackend`]); this only shows names and states.

use egui::{RichText, vec2};
use redrob_launcher_core::SUITE;
use redrob_launcher_core::account::{AppLink, ConnectState, LoginState};
use redrob_launcher_core::protocol::LatestRelease;

use crate::app::{Host, LauncherUi, Page};
use crate::glyphs::Icon;
use crate::i18n::{fmt, msg};
use crate::theme::{self, Tokens};
use crate::widgets::{self, Kind};

/// What the header shows for the account.
pub(crate) enum HeaderAccount {
    /// No account backend (the web build).
    None,
    SignedOut,
    Busy,
    /// The Redrob account's browser sign-in is open.
    BrowserSignIn,
    SignedIn(String),
}

impl LauncherUi {
    pub(crate) fn header_account(&mut self) -> HeaderAccount {
        let Some(acct) = self.backend.account() else { return HeaderAccount::None };
        match (acct.workspace(), acct.state(), acct.login()) {
            (_, ConnectState::Starting | ConnectState::Waiting(_), _) => HeaderAccount::Busy,
            (_, _, LoginState::SigningIn) => HeaderAccount::BrowserSignIn,
            (Some(w), _, _) => HeaderAccount::SignedIn(w.account_name),
            (None, _, LoginState::SignedIn(name)) => HeaderAccount::SignedIn(if name.is_empty() { tl!("Redrob account").to_string() } else { name }),
            (None, _, _) => HeaderAccount::SignedOut,
        }
    }

    /// The sidebar foot's one account row (Redrob Cowork's account menu): an avatar, the account
    /// ("Log in or sign up", or the workspace; "Settings" with no account backend) and a status
    /// line. A click opens a menu above it: the account page, Settings, Quit and the version. It is
    /// the one way into Settings. Escape or a click outside closes it.
    pub(crate) fn account_menu(&mut self, ui: &mut egui::Ui, rail: bool, host: &mut dyn Host) {
        let version = fmt(tl!("Redrob Launcher {version}"), &[("version", env!("CARGO_PKG_VERSION"))]);
        let (title, status, initial, account_item) = match self.header_account() {
            HeaderAccount::None => (tl!("Settings").to_string(), version.clone(), None, None),
            HeaderAccount::SignedOut => {
                (tl!("Log in or sign up").to_string(), tl!("Not connected").to_string(), None, Some(tl!("Log in or sign up")))
            }
            HeaderAccount::Busy => {
                (tl!("Log in or sign up").to_string(), tl!("Waiting for approval").to_string(), None, Some(tl!("Log in or sign up")))
            }
            HeaderAccount::BrowserSignIn => {
                (tl!("Log in or sign up").to_string(), tl!("Finish signing in in your browser").to_string(), None, Some(tl!("Log in or sign up")))
            }
            HeaderAccount::SignedIn(name) => {
                let initial = name.chars().next();
                (name, tl!("Signed in").to_string(), initial, Some(tl!("Redrob account")))
            }
        };
        let current = matches!(self.page, Page::Account | Page::Settings);
        let row = widgets::account_row(ui, &title, &status, initial, current, rail, &status);
        // DS Menu: as wide as the row (at least 220), above it.
        let width = if rail { 220.0 } else { row.rect.width().max(220.0) } - 2.0 * theme::space::S2;
        let quit_offered = self.caps.manage_apps;
        let mut go = None;
        let mut quit = false;
        egui::Popup::menu(&row).align(egui::RectAlign::TOP_START).gap(theme::space::S1).show(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            if let Some(label) = account_item {
                if widgets::menu_item(ui, Some(Icon::Account), label, true, width).clicked() {
                    go = Some(Page::Account);
                    ui.close();
                }
                ui.separator();
            }
            if widgets::menu_item(ui, Some(Icon::Settings), tl!("Settings"), true, width).clicked() {
                go = Some(Page::Settings);
                ui.close();
            }
            if quit_offered && widgets::menu_item(ui, None, tl!("Quit Redrob Launcher"), true, width).clicked() {
                quit = true;
                ui.close();
            }
            ui.separator();
            widgets::menu_item(ui, None, &version, false, width);
        });
        if let Some(page) = go {
            self.set_page(page);
        }
        if quit {
            host.quit();
        }
    }

    /// The account page (its title is in the page's top bar).
    pub(crate) fn account_page(&mut self, ui: &mut egui::Ui) {
        let t = Tokens::get(ui.ctx());
        let Some(acct) = self.backend.account() else { return };
        let workspace = acct.workspace();
        let state = acct.state();
        let mut action: Option<fn(&mut dyn redrob_launcher_core::account::AccountBackend)> = None;

        // The Redrob account (redrob-auth), when the backend offers it.
        let login = acct.login();
        if login != LoginState::Unavailable {
            ui.horizontal(|ui| {
                match &login {
                    LoginState::SignedIn(name) => {
                        let name = if name.is_empty() { tl!("Redrob account") } else { name.as_str() };
                        ui.label(RichText::new(fmt(tl!("Account: {name}"), &[("name", name)])).family(theme::medium()).color(t.ink_strong));
                    }
                    LoginState::SigningIn => {
                        ui.spinner();
                        ui.label(RichText::new(tl!("Finish signing in in your browser…")).color(t.muted));
                        // The browser hands the sign-in back on its own; check for it.
                        ui.ctx().request_repaint_after(std::time::Duration::from_secs(1));
                    }
                    LoginState::Failed(why) => {
                        ui.label(RichText::new(msg(why)).color(t.bad));
                    }
                    LoginState::SignedOut | LoginState::Unavailable => {
                        ui.label(RichText::new(tl!("Sign in with your Redrob account in the browser.")).color(t.muted));
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| match &login {
                    LoginState::SignedIn(_) => {
                        if widgets::button(ui, tl!("Sign out"), Kind::Secondary, 100.0)
                            .on_hover_text(tl!("Signs the Redrob account out on this computer"))
                            .clicked()
                        {
                            action = Some(|a| a.sign_out());
                        }
                    }
                    LoginState::SigningIn => {
                        if widgets::button(ui, tl!("Cancel"), Kind::Secondary, 90.0).clicked() {
                            action = Some(|a| a.sign_out());
                        }
                    }
                    _ => {
                        if widgets::button(ui, tl!("Sign in with browser"), Kind::Secondary, 0.0).clicked() {
                            action = Some(|a| a.sign_in());
                        }
                    }
                });
            });
            ui.add_space(14.0);
        }

        match (&workspace, &state) {
            (_, ConnectState::Starting) => {
                ui.horizontal(|ui| {
                    widgets::loader_inline(ui, 16.0, Tokens::get(ui.ctx()).accent);
                    ui.label(RichText::new(tl!("Asking console.redrob.ai for a code…")).color(t.muted));
                });
            }
            (_, ConnectState::Waiting(p)) => {
                ui.label(RichText::new(tl!("Approve this code on console.redrob.ai:")).color(t.muted));
                ui.add_space(6.0);
                ui.label(RichText::new(&p.user_code).size(28.0).family(theme::semibold()).color(t.ink_strong));
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    widgets::hud_link(ui, tl!("Open console.redrob.ai"), &p.url);
                    ui.add_space(12.0);
                    let left = format!("{}:{:02}", p.seconds_left / 60, p.seconds_left % 60);
                    widgets::hud(ui, &fmt(tl!("{time} left"), &[("time", &left)]), t.faint);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if widgets::button(ui, tl!("Cancel"), Kind::Secondary, 90.0).clicked() {
                            action = Some(|a| a.cancel());
                        }
                    });
                });
                // The countdown moves even when nothing else does.
                ui.ctx().request_repaint_after(std::time::Duration::from_secs(1));
            }
            (Some(w), _) => {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(fmt(tl!("Workspace: {name}"), &[("name", &w.account_name)])).family(theme::medium()).color(t.ink_strong),
                        );
                        if !w.key_name.is_empty() {
                            widgets::hud(ui, &fmt(tl!("Key: {name}"), &[("name", &w.key_name)]), t.faint);
                        }
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if widgets::button(ui, tl!("Sign out"), Kind::Secondary, 100.0)
                            .on_hover_text(tl!("Forgets the key on this computer. Revoke it on console.redrob.ai to stop it working everywhere."))
                            .clicked()
                        {
                            action = Some(|a| a.disconnect());
                        }
                    });
                });
            }
            (None, s) => {
                if let ConnectState::Failed(why) = s {
                    ui.label(RichText::new(msg(why)).color(t.bad));
                    ui.add_space(6.0);
                }
                ui.label(RichText::new(tl!("Sign in once and every Redrob app on this computer can use your workspace. You approve a short code on console.redrob.ai; nobody copies a key.")).color(t.muted));
                ui.add_space(10.0);
                if widgets::button_sized(ui, tl!("Sign in to Redrob"), Kind::Primary, widgets::Size::Lg, 180.0).clicked() {
                    action = Some(|a| a.connect());
                }
            }
        }
        if let Some(f) = action {
            f(acct);
        }

        ui.add_space(18.0);
        widgets::hud(ui, tl!("Apps"), t.faint);
        ui.add_space(6.0);
        let signed_in = workspace.is_some();
        let mut connect: Option<&'static str> = None;
        // The page scrolls as a whole; the apps are a card of rows on hairlines.
        widgets::card_frame(&t).show(ui, |ui| {
            ui.set_width(ui.available_width());
            // Page order, and only the apps the Apps page shows.
            let apps: Vec<&'static redrob_launcher_core::CraftApp> =
                crate::shelves::page_order().into_iter().filter_map(|id| self.entry(id)).filter(|e| self.shown(e)).map(|e| e.app).collect();
            for (i, app) in apps.into_iter().enumerate() {
                if i > 0 {
                    widgets::row_rule(ui, 4.0);
                }
                let entry = self.entry(app.id);
                let installed = entry.and_then(|e| e.installed.as_ref()).and_then(|i| i.version.clone());
                let latest = entry.and_then(|e| e.latest.as_ref()).and_then(|l| match l {
                    LatestRelease::Found(r) => Some(r.clone()),
                    _ => None,
                });
                let link = self.backend.account().map_or(AppLink::Unsupported, |a| a.app_link(app.id, installed.as_ref(), latest.as_ref()));
                ui.horizontal(|ui| {
                    if let Some(icon) = crate::icons::app(app.id) {
                        ui.add(egui::Image::new(icon).fit_to_exact_size(vec2(22.0, 22.0)));
                    }
                    ui.label(RichText::new(app.name).color(t.ink));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (text, tone, button) = match &link {
                            AppLink::Shared => (tl!("Uses this sign-in"), crate::model::Tone::Good, None),
                            AppLink::SharedSoon => (tl!("Next release uses this sign-in"), crate::model::Tone::Info, None),
                            AppLink::SharedAfterUpdate => (tl!("Update to use this sign-in"), crate::model::Tone::Info, None),
                            AppLink::NoKey => (tl!("Needs no sign-in"), crate::model::Tone::Faint, None),
                            AppLink::Connected { same_key: true, .. } => (tl!("Connected"), crate::model::Tone::Good, None),
                            AppLink::Connected { same_key: false, .. } => {
                                (tl!("Connected with another key"), crate::model::Tone::Info, Some(tl!("Use this sign-in")))
                            }
                            AppLink::NotConnected => (tl!("Not connected"), crate::model::Tone::Neutral, Some(tl!("Connect"))),
                            AppLink::Unsupported => (tl!("Not supported yet"), crate::model::Tone::Faint, None),
                        };
                        if let Some(label) = button {
                            let clicked = ui
                                .add_enabled_ui(signed_in, |ui| {
                                    widgets::button(ui, label, Kind::Secondary, 0.0)
                                        .on_disabled_hover_text(tl!("Sign in first"))
                                        .on_hover_text(tl!("Gives the app this computer's workspace key"))
                                        .clicked()
                                })
                                .inner;
                            if clicked {
                                connect = Some(app.id);
                            }
                        }
                        widgets::badge(ui, text, widgets::tone_color(&t, tone));
                    });
                });
            }
        });
        if let Some(id) = connect {
            let ctx = ui.ctx().clone();
            let name = SUITE.iter().find(|a| a.id == id).map_or(id, |a| a.name);
            match self.backend.account().map(|a| a.connect_app(id)) {
                Some(Ok(())) => self.toast(&ctx, fmt(tl!("{name} is connected. Restart it if it's open."), &[("name", name)]), false),
                Some(Err(e)) => self.toast(&ctx, msg(&e), true),
                None => {}
            }
        }
    }
}
