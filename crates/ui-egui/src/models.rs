//! The Models page: what this computer is, and every model the apps can use on it, grouped by
//! app. Each row says whether it runs here (from the device record), lets the person download,
//! pause, resume or delete it, and, for a model the launcher serves, start and stop the server.
//! The backend does the work ([`ModelBackend`]); this only shows states and sends requests.

use egui::{RichText, vec2};
use redrob_launcher_core::device::{Device, Fit};
use redrob_launcher_core::models::{ModelState, ModelView, Runtime, RuntimeState, SHARED_APP, Serving};

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
    Start,
    Stop,
    Remeasure,
}

/// The verdict chip's words and tone.
pub(crate) fn fit_label(fit: Fit) -> (String, Tone) {
    match fit {
        Fit::Runs => (tl!("Runs here").into(), Tone::Good),
        Fit::Slow => (tl!("Runs slowly").into(), Tone::Warn),
        Fit::WontRun => (tl!("Won't run here").into(), Tone::Neutral),
        Fit::Unknown => (tl!("Not measured").into(), Tone::Faint),
    }
}

/// The task label shown on a row (`chat` -> "Chat"). English keys; the catalog translates.
pub(crate) fn task_label(task: &str) -> String {
    match task {
        "chat" => tl!("Chat").to_string(),
        "code" => tl!("Coding").to_string(),
        "vision" => tl!("Chat and vision").to_string(),
        "embedding" => tl!("Search embeddings").to_string(),
        "text-to-image" => tl!("Text to image").to_string(),
        "image-edit" => tl!("Image editing").to_string(),
        "inpaint" => tl!("Inpainting").to_string(),
        "background-removal" => tl!("Background removal").to_string(),
        "upscale" => tl!("Upscaling").to_string(),
        "segmentation" => tl!("Selection").to_string(),
        "vectorize" => tl!("Vectorizing").to_string(),
        "stt" => tl!("Speech to text").to_string(),
        "tts" => tl!("Text to speech").to_string(),
        "vad" => tl!("Voice detection").to_string(),
        "stem-separation" => tl!("Stem separation").to_string(),
        "image-to-3d" => tl!("Image to 3D").to_string(),
        "ocr" => tl!("Text recognition").to_string(),
        other => other.to_string(),
    }
}

fn group_name(app: &str) -> String {
    if app == SHARED_APP {
        tl!("Every app").to_string()
    } else {
        redrob_launcher_core::CraftApp::by_id(app).map_or_else(|| app.to_string(), |a| a.name.to_string())
    }
}

/// Rows in display order: shared models first, then by app in catalogue order.
fn ordered(views: Vec<ModelView>) -> Vec<ModelView> {
    let mut v = views;
    v.sort_by_key(|x| if x.model.shared() { 0 } else { 1 });
    v
}

impl LauncherUi {
    /// The header's models link, on backends that can download models.
    pub(crate) fn models_link(&mut self, ui: &mut egui::Ui, height: f32) {
        let Some(models) = self.backend.models() else { return };
        let views = models.models();
        let busy = views.iter().any(|v| matches!(v.state, ModelState::Downloading { .. }) || v.serving == Serving::Starting);
        let label = if busy { tl!("Models…") } else { tl!("Models") };
        if widgets::nav_link(ui, label, tl!("Local models: what runs on this computer"), height, matches!(self.dialog, Some(Dialog::Models)))
            .clicked()
        {
            self.dialog = Some(Dialog::Models);
        }
    }

