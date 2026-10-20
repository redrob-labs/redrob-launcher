//! The engine dialog: the shared Redrob Code daemon every Redrob app on this computer runs its AI
//! through. Whether it is up, which version, which apps are attached, and Start / Stop / Restart.
//! The backend does the work ([`EngineBackend`]); this only shows states and sends requests.

use egui::{RichText, vec2};
use redrob_launcher_core::engine::{EngineState, app_display_name};
use redrob_launcher_core::protocol::Command;

use crate::app::{LauncherUi, Page};
use crate::glyphs::Icon;
use crate::i18n::{fmt, msg};
use crate::model::Tone;
use crate::theme::{self, Tokens};
use crate::widgets::{self, Kind};

/// The engine's suite id, for its icon and its install button.
const ENGINE_APP: &str = "redrob-code";

enum Ask {
    Start,
    Stop,
    Restart,
    Install,
    /// Update the engine now, even while apps are attached (they reconnect on their own).
    UpdateNow,
}

fn minutes(secs: u64) -> String {
    secs.div_ceil(60).to_string()
}

impl LauncherUi {
    /// The sidebar's engine row, on backends that can reach the daemon: its state as a dot.
    pub(crate) fn engine_nav(&mut self, ui: &mut egui::Ui, rail: bool) {
        let t = Tokens::get(ui.ctx());
        let Some(engine) = self.backend.engine() else { return };
        let (label, tip, dot): (String, String, egui::Color32) = match engine.state() {
            EngineState::Running(r) => (
                if r.attached.is_empty() { tl!("Engine").to_string() } else { fmt(tl!("Engine · {n}"), &[("n", &r.attached.len().to_string())]) },
                tl!("The Redrob Code engine is running; every app shares it").to_string(),
                t.good,
            ),
            EngineState::Busy => (tl!("Engine…").to_string(), tl!("The engine is starting or stopping").to_string(), t.accent),
            EngineState::Stopped => (tl!("Engine off").to_string(), tl!("The Redrob Code engine is not running").to_string(), t.faint),
            EngineState::NotInstalled => {
                (tl!("Engine").to_string(), tl!("Install Redrob Code, the engine every app runs its AI through").to_string(), t.warn)
            }
        };
        if widgets::nav_row(ui, Icon::Engine, &label, &tip, widgets::NavMeta::Dot(dot), self.page == Page::Engine, rail).clicked() {
            if let Some(engine) = self.backend.engine() {
                engine.refresh();
            }
            self.set_page(Page::Engine);
        }
    }

