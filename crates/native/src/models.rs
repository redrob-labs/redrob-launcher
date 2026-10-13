//! Downloading the apps' models into the folders the apps read them from.
//!
//! Each model's files come from `assets/models/models.json` (pinned URL, SHA-256 and size). They are
//! downloaded with the same machinery as the apps themselves ([`Http::download`]): resumable, paused
//! by the person or by a dropped connection, checked against their SHA-256 before they are renamed
//! into place, so an app never sees a half-written or wrong file. A model counts as present the way
//! the app itself decides it: every file there with its pinned size.
//!
//! Deleting removes only the files the manifest names (and their partial downloads), never
//! anything else in the folder, and the model's folder only once it is empty.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};

use redrob_launcher_core::models::{Model, ModelBackend, ModelState, ModelView};

use crate::http::{self, Http, Stop, Transfer};
use redrob_launcher_core::protocol::{CANCELLED, PAUSED};

fn env_dir(k: &str) -> Option<PathBuf> {
    std::env::var_os(k).filter(|v| !v.is_empty()).map(PathBuf::from)
}

/// The folder an app reads its downloaded models from, as that app computes it. `None`: unknown.
///
/// Redrob Cut: `<data dir>/models`, where the data dir is `FILMCRAFT_DATA_DIR`, else
/// `~/Library/Application Support/Redrob Cut`, `%APPDATA%\Redrob Cut` or
/// `$XDG_DATA_HOME/redrob-cut` (`~/.local/share/redrob-cut`). A portable Redrob Cut keeps its data
/// beside itself; the launcher installs the regular kind, so that case isn't looked for.
pub fn models_dir(app: &str) -> Option<PathBuf> {
    match app {
        "redrob-cut" => {
            let data = env_dir("FILMCRAFT_DATA_DIR").or_else(|| {
                if cfg!(target_os = "macos") {
                    env_dir("HOME").map(|h| h.join("Library/Application Support/Redrob Cut"))
                } else if cfg!(windows) {
                    env_dir("APPDATA").map(|h| h.join("Redrob Cut"))
                } else {
                    env_dir("XDG_DATA_HOME").map(|d| d.join("redrob-cut")).or_else(|| env_dir("HOME").map(|h| h.join(".local/share/redrob-cut")))
                }
            })?;
            Some(data.join("models"))
        }
        _ => None,
    }
}

/// Where the model lives, given its app's models folder.
pub fn model_dir(models: &Path, m: &Model) -> PathBuf {
    models.join(&m.id)
}

/// The model's state from what is on disk.
pub fn disk_state(dir: &Path, m: &Model) -> ModelState {
    let mut done = 0;
    let mut complete = 0;
    for f in &m.files {
        let dest = dir.join(&f.name);
        if std::fs::metadata(&dest).is_ok_and(|md| md.len() == f.size) {
            done += f.size;
            complete += 1;
        } else if let Some(n) = http::partial_bytes(&dest, f.size) {
            done += n.min(f.size);
        }
    }
    if complete == m.files.len() {
        ModelState::Installed
    } else if done > 0 {
        ModelState::Partial { done, total: m.size() }
    } else {
        ModelState::NotDownloaded
    }
}

/// Download every missing file of `m` into `dir`. `progress` gets bytes done over the whole model.
pub fn download(http: &Http, dir: &Path, m: &Model, stop: &Stop, progress: &mut dyn FnMut(u64, u64)) -> Result<(), String> {
    let total = m.size();
    let mut before = 0;
    for f in &m.files {
        let dest = dir.join(&f.name);
        let base = before;
        http.download(&f.url, &dest, f.size, Some(&f.sha256), stop, &mut |t| {
            let (Transfer::Bytes { done, .. } | Transfer::Reconnecting { done, .. }) = t;
            progress(base + done.min(f.size), total);
        })?;
        before += f.size;
    }
    Ok(())
}

/// Remove the model's files and partial downloads, then its folder if nothing else is in it.
pub fn delete(dir: &Path, m: &Model) -> Result<(), String> {
    for f in &m.files {
        http::discard(&dir.join(&f.name), f.size);
        let left = dir.join(&f.name);
        if left.exists() {
            std::fs::remove_file(&left).map_err(|e| format!("couldn't delete {} ({e})", left.display()))?;
        }
    }
    // Only an empty folder goes; anything the app put there itself stays.
    let _ = std::fs::remove_dir(dir);
    Ok(())
}

