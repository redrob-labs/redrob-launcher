//! Using this PC from the web and the phone ("desk"): the Code card's line, the AI engine page's
//! section with sign-in and the connected devices, and the "allow this device once" prompt.
//!
//! The shared Redrob Code daemon holds the relay connection and the trusted devices; the backend
//! ([`redrob_launcher_core::desk::DeskBackend`]) keeps it running with desk on and asks it every
//! few seconds. This only shows what it said and sends the person's answers.

use std::collections::BTreeSet;
use std::time::Duration;

use egui::{RichText, Sense};
use redrob_launcher_core::desk::{DeskState, DeskView, DeviceKind, POLL_SECS, PendingDevice, new_pending, view};

use crate::app::{Host, LauncherUi, Page};
use crate::i18n::{fmt, msg, tn};
use crate::model::Tone;
use crate::theme::{self, Tokens};
use crate::widgets::{self, Kind};

/// Redrob Code's suite id: its daemon is the one desk runs in.
pub(crate) const CODE_APP: &str = "redrob-code";

#[derive(Default)]
pub(crate) struct DeskUi {
    /// The start-up `ensure` was sent (once per run, when Code is installed).
    started: bool,
    /// Pending codes already announced, so each gets one notification.
    seen: BTreeSet<String>,
    /// Pending codes the person already answered, until the daemon drops them.
    answered: BTreeSet<String>,
    /// The device asking now: the prompt shows until it is answered or gone.
    pub(crate) asking: Option<PendingDevice>,
    /// Bring the window forward at the next tick.
    front: bool,
}

enum Ask {
    Login,
    Logout,
    Enable,
    Revoke(String),
}

/// "Phone", "Web browser": what kind of device it is.
pub(crate) fn kind_label(kind: DeviceKind) -> &'static str {
    match kind {
        DeviceKind::Phone => tl!("Phone"),
        DeviceKind::Web => tl!("Web browser"),
        DeviceKind::Other => tl!("Device"),
    }
}

/// The device's own name, or its kind when it sent none.
pub(crate) fn device_name(p: &PendingDevice) -> String {
    if p.name.is_empty() { kind_label(p.kind).to_string() } else { p.name.clone() }
}

impl LauncherUi {
    /// Makes sure the shared daemon runs with desk on: at start with Code installed, on
    /// `redrob://open/code` and on the Code card's Open.
    pub(crate) fn desk_ensure(&mut self) {
        if let Some(desk) = self.backend.desk() {
            desk.ensure();
        }
    }

    /// A device asked to connect since the last call: the shell brings the window forward.
    pub fn take_bring_to_front(&mut self) -> bool {
        std::mem::take(&mut self.desk.front)
    }

    /// One tick of desk: start it once, then announce new pending devices (an OS notification and
    /// the prompt, brought to the front). Runs every frame and in the tray; the state it reads is
    /// cached by the backend.
    pub(crate) fn desk_tick(&mut self, ctx: &egui::Context, host: &mut dyn Host) {
        if !self.caps.manage_apps {
            return;
        }
        let installed = self.entry(CODE_APP).is_some_and(|e| e.detected && e.installed.is_some());
        if installed && !self.desk.started {
            self.desk.started = true;
            self.desk_ensure();
        }
        let Some(desk) = self.backend.desk() else { return };
        let state = desk.state();
        match &state {
            DeskState::Up(status) => {
                let fresh: Vec<PendingDevice> = new_pending(&self.desk.seen, status).into_iter().cloned().collect();
                for p in &fresh {
                    let title = fmt(tl!("{name} wants to connect to this PC."), &[("name", &device_name(p))]);
                    let body = fmt(tl!("Code {code}. Allow or deny it in Redrob Launcher."), &[("code", &p.code)]);
                    host.notify(&title, &body);
                    self.desk.front = true;
                }
                self.desk.seen = status.pending.iter().map(|p| p.code.clone()).collect();
                let seen = &self.desk.seen;
                self.desk.answered.retain(|c| seen.contains(c));
                if self.desk.asking.as_ref().is_some_and(|a| !seen.contains(&a.code)) {
                    // Answered on another surface, or expired.
                    self.desk.asking = None;
                }
                if self.desk.asking.is_none() {
                    let answered = &self.desk.answered;
                    self.desk.asking = status.pending.iter().find(|p| !answered.contains(&p.code)).cloned();
                }
            }
            // A request is running; what it changes shows on the next look.
            DeskState::Busy => {}
            _ => {
                self.desk.asking = None;
                self.desk.seen.clear();
                self.desk.answered.clear();
            }
        }
        if matches!(state, DeskState::Up(_) | DeskState::Busy) {
            ctx.request_repaint_after(Duration::from_secs(POLL_SECS));
        }
    }

