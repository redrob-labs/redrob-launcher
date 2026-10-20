//! The account dialog: sign this computer in to a Redrob workspace once, and see which apps can
//! use it. The backend does the work ([`AccountBackend`]); this only shows names and states.

use egui::{RichText, vec2};
use redrob_launcher_core::SUITE;
use redrob_launcher_core::account::{AppLink, ConnectState, LoginState};
use redrob_launcher_core::protocol::LatestRelease;

use crate::app::{Dialog, LauncherUi};
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
            (None, _, LoginState::SignedIn(name)) => {
                HeaderAccount::SignedIn(if name.is_empty() { tl!("Redrob account").to_string() } else { name })
            }
            (None, _, _) => HeaderAccount::SignedOut,
        }
    }

    /// The header's account link: "Sign in", or the workspace's name. Opens the account dialog.
    pub(crate) fn account_link(&mut self, ui: &mut egui::Ui, height: f32) {
        let (label, tip) = match self.header_account() {
            HeaderAccount::None => return,
            HeaderAccount::SignedOut => (tl!("Sign in").to_string(), tl!("Sign in to Redrob once for every app on this computer").to_string()),
            HeaderAccount::Busy => (tl!("Signing in…").to_string(), tl!("Approve the code on console.redrob.ai").to_string()),
            HeaderAccount::BrowserSignIn => (tl!("Signing in…").to_string(), tl!("Finish signing in in your browser").to_string()),
            HeaderAccount::SignedIn(name) => (name, tl!("Your Redrob workspace and the apps that use it").to_string()),
        };
        if widgets::nav_link(ui, &label, &tip, height, matches!(self.dialog, Some(Dialog::Account))).clicked() {
            self.dialog = Some(Dialog::Account);
        }
    }

    pub(crate) fn account_dialog(&mut self, ui: &mut egui::Ui) -> bool {
        let t = Tokens::get(ui.ctx());
        ui.set_width(560.0f32.min(ui.ctx().content_rect().width() - 80.0));
        Self::title(ui, Some(crate::icons::redrob_mark()), tl!("Redrob account"), tl!("One sign-in for every Redrob app on this computer"));
        let Some(acct) = self.backend.account() else { return true };
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
                    ui.spinner();
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
                if widgets::button(ui, tl!("Sign in to Redrob"), Kind::Primary, 180.0).clicked() {
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
        egui::ScrollArea::vertical().max_height(ui.ctx().content_rect().height() * 0.45).show(ui, |ui| {
            for app in &SUITE {
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
                ui.add_space(4.0);
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

        ui.add_space(14.0);
        let mut close = false;
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            close = widgets::button(ui, tl!("Done"), Kind::Secondary, 90.0).clicked();
        });
        close
    }
}
