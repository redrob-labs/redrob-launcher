//! Settings > Channels and Settings > Apps.
//!
//! Channels: one card per chat platform (WhatsApp, Telegram, Discord, Slack), each a short wizard
//! drawn inline: create the bot on the platform's own page (a button opens it), paste what the
//! platform handed out, then message the bot once. Nobody types their own id: the engine makes the
//! first person who writes to the bot its owner (`/channels/status` reports `owner` and
//! `claimOpen`), and the allowlist editor waits under "Advanced" for people who want more than one.
//! Save writes the engine's files, turns the channel on or off through the engine's CLI and
//! reloads the daemon. Apps: the Redrob apps bridged to the engine right now.
//! The backend ([`redrob_launcher_core::channels::ChannelsBackend`]) does the IO; this only draws snapshots and sends requests.

use egui::{Color32, RichText, vec2};
use redrob_launcher_core::channels::{
    Apps, ChannelDraft, ChannelName, DISCORD_APPLICATION_ID_KEY, DISCORD_NEW_APP_URL, Live, Loaded, SaveOutcome, SaveState, TELEGRAM_BOTFATHER_URL,
    WhatsAppPairing, discord_invite_url, slack_manifest_url,
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

/// One channel's card, as edited.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Card {
    enabled: bool,
    /// `enabled` as read from the file; a difference means Save runs `redrob channels enable|disable`.
    was_enabled: bool,
    /// The ids allowed to talk, as the file has them (the owner's, once claimed, is among them).
    allowlist: Vec<String>,
    /// The Advanced section's "Another id" field, not yet added.
    new_extra: String,
    /// `(key, value)` per secret key of the channel.
    secrets: Vec<(String, String)>,
}

impl Card {
    fn from_draft(d: &ChannelDraft) -> Self {
        Self { enabled: d.enabled, was_enabled: d.enabled, allowlist: d.allowlist.clone(), new_extra: String::new(), secrets: d.secrets.clone() }
    }

    fn to_draft(&self) -> ChannelDraft {
        let mut allowlist = self.allowlist.clone();
        if !self.new_extra.trim().is_empty() {
            allowlist.push(self.new_extra.clone());
        }
        ChannelDraft { enabled: self.enabled, allowlist, secrets: self.secrets.clone() }
    }

    fn nobody_allowed(&self) -> bool {
        self.allowlist.iter().all(|s| s.trim().is_empty()) && self.new_extra.trim().is_empty()
    }

    fn secret(&self, key: &str) -> &str {
        self.secrets.iter().find(|(k, _)| k == key).map_or("", |(_, v)| v.as_str())
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

/// The last step's status line: what the running engine says about the bot and who owns it.
struct StatusLine {
    color: Color32,
    text: String,
    /// Something is being waited for (the first message, the connection).
    busy: bool,
}

fn status_line(t: &Tokens, live: &Live, name: ChannelName, card: &Card) -> StatusLine {
    let line = |color, text: String, busy| StatusLine { color, text, busy };
    match live {
        Live::Unknown => line(t.faint, tl!("Checking…").to_string(), true),
        Live::NoEngine if card.enabled => line(t.muted, tl!("The engine is not running. Start it and the bot connects.").to_string(), false),
        Live::NoEngine => line(t.faint, tl!("Off").to_string(), false),
        Live::Unreachable(e) => line(t.warn, msg(e), false),
        Live::Up(status) => {
            let Some(c) = status.get(name).filter(|c| c.enabled) else { return line(t.faint, tl!("Off").to_string(), false) };
            if !c.connected {
                return match &c.last_error {
                    Some(e) if !e.is_empty() => line(t.bad, fmt(tl!("Could not connect: {e}"), &[("e", &msg(e))]), false),
                    _ => line(t.muted, tl!("Connecting…").to_string(), true),
                };
            }
            match &c.owner {
                Some(owner) if !owner.is_empty() => line(t.good, fmt(tl!("Connected. Owner: {owner}"), &[("owner", owner.as_str())]), false),
                _ if c.claim_open => line(t.warn, tl!("Connected. Send your bot any message now to become its owner.").to_string(), true),
                _ if card.nobody_allowed() => {
                    line(t.warn, tl!("Nobody owns this bot yet. Turn it off and on again, then message it within 15 minutes.").to_string(), false)
                }
                _ => line(t.good, tl!("Connected").to_string(), false),
            }
        }
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

/// A wizard step's heading: a numbered square and the step's title.
fn step(ui: &mut egui::Ui, t: &Tokens, n: u8, title: &str) {
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        let (rect, resp) = ui.allocate_exact_size(vec2(18.0, 18.0), egui::Sense::hover());
        ui.painter().rect_filled(rect, egui::CornerRadius::ZERO, t.invert_bg);
        ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, n.to_string(), egui::FontId::new(11.0, theme::semibold()), t.invert_fg);
        resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, n.to_string()));
        ui.add_space(2.0);
        ui.label(RichText::new(title).family(theme::medium()).size(13.0).color(t.ink_strong));
    });
    ui.add_space(2.0);
}

