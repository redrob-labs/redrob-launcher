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

    /// The engine page (its title is in the page's top bar). Returns true when Done is pressed.
    pub(crate) fn engine_page(&mut self, ui: &mut egui::Ui) -> bool {
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
        let Some(engine) = self.backend.engine() else { return true };
        let state = engine.state();
        let error = engine.last_error();
        let mut ask: Option<Ask> = None;

        egui::Frame::new().stroke(t.hairline()).inner_margin(12).show(ui, |ui| {
            ui.set_width(ui.available_width());
            match &state {
                EngineState::NotInstalled => {
                    widgets::badge(ui, tl!("Not installed"), widgets::tone_color(&t, Tone::Warn));
                    ui.add_space(6.0);
                    ui.add(egui::Label::new(RichText::new(tl!("Redrob Code is the engine Cowork, Office, Design, Browser, Film and CAD run their AI through. Install it once and they share it.")).size(12.5).color(t.muted)).wrap());
                    ui.add_space(8.0);
                    if widgets::button(ui, tl!("Install Redrob Code"), Kind::Primary, 0.0).clicked() {
                        ask = Some(Ask::Install);
                    }
                }
                EngineState::Stopped => {
                    widgets::badge(ui, tl!("Not running"), widgets::tone_color(&t, Tone::Neutral));
                    ui.add_space(6.0);
                    ui.add(egui::Label::new(RichText::new(tl!("The first app that needs the engine starts it, and it stops on its own when no app has used it for a while. You can also start it here.")).size(12.5).color(t.muted)).wrap());
                    ui.add_space(8.0);
                    if widgets::button(ui, tl!("Start"), Kind::Primary, 0.0).clicked() {
                        ask = Some(Ask::Start);
                    }
                }
                EngineState::Busy => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(RichText::new(tl!("Working…")).color(t.muted));
                    });
                    ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
                }
                EngineState::Running(r) => {
                    ui.horizontal(|ui| {
                        widgets::badge(ui, tl!("Running"), widgets::tone_color(&t, Tone::Good));
                        ui.add_space(8.0);
                        widgets::hud(ui, &fmt(tl!("version {v}"), &[("v", &r.version)]), t.faint);
                        ui.add_space(8.0);
                        widgets::hud(ui, &r.url, t.faint);
                    });
                    ui.add_space(8.0);
                    if r.attached.is_empty() {
                        let text = match r.exits_in_secs() {
                            Some(s) => fmt(tl!("No app is using it. It stops on its own in about {m} min."), &[("m", &minutes(s))]),
                            None => tl!("No app is using it.").to_string(),
                        };
                        ui.label(RichText::new(text).size(12.5).color(t.muted));
                    } else {
                        widgets::hud(ui, tl!("Apps using it"), t.muted);
                        ui.add_space(4.0);
                        for a in &r.attached {
                            ui.horizontal(|ui| {
                                let id = if a.app.starts_with("redrob-") { a.app.clone() } else { format!("redrob-{}", a.app) };
                                if let Some(icon) = crate::icons::app(&id) {
                                    ui.add(egui::Image::new(icon).fit_to_exact_size(vec2(16.0, 16.0)));
                                }
                                ui.label(RichText::new(app_display_name(&a.app)).family(theme::medium()).color(t.ink_strong));
                                if let Some(pid) = a.pid {
                                    widgets::hud(ui, &fmt(tl!("pid {pid}"), &[("pid", &pid.to_string())]), t.faint);
                                }
                            });
                        }
                    }
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if widgets::button(ui, tl!("Stop"), Kind::Secondary, 0.0).clicked() {
                            ask = Some(Ask::Stop);
                        }
                        if widgets::button(ui, tl!("Restart"), Kind::Secondary, 0.0).clicked() {
                            ask = Some(Ask::Restart);
                        }
                        if !r.attached.is_empty() {
                            widgets::hud(ui, tl!("Apps reconnect on their own after a stop or restart."), t.faint);
                        }
                    });
                }
            }
            if let Some(e) = &error {
                ui.add_space(8.0);
                ui.add(egui::Label::new(RichText::new(msg(e)).size(12.5).color(widgets::tone_color(&t, Tone::Warn))).wrap());
            }
            if let Some((installed, latest)) = &newer {
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    widgets::badge(ui, tl!("Update available"), widgets::tone_color(&t, Tone::Info));
                    ui.add_space(8.0);
                    widgets::hud(ui, &fmt(tl!("{installed} → {latest}"), &[("installed", installed), ("latest", latest)]), t.faint);
                });
                ui.add_space(6.0);
                if updating {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(RichText::new(tl!("Updating the engine…")).size(12.5).color(t.muted));
                    });
                    ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
                } else if !waiting_on.is_empty() {
                    ui.add(
                        egui::Label::new(
                            RichText::new(fmt(
                                tl!("Waiting until no app is using it. Using it now: {apps}. Update now stops the engine; apps reconnect on their own."),
                                &[("apps", &waiting_on.join(", "))],
                            ))
                            .size(12.5)
                            .color(t.muted),
                        )
                        .wrap(),
                    );
                    ui.add_space(6.0);
                    if widgets::button(ui, tl!("Update now"), Kind::Primary, 0.0).clicked() {
                        ask = Some(Ask::UpdateNow);
                    }
                } else {
                    ui.add(egui::Label::new(RichText::new(tl!("It updates on its own when no app is using it. You can also update it now.")).size(12.5).color(t.muted)).wrap());
                    ui.add_space(6.0);
                    if widgets::button(ui, tl!("Update now"), Kind::Primary, 0.0).clicked() {
                        ask = Some(Ask::UpdateNow);
                    }
                }
            }
        });

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

        ui.add_space(12.0);
        let mut close = false;
        ui.horizontal(|ui| {
            widgets::hud(
                ui,
                tl!("The engine holds the Redrob key, picks the model provider and masks personal details, once, for every app."),
                t.faint,
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                close = widgets::button(ui, tl!("Done"), Kind::Secondary, 90.0).clicked();
            });
        });
        close
    }
}
