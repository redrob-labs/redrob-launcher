//! Chat channels and bridged apps of the shared Redrob Code engine, as the UI sees them.
//!
//! The engine's bots (Slack, Telegram, Discord, WhatsApp) run inside the `redrob serve --register`
//! daemon and are configured by two files the engine's `docs/CHANNELS.md` describes:
//! `channels.json` (which channels are on, who may talk) and `channels/secrets.json` (bot tokens,
//! 0600). This module holds the file shapes, the reading and writing that preserves whatever keys
//! the engine adds later, the daemon's `/channels/status` and `/v1/apps` answer shapes, and the
//! [`ChannelsBackend`] trait the desktop backend implements. The HTTP calls and the processes live
//! in the desktop crate; nothing here talks to the network.

use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::{Map, Value};

/// The four channels the engine knows, in the order the Settings page shows them: the one that
/// needs no token first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChannelName {
    WhatsApp,
    Telegram,
    Discord,
    Slack,
}

impl ChannelName {
    pub const ALL: [Self; 4] = [Self::WhatsApp, Self::Telegram, Self::Discord, Self::Slack];

    /// The name in `channels.json` and on the `redrob channels` command line.
    pub fn id(self) -> &'static str {
        match self {
            Self::Telegram => "telegram",
            Self::Slack => "slack",
            Self::Discord => "discord",
            Self::WhatsApp => "whatsapp",
        }
    }

    /// The platform's own name, as shown to people (never translated).
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Telegram => "Telegram",
            Self::Slack => "Slack",
            Self::Discord => "Discord",
            Self::WhatsApp => "WhatsApp",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.id() == id)
    }

    /// The secret key names this channel reads from `secrets.json` (CHANNELS.md, "Secret key names").
    /// Discord's `applicationId` is not a secret the engine reads; the launcher keeps it there so
    /// the "Invite the bot" link can be built without another file.
    pub fn secret_keys(self) -> &'static [&'static str] {
        match self {
            Self::Slack => &["botToken", "appToken"],
            Self::Discord => &["botToken", DISCORD_APPLICATION_ID_KEY],
            Self::Telegram => &["botToken"],
            Self::WhatsApp => &[],
        }
    }
}

/// The secrets key under which the launcher keeps the Discord application id.
pub const DISCORD_APPLICATION_ID_KEY: &str = "applicationId";

/// How long a freshly enabled bot with nobody allowed waits for its first message, which makes
/// the sender its owner (the engine's `ChannelConfig.claimUntil`).
pub const CLAIM_WINDOW_MS: u64 = 15 * 60 * 1000;

/// One channel's entry in `channels.json`, the two fields the launcher edits.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChannelConfig {
    pub enabled: bool,
    /// Platform sender ids allowed to talk. Empty means nobody.
    pub allowlist: Vec<String>,
}

/// `channels.json` as a document: the launcher edits `enabled` and `allowlist` of each channel and
/// leaves every other key (`directory`, `maxChunk`, `options`, future ones) exactly as it found it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelsFile {
    root: Map<String, Value>,
}

impl Default for ChannelsFile {
    fn default() -> Self {
        let mut root = Map::new();
        root.insert("version".into(), Value::from(1));
        root.insert("channels".into(), Value::Object(Map::new()));
        Self { root }
    }
}

impl ChannelsFile {
    /// Parses the file's text; anything that is not a JSON object starts over from an empty file.
    pub fn parse(text: &str) -> Self {
        match serde_json::from_str::<Value>(text) {
            Ok(Value::Object(mut root)) => {
                root.entry("version").or_insert(Value::from(1));
                if !root.get("channels").is_some_and(Value::is_object) {
                    root.insert("channels".into(), Value::Object(Map::new()));
                }
                Self { root }
            }
            _ => Self::default(),
        }
    }

    fn channels(&self) -> Option<&Map<String, Value>> {
        self.root.get("channels")?.as_object()
    }

    pub fn get(&self, name: ChannelName) -> ChannelConfig {
        let Some(entry) = self.channels().and_then(|c| c.get(name.id())).and_then(Value::as_object) else { return ChannelConfig::default() };
        ChannelConfig {
            enabled: entry.get("enabled").and_then(Value::as_bool).unwrap_or(false),
            allowlist: entry
                .get("allowlist")
                .and_then(Value::as_array)
                .map(|a| a.iter().filter_map(Value::as_str).map(str::to_string).collect())
                .unwrap_or_default(),
        }
    }

