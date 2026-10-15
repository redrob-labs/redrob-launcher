//! Models the launcher downloads for the apps, and what the UI is told about them.
//!
//! The list is data (`assets/models/models.json`), generated from each app's own pinned
//! catalogue, so a model the launcher fetches is exactly the file the app would have fetched: same
//! URL, same SHA-256, same size, in the folder the app reads it from. Every field is checked when
//! the manifest is read; a bad entry is dropped, never half-trusted.

use serde::{Deserialize, Deserializer, Serialize};

/// One file of a model.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ModelFile {
    /// File name inside the model's folder.
    pub name: String,
    pub url: String,
    pub sha256: String,
    pub size: u64,
}

/// The id a model shared by every Redrob app carries in `app`. Its files go in the shared
/// models folder (`~/.redrob/models/<id>`), which every app reads through the installed registry.
pub const SHARED_APP: &str = "redrob";

/// What runs a model once it is on disk.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Runtime {
    /// Plain files: the app loads them itself (Whisper, LaMa, redrob-image).
    #[default]
    Files,
    /// A GGUF the launcher serves with stock `llama-server`, OpenAI-compatible, on loopback.
    LlamaServer,
    /// A ternary GGUF that only PrismML's `llama.cpp` fork can load (Bonsai 2).
    LlamaServerPrism,
}

impl Runtime {
    /// The launcher starts a process for this kind of model.
    pub fn served(self) -> bool {
        !matches!(self, Runtime::Files)
    }

    /// The key in `runtimes.json`.
    pub fn id(self) -> &'static str {
        match self {
            Runtime::Files => "files",
            Runtime::LlamaServer => "llama-server",
            Runtime::LlamaServerPrism => "llama-server-prism",
        }
    }
}

/// A compute backend a model's weights can run on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Cuda,
    Metal,
    Vulkan,
    Cpu,
}

impl Backend {
    pub fn id(self) -> &'static str {
        match self {
            Backend::Cuda => "cuda",
            Backend::Metal => "metal",
            Backend::Vulkan => "vulkan",
            Backend::Cpu => "cpu",
        }
    }
}

fn gb_to_mb<'de, D: Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
    let gb = f64::deserialize(d)?;
    if !gb.is_finite() || gb < 0.0 {
        return Err(serde::de::Error::custom("memory_gb must be a non-negative number"));
    }
    Ok((gb * 1024.0).round() as u32)
}

/// What a model needs from the computer. Memory is whichever the backend uses: the GPU's, a
/// Mac's unified memory, or RAM on the CPU.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub struct Requires {
    /// `memory_gb` in the manifest, kept in MiB so the struct stays `Eq`.
    #[serde(rename = "memory_gb", deserialize_with = "gb_to_mb", default)]
    pub memory_mb: u32,
    #[serde(default)]
    pub backends: Vec<Backend>,
}

/// How a served model is started: which file is the model, which (if any) its vision projector.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Serve {
    pub model: String,
    #[serde(default)]
    pub mmproj: Option<String>,
}

/// A downloadable model for one app, or for every app (`app == SHARED_APP`).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Model {
    /// Folder name under the app's models folder (`whisper-base`).
    pub id: String,
    /// The app that uses it (`redrob-cut`), or `redrob` for a shared one.
    pub app: String,
    pub name: String,
    pub description: String,
    pub license: String,
    /// The model's page, for its card and licence.
    pub source: String,
    /// What it does (`chat`, `inpaint`, `stt`, ...). Free text, shown as a label.
    #[serde(default)]
    pub task: String,
    #[serde(default)]
    pub runtime: Runtime,
    #[serde(default)]
    pub requires: Requires,
    #[serde(default)]
    pub serve: Option<Serve>,
    pub files: Vec<ModelFile>,
}

impl Model {
    pub fn size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
    }

    pub fn shared(&self) -> bool {
        self.app == SHARED_APP
    }
}

#[derive(Deserialize)]
struct Manifest {
    version: u32,
    models: Vec<Model>,
}

/// The built-in manifest.
pub const MANIFEST: &str = include_str!("../../../assets/models/models.json");

fn hex64(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit())
}

/// A folder or file name we'd create: no paths, no dot names.
fn plain_name(s: &str) -> bool {
    crate::release::valid_file_name(s) && !s.contains(' ')
}

