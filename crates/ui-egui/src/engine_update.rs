//! Keeping the shared Redrob Code engine current.
//!
//! The engine is one daemon every Redrob app runs its AI through, so it is not updated like an
//! app: swapping the binary under a daemon that apps are using would leave them on the old one
//! (or fail outright on Windows, which locks a running exe). This flow decides, from what the
//! launcher already knows (installed version, newest release, the daemon's state), what to do
//! this frame:
//!
//! | Daemon | Decision |
//! |---|---|
//! | not running | install the new version now |
//! | running, nobody attached | stop it, install, start it again |
//! | running, apps attached | wait, and say so; "Update now" overrides (apps reconnect on their own) |
//!
//! The decision is a pure function so it can be tested without a daemon or a download.

use redrob_launcher_core::engine::EngineState;

/// The engine's suite id in the catalog.
pub const ENGINE_APP: &str = "redrob-code";

/// What the flow is in the middle of.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Step {
    #[default]
    Idle,
    /// Asked the daemon to stop; installing once it has.
    Stopping,
    /// The install job is running; `restart` says the daemon was up before and comes back after.
    Installing { restart: bool },
}

/// What the launcher knows this frame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Facts {
    /// The newest release's version label, when it is newer than the installed engine.
    pub newer: Option<String>,
    /// An install job for the engine is already running.
    pub job_running: bool,
    pub engine: EngineState,
    /// The person pressed "Update now": attached apps do not hold the update back.
    pub forced: bool,
    /// This version was already tried automatically (it is not retried on its own).
    pub attempted: bool,
}

/// What to do now.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    None,
    /// A newer engine waits because these apps are attached (suite names, for the dialog).
    Wait {
        apps: Vec<String>,
    },
    /// Stop the daemon; the install follows once it is down.
    Stop,
    /// Start the install job. `restart`: start the daemon again when it is done.
    Install {
        restart: bool,
    },
}

#[derive(Debug, Default)]
pub struct Flow {
    pub step: Step,
    /// Set by "Update now"; cleared once the install starts.
    pub forced: bool,
    /// Versions tried automatically, so a failed update is not retried every frame.
    pub attempted: std::collections::BTreeSet<String>,
    /// What the dialog shows: the apps holding the update back, if any.
    pub waiting_on: Vec<String>,
    /// The last (status, newer) logged, so the log says when the picture changes and not every frame.
    pub last_seen: Option<(redrob_launcher_core::AppStatus, Option<String>)>,
}

impl Flow {
    /// Decides this frame's action and moves the step along. Call every frame; it is cheap.
    pub fn tick(&mut self, facts: &Facts) -> Action {
        let action = decide(&self.step, facts);
        match &action {
            Action::Wait { apps } => self.waiting_on.clone_from(apps),
            _ => self.waiting_on.clear(),
        }
        match &action {
            Action::Stop => self.step = Step::Stopping,
            Action::Install { restart } => {
                self.step = Step::Installing { restart: *restart };
                self.forced = false;
                if let Some(v) = &facts.newer {
                    self.attempted.insert(v.clone());
                }
            }
            Action::None if self.step == Step::Stopping && matches!(facts.engine, EngineState::Running(_)) => {
                // The stop did not take: do not keep asking every frame.
                self.step = Step::Idle;
                self.forced = false;
                if let Some(v) = &facts.newer {
                    self.attempted.insert(v.clone());
                }
            }
            _ => {}
        }
        action
    }

    /// The engine's install job ended. Returns whether the daemon should be started again.
    pub fn job_finished(&mut self, ok: bool) -> bool {
        let restart = matches!(self.step, Step::Installing { restart: true });
        self.step = Step::Idle;
        self.waiting_on.clear();
        ok && restart
    }

    /// Nothing newer any more (the update landed): the version list can forget it.
    pub fn settled(&mut self) {
        self.attempted.clear();
    }
}

