//! The models dialog: each app's downloadable models, with download, pause, resume and delete.
//! The backend does the work ([`ModelBackend`]); this only shows states and sends requests.

use egui::{RichText, vec2};
use redrob_launcher_core::models::ModelState;

use crate::app::{Dialog, LauncherUi};
use crate::i18n::{fmt, msg};
use crate::model::Tone;
use crate::theme::{self, Tokens};
use crate::widgets::{self, Kind};

/// What one row asks the backend to do.
enum Ask {
    Download,
    Pause,
    Delete,
}

impl LauncherUi {
    /// The header's models link, on backends that can download models.
    pub(crate) fn models_link(&mut self, ui: &mut egui::Ui, height: f32) {
        let Some(models) = self.backend.models() else { return };
        let busy = models.models().iter().any(|v| matches!(v.state, ModelState::Downloading { .. }));
        let label = if busy { tl!("Models…") } else { tl!("Models") };
        if widgets::nav_link(ui, label, tl!("Download the models the apps use"), height, matches!(self.dialog, Some(Dialog::Models))).clicked() {
            self.dialog = Some(Dialog::Models);
        }
    }

    pub(crate) fn models_dialog(&mut self, ui: &mut egui::Ui) -> bool {
        let t = Tokens::get(ui.ctx());
        ui.set_width(620.0f32.min(ui.ctx().content_rect().width() - 80.0));
        Self::title(ui, Some(crate::icons::redrob_mark()), tl!("Models"), tl!("Downloaded into the folder each app reads them from"));
        let Some(models) = self.backend.models() else { return true };
        let views = models.models();
        let mut ask: Option<(Ask, String, String)> = None;
        egui::ScrollArea::vertical().max_height(ui.ctx().content_rect().height() * 0.6).show(ui, |ui| {
            let mut last_app = "";
            for v in &views {
                let m = &v.model;
                if m.app != last_app {
                    last_app = &m.app;
                    let app_name = redrob_launcher_core::CraftApp::by_id(&m.app).map_or(m.app.as_str(), |a| a.name);
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        if let Some(icon) = crate::icons::app(&m.app) {
                            ui.add(egui::Image::new(icon).fit_to_exact_size(vec2(20.0, 20.0)));
                        }
                        widgets::hud(ui, app_name, t.muted);
                    });
                    ui.add_space(6.0);
                }
                egui::Frame::new().stroke(t.hairline()).inner_margin(12).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 3.0;
                            ui.label(RichText::new(&m.name).family(theme::medium()).color(t.ink_strong));
                            if !m.description.is_empty() {
                                ui.add(egui::Label::new(RichText::new(crate::i18n::t(&m.description)).size(12.5).color(t.muted)).wrap());
                            }
                            ui.horizontal(|ui| {
                                widgets::hud(ui, &widgets::human_bytes(m.size()), t.faint);
                                ui.add_space(8.0);
                                widgets::hud_link(ui, tl!("Licence and source"), &m.source).on_hover_text(&m.license);
                            });
                        });
                    });
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        let (text, tone): (String, Tone) = match &v.state {
                            ModelState::NotDownloaded => (tl!("Not downloaded").into(), Tone::Neutral),
                            ModelState::Partial { done, total } => (
                                fmt(
                                    tl!("{done} of {total} downloaded"),
                                    &[("done", &widgets::human_bytes(*done)), ("total", &widgets::human_bytes(*total))],
                                ),
                                Tone::Warn,
                            ),
                            ModelState::Downloading { .. } => (tl!("Downloading").into(), Tone::Info),
                            ModelState::Installed => (tl!("Downloaded").into(), Tone::Good),
                            ModelState::Failed(e) => (msg(e), Tone::Warn),
                            ModelState::NoFolder => (tl!("Not available on this system").into(), Tone::Faint),
                        };
                        let r = widgets::badge(ui, &text, widgets::tone_color(&t, tone));
                        if let Some(folder) = &v.folder {
                            r.on_hover_text(folder.as_str());
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let id = || (m.app.clone(), m.id.clone());
                            match &v.state {
                                ModelState::NotDownloaded => {
                                    if widgets::button(ui, tl!("Download"), Kind::Primary, 0.0).clicked() {
                                        ask = Some((Ask::Download, id().0, id().1));
                                    }
                                }
                                ModelState::Partial { .. } | ModelState::Failed(_) => {
                                    if widgets::button(ui, tl!("Delete"), Kind::Secondary, 0.0).clicked() {
                                        ask = Some((Ask::Delete, id().0, id().1));
                                    }
                                    let label = if matches!(v.state, ModelState::Failed(_)) { tl!("Try again") } else { tl!("Resume") };
                                    if widgets::button(ui, label, Kind::Primary, 0.0).clicked() {
                                        ask = Some((Ask::Download, id().0, id().1));
                                    }
                                }
                                ModelState::Downloading { done, total } => {
                                    if widgets::button(ui, tl!("Pause"), Kind::Secondary, 0.0).clicked() {
                                        ask = Some((Ask::Pause, id().0, id().1));
                                    }
                                    let fraction = (*total > 0).then(|| *done as f32 / *total as f32);
                                    let caption = fmt(
                                        tl!("Downloading {done} of {total}"),
                                        &[("done", &widgets::human_bytes(*done)), ("total", &widgets::human_bytes(*total))],
                                    );
                                    widgets::progress(ui, fraction, &caption, t.accent, 240.0);
                                }
                                ModelState::Installed => {
                                    if widgets::button(ui, tl!("Delete"), Kind::Secondary, 0.0).clicked() {
                                        ask = Some((Ask::Delete, id().0, id().1));
                                    }
                                }
                                ModelState::NoFolder => {}
                            }
                        });
                    });
                });
                ui.add_space(6.0);
            }
        });
        if views.iter().any(|v| matches!(v.state, ModelState::Downloading { .. })) {
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
        }
        if let Some((what, app, id)) = ask {
            let ctx = ui.ctx().clone();
            if let Some(models) = self.backend.models() {
                match what {
                    Ask::Download => models.download(&app, &id),
                    Ask::Pause => models.pause(&app, &id),
                    Ask::Delete => {
                        if let Err(e) = models.delete(&app, &id) {
                            self.toast(&ctx, msg(&e), true);
                        }
                    }
                }
            }
        }
        ui.add_space(12.0);
        let mut close = false;
        ui.horizontal(|ui| {
            widgets::hud(ui, tl!("Every file is checked against its published SHA-256 before an app can use it."), t.faint);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                close = widgets::button(ui, tl!("Done"), Kind::Secondary, 90.0).clicked();
            });
        });
        close
    }
}
