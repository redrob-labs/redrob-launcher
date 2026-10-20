//! The Models page: what this computer is, and every model the apps can use on it, grouped by
//! what it is for (subtitles, pictures, chat...), since that is how a person who has never heard
//! of a model chooses one. Each row leads with a plain sentence and the verdict for this computer;
//! the model's name, size and app come second, small. The person can install, pause, resume or
//! delete a model and, for one the launcher serves, start and stop the server. The backend does
//! the work ([`ModelBackend`]); this only shows states and sends requests.

use egui::RichText;
use redrob_launcher_core::device::{Device, Fit};
use redrob_launcher_core::models::{ModelState, ModelView, Runtime, RuntimeState, SHARED_APP, Serving};

use crate::app::{LauncherUi, Page};
use crate::i18n::{fmt, msg};
use crate::model::Tone;
use crate::theme::{self, Tokens};
use crate::widgets::{self, Kind};

/// The page's reading measure: the same column as the Engine, Account and Settings pages.
const MEASURE: f32 = 720.0;

/// Width of an installing model's progress bar.
const PROGRESS_W: f32 = 240.0;

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

/// The purposes in the order the page lists them: what most people come for first.
const PURPOSES: [&str; 17] = [
    "chat",
    "code",
    "vision",
    "stt",
    "tts",
    "text-to-image",
    "image-edit",
    "inpaint",
    "background-removal",
    "upscale",
    "segmentation",
    "vectorize",
    "stem-separation",
    "vad",
    "image-to-3d",
    "ocr",
    "embedding",
];

/// One plain sentence under a purpose's heading: what these models do for the person.
fn purpose_blurb(task: &str) -> Option<String> {
    Some(match task {
        "chat" => tl!("Talk to an assistant inside the apps, without sending anything off this computer.").to_string(),
        "code" => tl!("An assistant that writes and fixes code in Redrob Code.").to_string(),
        "vision" => tl!("An assistant that can also look at your images.").to_string(),
        "stt" => tl!("Turns speech in your videos into subtitles and text.").to_string(),
        "tts" => tl!("Reads text aloud as narration.").to_string(),
        "text-to-image" => tl!("Makes pictures from a description.").to_string(),
        "image-edit" => tl!("Changes a picture the way you describe.").to_string(),
        "inpaint" => tl!("Removes things from photos and fills the gap.").to_string(),
        "background-removal" => tl!("Cuts the background out of a photo.").to_string(),
        "upscale" => tl!("Makes small images bigger and sharper.").to_string(),
        "segmentation" => tl!("Selects an object in a picture with one click.").to_string(),
        "stem-separation" => tl!("Splits music into vocals, drums, bass and the rest.").to_string(),
        "vad" => tl!("Finds where people speak in a recording.").to_string(),
        "image-to-3d" => tl!("Turns a photo into a 3D model.").to_string(),
        "ocr" => tl!("Reads the text in images and scans.").to_string(),
        "embedding" => tl!("Lets Redrob apps search your files by meaning.").to_string(),
        _ => return None,
    })
}

fn purpose_rank(task: &str) -> usize {
    PURPOSES.iter().position(|p| *p == task).unwrap_or(PURPOSES.len())
}

fn group_name(app: &str) -> String {
    if app == SHARED_APP {
        tl!("Every app").to_string()
    } else {
        redrob_launcher_core::CraftApp::by_id(app).map_or_else(|| app.to_string(), |a| a.name.to_string())
    }
}

/// Rows in display order: by purpose, and within one, what runs here first, then smaller first.
fn ordered(views: Vec<ModelView>) -> Vec<ModelView> {
    let mut v = views;
    v.sort_by_key(|x| (purpose_rank(&x.model.task), fit_rank(x.fit), x.model.size()));
    v
}

fn fit_rank(fit: Fit) -> u8 {
    match fit {
        Fit::Runs => 0,
        Fit::Slow => 1,
        Fit::Unknown => 2,
        Fit::WontRun => 3,
    }
}