    fn device_card(ui: &mut egui::Ui, t: &Tokens, device: &Device, disk_free: Option<u64>) -> bool {
        let mut remeasure = false;
        egui::Frame::new().fill(t.sunken).stroke(t.hairline()).inner_margin(12).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 3.0;
                    widgets::hud(ui, tl!("This computer"), t.muted);
                    let gpu = match &device.gpu {
                        Some(g) => match g.memory_mb {
                            Some(mb) if g.unified => {
                                fmt(tl!("{name}, {gb} GB unified memory"), &[("name", &g.name), ("gb", &(mb / 1024).to_string())])
                            }
                            Some(mb) => fmt(tl!("{name}, {gb} GB"), &[("name", &g.name), ("gb", &mb.div_ceil(1024).to_string())]),
                            None if g.unified => {
                                fmt(tl!("{name}, {gb} GB unified memory"), &[("name", &g.name), ("gb", &(device.ram_mb / 1024).to_string())])
                            }
                            None => fmt(tl!("{name} (memory not measured)"), &[("name", &g.name)]),
                        },
                        None if device.gpu_probed => tl!("No GPU found; models run on the CPU").to_string(),
                        None => tl!("GPU not measured yet").to_string(),
                    };
                    ui.label(RichText::new(gpu).family(theme::medium()).color(t.ink_strong));
                    let mut line = Vec::new();
                    if !device.cpu.is_empty() {
                        line.push(device.cpu.clone());
                    }
                    if device.cores > 0 {
                        line.push(fmt(tl!("{n} cores"), &[("n", &device.cores.to_string())]));
                    }
                    if device.ram_mb > 0 {
                        line.push(fmt(tl!("{gb} GB RAM"), &[("gb", &(device.ram_mb / 1024).to_string())]));
                    }
                    if let Some(npu) = &device.npu {
                        line.push(fmt(tl!("NPU: {name}"), &[("name", &npu.name)]));
                    }
                    if !line.is_empty() {
                        ui.add(egui::Label::new(RichText::new(line.join(" · ")).size(12.5).color(t.muted)).wrap());
                    }
                    ui.horizontal(|ui| {
                        widgets::hud(ui, &fmt(tl!("Backend: {backend}"), &[("backend", device.backend().id())]), t.faint);
                        if let Some(gb) = device.runs_up_to_gb().filter(|gb| *gb > 0) {
                            ui.add_space(8.0);
                            widgets::hud(ui, &fmt(tl!("Runs models up to about {gb} GB"), &[("gb", &gb.to_string())]), t.faint);
                        }
                        if let Some(free) = disk_free {
                            ui.add_space(8.0);
                            widgets::hud(ui, &fmt(tl!("{free} free for models"), &[("free", &widgets::human_bytes(free))]), t.faint);
                        }
                    });
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    remeasure = widgets::button(ui, tl!("Measure again"), Kind::Secondary, 0.0).clicked();
                });
            });
        });
        remeasure
    }

    fn runtime_line(ui: &mut egui::Ui, t: &Tokens, runtime: Runtime, state: &RuntimeState) {
        let name = match runtime {
            Runtime::LlamaServerPrism => "llama-server (PrismML)",
            _ => "llama-server",
        };
        let text = match state {
            RuntimeState::Unavailable => fmt(tl!("No {runtime} build for this computer"), &[("runtime", name)]),
            RuntimeState::NotInstalled => fmt(tl!("{runtime} is installed on first start"), &[("runtime", name)]),
            RuntimeState::Downloading { done, total } if *total > 0 => fmt(
                tl!("Installing {runtime}: {done} of {total}"),
                &[("runtime", name), ("done", &widgets::human_bytes(*done)), ("total", &widgets::human_bytes(*total))],
            ),
            RuntimeState::Downloading { .. } => fmt(tl!("Installing {runtime}…"), &[("runtime", name)]),
            RuntimeState::Installed => fmt(tl!("{runtime} ready"), &[("runtime", name)]),
            RuntimeState::Failed(e) => fmt(tl!("{runtime}: {e}"), &[("runtime", name), ("e", &msg(e))]),
        };
        let color = if matches!(state, RuntimeState::Failed(_)) { t.warn } else { t.faint };
        widgets::hud(ui, &text, color);
    }

    pub(crate) fn models_dialog(&mut self, ui: &mut egui::Ui) -> bool {
        let t = Tokens::get(ui.ctx());
        ui.set_width(680.0f32.min(ui.ctx().content_rect().width() - 80.0));
        Self::title(
            ui,
            Some(crate::icons::redrob_mark()),
            tl!("Models"),
            tl!("Local models for every Redrob app. Nothing here is required: an app with no local model uses the Redrob Console."),
        );
        let prism = self.settings.prism_runtime;
        let Some(models) = self.backend.models() else { return true };
        let device = models.device();
        let disk_free = models.disk_free();
        let views = ordered(models.models().into_iter().filter(|v| prism || v.model.runtime != Runtime::LlamaServerPrism).collect());
        let runtimes: Vec<(Runtime, RuntimeState)> = [Runtime::LlamaServer, Runtime::LlamaServerPrism]
            .into_iter()
            .filter(|r| views.iter().any(|v| v.model.runtime == *r))
            .map(|r| (r, models.runtime(r)))
            .collect();
        let mut ask: Option<(Ask, String, String)> = None;
        if Self::device_card(ui, &t, &device, disk_free) {
            ask = Some((Ask::Remeasure, String::new(), String::new()));
        }
        ui.add_space(8.0);
        // Everything above and below the list (title, device card, footer) is about 400 px.
        egui::ScrollArea::vertical().max_height((ui.ctx().content_rect().height() - 400.0).max(240.0)).show(ui, |ui| {
            let mut last_app = "";
            for v in &views {
                let m = &v.model;
                if m.app != last_app {
                    last_app = &m.app;
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        if let Some(icon) = crate::icons::app(&m.app) {
                            ui.add(egui::Image::new(icon).fit_to_exact_size(vec2(20.0, 20.0)));
                        }
                        widgets::hud(ui, &group_name(&m.app), t.muted);
                    });
                    if m.shared() {
                        for (r, s) in &runtimes {
                            Self::runtime_line(ui, &t, *r, s);
                        }
                    }
                    ui.add_space(6.0);
                }
                egui::Frame::new().stroke(t.hairline()).inner_margin(12).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 3.0;
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&m.name).family(theme::medium()).color(t.ink_strong));
                                let (fit_text, tone) = fit_label(v.fit);
                                let need = fmt(
                                    tl!("Needs about {gb} GB of {kind}"),
                                    &[
                                        ("gb", &format!("{:.0}", f64::from(m.requires.memory_mb) / 1024.0)),
                                        (
                                            "kind",
                                            if device.backend() == redrob_launcher_core::models::Backend::Cpu {
                                                tl!("RAM")
                                            } else {
                                                tl!("GPU memory")
                                            },
                                        ),
                                    ],
                                );
                                widgets::badge(ui, &fit_text, widgets::tone_color(&t, tone)).on_hover_text(need);
                            });
                            if !m.description.is_empty() {
                                ui.add(egui::Label::new(RichText::new(crate::i18n::t(&m.description)).size(12.5).color(t.muted)).wrap());
                            }
                            ui.horizontal(|ui| {
                                widgets::hud(ui, &task_label(&m.task), t.faint);
                                ui.add_space(8.0);
                                widgets::hud(ui, &widgets::human_bytes(m.size()), t.faint);
                                ui.add_space(8.0);
                                widgets::hud_link(ui, tl!("Licence and source"), &m.source).on_hover_text(&m.license);
                            });
                        });
                    });
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        let (text, tone): (String, Tone) = match (&v.state, &v.serving) {
                            (ModelState::Installed, Serving::Running { url }) => (fmt(tl!("Serving at {url}"), &[("url", url)]), Tone::Good),
                            (ModelState::Installed, Serving::Starting) => (tl!("Starting…").into(), Tone::Info),
                            (ModelState::Installed, Serving::Failed(e)) => (msg(e), Tone::Warn),
                            (ModelState::NotDownloaded, _) => (tl!("Not downloaded").into(), Tone::Neutral),
                            (ModelState::Partial { done, total }, _) => (
                                fmt(
                                    tl!("{done} of {total} downloaded"),
                                    &[("done", &widgets::human_bytes(*done)), ("total", &widgets::human_bytes(*total))],
                                ),
                                Tone::Warn,
                            ),
                            (ModelState::Downloading { .. }, _) => (tl!("Downloading").into(), Tone::Info),
                            (ModelState::Installed, Serving::No) => (tl!("Downloaded").into(), Tone::Good),
                            (ModelState::Failed(e), _) => (msg(e), Tone::Warn),
                            (ModelState::NoFolder, _) => (tl!("Not available on this system").into(), Tone::Faint),
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
                                    if m.runtime.served() {
                                        match &v.serving {
                                            Serving::Running { .. } | Serving::Starting => {
                                                if widgets::button(ui, tl!("Stop"), Kind::Secondary, 0.0).clicked() {
                                                    ask = Some((Ask::Stop, id().0, id().1));
                                                }
                                            }
                                            Serving::No | Serving::Failed(_) => {
                                                let label = if matches!(v.serving, Serving::Failed(_)) { tl!("Try again") } else { tl!("Start") };
                                                if widgets::button(ui, label, Kind::Primary, 0.0).clicked() {
                                                    ask = Some((Ask::Start, id().0, id().1));
                                                }
                                            }
                                        }
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
        let live = views.iter().any(|v| matches!(v.state, ModelState::Downloading { .. }) || v.serving == Serving::Starting)
            || runtimes.iter().any(|(_, s)| matches!(s, RuntimeState::Downloading { .. }));
        if live {
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
        }
        if let Some((what, app, id)) = ask {
            let ctx = ui.ctx().clone();
            if let Some(models) = self.backend.models() {
                match what {
                    Ask::Download => models.download(&app, &id),
                    Ask::Pause => models.pause(&app, &id),
                    Ask::Start => models.start(&app, &id),
                    Ask::Stop => models.stop(&app, &id),
                    Ask::Remeasure => models.remeasure(),
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
