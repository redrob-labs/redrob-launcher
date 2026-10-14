//! The shared Redrob Code engine, from the desktop launcher.
//!
//! Reads the daemon's record and asks it who is attached (`redrob-engine-client`); starts, stops
//! and restarts it through the engine's own `redrob service` commands on a thread, so a frame never
//! waits on a process. The UI sees [`EngineState`] and nothing else: the record's secret stays in
//! this module.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use redrob_engine_client as engine;
use redrob_launcher_core::engine::{AttachedApp, EngineBackend, EngineState, RunningEngine};

use crate::account::Wake;

/// How long a daemon the launcher starts stays up with nobody attached. The engine's default.
const IDLE_EXIT_MINUTES: u32 = 10;
/// A fresh look is at most this old before `state()` reads the record again.
const FRESH_FOR: Duration = Duration::from_secs(5);

#[derive(Clone, Debug)]
struct Snapshot {
    state: EngineState,
    taken: Instant,
}

pub struct Engine {
    snapshot: Arc<Mutex<Option<Snapshot>>>,
    busy: Arc<AtomicBool>,
    last_error: Arc<Mutex<Option<String>>>,
    wake: Wake,
    /// What runs the work: the real engine, or a test double.
    ops: Arc<dyn Ops + Send + Sync>,
}

/// The four things the backend does to the engine, so tests can stand in for the real one.
pub trait Ops {
    fn installed(&self) -> bool;
    /// The daemon's record if it answers, plus who is attached.
    fn look(&self) -> Option<(engine::Record, Option<engine::Clients>)>;
    fn start(&self) -> Result<engine::Record, String>;
    fn stop(&self) -> Result<bool, String>;
}

struct RealOps;

impl Ops for RealOps {
    fn installed(&self) -> bool {
        engine::installed()
    }
    fn look(&self) -> Option<(engine::Record, Option<engine::Clients>)> {
        let record = engine::current()?;
        let clients = engine::clients(&record).ok();
        Some((record, clients))
    }
    fn start(&self) -> Result<engine::Record, String> {
        engine::connect_or_start(IDLE_EXIT_MINUTES)
    }
    fn stop(&self) -> Result<bool, String> {
        engine::stop()
    }
}

fn running(record: &engine::Record, clients: Option<&engine::Clients>) -> RunningEngine {
    RunningEngine {
        version: record.version.clone(),
        url: record.url.clone(),
        pid: record.pid,
        attached: clients.map(|c| c.clients.iter().map(|a| AttachedApp { app: a.app.clone(), pid: a.pid }).collect()).unwrap_or_default(),
        idle_for_secs: clients.and_then(|c| c.idle_for_ms).map(|ms| ms / 1000),
        idle_exit_after_secs: clients.and_then(|c| c.idle_exit_after_ms).map(|ms| ms / 1000),
    }
}

impl Engine {
    pub fn desktop(wake: Wake) -> Self {
        Self::new(Arc::new(RealOps), wake)
    }

    pub fn new(ops: Arc<dyn Ops + Send + Sync>, wake: Wake) -> Self {
        Self { snapshot: Arc::new(Mutex::new(None)), busy: Arc::new(AtomicBool::new(false)), last_error: Arc::new(Mutex::new(None)), wake, ops }
    }

    fn look_now(ops: &dyn Ops) -> EngineState {
        match ops.look() {
            Some((record, clients)) => EngineState::Running(running(&record, clients.as_ref())),
            None if ops.installed() => EngineState::Stopped,
            None => EngineState::NotInstalled,
        }
    }

    fn store(snapshot: &Mutex<Option<Snapshot>>, state: EngineState) {
        *snapshot.lock().unwrap_or_else(PoisonError::into_inner) = Some(Snapshot { state, taken: Instant::now() });
    }

    /// Run `work` on a thread, then look again. One at a time: a second press while busy is ignored.
    fn run(&mut self, work: impl FnOnce(&dyn Ops) -> Result<(), String> + Send + 'static) {
        if self.busy.swap(true, Ordering::SeqCst) {
            return;
        }
        *self.last_error.lock().unwrap_or_else(PoisonError::into_inner) = None;
        Self::store(&self.snapshot, EngineState::Busy);
        let (ops, snapshot, busy, last_error, wake) =
            (self.ops.clone(), self.snapshot.clone(), self.busy.clone(), self.last_error.clone(), self.wake.clone());
        std::thread::spawn(move || {
            if let Err(e) = work(ops.as_ref()) {
                log::warn!("engine: {e}");
                *last_error.lock().unwrap_or_else(PoisonError::into_inner) = Some(e);
            }
            Self::store(&snapshot, Self::look_now(ops.as_ref()));
            busy.store(false, Ordering::SeqCst);
            wake();
        });
    }
}

