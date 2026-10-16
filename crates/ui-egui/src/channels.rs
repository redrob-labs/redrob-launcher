//! Settings > Channels and Settings > Apps.
//!
//! Channels: one card per chat platform (Telegram, Slack, Discord, WhatsApp) with an on/off
//! switch, the bot token, the person's own id for the allowlist, and a connection dot from the
//! running engine. Save writes the engine's files, turns the channel on or off through the
//! engine's CLI and reloads the daemon. Apps: the Redrob apps bridged to the engine right now.
//! The backend ([`redrob_launcher_core::channels::ChannelsBackend`]) does the IO; this only draws snapshots and sends requests.

use egui::{Color32, RichText, vec2};
use redrob_launcher_core::channels::{
    Apps, ChannelDraft, ChannelName, DISCORD_DEVELOPER_PORTAL_URL, Live, Loaded, SaveOutcome, SaveState, WhatsAppPairing,
    slack_manifest_url,
};
use redrob_launcher_core::engine::app_display_name;

use crate::app::LauncherUi;
use crate::i18n::{fmt, msg, tn};
use crate::model::Tone;
use crate::theme::{self, Tokens, rgb};
use crate::widgets::{self, Cell, Kind};

/// Which page of the Settings dialog is open.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum SettingsTab {
    #[default]
    General,
    Channels,
    Apps,
}

/// One channel's card, as edited: the allowlist split into "your id" and the extra ids.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Card {
    enabled: bool,
    /// `enabled` as read from the file; a difference means Save runs `redrob channels enable|disable`.
    was_enabled: bool,
    your_id: String,
    extras: Vec<String>,
    /// The "Allow another" field's text, not yet added.
    new_extra: String,
    /// `(key, value)` per secret key of the channel.
    secrets: Vec<(String, String)>,
}

impl Card {
    fn from_draft(d: &ChannelDraft) -> Self {
        let mut ids = d.allowlist.iter();
        Self {
            enabled: d.enabled,
            was_enabled: d.enabled,
            your_id: ids.next().cloned().unwrap_or_default(),
            extras: ids.cloned().collect(),
            new_extra: String::new(),
            secrets: d.secrets.clone(),
        }
    }

    fn to_draft(&self) -> ChannelDraft {
        let mut allowlist = vec![self.your_id.clone()];
        allowlist.extend(self.extras.iter().cloned());
        if !self.new_extra.trim().is_empty() {
            allowlist.push(self.new_extra.clone());
        }
        ChannelDraft { enabled: self.enabled, allowlist, secrets: self.secrets.clone() }
    }

    fn nobody_allowed(&self) -> bool {
        self.your_id.trim().is_empty() && self.extras.iter().all(|s| s.trim().is_empty()) && self.new_extra.trim().is_empty()
    }
}

/// The Channels page's state between frames.
#[derive(Debug, Default)]
pub(crate) struct ChannelsPanel {
    cards: Vec<(ChannelName, Card)>,
    /// What the cards were filled from, so a new read refills them only when it differs.
    filled_from: Option<Loaded>,
    /// `load()` was asked of the backend since the page opened.
    requested: bool,
    /// A save started here is running; its result replaces the cards when it lands.
    saving: bool,
    /// "Unlink" was pressed and waits for a confirmation.
    confirm_unlink: bool,
}

impl ChannelsPanel {
    /// Forget the page's state (the dialog closed): the next open reads the files again.
    pub(crate) fn reset(&mut self) {
        *self = Self::default();
    }

    fn fill(&mut self, loaded: Loaded) {
        self.cards = loaded.channels.iter().map(|(name, draft)| (*name, Card::from_draft(draft))).collect();
        self.filled_from = Some(loaded);
    }
}

enum Ask {
    OpenUrl(String),
    Unlink,
    Save,
}

/// The platform's mark: a disc in its brand colour with its initial, since the launcher ships no
/// third-party logos.
fn channel_mark(ui: &mut egui::Ui, name: ChannelName) {
    let (color, letter) = match name {
        ChannelName::Telegram => (rgb([0x26, 0xA5, 0xE4]), "T"),
        ChannelName::Slack => (rgb([0x4A, 0x15, 0x4B]), "S"),
        ChannelName::Discord => (rgb([0x58, 0x65, 0xF2]), "D"),
        ChannelName::WhatsApp => (rgb([0x25, 0xD3, 0x66]), "W"),
    };
    let (rect, resp) = ui.allocate_exact_size(vec2(22.0, 22.0), egui::Sense::hover());
    ui.painter().circle_filled(rect.center(), 11.0, color);
    ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, letter, egui::FontId::new(12.0, theme::semibold()), Color32::WHITE);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, name.display_name()));
}

