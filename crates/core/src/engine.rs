//! The shared Redrob Code engine, as the UI sees it.
//!
//! Every Redrob app on this computer runs its AI through one `redrob serve --register` daemon
//! (the engine's `docs/LOCAL-ENGINE-API.md`). The launcher is where a person sees that daemon:
//! whether it is up, which version, which apps are attached, and buttons to start, stop and
//! restart it. The desktop backend implements [`EngineBackend`]; the web build has none.

/// One app attached to the engine.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttachedApp {
    /// The label the app gave (`cowork`, `design`, ...), mapped to a suite name when known.
    pub app: String,
    pub pid: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineState {
    /// No `redrob` binary found: install Redrob Code first.
    NotInstalled,
    /// A binary exists, but no daemon answers.
    Stopped,
    /// Starting or stopping: a command is running on a thread.
    Busy,
    Running(RunningEngine),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunningEngine {
    pub version: String,
    pub url: String,
    pub pid: u32,
    pub attached: Vec<AttachedApp>,
    /// Seconds nobody has been attached; `None` while someone is.
    pub idle_for_secs: Option<u64>,
    /// The daemon exits after this many seconds idle; `None` when it stays up.
    pub idle_exit_after_secs: Option<u64>,
}

impl RunningEngine {
    /// Seconds until the daemon exits on its own, when nobody is attached.
    pub fn exits_in_secs(&self) -> Option<u64> {
        Some(self.idle_exit_after_secs?.saturating_sub(self.idle_for_secs?))
    }
}

/// The suite's name for an app label the engine reports (`design` -> "Redrob Design").
pub fn app_display_name(label: &str) -> String {
    let id = if label.starts_with("redrob-") { label.to_string() } else { format!("redrob-{label}") };
    match crate::CraftApp::by_id(&id) {
        Some(app) => app.name.to_string(),
        None => label.to_string(),
    }
}

/// The desktop backend's engine control. Every call returns at once; start and stop run on a
/// thread and show up in [`EngineBackend::state`] as `Busy`, then the result.
pub trait EngineBackend {
    fn state(&mut self) -> EngineState;
    /// Look again now (the dialog opened, or the person pressed Refresh).
    fn refresh(&mut self);
    /// Find the daemon or start it.
    fn start(&mut self);
    fn stop(&mut self);
    fn restart(&mut self);
    /// What the last start/stop said when it failed, in plain words; cleared by the next one.
    fn last_error(&self) -> Option<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_labels_map_to_suite_names() {
        assert_eq!(app_display_name("design"), "Redrob Design");
        assert_eq!(app_display_name("redrob-cowork"), "Redrob Cowork");
        assert_eq!(app_display_name("probe-app"), "probe-app");
    }

    #[test]
    fn exit_countdown_needs_both_numbers() {
        let mut e = RunningEngine {
            version: "0.6.0".into(),
            url: "u".into(),
            pid: 1,
            attached: vec![],
            idle_for_secs: Some(100),
            idle_exit_after_secs: Some(600),
        };
        assert_eq!(e.exits_in_secs(), Some(500));
        e.idle_for_secs = None;
        assert_eq!(e.exits_in_secs(), None);
        e.idle_for_secs = Some(900);
        assert_eq!(e.exits_in_secs(), Some(0));
    }
}
