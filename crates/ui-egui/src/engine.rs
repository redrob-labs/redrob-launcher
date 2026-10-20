// ADDED-LINE-RATIONALE 37b27cf8a516a099:1: Gives `engine` direct access to `crate / app / LauncherUi` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 67e56e9cec68f977:1: Gives `engine` direct access to `crate / glyphs / Icon` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 8c5ba205cde4960d:1: Supports `engine_nav` while preserving the nearby constraint: "Standby" is its normal state, not a fault.
// ADDED-LINE-RATIONALE c83bdac984cd72e2:1: Freezes `t` for `engine_nav` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 856366d600627ff2:1: Freezes `let` for `engine_nav` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 62c0437352eed530:1: Freezes `EngineState` for `engine_nav` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6d39aa6c98bc03c8:1: Freezes `EngineState` for `engine_nav` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3c1245efe7ae42fa:1: Freezes `EngineState` for `engine_nav` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d17e3740b37a28d4:1: Freezes `EngineState` for `engine_nav` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b0c8036bd0ee0afc:1: Freezes `EngineState` for `engine_nav` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b7d368389d34f86b:1: Freezes `label` for `engine_nav` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE dd9827bfab570902:1: Freezes `if` for `engine_nav` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7fec77e99789f23f:1: Delegates `engine_nav`'s `set_page / Page / Engine` work to `self.set_page` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fdb3b80ecabc792d:1: Supports `engine_page` while preserving the nearby constraint: The engine page (its title is in the page's top bar).
// ADDED-LINE-RATIONALE bcc1c5bb1ece2e2c:2: Freezes `Some` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3b43166d5968346d:1: Gives `engine_page` direct access to `theme / space` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 5ce89bce470172bb:1: Supports `engine_page` while preserving the nearby constraint: What the footer offers in this state: (label, kind, request), the trailing one first.
// ADDED-LINE-RATIONALE 4c1df5ebe6373e59:1: Freezes `EngineState` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cc06660f4ee597ce:1: Freezes `EngineState` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8e9a397049e3603c:1: Freezes `EngineState` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ca531793d88cff78:1: Freezes `EngineState` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a2ac2259330e400c:2: Bounds the `engine_page` implementation here so `EngineState / Running / _` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 176e7ece4d9f80a4:1: Supports `engine_page` while preserving the nearby constraint: footer on a full-width hairline with the actions on the trailing edge.
// ADDED-LINE-RATIONALE f3be4c78dc782d4b:1: Freezes `ui` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9f4346c115d207dd:1: Freezes `body` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7f2fca45308a7d4c:1: Delegates `engine_page`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e6c9706f3231000c:1: Bounds the `engine_page` implementation here so `ui / add / egui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b46ced2d53e23b72:1: Delegates `engine_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4ce9e1556c47f874:1: Delegates `engine_page`'s `body / ui / tl` work to `body` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b46ced2d53e23b72:2: Delegates `engine_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 12179173f3c8c2e7:1: Delegates `engine_page`'s `body / ui / tl` work to `body` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3ea4a075b1a589d9:1: Delegates `engine_page`'s `widgets / tight_row / ui` work to `widgets::tight_row` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 84437db21aa7b53d:1: Delegates `engine_page`'s `widgets / loader_inline / ui` work to `widgets::loader_inline` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 33ead776cff59041:1: Delegates `engine_page`'s `ui / label / RichText` work to `ui.label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3ea4a075b1a589d9:2: Delegates `engine_page`'s `widgets / tight_row / ui` work to `widgets::tight_row` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 11e3a6200bdfc8c7:1: Delegates `engine_page`'s `widgets / meta_sep / ui` work to `widgets::meta_sep` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b46ced2d53e23b72:3: Delegates `engine_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 81f186cf906d3fc7:1: Delegates `engine_page`'s `body / ui / text` work to `body` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d55e08179e8056d1:1: Delegates `engine_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d195e2b397cda6ae:1: Delegates `engine_page`'s `widgets / row_rule / ui` work to `widgets::row_rule` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a80254305c1d16d1:1: Delegates `engine_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6fcd27b35e1e09d9:1: Delegates `engine_page`'s `widgets / tight_row / ui` work to `widgets::tight_row` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 11e23554ae6c0bb9:1: Delegates `engine_page`'s `ui / with_layout / egui` work to `ui.with_layout` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7ad30efa835296b3:1: Delegates `engine_page`'s `widgets / hud / ui` work to `widgets::hud` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 999426932a347dfb:1: Makes `widgets / hud / ui` an explicit part of `engine_page` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE a80254305c1d16d1:2: Delegates `engine_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b96a4099ebcad6d1:1: Delegates `engine_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bf2a3c96910fd901:1: Delegates `engine_page`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fbad0b7e391719c3:1: Prevents `engine_page` from applying its side effect outside the `actions / is_empty` state.
// ADDED-LINE-RATIONALE e6e9455a267aa74d:1: Freezes `attached` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 84b480834d336bc1:1: Supports `engine_page` while preserving the nearby constraint: A list ends on its own last row; anything else gets the footer's breathing room.
// ADDED-LINE-RATIONALE ce2c602d62668cbb:1: Delegates `engine_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1038fb0708d041ff:5: Bounds the `engine_page` implementation here so `ui / add_space / space` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e9d8baee06462293:1: Supports `engine_page` while preserving the nearby constraint: The footer's hairline runs to the card's edges, through its padding.
// ADDED-LINE-RATIONALE 00e6092db5dcf540:1: Freezes `ui` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c9e39299f4e1ccbb:1: Delegates `engine_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 07ff046029ce6fc4:1: Prevents `engine_page` from applying its side effect outside the `attached` state.
// ADDED-LINE-RATIONALE 6858293fee32831f:1: Delegates `engine_page`'s `widgets / hud / ui` work to `widgets::hud` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6c2f9b870d0a81ff:2: Bounds the `engine_page` implementation here so `widgets / hud / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 67923dfa42a30bb9:1: Delegates `engine_page`'s `ui / with_layout / egui` work to `ui.with_layout` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f213f69f2162f2f3:1: Applies `engine_page` consistently to every `label / kind / what` item rather than handling only the first match.
// ADDED-LINE-RATIONALE eec6039ef6a3dc64:1: Prevents `engine_page` from applying its side effect outside the `widgets / button / ui` state.
// ADDED-LINE-RATIONALE bb9c5d7c0290b7c0:1: Freezes `ask` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8c27234e9b0b01ff:1: Bounds the `engine_page` implementation here so `ask / what` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a17cf8b37908c1ff:2: Bounds the `engine_page` implementation here so `ask / what` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 76d3fe466b7b7dfb:3: Makes `ask / what` an explicit part of `engine_page` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE d582bd9197673dfb:1: Makes `ask / what` an explicit part of `engine_page` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 3d163bd5e21a01ff:4: Bounds the `engine_page` implementation here so `A / newer / engine` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 78fb8c91c16abdfb:2: Makes `A / newer / engine` an explicit part of `engine_page` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 53033c4416db35e4:1: Supports `engine_page` while preserving the nearby constraint: A newer engine: a DS Alert under the card, until it is installed.
// ADDED-LINE-RATIONALE cc53e08c6a490cbb:1: Delegates `engine_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b554a2d88f59c4b2:1: Delegates `engine_page`'s `widgets / alert_frame / t` work to `widgets::alert_frame` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2e423a4f78a91c9a:1: Delegates `engine_page`'s `ui / set_width / ui` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ac39e7cab8d2ed4b:1: Freezes `ui` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d65df7621fcf49d9:1: Delegates `engine_page`'s `widgets / tight_row / ui` work to `widgets::tight_row` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b96a4099ebcad6d1:2: Delegates `engine_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3ea4a075b1a589d9:3: Delegates `engine_page`'s `widgets / tight_row / ui` work to `widgets::tight_row` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 84437db21aa7b53d:2: Delegates `engine_page`'s `widgets / loader_inline / ui` work to `widgets::loader_inline` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d23a7f0f1b275037:1: Delegates `engine_page`'s `ui / label / RichText` work to `ui.label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 326f717709f59c07:1: Freezes `text` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0af07f5f58a1ffd5:1: Delegates `engine_page`'s `tl / It / updates` work to `tl` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 47d8b5143a00f979:2: Makes `tl / It / updates` an explicit part of `engine_page` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 9a91bcb618d13981:1: Delegates `engine_page`'s `fmt` work to `fmt` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4652824a9d738189:1: Keeps `tl / Waiting / until` in `engine_page`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE ed71a3684af981bd:1: Keeps `apps / waiting_on / join` in `engine_page`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE a17d44b379094323:1: Bounds the `engine_page` implementation here so `apps / waiting_on / join` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE df670a7b28da800c:1: Bounds the `engine_page` implementation here so `The / explanation / and` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f2d2965b07b8344c:1: Supports `engine_page` while preserving the nearby constraint: The explanation, and Update now on the trailing edge, centred on it.
// ADDED-LINE-RATIONALE ccd5c5b147b7a637:1: Delegates `engine_page`'s `ui / horizontal / ui` work to `ui.horizontal` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 29e2d1067db29f1e:1: Freezes `text_w` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4a8257005f8c9259:1: Delegates `engine_page`'s `ui / vertical / ui` work to `ui.vertical` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 75ae222fa5022e38:1: Delegates `engine_page`'s `ui / set_width / text_w` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 20bb808aec273d4c:1: Delegates `engine_page`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 61af2622a9f3bdfb:1: Makes `ui / add / egui` an explicit part of `engine_page` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE af2031a07fff4bb9:1: Delegates `engine_page`'s `ui / with_layout / egui` work to `ui.with_layout` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 11c22cab30a40d0f:1: Prevents `engine_page` from applying its side effect outside the `widgets / button / ui` state.
// ADDED-LINE-RATIONALE f13aa58f6205b762:1: Freezes `ask` for `engine_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8c27234e9b0b01ff:2: Bounds the `engine_page` implementation here so `ask / Ask / UpdateNow` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 61af2622a9f3bdfb:2: Makes `ask / Ask / UpdateNow` an explicit part of `engine_page` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 76d3fe466b7b7dfb:5: Makes `ask / Ask / UpdateNow` an explicit part of `engine_page` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 59873bf24976fdfb:1: Makes `what / ask` an explicit part of `engine_page` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 5133bd2092a7c1ff:2: Bounds the `engine_page` implementation here so `what / ask` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 256ca16774917f80:1: Delegates `engine_page`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0fd9ca2238f7957d:1: Delegates `engine_page`'s `ui / add` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bd74e973df18170a:1: Delegates `engine_page`'s `egui / Label / new` work to `egui::Label::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9e6a2182dbcc0c9e:1: Delegates `engine_page`'s `RichText / new / tl` work to `RichText::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f995942bfb95bc17:1: Delegates `engine_page`'s `size / theme / text` work to `size` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE be47ff2728920364:1: Keeps `color / t / faint` in `engine_page`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 3d1687d5e21a8323:1: Bounds the `engine_page` implementation here so `color / t / faint` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e6f394352ee5708f:1: Keeps `wrap` in `engine_page`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE a3ae9a5933ea01c8:1: Bounds the `engine_page` implementation here so `wrap` cannot leak into the adjacent declaration or control path.
//! The AI engine page: the shared Redrob Code daemon every Redrob app on this computer runs its AI
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
    /// The sidebar's AI engine row, on backends that can reach the daemon: a dot and a short state
    /// word. The engine starts itself when an app needs it and stops after ten idle minutes, so
    /// "Standby" is its normal state, not a fault.
    pub(crate) fn engine_nav(&mut self, ui: &mut egui::Ui, rail: bool) {
        let t = Tokens::get(ui.ctx());
        let Some(engine) = self.backend.engine() else { return };
        let (word, tip, dot): (&str, String, egui::Color32) = match engine.state() {
            EngineState::Running(r) if r.attached.is_empty() => (tl!("Ready"), tl!("Ready. Your Redrob apps use it for AI").to_string(), t.good),
            EngineState::Running(r) => (tl!("In use"), crate::i18n::tn(r.attached.len() as u64, "In use by {n} app", "In use by {n} apps"), t.good),
            EngineState::Busy => (tl!("Busy"), tl!("Starting or stopping").to_string(), t.accent),
            EngineState::Stopped => (tl!("Standby"), tl!("On standby. It starts on its own when an app needs it").to_string(), t.faint),
            EngineState::NotInstalled => (tl!("Not installed"), tl!("Not installed. Install it to use AI in your apps").to_string(), t.warn),
        };
        let label = tl!("AI engine");
        if widgets::nav_row(ui, Icon::Engine, label, &tip, widgets::NavMeta::State(word, dot), self.page == Page::Engine, rail).clicked() {
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