fn valid(m: &Model) -> bool {
    let known_app = m.shared() || crate::catalog::CraftApp::by_id(&m.app).is_some();
    let files_ok = !m.files.is_empty()
        && m.files
            .iter()
            .all(|f| plain_name(&f.name) && f.url.starts_with("https://") && !f.url.contains(['\\', '\n']) && hex64(&f.sha256) && f.size > 0);
    // A served model names the file to serve, and that file is one of its own.
    let serve_ok = match (&m.serve, m.runtime.served()) {
        (None, false) => true,
        (Some(s), true) => {
            let has = |name: &str| m.files.iter().any(|f| f.name == name);
            has(&s.model) && s.mmproj.as_deref().is_none_or(has)
        }
        _ => false,
    };
    plain_name(&m.id) && known_app && !m.name.is_empty() && m.source.starts_with("https://") && files_ok && serve_ok
}

/// Reads a manifest, keeping only well-formed entries for known apps (ids unique).
pub fn parse(json: &str) -> Result<Vec<Model>, String> {
    let m: Manifest = serde_json::from_str(json).map_err(|e| format!("unreadable model manifest ({e})"))?;
    if !(1..=2).contains(&m.version) {
        return Err(format!("model manifest version {} isn't supported", m.version));
    }
    let mut out: Vec<Model> = Vec::new();
    for model in m.models {
        if valid(&model) && !out.iter().any(|o| o.app == model.app && o.id == model.id) {
            out.push(model);
        }
    }
    Ok(out)
}

/// The built-in models.
pub fn builtin() -> Vec<Model> {
    parse(MANIFEST).unwrap_or_default()
}

/// Where a model stands on this computer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModelState {
    NotDownloaded,
    /// Some of it is here (finished files and partial ones); `Download` continues from there.
    Partial {
        done: u64,
        total: u64,
    },
    Downloading {
        done: u64,
        total: u64,
    },
    /// Every file is in the app's models folder.
    Installed,
    /// The last download stopped with this error (plain English; the UI translates it).
    Failed(String),
    /// The launcher doesn't know where this app keeps its models on this system.
    NoFolder,
}

/// Whether the launcher is serving a model right now.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Serving {
    /// Not a served model, or not started.
    No,
    Starting,
    /// `llama-server` answers at this loopback URL (`http://127.0.0.1:4097`).
    Running { url: String },
    Failed(String),
}

/// A model as the UI shows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelView {
    pub model: Model,
    pub state: ModelState,
    /// The folder it goes into, for the person to see (`~/.local/share/redrob-cut/models/whisper-base`).
    pub folder: Option<String>,
    /// Whether this computer can run it, from [`crate::device::Device::fit`].
    pub fit: crate::device::Fit,
    pub serving: Serving,
}

/// Where a runtime binary (`llama-server`) stands on this computer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeState {
    /// There is no build of it for this platform and backend.
    Unavailable,
    NotInstalled,
    Downloading { done: u64, total: u64 },
    Installed,
    Failed(String),
}