/// The decision for one frame, given the step the flow is in.
pub fn decide(step: &Step, facts: &Facts) -> Action {
    match step {
        Step::Installing { .. } => Action::None,
        Step::Stopping => match &facts.engine {
            EngineState::Stopped | EngineState::NotInstalled => Action::Install { restart: true },
            // Still stopping, or the stop failed (handled by `Flow::tick`).
            EngineState::Busy | EngineState::Running(_) => Action::None,
        },
        Step::Idle => {
            let Some(newer) = &facts.newer else { return Action::None };
            if facts.job_running || (facts.attempted && !facts.forced) {
                return Action::None;
            }
            let _ = newer;
            match &facts.engine {
                EngineState::Busy => Action::None,
                EngineState::Stopped | EngineState::NotInstalled => Action::Install { restart: false },
                EngineState::Running(r) if r.attached.is_empty() || facts.forced => Action::Stop,
                EngineState::Running(r) => {
                    Action::Wait { apps: r.attached.iter().map(|a| redrob_launcher_core::engine::app_display_name(&a.app)).collect() }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use redrob_launcher_core::engine::{AttachedApp, RunningEngine};

    fn running(apps: &[&str]) -> EngineState {
        EngineState::Running(RunningEngine {
            version: "0.6.4".into(),
            url: "http://127.0.0.1:4096".into(),
            pid: 1,
            attached: apps.iter().map(|a| AttachedApp { app: (*a).into(), pid: Some(2) }).collect(),
            idle_for_secs: None,
            idle_exit_after_secs: Some(600),
        })
    }

    fn facts(engine: EngineState) -> Facts {
        Facts { newer: Some("0.6.5".into()), job_running: false, engine, forced: false, attempted: false }
    }

    #[test]
    fn nothing_newer_means_nothing_to_do() {
        let mut flow = Flow::default();
        let f = Facts { newer: None, ..facts(running(&[])) };
        assert_eq!(flow.tick(&f), Action::None);
        assert_eq!(flow.step, Step::Idle);
    }

    #[test]
    fn a_stopped_engine_is_updated_at_once_and_not_restarted() {
        let mut flow = Flow::default();
        assert_eq!(flow.tick(&facts(EngineState::Stopped)), Action::Install { restart: false });
        assert_eq!(flow.step, Step::Installing { restart: false });
        // While the job runs nothing else is asked.
        assert_eq!(flow.tick(&Facts { job_running: true, ..facts(EngineState::Stopped) }), Action::None);
        assert!(!flow.job_finished(true));
        assert_eq!(flow.step, Step::Idle);
        // The version was tried; it is not tried again on its own…
        assert_eq!(flow.tick(&Facts { attempted: true, ..facts(EngineState::Stopped) }), Action::None);
        // …but "Update now" tries it again.
        assert_eq!(flow.tick(&Facts { attempted: true, forced: true, ..facts(EngineState::Stopped) }), Action::Install { restart: false });
    }

    #[test]
    fn an_idle_running_engine_is_stopped_installed_and_started_again() {
        let mut flow = Flow::default();
        assert_eq!(flow.tick(&facts(running(&[]))), Action::Stop);
        assert_eq!(flow.step, Step::Stopping);
        assert_eq!(flow.tick(&facts(EngineState::Busy)), Action::None, "still stopping");
        assert_eq!(flow.tick(&facts(EngineState::Stopped)), Action::Install { restart: true });
        assert!(flow.attempted.contains("0.6.5"));
        assert!(flow.job_finished(true), "the daemon was up before, so it comes back");
    }

    #[test]
    fn a_failed_install_does_not_restart_and_is_not_retried() {
        let mut flow = Flow::default();
        flow.tick(&facts(running(&[])));
        flow.tick(&facts(EngineState::Stopped));
        assert!(!flow.job_finished(false));
        assert_eq!(flow.tick(&Facts { attempted: true, ..facts(EngineState::Stopped) }), Action::None);
    }

    #[test]
    fn attached_apps_hold_the_update_back_until_update_now() {
        let mut flow = Flow::default();
        let busy = facts(running(&["design", "cowork"]));
        assert_eq!(flow.tick(&busy), Action::Wait { apps: vec!["Redrob Design".into(), "Redrob Cowork".into()] });
        assert_eq!(flow.waiting_on, vec!["Redrob Design".to_string(), "Redrob Cowork".to_string()]);
        assert_eq!(flow.step, Step::Idle);
        // The person says go: the daemon is stopped even with apps attached (they reconnect).
        flow.forced = true;
        assert_eq!(flow.tick(&Facts { forced: true, ..busy.clone() }), Action::Stop);
        assert!(flow.waiting_on.is_empty());
        assert_eq!(flow.tick(&Facts { forced: true, ..facts(EngineState::Stopped) }), Action::Install { restart: true });
        assert!(!flow.forced, "forced is spent by the install");
    }

    #[test]
    fn a_stop_that_does_not_take_gives_up_instead_of_looping() {
        let mut flow = Flow::default();
        assert_eq!(flow.tick(&facts(running(&[]))), Action::Stop);
        assert_eq!(flow.tick(&facts(EngineState::Busy)), Action::None);
        // The daemon is back up with nobody attached: the stop failed.
        assert_eq!(flow.tick(&facts(running(&[]))), Action::None);
        assert_eq!(flow.step, Step::Idle);
        assert!(flow.attempted.contains("0.6.5"));
    }

    #[test]
    fn a_running_job_is_never_doubled() {
        let mut flow = Flow::default();
        assert_eq!(flow.tick(&Facts { job_running: true, ..facts(EngineState::Stopped) }), Action::None);
        assert_eq!(flow.step, Step::Idle);
    }
}
