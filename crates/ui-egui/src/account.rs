//! The account page and the sidebar's account menu: sign this computer in to a Redrob workspace once, and see which apps can
//! use it. The backend does the work ([`AccountBackend`]); this only shows names and states.

use egui::{RichText, vec2};
use redrob_launcher_core::SUITE;
use redrob_launcher_core::account::{AppLink, ConnectState};
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
    SignedIn(String),
}

impl LauncherUi {
    pub(crate) fn header_account(&mut self) -> HeaderAccount {
        let Some(acct) = self.backend.account() else { return HeaderAccount::None };
        match (acct.workspace(), acct.state()) {
            (_, ConnectState::Starting | ConnectState::Waiting(_)) => HeaderAccount::Busy,
            (Some(w), _) => HeaderAccount::SignedIn(w.account_name),
            (None, _) => HeaderAccount::SignedOut,
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
            for (i, app) in SUITE.iter().enumerate() {
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