/// The desktop backend's model downloads and local serving. Every call returns at once.
pub trait ModelBackend {
    fn models(&self) -> Vec<ModelView>;
    /// Download (or resume) every missing file of a model.
    fn download(&mut self, app: &str, id: &str);
    /// Pause a running download, keeping what's downloaded.
    fn pause(&mut self, app: &str, id: &str);
    /// Delete a model's files (and any partial ones) from the app's folder.
    fn delete(&mut self, app: &str, id: &str) -> Result<(), String>;
    /// What this computer is, as measured at start (and re-measured on request).
    fn device(&self) -> crate::device::Device;
    fn remeasure(&mut self);
    /// The runtime a served model needs, and where it stands.
    fn runtime(&self, runtime: Runtime) -> RuntimeState;
    fn install_runtime(&mut self, runtime: Runtime);
    /// Start serving a downloaded model (installing its runtime first if needed).
    fn start(&mut self, app: &str, id: &str);
    fn stop(&mut self, app: &str, id: &str);
    /// Free space in the shared models folder, in bytes, if known.
    fn disk_free(&self) -> Option<u64>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_builtin_manifest_is_complete_and_pinned() {
        let models = builtin();
        let ids: Vec<&str> = models.iter().map(|m| m.id.as_str()).collect();
        assert_eq!(&ids[..4], ["whisper-tiny", "whisper-base", "whisper-small", "kokoro-82m"]);
        assert_eq!(models.len(), 21, "{ids:?}");
        for m in &models {
            assert!(m.shared() || crate::catalog::CraftApp::by_id(&m.app).is_some(), "{}", m.app);
            assert!(!m.task.is_empty(), "{} has no task", m.id);
            assert!(m.requires.memory_mb > 0 && !m.requires.backends.is_empty(), "{} has no requirements", m.id);
            assert_eq!(m.serve.is_some(), m.runtime.served(), "{}", m.id);
            for f in &m.files {
                // Pinned to a revision, never to a moving branch.
                assert!(!f.url.contains("/resolve/main/") && !f.url.contains("/master/"), "{}", f.url);
            }
        }
        // The narration package is 336 MB; Whisper base is the default speech model.
        assert_eq!(models[3].size(), 335_543_900);
        assert_eq!(models[1].files.len(), 4);
        // Served models: a stock GGUF and the one ternary pack that needs PrismML's fork.
        let bonsai = models.iter().find(|m| m.id == "bonsai-2-27b-pq2").map(|m| m.runtime);
        assert_eq!(bonsai, Some(Runtime::LlamaServerPrism));
        let qwen = models.iter().find(|m| m.id == "qwen3.8-27b-q4").map(|m| (m.runtime, m.requires.memory_mb));
        assert_eq!(qwen, Some((Runtime::LlamaServer, 19 * 1024)));
        // Our own text-to-image model is in, for Canvas.
        assert!(models.iter().any(|m| m.id == "redrob-image" && m.app == "redrob-canvas" && m.task == "text-to-image"));
    }

    #[test]
    fn bad_entries_are_dropped() {
        let f = r#"{"name":"a.bin","url":"https://x/a.bin","sha256":"0000000000000000000000000000000000000000000000000000000000000000","size":1}"#;
        let m = |id: &str, app: &str, file: &str| {
            format!(r#"{{"id":"{id}","app":"{app}","name":"N","description":"","license":"","source":"https://x","files":[{file}]}}"#)
        };
        let doc = |entries: Vec<String>| format!(r#"{{"version":1,"models":[{}]}}"#, entries.join(","));
        let ok = m("good", "redrob-cut", f);
        let models = parse(&doc(vec![
            ok.clone(),
            ok.clone(),
            m("../escape", "redrob-cut", f),
            m("x", "not-an-app", f),
            m("y", "redrob-cut", &f.replace("https://", "http://")),
            m("z", "redrob-cut", &f.replace("a.bin", "../a.bin")),
            m("w", "redrob-cut", &f.replace("0000", "zzzz")),
        ]))
        .unwrap();
        assert_eq!(models.iter().map(|m| m.id.as_str()).collect::<Vec<_>>(), ["good"]);
        assert!(parse(r#"{"version":3,"models":[]}"#).is_err());
        assert!(parse("<html>").is_err());
    }

    #[test]
    fn served_models_must_name_one_of_their_files() {
        let doc = |runtime: &str, serve: &str| {
            format!(
                r#"{{"version":2,"models":[{{"id":"m","app":"redrob","name":"N","description":"","license":"","source":"https://x",
                "task":"chat","runtime":"{runtime}","requires":{{"memory_gb":9.5,"backends":["cuda","cpu"]}},{serve}
                "files":[{{"name":"m.gguf","url":"https://x/m.gguf","sha256":"{z}","size":1}}]}}]}}"#,
                z = "0".repeat(64)
            )
        };
        let ok = parse(&doc("llama-server", r#""serve":{"model":"m.gguf"},"#)).unwrap();
        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0].requires.memory_mb, 9728);
        assert_eq!(ok[0].requires.backends, [Backend::Cuda, Backend::Cpu]);
        assert!(ok[0].shared());
        // The served file must exist; a plain-files model takes no `serve`; a served one needs it.
        assert!(parse(&doc("llama-server", r#""serve":{"model":"other.gguf"},"#)).unwrap().is_empty());
        assert!(parse(&doc("files", r#""serve":{"model":"m.gguf"},"#)).unwrap().is_empty());
        assert!(parse(&doc("llama-server-prism", "")).unwrap().is_empty());
    }
}