impl EngineBackend for Engine {
    fn state(&mut self) -> EngineState {
        if self.busy.load(Ordering::SeqCst) {
            return EngineState::Busy;
        }
        let fresh = self.snapshot.lock().unwrap_or_else(PoisonError::into_inner).clone().filter(|s| s.taken.elapsed() < FRESH_FOR);
        match fresh {
            Some(s) => s.state,
            None => {
                let state = Self::look_now(self.ops.as_ref());
                Self::store(&self.snapshot, state.clone());
                state
            }
        }
    }

    fn refresh(&mut self) {
        *self.snapshot.lock().unwrap_or_else(PoisonError::into_inner) = None;
    }

    fn start(&mut self) {
        self.run(|ops| ops.start().map(|_| ()));
    }

    fn stop(&mut self) {
        self.run(|ops| ops.stop().map(|_| ()));
    }

    fn restart(&mut self) {
        self.run(|ops| {
            ops.stop()?;
            ops.start().map(|_| ())
        });
    }

    fn last_error(&self) -> Option<String> {
        self.last_error.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU32;

    struct Fake {
        installed: bool,
        up: Mutex<Option<engine::Record>>,
        starts: AtomicU32,
        fail_start: bool,
    }

    fn record(pid: u32) -> engine::Record {
        engine::Record { id: format!("id{pid}"), version: "0.6.0".into(), url: "http://127.0.0.1:4096".into(), port: 4096, pid, secret: "s".into() }
    }

    impl Ops for Fake {
        fn installed(&self) -> bool {
            self.installed
        }
        fn look(&self) -> Option<(engine::Record, Option<engine::Clients>)> {
            let r = self.up.lock().unwrap().clone()?;
            let clients = engine::Clients {
                clients: vec![engine::Client { id: "c".into(), app: "design".into(), pid: Some(7), attached_at: 1, last_seen: 2 }],
                ttl_ms: 90_000,
                idle_for_ms: None,
                idle_exit_after_ms: Some(600_000),
            };
            Some((r, Some(clients)))
        }
        fn start(&self) -> Result<engine::Record, String> {
            self.starts.fetch_add(1, Ordering::SeqCst);
            if self.fail_start {
                // Stay busy for a moment: the test presses twice and the second press must find
                // the first still running, on a fast runner too.
                std::thread::sleep(Duration::from_millis(50));
                return Err("could not run redrob".into());
            }
            let r = record(42);
            *self.up.lock().unwrap() = Some(r.clone());
            Ok(r)
        }
        fn stop(&self) -> Result<bool, String> {
            Ok(self.up.lock().unwrap().take().is_some())
        }
    }

    fn settle(e: &mut Engine) {
        for _ in 0..200 {
            if !e.busy.load(Ordering::SeqCst) {
                return;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        panic!("engine stayed busy");
    }

    #[test]
    fn states_follow_the_daemon_and_the_binary() {
        let wake: Wake = Arc::new(|| {});
        let mut e =
            Engine::new(Arc::new(Fake { installed: false, up: Mutex::new(None), starts: AtomicU32::new(0), fail_start: false }), wake.clone());
        assert_eq!(e.state(), EngineState::NotInstalled);
        let mut e = Engine::new(Arc::new(Fake { installed: true, up: Mutex::new(None), starts: AtomicU32::new(0), fail_start: false }), wake.clone());
        assert_eq!(e.state(), EngineState::Stopped);
        e.start();
        // The start runs on a thread: it is usually still Busy here, but a fast machine can already
        // report Running. Either is right; only a Stopped/NotInstalled read would be a bug.
        assert!(matches!(e.state(), EngineState::Busy | EngineState::Running(_)), "{:?}", e.state());
        settle(&mut e);
        match e.state() {
            EngineState::Running(r) => {
                assert_eq!(r.pid, 42);
                assert_eq!(r.attached.len(), 1);
                assert_eq!(r.attached[0].app, "design");
                assert_eq!(r.idle_exit_after_secs, Some(600));
            }
            other => panic!("{other:?}"),
        }
        e.stop();
        settle(&mut e);
        assert_eq!(e.state(), EngineState::Stopped);
        assert_eq!(e.last_error(), None);
    }

    #[test]
    fn a_failed_start_is_reported_and_a_press_while_busy_is_ignored() {
        let wake: Wake = Arc::new(|| {});
        let fake = Arc::new(Fake { installed: true, up: Mutex::new(None), starts: AtomicU32::new(0), fail_start: true });
        let mut e = Engine::new(fake.clone(), wake);
        e.start();
        e.start();
        settle(&mut e);
        assert_eq!(fake.starts.load(Ordering::SeqCst), 1);
        assert_eq!(e.state(), EngineState::Stopped);
        assert!(e.last_error().unwrap().contains("could not run redrob"));
    }
}