    /// Sets `enabled` and `allowlist` for `name`, keeping the entry's other keys.
    pub fn set(&mut self, name: ChannelName, config: &ChannelConfig) {
        let Some(channels) = object_entry(&mut self.root, "channels") else { return };
        let Some(entry) = object_entry(channels, name.id()) else { return };
        entry.insert("enabled".into(), Value::Bool(config.enabled));
        entry.insert("allowlist".into(), Value::Array(config.allowlist.iter().map(|s| Value::String(s.clone())).collect()));
    }

    /// Opens the ownership claim window for `name` until `until_ms` (Unix milliseconds): the first
    /// person to message the bot before then becomes its owner. The engine's `redrob channels
    /// enable` does the same; writing it here too covers an engine that is not running.
    pub fn open_claim(&mut self, name: ChannelName, until_ms: u64) {
        let Some(channels) = object_entry(&mut self.root, "channels") else { return };
        let Some(entry) = object_entry(channels, name.id()) else { return };
        entry.insert("claimUntil".into(), Value::from(until_ms));
    }

    /// The document as the engine reads it, pretty-printed with a trailing newline.
    pub fn to_json(&self) -> String {
        let mut text = serde_json::to_string_pretty(&Value::Object(self.root.clone())).unwrap_or_else(|_| "{}".into());
        text.push('\n');
        text
    }
}

/// `map[key]` as an object, made one (replacing a non-object) when it isn't.
fn object_entry<'a>(map: &'a mut Map<String, Value>, key: &str) -> Option<&'a mut Map<String, Value>> {
    let slot = map.entry(key).or_insert_with(|| Value::Object(Map::new()));
    if !slot.is_object() {
        *slot = Value::Object(Map::new());
    }
    slot.as_object_mut()
}

/// `channels/secrets.json`: `{ "<channel>": { "<key>": "<value>" } }`. Other channels' entries and
/// unknown keys are kept as found.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SecretsFile {
    root: Map<String, Value>,
}

impl SecretsFile {
    pub fn parse(text: &str) -> Self {
        match serde_json::from_str::<Value>(text) {
            Ok(Value::Object(root)) => Self { root },
            _ => Self::default(),
        }
    }

    pub fn get(&self, channel: ChannelName, key: &str) -> Option<&str> {
        self.root.get(channel.id())?.as_object()?.get(key)?.as_str()
    }

    /// Stores `value`; an empty value removes the key, and a channel left with no keys is dropped,
    /// like the engine's own `delete`.
    pub fn set(&mut self, channel: ChannelName, key: &str, value: &str) {
        if value.is_empty() {
            if let Some(Value::Object(entry)) = self.root.get_mut(channel.id()) {
                entry.remove(key);
                if entry.is_empty() {
                    self.root.remove(channel.id());
                }
            }
            return;
        }
        if let Some(entry) = object_entry(&mut self.root, channel.id()) {
            entry.insert(key.into(), Value::String(value.into()));
        }
    }

    pub fn to_json(&self) -> String {
        let mut text = serde_json::to_string_pretty(&Value::Object(self.root.clone())).unwrap_or_else(|_| "{}".into());
        text.push('\n');
        text
    }
}

/// `<home>/channels.json`. `home` is the engine's channels home: `REDROB_CHANNELS_HOME`, else
/// `~/.redrob` (the desktop crate resolves it).
pub fn config_path(home: &Path) -> PathBuf {
    home.join("channels.json")
}

/// `<home>/channels/secrets.json`.
pub fn secrets_path(home: &Path) -> PathBuf {
    home.join("channels").join("secrets.json")
}

/// `<home>/channels/whatsapp`: the WhatsApp pairing session, written by the engine.
pub fn whatsapp_dir(home: &Path) -> PathBuf {
    home.join("channels").join("whatsapp")
}

/// The QR code the engine writes while WhatsApp is unpaired; removed once paired.
pub fn whatsapp_qr_path(home: &Path) -> PathBuf {
    whatsapp_dir(home).join("qr.png")
}