/// Where an app's models go: the app's real folder, or a test's.
pub type FolderOf = Arc<dyn Fn(&str) -> Option<PathBuf> + Send + Sync>;

struct Job {
    stop: Arc<Stop>,
    done: u64,
    total: u64,
}

/// The launcher's model downloads: at most one job per model, each on its own thread.
pub struct Models {
    models: Vec<Model>,
    http: Option<Http>,
    jobs: Arc<Mutex<HashMap<(String, String), Job>>>,
    failed: Arc<Mutex<HashMap<(String, String), String>>>,
    wake: Arc<dyn Fn() + Send + Sync>,
    /// Where each app's models go (overridable in tests).
    dir: FolderOf,
}

impl Models {
    pub fn new(models: Vec<Model>, http: Option<Http>, wake: Arc<dyn Fn() + Send + Sync>, dir: FolderOf) -> Self {
        Self { models, http, jobs: Arc::default(), failed: Arc::default(), wake, dir }
    }

    /// The built-in manifest and the apps' real folders.
    pub fn desktop(http: Option<Http>, wake: Arc<dyn Fn() + Send + Sync>) -> Self {
        Self::new(redrob_launcher_core::models::builtin(), http, wake, Arc::new(models_dir))
    }

    fn find(&self, app: &str, id: &str) -> Option<&Model> {
        self.models.iter().find(|m| m.app == app && m.id == id)
    }
}

fn key(app: &str, id: &str) -> (String, String) {
    (app.to_string(), id.to_string())
}

impl ModelBackend for Models {
    fn models(&self) -> Vec<ModelView> {
        let jobs = self.jobs.lock().unwrap_or_else(PoisonError::into_inner);
        let failed = self.failed.lock().unwrap_or_else(PoisonError::into_inner);
        self.models
            .iter()
            .map(|m| {
                let folder = (self.dir)(&m.app).map(|d| model_dir(&d, m));
                let k = key(&m.app, &m.id);
                let state = match (&folder, jobs.get(&k), failed.get(&k)) {
                    (None, ..) => ModelState::NoFolder,
                    (Some(_), Some(j), _) => ModelState::Downloading { done: j.done, total: j.total },
                    (Some(d), None, f) => match (disk_state(d, m), f) {
                        (ModelState::Installed, _) => ModelState::Installed,
                        (_, Some(e)) => ModelState::Failed(e.clone()),
                        (s, None) => s,
                    },
                };
                ModelView { model: m.clone(), state, folder: folder.map(|d| d.display().to_string()) }
            })
            .collect()
    }

    fn download(&mut self, app: &str, id: &str) {
        let Some(m) = self.find(app, id).cloned() else { return };
        let Some(dir) = (self.dir)(app).map(|d| model_dir(&d, &m)) else { return };
        let k = key(app, id);
        let stop = Arc::new(Stop::default());
        {
            let mut jobs = self.jobs.lock().unwrap_or_else(PoisonError::into_inner);
            if jobs.contains_key(&k) {
                return;
            }
            let done = match disk_state(&dir, &m) {
                ModelState::Partial { done, .. } => done,
                _ => 0,
            };
            jobs.insert(k.clone(), Job { stop: stop.clone(), done, total: m.size() });
        }
        self.failed.lock().unwrap_or_else(PoisonError::into_inner).remove(&k);
        let Some(http) = self.http.clone() else {
            self.jobs.lock().unwrap_or_else(PoisonError::into_inner).remove(&k);
            self.failed.lock().unwrap_or_else(PoisonError::into_inner).insert(k, "HTTPS isn't available on this computer.".into());
            (self.wake)();
            return;
        };
        let (jobs, failed, wake) = (self.jobs.clone(), self.failed.clone(), self.wake.clone());
        std::thread::spawn(move || {
            let mut last = std::time::Instant::now();
            let result = download(&http, &dir, &m, &stop, &mut |done, total| {
                if let Some(j) = jobs.lock().unwrap_or_else(PoisonError::into_inner).get_mut(&k) {
                    j.done = done;
                    j.total = total;
                }
                // A repaint every quarter second is plenty for a progress bar.
                if last.elapsed() >= std::time::Duration::from_millis(250) {
                    last = std::time::Instant::now();
                    wake();
                }
            });
            jobs.lock().unwrap_or_else(PoisonError::into_inner).remove(&k);
            match result {
                Ok(()) => {}
                // Paused or cancelled: what's on disk says where it stands.
                Err(e) if e == PAUSED || e == CANCELLED => {}
                Err(e) => {
                    log::warn!("download of model {} for {} failed: {e}", m.id, m.app);
                    failed.lock().unwrap_or_else(PoisonError::into_inner).insert(k, e);
                }
            }
            wake();
        });
        (self.wake)();
    }