    /// The engine page (its title is in the page's top bar).
    pub(crate) fn engine_page(&mut self, ui: &mut egui::Ui) {
        let t = Tokens::get(ui.ctx());
        // The newer engine, when there is one, and what holds its install back.
        let newer: Option<(String, String)> = self.entry(ENGINE_APP).and_then(|e| {
            (e.status(self.platform, true) == redrob_launcher_core::AppStatus::UpdateAvailable)
                .then(|| Some((e.installed_label()?, e.latest_label()?)))
                .flatten()
        });
        let engine_job = self.entry(ENGINE_APP).is_some_and(|e| e.job.is_some());
        let updating = engine_job || self.engine_update.step != crate::engine_update::Step::Idle;
        let waiting_on = self.engine_update.waiting_on.clone();
        let Some(engine) = self.backend.engine() else { return };
        let state = engine.state();
        let error = engine.last_error();
        let mut ask: Option<Ask> = None;

        use theme::space;
        // What the footer offers in this state: (label, kind, request), the trailing one first.
        let actions: Vec<(&str, Kind, Ask)> = match &state {
            EngineState::NotInstalled => vec![(tl!("Install Redrob Code"), Kind::Primary, Ask::Install)],
            EngineState::Stopped => vec![(tl!("Start"), Kind::Primary, Ask::Start)],
            EngineState::Busy => Vec::new(),
            EngineState::Running(_) => vec![(tl!("Restart"), Kind::Secondary, Ask::Restart), (tl!("Stop"), Kind::Secondary, Ask::Stop)],
        };
        // A DS Card: head (state and version), body (what that means, the apps using it), and a
        // footer on a full-width hairline with the actions on the trailing edge.
        widgets::card_frame(&t).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 0.0;
            let body = |ui: &mut egui::Ui, text: &str| {
                ui.add(egui::Label::new(RichText::new(text).size(theme::text::BODY_SM).color(t.muted)).wrap());
            };
            match &state {
                EngineState::NotInstalled => {
                    widgets::badge(ui, tl!("Not installed"), widgets::tone_color(&t, Tone::Warn));
                    ui.add_space(space::S3);
                    body(ui, tl!("Redrob Code is the engine Cowork, Office, Design, Browser, Film and CAD run their AI through. Install it once and they share it."));
                }
                EngineState::Stopped => {
                    widgets::badge(ui, tl!("Not running"), widgets::tone_color(&t, Tone::Neutral));
                    ui.add_space(space::S3);
                    body(ui, tl!("The first app that needs the engine starts it, and it stops on its own when no app has used it for a while. You can also start it here."));
                }
                EngineState::Busy => {
                    widgets::tight_row(ui, |ui| {
                        widgets::loader_inline(ui, 16.0, t.accent);
                        ui.label(RichText::new(tl!("Working…")).size(theme::text::BODY_SM).color(t.muted));
                    });
                    ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
                }
                EngineState::Running(r) => {
                    widgets::tight_row(ui, |ui| {
                        widgets::badge(ui, tl!("Running"), widgets::tone_color(&t, Tone::Good));
                        widgets::hud(ui, &fmt(tl!("version {v}"), &[("v", &r.version)]), t.faint);
                        widgets::meta_sep(ui);
                        widgets::hud(ui, &r.url, t.faint);
                    });
                    ui.add_space(space::S3);
                    if r.attached.is_empty() {
                        let text = match r.exits_in_secs() {
                            Some(s) => fmt(tl!("No app is using it. It stops on its own in about {m} min."), &[("m", &minutes(s))]),
                            None => tl!("No app is using it.").to_string(),
                        };
                        body(ui, &text);
                    } else {
                        widgets::hud(ui, tl!("Apps using it"), t.muted);
                        ui.add_space(space::S2);
                        // The attached apps on hairlines, their pid in a trailing column.
                        for a in &r.attached {
                            widgets::row_rule(ui, 0.0);
                            ui.add_space(space::S2);
                            widgets::tight_row(ui, |ui| {
                                let id = if a.app.starts_with("redrob-") { a.app.clone() } else { format!("redrob-{}", a.app) };
                                if let Some(icon) = crate::icons::app(&id) {
                                    ui.add(egui::Image::new(icon).fit_to_exact_size(vec2(16.0, 16.0)));
                                }
                                ui.label(RichText::new(app_display_name(&a.app)).family(theme::medium()).color(t.ink_strong));
                                if let Some(pid) = a.pid {
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        widgets::hud(ui, &fmt(tl!("pid {pid}"), &[("pid", &pid.to_string())]), t.faint);
                                    });
                                }
                            });
                            ui.add_space(space::S2);
                        }
                    }
                }
            }
            if let Some(e) = &error {
                ui.add_space(space::S2);
                ui.add(egui::Label::new(RichText::new(msg(e)).size(theme::text::BODY_SM).color(widgets::tone_color(&t, Tone::Warn))).wrap());
            }
            if !actions.is_empty() {
                let attached = matches!(&state, EngineState::Running(r) if !r.attached.is_empty());
                // A list ends on its own last row; anything else gets the footer's breathing room.
                if !attached {
                    ui.add_space(space::S4);
                }
                // The footer's hairline runs to the card's edges, through its padding.
                let (rule, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), egui::Sense::hover());
                ui.painter().hline(rule.left() - space::S5..=rule.right() + space::S5, rule.center().y, t.hairline());
                ui.add_space(space::S4);
                ui.horizontal(|ui| {
                    if attached {
                        widgets::hud(ui, tl!("Apps reconnect on their own after a stop or restart."), t.faint);
                    }
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

        // A newer engine: a DS Alert under the card, until it is installed.
        if let Some((installed, latest)) = &newer {
            ui.add_space(space::S4);
            widgets::alert_frame(&t).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.spacing_mut().item_spacing.y = 0.0;
                widgets::tight_row(ui, |ui| {
                    widgets::badge(ui, tl!("Update available"), widgets::tone_color(&t, Tone::Info));
                    widgets::hud(ui, &fmt(tl!("{installed} → {latest}"), &[("installed", installed), ("latest", latest)]), t.faint);
                });
                ui.add_space(space::S2);
                if updating {
                    widgets::tight_row(ui, |ui| {
                        widgets::loader_inline(ui, 16.0, t.accent);
                        ui.label(RichText::new(tl!("Updating the engine…")).size(theme::text::BODY_SM).color(t.muted));
                    });
                    ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
                } else {
                    let text = if waiting_on.is_empty() {
                        tl!("It updates on its own when no app is using it. You can also update it now.").to_string()
                    } else {
                        fmt(
                            tl!("Waiting until no app is using it. Using it now: {apps}. Update now stops the engine; apps reconnect on their own."),
                            &[("apps", &waiting_on.join(", "))],
                        )
                    };
                    // The explanation, and Update now on the trailing edge, centred on it.
                    let button_w = widgets::button_width(ui, tl!("Update now"), widgets::Size::Md);
                    ui.horizontal(|ui| {
                        let text_w = (ui.available_width() - button_w - space::S6).max(0.0);
                        ui.vertical(|ui| {
                            ui.set_width(text_w);
                            ui.add(egui::Label::new(RichText::new(text).size(theme::text::BODY_SM).color(t.muted)).wrap());
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if widgets::button(ui, tl!("Update now"), Kind::Primary, 0.0).clicked() {
                                ask = Some(Ask::UpdateNow);
                            }
                        });
                    });
                }
            });
        }

        if let Some(what) = ask {
            match what {
                Ask::Install => self.send(Command::Install { app: ENGINE_APP.to_string() }),
                Ask::Start => {
                    if let Some(engine) = self.backend.engine() {
                        engine.start();
                    }
                }
                Ask::Stop => {
                    if let Some(engine) = self.backend.engine() {
                        engine.stop();
                    }
                }
                Ask::Restart => {
                    if let Some(engine) = self.backend.engine() {
                        engine.restart();
                    }
                }
                Ask::UpdateNow => {
                    self.engine_update.forced = true;
                    self.engine_update_tick();
                }
            }
        }
        // The attached list changes as apps open and close; look again every few seconds while open.
        ui.ctx().request_repaint_after(std::time::Duration::from_secs(5));

        ui.add_space(theme::space::S4);
        ui.add(
            egui::Label::new(
                RichText::new(tl!("The engine holds the Redrob key, picks the model provider and masks personal details, once, for every app."))
                    .size(theme::text::META)
                    .color(t.faint),
            )
            .wrap(),
        );
    }
}