/// Reads `channels.json`; a missing file is an empty configuration.
pub fn read_config(home: &Path) -> Result<ChannelsFile, String> {
    match std::fs::read_to_string(config_path(home)) {
        Ok(text) => Ok(ChannelsFile::parse(&text)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(ChannelsFile::default()),
        Err(e) => Err(format!("couldn't read channels.json: {e}")),
    }
}

/// Writes `channels.json` (0644, the engine's own mode), creating the home folder when needed.
pub fn write_config(home: &Path, file: &ChannelsFile) -> Result<(), String> {
    std::fs::create_dir_all(home).map_err(|e| format!("couldn't create {}: {e}", home.display()))?;
    write_atomic(&config_path(home), file.to_json().as_bytes(), false)
}

/// Reads `channels/secrets.json`; a missing file holds no secrets.
pub fn read_secrets(home: &Path) -> Result<SecretsFile, String> {
    match std::fs::read_to_string(secrets_path(home)) {
        Ok(text) => Ok(SecretsFile::parse(&text)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(SecretsFile::default()),
        Err(e) => Err(format!("couldn't read the secrets file: {e}")),
    }
}

/// Writes `channels/secrets.json`: the folder 0700, the file 0600 (on Unix), like the engine.
pub fn write_secrets(home: &Path, file: &SecretsFile) -> Result<(), String> {
    let path = secrets_path(home);
    let dir = path.parent().unwrap_or(home);
    std::fs::create_dir_all(dir).map_err(|e| format!("couldn't create {}: {e}", dir.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700));
    }
    write_atomic(&path, file.to_json().as_bytes(), true)
}

/// Write to a sibling temp file, then rename over `path`, so a crash leaves the old file intact.
fn write_atomic(path: &Path, bytes: &[u8], private: bool) -> Result<(), String> {
    let tmp = path.with_extension("json.tmp");
    let describe = |e: std::io::Error| format!("couldn't write {}: {e}", path.display());
    {
        let mut open = std::fs::OpenOptions::new();
        open.write(true).create(true).truncate(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            open.mode(if private { 0o600 } else { 0o644 });
        }
        #[cfg(not(unix))]
        let _ = private;
        let mut f = open.open(&tmp).map_err(describe)?;
        std::io::Write::write_all(&mut f, bytes).map_err(describe)?;
        std::io::Write::flush(&mut f).map_err(describe)?;
    }
    #[cfg(unix)]
    if private {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o600));
    }
    std::fs::rename(&tmp, path).map_err(describe)
}

/// The Slack app manifest for the Redrob bot, the engine's `redrob channels slack-manifest` text.
pub const SLACK_MANIFEST_YAML: &str = "display_information:
  name: Redrob
  description: Talk to your Redrob Code engine from Slack.
  background_color: '#1a1a1a'
features:
  bot_user:
    display_name: Redrob
    always_online: true
oauth_config:
  scopes:
    bot:
      - app_mentions:read
      - chat:write
      - im:history
      - im:read
      - im:write
      - channels:history
      - groups:history
      - mpim:history
      - users:read
settings:
  event_subscriptions:
    bot_events:
      - app_mention
      - message.im
      - message.channels
  interactivity:
    is_enabled: false
  org_deploy_enabled: false
  socket_mode_enabled: true
  token_rotation_enabled: false
";

/// Discord's Developer Portal, opened straight on the "new application" dialog.
pub const DISCORD_NEW_APP_URL: &str = "https://discord.com/developers/applications?new_application=true";

/// Telegram's BotFather, which makes bots and hands out their tokens.
pub const TELEGRAM_BOTFATHER_URL: &str = "https://t.me/BotFather";

/// Discord's permissions bitfield the Redrob bot asks for when invited: View Channels, Send
/// Messages, Read Message History.
pub const DISCORD_INVITE_PERMISSIONS: u64 = 68608;

/// The link that adds the Discord bot of application `app_id` to a server the person manages.
/// `None` when the id is empty or not a Discord snowflake (digits only).
pub fn discord_invite_url(app_id: &str) -> Option<String> {
    let id = app_id.trim();
    if id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some(format!("https://discord.com/oauth2/authorize?client_id={id}&scope=bot&permissions={DISCORD_INVITE_PERMISSIONS}"))
}

/// The api.slack.com "create an app" page with the manifest pre-filled
/// (`redrob channels slack-manifest --url`).
pub fn slack_manifest_url() -> String {
    format!("https://api.slack.com/apps?new_app=1&manifest_yaml={}", encode_uri_component(SLACK_MANIFEST_YAML))
}

/// JavaScript's `encodeURIComponent`: everything but `A-Z a-z 0-9 - _ . ! ~ * ' ( )` as `%XX`.
pub fn encode_uri_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')' => out.push(char::from(b)),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Whether the engine is registered to start at login (CHANNELS.md, "Start at login"): the systemd
/// user unit on Linux, the launchd agent on macOS, the `Redrob Engine` scheduled task on Windows.
/// `home_dir` is the user's home.
pub fn autostart_installed(home_dir: &Path) -> bool {
    #[cfg(target_os = "linux")]
    {
        home_dir.join(".config").join("systemd").join("user").join("redrob-engine.service").is_file()
    }
    #[cfg(target_os = "macos")]
    {
        home_dir.join("Library").join("LaunchAgents").join("ai.redrob.engine.plist").is_file()
    }
    #[cfg(target_os = "windows")]
    {
        let _ = home_dir;
        std::process::Command::new("schtasks.exe").args(["/Query", "/TN", "Redrob Engine"]).output().is_ok_and(|o| o.status.success())
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        let _ = home_dir;
        false
    }
}