    fn pause(&mut self, app: &str, id: &str) {
        if let Some(j) = self.jobs.lock().unwrap_or_else(PoisonError::into_inner).get(&key(app, id)) {
            j.stop.pause();
        }
    }

    fn delete(&mut self, app: &str, id: &str) -> Result<(), String> {
        let m = self.find(app, id).cloned().ok_or("unknown model")?;
        let dir = (self.dir)(app).map(|d| model_dir(&d, &m)).ok_or("the app's models folder isn't known")?;
        if let Some(j) = self.jobs.lock().unwrap_or_else(PoisonError::into_inner).get(&key(app, id)) {
            // Stop it first: its thread deletes its partial file and ends.
            j.stop.cancel();
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
        self.failed.lock().unwrap_or_else(PoisonError::into_inner).remove(&key(app, id));
        let r = delete(&dir, &m);
        (self.wake)();
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use redrob_launcher_core::models::ModelFile;

    fn model(files: &[(&str, &[u8])]) -> Model {
        Model {
            id: "m".into(),
            app: "redrob-cut".into(),
            name: "M".into(),
            description: String::new(),
            license: String::new(),
            source: "https://x".into(),
            files: files
                .iter()
                .map(|(n, b)| ModelFile {
                    name: (*n).into(),
                    url: format!("https://example.invalid/{n}"),
                    sha256: http::hex(&sha2_of(b)),
                    size: b.len() as u64,
                })
                .collect(),
        }
    }

    fn sha2_of(b: &[u8]) -> Vec<u8> {
        use sha2::Digest;
        sha2::Sha256::digest(b).to_vec()
    }

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("redrob-launcher-models-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn state_follows_what_is_on_disk() {
        let dir = temp("state");
        let m = model(&[("a.bin", b"aaaa"), ("b.bin", b"bbbbbbbb")]);
        assert_eq!(disk_state(&dir, &m), ModelState::NotDownloaded);
        std::fs::write(dir.join("a.bin"), b"aaaa").unwrap();
        assert_eq!(disk_state(&dir, &m), ModelState::Partial { done: 4, total: 12 });
        // A partial download of the second file counts too.
        std::fs::write(dir.join("b.bin.8.part"), b"bbb").unwrap();
        assert_eq!(disk_state(&dir, &m), ModelState::Partial { done: 7, total: 12 });
        // A file of the wrong size is not "there".
        std::fs::write(dir.join("b.bin"), b"short").unwrap();
        assert_eq!(disk_state(&dir, &m), ModelState::Partial { done: 7, total: 12 });
        std::fs::write(dir.join("b.bin"), b"bbbbbbbb").unwrap();
        assert_eq!(disk_state(&dir, &m), ModelState::Installed);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn delete_removes_only_the_models_files() {
        let root = temp("delete");
        let dir = root.join("m");
        std::fs::create_dir_all(&dir).unwrap();
        let m = model(&[("a.bin", b"aaaa"), ("b.bin", b"bbbbbbbb")]);
        std::fs::write(dir.join("a.bin"), b"aaaa").unwrap();
        std::fs::write(dir.join("b.bin.8.part"), b"bb").unwrap();
        std::fs::write(dir.join("notes.txt"), b"the app's own").unwrap();
        delete(&dir, &m).unwrap();
        assert!(!dir.join("a.bin").exists() && !dir.join("b.bin.8.part").exists());
        assert!(dir.join("notes.txt").exists(), "a file the manifest doesn't name stays");
        std::fs::remove_file(dir.join("notes.txt")).unwrap();
        delete(&dir, &m).unwrap();
        assert!(!dir.exists(), "an emptied folder goes");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn views_report_unknown_folders_and_failures() {
        let root = temp("views");
        let r = root.clone();
        let m = model(&[("a.bin", b"aaaa")]);
        let mut models = Models::new(
            vec![m.clone(), Model { app: "redrob-code".into(), ..m.clone() }],
            None,
            Arc::new(|| {}),
            Arc::new(move |app: &str| (app == "redrob-cut").then(|| r.clone())),
        );
        let v = models.models();
        assert_eq!(v[0].state, ModelState::NotDownloaded);
        assert_eq!(v[1].state, ModelState::NoFolder);
        assert!(v[0].folder.as_deref().is_some_and(|f| f.ends_with("m")));
        // Without HTTPS the download fails with a reason, and nothing is left running.
        models.download("redrob-cut", "m");
        assert_eq!(models.models()[0].state, ModelState::Failed("HTTPS isn't available on this computer.".into()));
        // Present on disk wins over an old failure.
        std::fs::create_dir_all(root.join("m")).unwrap();
        std::fs::write(root.join("m").join("a.bin"), b"aaaa").unwrap();
        assert_eq!(models.models()[0].state, ModelState::Installed);
        models.delete("redrob-cut", "m").unwrap();
        assert_eq!(models.models()[0].state, ModelState::NotDownloaded);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn redrob_cut_models_go_where_it_reads_them() {
        let dir = models_dir("redrob-cut");
        assert!(dir.as_ref().is_some_and(|d| d.ends_with("models")), "{dir:?}");
        assert_eq!(models_dir("redrob-office"), None);
    }

    /// Live, end to end through the same backend the UI drives: Whisper tiny (146 MB) from
    /// Hugging Face, paused part way, resumed, finished, and every file's SHA-256 checked again
    /// (`cargo test -p redrob-launcher-native -- --ignored a_real_model_pauses_resumes_and_verifies`).
    #[test]
    #[ignore = "needs the network; downloads 146 MB"]
    fn a_real_model_pauses_resumes_and_verifies() {
        let root = temp("live-full");
        let r = root.clone();
        let tiny = redrob_launcher_core::models::builtin().into_iter().find(|m| m.id == "whisper-tiny").unwrap();
        let mut models = Models::new(vec![tiny.clone()], Some(Http::new().unwrap()), Arc::new(|| {}), Arc::new(move |_: &str| Some(r.clone())));
        let state = |m: &Models| m.models()[0].state.clone();
        let wait = |m: &Models, done: &dyn Fn(&ModelState) -> bool| {
            let until = std::time::Instant::now() + std::time::Duration::from_secs(600);
            while !done(&state(m)) && std::time::Instant::now() < until {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        };
        models.download("redrob-cut", "whisper-tiny");
        wait(&models, &|s| matches!(s, ModelState::Downloading { done, .. } if *done > 20 << 20));
        models.pause("redrob-cut", "whisper-tiny");
        wait(&models, &|s| !matches!(s, ModelState::Downloading { .. }));
        let paused = state(&models);
        assert!(matches!(paused, ModelState::Partial { done, total } if done > 0 && done < total), "{paused:?}");
        models.download("redrob-cut", "whisper-tiny");
        wait(&models, &|s| *s == ModelState::Installed);
        assert_eq!(state(&models), ModelState::Installed);
        for f in &tiny.files {
            assert_eq!(http::sha256_file(&root.join("whisper-tiny").join(&f.name)).unwrap(), f.sha256, "{}", f.name);
        }
        eprintln!("paused at {paused:?}, then finished and verified {} files", tiny.files.len());
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Live: downloads Whisper tiny's config from Hugging Face and checks its pin
    /// (`cargo test -p redrob-launcher-native -- --ignored real_model_file`).
    #[test]
    #[ignore = "needs the network"]
    fn real_model_file() {
        let models = redrob_launcher_core::models::builtin();
        let tiny = models.iter().find(|m| m.id == "whisper-tiny").unwrap();
        let one = Model { files: vec![tiny.files[0].clone()], ..tiny.clone() };
        let dir = temp("live");
        let http = Http::new().unwrap();
        download(&http, &dir, &one, &Stop::default(), &mut |_, _| {}).unwrap();
        assert_eq!(disk_state(&dir, &one), ModelState::Installed);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