    /// The person answered the prompt: allow or deny the device.
    pub(crate) fn desk_answer(&mut self, code: &str, allow: bool) {
        self.desk.answered.insert(code.to_string());
        self.desk.asking = None;
        if let Some(desk) = self.backend.desk() {
            if allow {
                desk.approve(code);
            } else {
                desk.deny(code);
            }
        }
    }

    /// The Code card's line under its description, in place of the version, when desk has
    /// something to say: sign in, or how many devices are connected. Returns whether it drew.
    pub(crate) fn desk_card_line(&mut self, ui: &mut egui::Ui, font: &egui::FontId) -> bool {
        let Some(desk) = self.backend.desk() else { return false };
        let (text, login) = match view(&desk.state()) {
            DeskView::SignIn { .. } => (tl!("Use this PC from the web and phone · Sign in").to_string(), true),
            DeskView::Ready { devices, .. } => (tn(devices.len() as u64, "Web and phone: {n} device", "Web and phone: {n} devices"), false),
            _ => return false,
        };
        let t = Tokens::get(ui.ctx());
        let r = ui.add(egui::Label::new(RichText::new(text).font(font.clone()).color(t.accent)).truncate().sense(Sense::click()));
        if r.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            if !login {
                self.set_page(Page::Engine);
            } else if let Some(desk) = self.backend.desk() {
                desk.login();
            }
        }
        true
    }

    /// The AI engine page's desk section: sign in, the connected devices with Disconnect, sign out.
    pub(crate) fn desk_section(&mut self, ui: &mut egui::Ui) {
        use theme::space;
        let Some(desk) = self.backend.desk() else { return };
        let shown = view(&desk.state());
        let error = desk.last_error();
        if shown == DeskView::Hidden && error.is_none() {
            return;
        }
        let t = Tokens::get(ui.ctx());
        let mut ask: Option<Ask> = None;
        ui.add_space(space::S6);
        widgets::shelf_heading(ui, tl!("Use this PC from the web and phone"));
        ui.add_space(space::S3);
        widgets::card_frame(&t).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 0.0;
            let body = |ui: &mut egui::Ui, text: &str| {
                ui.add(egui::Label::new(RichText::new(text).size(theme::text::BODY_SM).color(t.muted)).wrap());
            };
            // The footer's actions, the trailing one first.
            let mut actions: Vec<(&str, Kind, Ask)> = Vec::new();
            match &shown {
                DeskView::Hidden => body(ui, tl!("The AI engine isn't running. Open Redrob Code to start it.")),
                DeskView::Working => {
                    widgets::tight_row(ui, |ui| {
                        widgets::loader_inline(ui, 16.0, t.accent);
                        ui.label(RichText::new(tl!("Working…")).size(theme::text::BODY_SM).color(t.muted));
                    });
                    ui.ctx().request_repaint_after(Duration::from_millis(250));
                }
                DeskView::NeedsUpdate => body(ui, tl!("Update Redrob Code to use this PC from Redrob Chat on the web and the phone app.")),
                DeskView::Problem(e) => {
                    ui.add(egui::Label::new(RichText::new(msg(e)).size(theme::text::BODY_SM).color(widgets::tone_color(&t, Tone::Warn))).wrap());
                }
                DeskView::SignIn { login } => {
                    body(ui, tl!("Sign in with your Redrob account, and Redrob Chat on the web and the phone app can use this PC."));
                    if let Some(l) = login {
                        ui.add_space(space::S2);
                        widgets::hud(ui, &fmt(tl!("Finish signing in in your browser. Code {code}"), &[("code", &l.user_code)]), t.faint);
                    }
                    let label = if login.is_some() { tl!("Open the sign-in page again") } else { tl!("Sign in") };
                    actions.push((label, Kind::Primary, Ask::Login));
                }
                DeskView::Ready { email, enabled, connected, devices } => {
                    widgets::tight_row(ui, |ui| {
                        if *connected {
                            widgets::badge(ui, tl!("Connected"), widgets::tone_color(&t, Tone::Good));
                        } else {
                            widgets::badge(ui, tl!("Not connected"), widgets::tone_color(&t, Tone::Neutral));
                        }
                        if let Some(email) = email {
                            widgets::hud(ui, email, t.faint);
                        }
                    });
                    ui.add_space(space::S3);
                    if !*enabled {
                        body(ui, tl!("Turned off. Turn it on so your devices can reach this PC."));
                        ui.add_space(space::S3);
                    }
                    widgets::hud(ui, tl!("Connected devices"), t.muted);
                    ui.add_space(space::S2);
                    if devices.is_empty() {
                        body(ui, tl!("No device yet. Open Redrob Chat on the web or the phone app, then allow it here."));
                    }
                    for d in devices {
                        widgets::row_rule(ui, 0.0);
                        ui.add_space(space::S2);
                        widgets::tight_row(ui, |ui| {
                            let name = if d.name.is_empty() { kind_label(d.kind) } else { d.name.as_str() };
                            ui.label(RichText::new(name).family(theme::medium()).color(t.ink_strong));
                            widgets::meta_sep(ui);
                            widgets::hud(ui, kind_label(d.kind), t.faint);
                            if d.added_at > 0 {
                                widgets::meta_sep(ui);
                                widgets::hud(ui, &crate::views::date_text(d.added_at / 1000), t.faint);
                            }
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if widgets::button_sized(ui, tl!("Disconnect"), Kind::Secondary, widgets::Size::Sm, 0.0).clicked() {
                                    ask = Some(Ask::Revoke(d.id.clone()));
                                }
                            });
                        });
                        ui.add_space(space::S2);
                    }
                    actions.push((tl!("Sign out"), Kind::Secondary, Ask::Logout));
                    if !*enabled {
                        actions.insert(0, (tl!("Turn on"), Kind::Primary, Ask::Enable));
                    }
                }
            }
            if let Some(e) = &error {
                ui.add_space(space::S2);
                ui.add(egui::Label::new(RichText::new(msg(e)).size(theme::text::BODY_SM).color(widgets::tone_color(&t, Tone::Warn))).wrap());
            }
            if !actions.is_empty() {
                ui.add_space(space::S4);
                let (rule, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), Sense::hover());
                ui.painter().hline(rule.left() - space::S5..=rule.right() + space::S5, rule.center().y, t.hairline());
                ui.add_space(space::S4);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        for (label, kind, what) in actions {
                            if widgets::button(ui, label, kind, 0.0).clicked() {
                                ask = Some(what);
                            }
                        }
                    });
                });
            }
        });
        let Some(what) = ask else { return };
        let Some(desk) = self.backend.desk() else { return };
        match what {
            Ask::Login => desk.login(),
            Ask::Logout => desk.logout(),
            Ask::Enable => desk.ensure(),
            Ask::Revoke(id) => desk.revoke(&id),
        }
    }
}