/// The status line of the "send a message" step.
fn status(ui: &mut egui::Ui, line: &StatusLine) {
    ui.horizontal(|ui| {
        let (rect, resp) = ui.allocate_exact_size(vec2(10.0, 10.0), egui::Sense::hover());
        ui.painter().circle_filled(rect.center(), 4.0, line.color);
        resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &line.text));
        if line.busy {
            ui.add(egui::Spinner::new().size(12.0).color(line.color));
        }
        ui.add(egui::Label::new(RichText::new(line.text.as_str()).size(12.5).color(line.color)).wrap());
    });
}

fn secret_field<'a>(card: &'a mut Card, key: &str) -> Option<&'a mut String> {
    card.secrets.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
}

/// Opens `url` in the person's browser when the button is pressed.
fn link_button(ui: &mut egui::Ui, label: &str, url: &str, ask: &mut Option<Ask>) {
    if widgets::button(ui, label, Kind::Secondary, 0.0).on_hover_text(url).clicked() {
        *ask = Some(Ask::OpenUrl(url.to_string()));
    }
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

        help(
            ui,
            &t,
            tl!(
                "Talk to your Redrob Code engine from a chat app on your phone. Pick one below, follow its steps, and save. The first person to message the bot becomes its owner."
            ),
        );
        if matches!(live, Live::NoEngine) {
            ui.add_space(4.0);
            help(ui, &t, tl!("The engine is not running: changes are saved and take effect when it starts."));
        }
        ui.add_space(8.0);

        for (name, card) in &mut panel.cards {
            let name = *name;
            let line = status_line(&t, &live, name, card);
            egui::Frame::new().stroke(t.hairline()).inner_margin(12).show(ui, |ui| {
                ui.set_width(ui.available_width());
                // Header: mark, name, state dot, on/off.
                ui.horizontal(|ui| {
                    channel_mark(ui, name);
                    ui.add_space(4.0);
                    ui.label(RichText::new(name.display_name()).family(theme::semibold()).size(14.0).color(t.ink_strong));
                    if name == ChannelName::WhatsApp {
                        ui.add_space(4.0);
                        widgets::badge(ui, tl!("Easiest: scan a QR code"), t.good);
                    }
                    let (rect, resp) = ui.allocate_exact_size(vec2(10.0, 10.0), egui::Sense::hover());
                    ui.painter().circle_filled(rect.center(), 4.0, line.color);
                    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &line.text));
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
                ui.add_space(6.0);

                match name {
                    ChannelName::WhatsApp => {
                        step(ui, &t, 1, tl!("Scan this QR with WhatsApp > Linked devices"));
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
                                        help(ui, &t, tl!("On your phone: WhatsApp > ⋮ or Settings > Linked devices > Link a device, then point the camera here."));
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
                        step(ui, &t, 2, tl!("Send yourself a message"));
                        help(ui, &t, tl!("Open the chat with your own number in WhatsApp and send anything. That makes you the owner."));
                        status(ui, &line);
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
                    ChannelName::Telegram => {
                        step(ui, &t, 1, tl!("Create your bot"));
                        link_button(ui, tl!("Open BotFather"), TELEGRAM_BOTFATHER_URL, &mut ask);
                        help(ui, &t, tl!("Send /newbot, pick a name, then copy the token it gives you."));
                        step(ui, &t, 2, tl!("Paste what the platform gave you"));
                        if let Some(v) = secret_field(card, "botToken") {
                            field(ui, tl!("Bot token"), v, true, "123456:ABC…");
                        }
                        help(ui, &t, tl!("The token is a long line of letters and numbers; it is the bot's key, so keep it to yourself."));
                        step(ui, &t, 3, tl!("Send your bot a message"));
                        help(ui, &t, tl!("Turn the channel on, save, then open your bot in Telegram and send it anything. That makes you the owner."));
                        status(ui, &line);
                    }
                    ChannelName::Discord => {
                        step(ui, &t, 1, tl!("Create your bot"));
                        link_button(ui, tl!("Create a Discord app"), DISCORD_NEW_APP_URL, &mut ask);
                        help(ui, &t, tl!("Name it, open the Bot tab, click Reset Token, copy it. Turn on Message Content Intent."));
                        step(ui, &t, 2, tl!("Paste what the platform gave you"));
                        if let Some(v) = secret_field(card, DISCORD_APPLICATION_ID_KEY) {
                            field(ui, tl!("Application ID"), v, false, "123456789012345678");
                        }
                        help(ui, &t, tl!("General Information, at the top of the same page."));
                        if let Some(v) = secret_field(card, "botToken") {
                            field(ui, tl!("Bot token"), v, true, "");
                        }
                        help(ui, &t, tl!("The token is a long line of letters and numbers; it is the bot's key, so keep it to yourself."));
                        ui.add_space(4.0);
                        match discord_invite_url(card.secret(DISCORD_APPLICATION_ID_KEY)) {
                            Some(url) => link_button(ui, tl!("Invite the bot to your server"), &url, &mut ask),
                            None => {
                                ui.add_enabled_ui(false, |ui| {
                                    widgets::button(ui, tl!("Invite the bot to your server"), Kind::Secondary, 0.0)
                                        .on_disabled_hover_text(tl!("Paste the Application ID first"));
                                });
                                help(ui, &t, tl!("Paste the Application ID above and this button opens the invite page."));
                            }
                        }
                        step(ui, &t, 3, tl!("Send your bot a message"));
                        help(ui, &t, tl!("Turn the channel on, save, then write to the bot in your server or by direct message. That makes you the owner."));
                        status(ui, &line);
                    }
                    ChannelName::Slack => {
                        step(ui, &t, 1, tl!("Create your bot"));
                        link_button(ui, tl!("Create the Slack app"), &slack_manifest_url(), &mut ask);
                        help(ui, &t, tl!("Click Create, then Install to Workspace. Copy the Bot token (xoxb-) and the App token (xapp-)."));
                        step(ui, &t, 2, tl!("Paste what the platform gave you"));
                        if let Some(v) = secret_field(card, "botToken") {
                            field(ui, tl!("Bot token (xoxb-…)"), v, true, "xoxb-…");
                        }
                        if let Some(v) = secret_field(card, "appToken") {
                            field(ui, tl!("App token (xapp-…)"), v, true, "xapp-…");
                        }
                        help(ui, &t, tl!("Bot token: OAuth & Permissions. App token: Basic Information > App-Level Tokens, with connections:write."));
                        step(ui, &t, 3, tl!("Send your bot a message"));
                        help(ui, &t, tl!("Turn the channel on, save, then send the Redrob app a direct message in Slack. That makes you the owner."));
                        status(ui, &line);
                    }
                }

                // Advanced: the allowlist, for more than one person.
                ui.add_space(6.0);
                egui::CollapsingHeader::new(RichText::new(tl!("Advanced")).size(12.0).color(t.muted))
                    .id_salt(("channel-advanced", name.id()))
                    .default_open(false)
                    .show(ui, |ui| {
                        help(ui, &t, tl!("Who may talk to this bot. The owner is added here by their first message; add more ids for other people."));
                        ui.add_space(4.0);
                        let mut remove: Option<usize> = None;
                        for (i, id) in card.allowlist.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::TextEdit::singleline(id).desired_width(250.0).min_size(vec2(0.0, theme::CONTROL_H)).vertical_align(egui::Align::Center),
                                );
                                if widgets::glyph_button(ui, widgets::Glyph::Close, tl!("Remove")).clicked() {
                                    remove = Some(i);
                                }
                            });
                        }
                        if let Some(i) = remove {
                            card.allowlist.remove(i);
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
                                card.allowlist.push(id.trim().to_string());
                            }
                        });
                        help(ui, &t, tl!("Telegram: send /start to @userinfobot. Discord: Developer Mode, then right-click a name > Copy User ID. Slack: profile > ⋯ > Copy member ID. WhatsApp: the phone number, country code first, digits only."));
                    });
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
        help(
            ui,
            &t,
            tl!(
                "A bot that is on must survive a reboot, so the engine registers itself to start at login while any channel is on, and removes that when the last one is turned off."
            ),
        );
        // The status lines follow the engine; look again every few seconds while the page is open.
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