/// The connection dot and the words behind it.
fn connection(t: &Tokens, live: &Live, name: ChannelName) -> (Color32, String) {
    match live {
        Live::Unknown => (t.faint, tl!("Checking…").to_string()),
        Live::NoEngine => (t.faint, tl!("Engine not running").to_string()),
        Live::Unreachable(e) => (t.warn, msg(e)),
        Live::Up(status) => match status.get(name) {
            Some(c) if c.connected => (t.good, tl!("Connected").to_string()),
            Some(c) if c.enabled => (
                t.warn,
                match &c.last_error {
                    Some(e) if !e.is_empty() => fmt(tl!("Not connected: {e}"), &[("e", &msg(e))]),
                    _ => tl!("Enabled, not connected yet").to_string(),
                },
            ),
            _ => (t.faint, tl!("Off").to_string()),
        },
    }
}

/// A labelled single-line field, the input on the right.
fn field(ui: &mut egui::Ui, label: &str, value: &mut String, password: bool, hint: &str) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(
                egui::TextEdit::singleline(value)
                    .password(password)
                    .hint_text(hint)
                    .desired_width(250.0)
                    .min_size(vec2(0.0, theme::CONTROL_H))
                    .vertical_align(egui::Align::Center),
            );
        });
    });
}

fn help(ui: &mut egui::Ui, t: &Tokens, text: &str) {
    ui.add(egui::Label::new(RichText::new(text).size(12.0).color(t.muted)).wrap());
}

fn secret_field<'a>(card: &'a mut Card, key: &str) -> Option<&'a mut String> {
    card.secrets.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
}