/// One row of `GET /channels/status`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ChannelStatus {
    pub name: String,
    pub enabled: bool,
    pub connected: bool,
    /// Whether this build of the engine has a transport for the channel.
    pub available: bool,
    pub last_error: Option<String>,
    /// The platform id of the person who owns this bot (the first to message it while the claim
    /// window was open, or the first allowlisted id); `None` while nobody does.
    pub owner: Option<String>,
    /// Whether the bot is waiting for its first message to pick an owner.
    pub claim_open: bool,
}

/// `GET /channels/status`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct ChannelsStatus {
    pub channels: Vec<ChannelStatus>,
}

impl ChannelsStatus {
    pub fn get(&self, name: ChannelName) -> Option<&ChannelStatus> {
        self.channels.iter().find(|c| c.name == name.id())
    }
}

/// `POST /channels/reload`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct ChannelsReload {
    pub running: Vec<String>,
}

/// One app bridged to the engine (`GET /v1/apps`, the engine's `docs/APP-BRIDGE.md`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct BridgedApp {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub tools: Option<Vec<String>>,
    pub pid: u64,
    /// Unix milliseconds of the last heartbeat.
    pub last_seen: u64,
    pub registered_at: u64,
}

/// `GET /v1/apps`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AppsList {
    pub apps: Vec<BridgedApp>,
    pub ttl_ms: u64,
}

/// One channel as the Settings page edits it: the config entry plus its secrets, by key name.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChannelDraft {
    pub enabled: bool,
    pub allowlist: Vec<String>,
    /// `(key, value)` for each of the channel's [`ChannelName::secret_keys`]; empty value = unset.
    pub secrets: Vec<(String, String)>,
}

/// What the backend read from the files.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Loaded {
    /// In [`ChannelName::ALL`] order.
    pub channels: Vec<(ChannelName, ChannelDraft)>,
    /// Whether the engine starts at login.
    pub autostart: bool,
}

impl Loaded {
    pub fn get(&self, name: ChannelName) -> Option<&ChannelDraft> {
        self.channels.iter().find(|(n, _)| *n == name).map(|(_, d)| d)
    }
}

/// What the running engine says about its channels.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Live {
    /// Not looked yet.
    #[default]
    Unknown,
    /// No daemon record, or the daemon does not answer.
    NoEngine,
    /// The engine runs but the status call failed.
    Unreachable(String),
    Up(ChannelsStatus),
}

/// WhatsApp's pairing, read from the engine's session folder.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum WhatsAppPairing {
    #[default]
    Unknown,
    /// No session and no QR code yet: the channel has not been started.
    Unpaired,
    /// The engine waits for a scan; the PNG it wrote.
    Pairing(std::sync::Arc<[u8]>),
    Paired,
}

/// How a save ended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SaveOutcome {
    /// Written, and the running engine picked it up.
    Reloaded,
    /// Written; no engine is running, so it applies when one starts.
    TakesEffectOnStart,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum SaveState {
    #[default]
    Idle,
    Saving,
    Saved(SaveOutcome),
    Failed(String),
}

/// The engine's bridged apps.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Apps {
    #[default]
    Unknown,
    NoEngine,
    Unreachable(String),
    Up {
        apps: Vec<BridgedApp>,
        /// Unix milliseconds when the list was read, for "last seen" ages.
        now_ms: u64,
    },
}

/// The desktop backend's channel control. Every call returns at once: reads and writes run on a
/// thread, and the getters hand back the last snapshot (refreshing it in the background when it is
/// stale), so a frame never waits on a file or the daemon.
pub trait ChannelsBackend {
    /// Read the files again now (the page opened).
    fn load(&mut self);
    /// The files as last read; `None` until the first load finishes.
    fn loaded(&mut self) -> Option<Loaded>;
    /// `/channels/status`, refreshed every few seconds while asked.
    fn live(&mut self) -> Live;
    /// WhatsApp's pairing state and QR code, re-read every couple of seconds while asked.
    fn whatsapp(&mut self) -> WhatsAppPairing;
    /// Forget the WhatsApp pairing (deletes the session folder); the engine shows a new QR code.
    fn unlink_whatsapp(&mut self);
    /// Write config and secrets, run `redrob channels enable|disable` for each channel in
    /// `toggled`, then ask the running engine to reload.
    fn save(&mut self, drafts: Vec<(ChannelName, ChannelDraft)>, toggled: Vec<ChannelName>);
    fn save_state(&mut self) -> SaveState;
    /// `/v1/apps`, refreshed every few seconds while asked.
    fn apps(&mut self) -> Apps;
    /// Look at the engine again now (the page opened, or the person pressed Refresh).
    fn refresh(&mut self);
}