impl LauncherUi {
    /// The sidebar's On-device AI row (the models page), on backends that can download models. A
    /// blue dot while one is coming in; the hover says what the page is for.
    pub(crate) fn models_nav(&mut self, ui: &mut egui::Ui, rail: bool) {
        let Some(models) = self.backend.models() else { return };
        let views = models.models();
        let busy = views.iter().any(|v| matches!(v.state, ModelState::Downloading { .. }) || v.serving == Serving::Starting);
        let meta = if busy { widgets::NavMeta::Dot(Tokens::get(ui.ctx()).accent) } else { widgets::NavMeta::None };
        let hint =
            if busy { tl!("A model is downloading or starting") } else { tl!("AI models that run on this computer. What you give them stays here.") };
        if widgets::nav_row(ui, crate::glyphs::Icon::Models, tl!("On-device AI"), hint, meta, self.page == Page::Models, rail).clicked() {
            self.set_page(Page::Models);
        }
    }

    fn device_card(ui: &mut egui::Ui, t: &Tokens, device: &Device, disk_free: Option<u64>) -> bool {
        use theme::space;
        let mut remeasure = false;
        widgets::card_frame(t).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 0.0;
            // Head: what this is and its GPU, Measure again on the trailing edge, centred on them.
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = space::S1;
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
                    ui.label(RichText::new(gpu).size(theme::text::CONTROL).family(theme::medium()).color(t.ink_strong));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    remeasure = widgets::button_sized(ui, tl!("Measure again"), Kind::Secondary, widgets::Size::Sm, 0.0).clicked();
                });
            });
            // Body: the processor line, then the facts that decide what runs, on one meta line.
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
            ui.add_space(space::S2);
            if !line.is_empty() {
                ui.add(egui::Label::new(RichText::new(line.join(" · ")).size(theme::text::BODY_SM).color(t.muted)).wrap());
                ui.add_space(space::S1);
            }
            widgets::tight_wrapped(ui, |ui| {
                widgets::hud(ui, &fmt(tl!("Backend: {backend}"), &[("backend", device.backend().id())]), t.faint);
                if let Some(gb) = device.runs_up_to_gb().filter(|gb| *gb > 0) {
                    widgets::meta_sep(ui);
                    widgets::hud(ui, &fmt(tl!("Runs models up to about {gb} GB"), &[("gb", &gb.to_string())]), t.faint);
                }
                if let Some(free) = disk_free {
                    widgets::meta_sep(ui);
                    widgets::hud(ui, &fmt(tl!("{free} free for models"), &[("free", &widgets::human_bytes(free))]), t.faint);
                }
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

    /// One model on its section's hairline list: what it gives the person, the details small, then
    /// the verdict for this computer and its state; its actions in one column on the trailing
    /// edge, centred on the row, the primary last. Returns what a click asked for.
    fn model_row(ui: &mut egui::Ui, t: &Tokens, cpu: bool, v: &ModelView) -> Option<(Ask, String, String)> {
        use theme::space;
        use widgets::Size;
        let m = &v.model;
        // The primary goes first: in the right-to-left column it ends up on the trailing edge.
        let mut actions: Vec<(&str, Kind, Ask)> = Vec::new();
        match &v.state {
            ModelState::NotDownloaded => actions.push((tl!("Install"), Kind::Primary, Ask::Download)),
            ModelState::Partial { .. } | ModelState::Failed(_) => {
                let label = if matches!(v.state, ModelState::Failed(_)) { tl!("Try again") } else { tl!("Resume") };
                actions.push((label, Kind::Primary, Ask::Download));
                actions.push((tl!("Delete"), Kind::Secondary, Ask::Delete));
            }
            ModelState::Downloading { .. } => actions.push((tl!("Pause"), Kind::Secondary, Ask::Pause)),
            ModelState::Installed => {
                if m.runtime.served() {
                    match &v.serving {
                        Serving::Running { .. } | Serving::Starting => actions.push((tl!("Stop"), Kind::Secondary, Ask::Stop)),
                        Serving::No | Serving::Failed(_) => {
                            let label = if matches!(v.serving, Serving::Failed(_)) { tl!("Try again") } else { tl!("Start") };
                            actions.push((label, Kind::Primary, Ask::Start));
                        }
                    }
                }
                actions.push((tl!("Delete"), Kind::Secondary, Ask::Delete));
            }
            ModelState::NoFolder => {}
        }
        let gap = ui.spacing().item_spacing.x;
        let actions_w = actions.iter().map(|(label, _, _)| widgets::button_width(ui, label, Size::Sm)).sum::<f32>()
            + gap * actions.len().saturating_sub(1) as f32;
        let mut ask = None;
        ui.horizontal(|ui| {
            let room = if actions.is_empty() { 0.0 } else { actions_w + space::S6 };
            let content_w = (ui.available_width() - room).max(0.0);
            ui.vertical(|ui| {
                ui.set_width(content_w);
                ui.spacing_mut().item_spacing.y = 0.0;
                let headline = if m.summary.is_empty() { m.name.clone() } else { crate::i18n::t(&m.summary).to_string() };
                ui.add(egui::Label::new(RichText::new(headline).size(theme::text::CONTROL).family(theme::medium()).color(t.ink_strong)).wrap());
                ui.add_space(space::S1);
                // The details, small: the model's name, its size, which app, the licence.
                widgets::tight_wrapped(ui, |ui| {
                    widgets::hud(ui, &m.name, t.faint).on_hover_text(crate::i18n::t(&m.description));
                    widgets::meta_sep(ui);
                    widgets::hud(ui, &widgets::human_bytes(m.size()), t.faint);
                    widgets::meta_sep(ui);
                    widgets::hud(ui, &fmt(tl!("for {app}"), &[("app", &group_name(&m.app))]), t.faint);
                    widgets::meta_sep(ui);
                    widgets::hud_link(ui, tl!("Licence and source"), &m.source).on_hover_text(&m.license);
                });
                ui.add_space(space::S2);
                // The verdict for this computer first, so it always starts at the content edge.
                let (fit_text, tone) = fit_label(v.fit);
                let need = fmt(
                    tl!("Needs about {gb} GB of {kind}"),
                    &[
                        ("gb", &format!("{:.0}", f64::from(m.requires.memory_mb) / 1024.0)),
                        ("kind", if cpu { tl!("RAM") } else { tl!("GPU memory") }),
                    ],
                );
                if let ModelState::Downloading { done, total } = &v.state {
                    // While installing, the bar and its caption say so: no second "Installing" chip.
                    // Top-aligned, so the chip sits on the caption's line, the bar under them.
                    let line = egui::vec2(ui.available_width(), theme::control::MD);
                    ui.allocate_ui_with_layout(line, egui::Layout::left_to_right(egui::Align::Min), |ui| {
                        widgets::badge_sized(ui, &fit_text, widgets::tone_color(t, tone), Size::Sm).on_hover_text(need);
                        let fraction = (*total > 0).then(|| *done as f32 / *total as f32);
                        let caption = fmt(
                            tl!("Installing {done} of {total}"),
                            &[("done", &widgets::human_bytes(*done)), ("total", &widgets::human_bytes(*total))],
                        );
                        widgets::progress(ui, fraction, &caption, t.accent, PROGRESS_W.min(ui.available_width()));
                    });
                } else {
                    widgets::tight_row(ui, |ui| {
                        widgets::badge_sized(ui, &fit_text, widgets::tone_color(t, tone), Size::Sm).on_hover_text(need);
                        let (text, tone): (String, Tone) = match (&v.state, &v.serving) {
                            (ModelState::Installed, Serving::Running { url }) => (fmt(tl!("Serving at {url}"), &[("url", url)]), Tone::Good),
                            (ModelState::Installed, Serving::Starting) => (tl!("Starting…").into(), Tone::Info),
                            (ModelState::Installed, Serving::Failed(e)) => (msg(e), Tone::Warn),
                            (ModelState::NotDownloaded, _) => (tl!("Not installed").into(), Tone::Neutral),
                            (ModelState::Partial { done, total }, _) => (
                                fmt(
                                    tl!("{done} of {total} installed"),
                                    &[("done", &widgets::human_bytes(*done)), ("total", &widgets::human_bytes(*total))],
                                ),
                                Tone::Warn,
                            ),
                            (ModelState::Downloading { .. }, _) => (tl!("Installing").into(), Tone::Info),
                            (ModelState::Installed, Serving::No) => (tl!("Installed").into(), Tone::Good),
                            (ModelState::Failed(e), _) => (msg(e), Tone::Warn),
                            (ModelState::NoFolder, _) => (tl!("Not available on this system").into(), Tone::Faint),
                        };
                        let r = widgets::badge_sized(ui, &text, widgets::tone_color(t, tone), Size::Sm);
                        if let Some(folder) = &v.folder {
                            r.on_hover_text(folder.as_str());
                        }
                    });
                }
            });
            if !actions.is_empty() {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    for (label, kind, what) in actions {
                        if widgets::button_sized(ui, label, kind, Size::Sm, 0.0).clicked() {
                            ask = Some((what, m.app.clone(), m.id.clone()));
                        }
                    }
                });
            }
        });
        ask
    }

    /// The page. Fills the central panel; the header's Apps tab leads back.
    pub(crate) fn models_page(&mut self, ui: &mut egui::Ui) {
        let t = Tokens::get(ui.ctx());
        let only_runnable = self.settings.models_only_runnable;
        let prism = self.settings.prism_runtime;
        let Some(models) = self.backend.models() else { return };
        let device = models.device();
        let disk_free = models.disk_free();
        let all = models.models();
        let views = ordered(all.into_iter().filter(|v| prism || v.model.runtime != Runtime::LlamaServerPrism).collect());
        let hidden = views.iter().filter(|v| only_runnable && v.fit == Fit::WontRun).count();
        let runtimes: Vec<(Runtime, RuntimeState)> = [Runtime::LlamaServer, Runtime::LlamaServerPrism]
            .into_iter()
            .filter(|r| views.iter().any(|v| v.model.runtime == *r))
            .map(|r| (r, models.runtime(r)))
            .collect();
        let mut ask: Option<(Ask, String, String)> = None;
        let mut toggle = only_runnable;

        widgets::page_scroll(ui, "models", |ui| {
            use theme::space;
            ui.set_width(ui.available_width().min(MEASURE));
            // The rhythm below is the DS's 4 px grid, set step by step.
            ui.spacing_mut().item_spacing.y = 0.0;
            // The title is in the page's top bar.
            ui.add(
                egui::Label::new(
                    RichText::new(tl!(
                        "Install what the apps need to work offline on this computer. Anything missing is done on Redrob's servers instead."
                    ))
                    .size(theme::text::BODY_SM)
                    .color(t.muted),
                )
                .wrap(),
            );
            ui.add_space(space::S6);
            if Self::device_card(ui, &t, &device, disk_free) {
                ask = Some((Ask::Remeasure, String::new(), String::new()));
            }
            ui.add_space(space::S4);
            // The filter toolbar: the switch and what it hides, the runtimes on the trailing edge.
            ui.horizontal(|ui| {
                widgets::switch(ui, &mut toggle, tl!("Only show what runs on this computer"));
                if hidden > 0 {
                    widgets::hud(ui, &crate::i18n::tn(hidden as u64, "{n} model hidden", "{n} models hidden"), t.faint);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = space::S3;
                    for (r, st) in &runtimes {
                        Self::runtime_line(ui, &t, *r, st);
                    }
                });
            });

            // One section per purpose: its heading, then its models as rows on hairlines in one
            // card (DS: N items at once are a list on hairlines, not a grid of identical cards).
            let shown: Vec<&ModelView> = views.iter().filter(|v| !(only_runnable && v.fit == Fit::WontRun)).collect();
            let cpu = device.backend() == redrob_launcher_core::models::Backend::Cpu;
            for group in shown.chunk_by(|a, b| a.model.task == b.model.task) {
                let task = &group[0].model.task;
                ui.add_space(space::S6);
                ui.label(RichText::new(task_label(task)).size(theme::text::BODY).family(theme::semibold()).color(t.ink_strong));
                if let Some(blurb) = purpose_blurb(task) {
                    ui.add_space(space::S1);
                    ui.add(egui::Label::new(RichText::new(blurb).size(theme::text::BODY_SM).color(t.muted)).wrap());
                }
                ui.add_space(space::S3);
                widgets::card_frame(&t).inner_margin(egui::Margin::symmetric(space::S5 as i8, 0)).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.spacing_mut().item_spacing.y = 0.0;
                    for (i, v) in group.iter().enumerate() {
                        if i > 0 {
                            widgets::row_rule(ui, 0.0);
                        }
                        ui.add_space(space::S4);
                        if let Some(a) = Self::model_row(ui, &t, cpu, v) {
                            ask = Some(a);
                        }
                        ui.add_space(space::S4);
                    }
                });
            }
            ui.add_space(space::S6);
            widgets::hud(ui, tl!("Every file is checked against its official copy before an app uses it."), t.faint);
            ui.add_space(space::S8);
        });

        if toggle != only_runnable {
            self.settings.models_only_runnable = toggle;
        }
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
    }
}