impl LauncherUi {
    /// The Channels page of Settings. Returns nothing; requests go to the backend.
    pub(crate) fn channels_tab(&mut self, ui: &mut egui::Ui) {
        let t = Tokens::get(ui.ctx());
        let Self { backend, channels: panel, .. } = self;
        let Some(be) = backend.channels() else {
            help(ui, &t, tl!("This build can't reach the engine's files."));
            return;
        };
        if !panel.requested {
            be.load();
            be.refresh();
            panel.requested = true;
        }
        // A finished save hands back what is on disk now; take it as the new baseline.
        let save_state = be.save_state();
        if panel.saving && !matches!(save_state, SaveState::Saving) {
            panel.saving = false;
            panel.filled_from = None;
        }
        if let Some(loaded) = be.loaded()
            && panel.filled_from.is_none()
        {
            panel.fill(loaded);
        }
        let Some(loaded) = &panel.filled_from else {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(RichText::new(tl!("Reading the engine's settings…")).color(t.muted));
            });
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(100));
            return;
        };
        let autostart = loaded.autostart;
        let live = be.live();
        let whatsapp = be.whatsapp();
        let mut ask: Option<Ask> = None;

        help(ui, &t, tl!("Talk to your Redrob Code engine from a chat app on your phone. Turn a channel on, paste its bot token, add your own id, and save."));
        if matches!(live, Live::NoEngine) {
            ui.add_space(4.0);
            help(ui, &t, tl!("The engine is not running: changes are saved and take effect when it starts."));
        }
        ui.add_space(8.0);

        for (name, card) in &mut panel.cards {
            let name = *name;
            egui::Frame::new().stroke(t.hairline()).inner_margin(12).show(ui, |ui| {
                ui.set_width(ui.available_width());
                // Header: mark, name, state dot, on/off.
                ui.horizontal(|ui| {
                    channel_mark(ui, name);
                    ui.add_space(4.0);
                    ui.label(RichText::new(name.display_name()).family(theme::semibold()).size(14.0).color(t.ink_strong));
                    let (color, words) = connection(&t, &live, name);
                    let (rect, resp) = ui.allocate_exact_size(vec2(10.0, 10.0), egui::Sense::hover());
                    ui.painter().circle_filled(rect.center(), 4.0, color);
                    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &words));
                    widgets::hud(ui, &words, t.muted);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let mut on = card.enabled;
                        widgets::cells(
                            ui,
                            &mut on,
                            &[
                                (true, Cell { label: tl!("On"), tip: tl!("Start this bot with the engine"), count: None }),
                                (false, Cell { label: tl!("Off"), tip: tl!("Stop this bot"), count: None }),
                            ],
                        );
                        card.enabled = on;
                    });
                });
                ui.add_space(8.0);

                match name {
                    ChannelName::Telegram => {
                        if let Some(v) = secret_field(card, "botToken") {
                            field(ui, tl!("Bot token"), v, true, "123456:ABC…");
                        }
                        help(ui, &t, tl!("Open @BotFather in Telegram, send /newbot, paste the token here"));
                        ui.add_space(6.0);
                        field(ui, tl!("Your Telegram user id"), &mut card.your_id, false, "123456789");
                        help(ui, &t, tl!("Send /start to @userinfobot to see it"));
                    }
                    ChannelName::Slack => {
                        if widgets::button(ui, tl!("Create the Slack app"), Kind::Secondary, 0.0)
                            .on_hover_text(tl!("Opens api.slack.com with the Redrob bot's manifest filled in. Install the app to your workspace, then copy its two tokens here."))
                            .clicked()
                        {
                            ask = Some(Ask::OpenUrl(slack_manifest_url()));
                        }
                        ui.add_space(6.0);
                        if let Some(v) = secret_field(card, "botToken") {
                            field(ui, tl!("Bot token (xoxb-…)"), v, true, "xoxb-…");
                        }
                        if let Some(v) = secret_field(card, "appToken") {
                            field(ui, tl!("App token (xapp-…)"), v, true, "xapp-…");
                        }
                        help(ui, &t, tl!("Bot token: OAuth & Permissions. App token: Basic Information > App-Level Tokens, with connections:write."));
                        ui.add_space(6.0);
                        field(ui, tl!("Your Slack member id"), &mut card.your_id, false, "U0123ABCDEF");
                        help(ui, &t, tl!("Your profile > ⋯ > Copy member ID"));
                    }
                    ChannelName::Discord => {
                        if widgets::button(ui, tl!("Open the Developer Portal"), Kind::Secondary, 0.0).on_hover_text(DISCORD_DEVELOPER_PORTAL_URL).clicked() {
                            ask = Some(Ask::OpenUrl(DISCORD_DEVELOPER_PORTAL_URL.to_string()));
                        }
                        ui.add_space(6.0);
                        if let Some(v) = secret_field(card, "botToken") {
                            field(ui, tl!("Bot token"), v, true, "");
                        }
                        help(ui, &t, tl!("New Application > Bot > Reset Token. On the same page, turn on Message Content Intent under Privileged Gateway Intents, or the bot sees empty messages."));
                        ui.add_space(6.0);
                        field(ui, tl!("Your Discord user id"), &mut card.your_id, false, "123456789012345678");
                        help(ui, &t, tl!("Settings > Advanced > Developer Mode, then right-click your name > Copy User ID"));
                    }
                    ChannelName::WhatsApp => {
                        match &whatsapp {
                            WhatsAppPairing::Paired => {
                                ui.horizontal(|ui| {
                                    widgets::badge(ui, tl!("Paired"), widgets::tone_color(&t, Tone::Good));
                                    ui.add_space(8.0);
                                    if panel.confirm_unlink {
                                        widgets::hud(ui, tl!("Unlink this phone? The engine shows a new QR code."), t.muted);
                                        if widgets::button(ui, tl!("Unlink"), Kind::Danger, 0.0).clicked() {
                                            ask = Some(Ask::Unlink);
                                        }
                                        if widgets::button(ui, tl!("Cancel"), Kind::Secondary, 0.0).clicked() {
                                            panel.confirm_unlink = false;
                                        }
                                    } else if widgets::button(ui, tl!("Unlink"), Kind::Secondary, 0.0).clicked() {
                                        panel.confirm_unlink = true;
                                    }
                                });
                            }
                            WhatsAppPairing::Pairing(png) if card.enabled => {
                                let hash = {
                                    use std::hash::{Hash, Hasher};
                                    let mut h = std::hash::DefaultHasher::new();
                                    png.hash(&mut h);
                                    h.finish()
                                };
                                let source = egui::ImageSource::Bytes {
                                    uri: format!("bytes://redrob-launcher/channels/whatsapp-qr-{hash:016x}.png").into(),
                                    bytes: egui::load::Bytes::Shared(png.clone()),
                                };
                                ui.horizontal(|ui| {
                                    ui.add(egui::Image::new(source).fit_to_exact_size(vec2(180.0, 180.0)).bg_fill(Color32::WHITE));
                                    ui.add_space(12.0);
                                    ui.vertical(|ui| {
                                        ui.label(RichText::new(tl!("Scan with WhatsApp > Linked devices")).family(theme::medium()).color(t.ink_strong));
                                        help(ui, &t, tl!("The code changes every so often; this picture follows it."));
                                    });
                                });
                                ui.ctx().request_repaint_after(std::time::Duration::from_secs(2));
                            }
                            WhatsAppPairing::Pairing(_) | WhatsAppPairing::Unpaired => {
                                help(
                                    ui,
                                    &t,
                                    if card.enabled && !matches!(live, Live::NoEngine) {
                                        tl!("Not paired. Save with the channel on and a QR code appears here.")
                                    } else {
                                        tl!("Not paired. Turn the channel on, save, and scan the QR code that appears here while the engine runs.")
                                    },
                                );
                                if card.enabled {
                                    ui.ctx().request_repaint_after(std::time::Duration::from_secs(2));
                                }
                            }
                            WhatsAppPairing::Unknown => {
                                ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
                            }
                        }
                        ui.add_space(6.0);
                        field(ui, tl!("Your phone number (digits only)"), &mut card.your_id, false, "821012345678");
                        help(ui, &t, tl!("Country code first, no plus sign or spaces"));
                        ui.add_space(4.0);
                        ui.add(
                            egui::Label::new(
                                RichText::new(tl!("This uses an unofficial WhatsApp client. WhatsApp may ban the account it is paired with."))
                                    .size(12.0)
                                    .color(t.warn),
                            )
                            .wrap(),
                        );
                    }
                }

                // The allowlist beyond "your id".
                ui.add_space(6.0);
                if !card.extras.is_empty() {
                    widgets::hud(ui, tl!("Also allowed"), t.faint);
                    let mut remove: Option<usize> = None;
                    for (i, id) in card.extras.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.add(egui::TextEdit::singleline(id).desired_width(250.0).min_size(vec2(0.0, theme::CONTROL_H)).vertical_align(egui::Align::Center));
                            if widgets::glyph_button(ui, widgets::Glyph::Close, tl!("Remove")).clicked() {
                                remove = Some(i);
                            }
                        });
                    }
                    if let Some(i) = remove {
                        card.extras.remove(i);
                    }
                }
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut card.new_extra)
                            .hint_text(tl!("Another id to allow"))
                            .desired_width(250.0)
                            .min_size(vec2(0.0, theme::CONTROL_H))
                            .vertical_align(egui::Align::Center),
                    );
                    if widgets::button(ui, tl!("Allow another"), Kind::Secondary, 0.0).clicked() && !card.new_extra.trim().is_empty() {
                        let id = std::mem::take(&mut card.new_extra);
                        card.extras.push(id.trim().to_string());
                    }
                });
                if card.enabled && card.nobody_allowed() {
                    ui.add_space(4.0);
                    ui.add(egui::Label::new(RichText::new(tl!("Nobody can talk to it until you add your id")).size(12.0).color(t.warn)).wrap());
                }
            });
            ui.add_space(8.0);
        }

        // Footer: Save and its result.
        ui.horizontal(|ui| {
            let saving = matches!(save_state, SaveState::Saving);
            ui.add_enabled_ui(!saving, |ui| {
                if widgets::button(ui, tl!("Save"), Kind::Primary, 90.0).clicked() {
                    ask = Some(Ask::Save);
                }
            });
            ui.add_space(8.0);
            match &save_state {
                SaveState::Idle => {}
                SaveState::Saving => {
                    ui.spinner();
                    widgets::hud(ui, tl!("Saving…"), t.muted);
                    ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
                }
                SaveState::Saved(SaveOutcome::Reloaded) => {
                    widgets::hud(ui, tl!("Saved - engine reloaded"), t.good);
                }
                SaveState::Saved(SaveOutcome::TakesEffectOnStart) => {
                    widgets::hud(ui, tl!("Saved - takes effect when the engine starts"), t.muted);
                }
                SaveState::Failed(e) => {
                    ui.add(egui::Label::new(RichText::new(msg(e)).size(12.0).color(t.warn)).wrap());
                }
            }
        });
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            widgets::hud(ui, tl!("Start at login:"), t.faint);
            widgets::hud(ui, if autostart { tl!("on") } else { tl!("off") }, if autostart { t.good } else { t.muted });
        });
        help(ui, &t, tl!("A bot that is on must survive a reboot, so the engine registers itself to start at login while any channel is on, and removes that when the last one is turned off."));
        // The dots follow the engine; look again every few seconds while the page is open.
        ui.ctx().request_repaint_after(std::time::Duration::from_secs(3));

        if let Some(what) = ask {
            match what {
                Ask::OpenUrl(url) => ui.ctx().open_url(egui::OpenUrl::new_tab(url)),
                Ask::Unlink => {
                    panel.confirm_unlink = false;
                    be.unlink_whatsapp();
                }
                Ask::Save => {
                    let drafts: Vec<(ChannelName, ChannelDraft)> = panel.cards.iter().map(|(n, c)| (*n, c.to_draft())).collect();
                    let toggled: Vec<ChannelName> = panel.cards.iter().filter(|(_, c)| c.enabled != c.was_enabled).map(|(n, _)| *n).collect();
                    panel.saving = true;
                    be.save(drafts, toggled);
                }
            }
        }
    }

    /// The Apps page of Settings: who is bridged to the engine right now.
    pub(crate) fn apps_tab(&mut self, ui: &mut egui::Ui) {
        let t = Tokens::get(ui.ctx());
        let Some(be) = self.backend.channels() else {
            help(ui, &t, tl!("This build can't reach the engine."));
            return;
        };
        help(ui, &t, tl!("Redrob apps that are open and bridged to the engine, so a prompt anywhere can use their tools."));
        ui.add_space(10.0);
        match be.apps() {
            Apps::Unknown => {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(RichText::new(tl!("Checking…")).color(t.muted));
                });
                ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
            }
            Apps::NoEngine => {
                widgets::badge(ui, tl!("Engine not running"), widgets::tone_color(&t, Tone::Neutral));
                ui.add_space(6.0);
                help(ui, &t, tl!("Apps appear here once the engine is running and one of them connects."));
            }
            Apps::Unreachable(e) => {
                ui.add(egui::Label::new(RichText::new(msg(&e)).size(12.5).color(t.warn)).wrap());
            }
            Apps::Up { apps, now_ms } => {
                if apps.is_empty() {
                    help(ui, &t, tl!("No app is connected to the engine. Open a Redrob app and it appears here."));
                } else {
                    let now = i64::try_from(now_ms / 1000).unwrap_or(i64::MAX);
                    egui::Grid::new("bridged-apps").num_columns(4).spacing(vec2(16.0, 6.0)).striped(true).show(ui, |ui| {
                        widgets::hud(ui, tl!("App"), t.faint);
                        widgets::hud(ui, tl!("Version"), t.faint);
                        widgets::hud(ui, tl!("Tools"), t.faint);
                        widgets::hud(ui, tl!("Last seen"), t.faint);
                        ui.end_row();
                        for app in &apps {
                            ui.horizontal(|ui| {
                                let id = if app.name.starts_with("redrob-") { app.name.clone() } else { format!("redrob-{}", app.name) };
                                if let Some(icon) = crate::icons::app(&id) {
                                    ui.add(egui::Image::new(icon).fit_to_exact_size(vec2(16.0, 16.0)));
                                }
                                ui.label(RichText::new(app_display_name(&app.name)).family(theme::medium()).color(t.ink_strong))
                                    .on_hover_text(fmt(tl!("pid {pid}"), &[("pid", &app.pid.to_string())]));
                            });
                            widgets::hud(ui, app.version.as_deref().unwrap_or("—"), t.muted);
                            let tools = app.tools.clone().unwrap_or_default();
                            let n = u64::try_from(tools.len()).unwrap_or(u64::MAX);
                            let label = widgets::hud(ui, &tn(n, "{n} tool", "{n} tools"), t.muted);
                            if !tools.is_empty() {
                                label.on_hover_text(tools.join("\n"));
                            }
                            let seen = i64::try_from(app.last_seen / 1000).unwrap_or(i64::MAX);
                            widgets::hud(ui, &crate::views::ago_text(seen, now), t.muted);
                            ui.end_row();
                        }
                    });
                }
            }
        }
        // Apps come and go; look again every few seconds while the page is open.
        ui.ctx().request_repaint_after(std::time::Duration::from_secs(5));
    }
}
