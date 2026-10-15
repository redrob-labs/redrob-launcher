//! The installed-models registry: what the launcher has put on this computer, for the apps.
//!
//! Every Redrob app reads one file, `~/.redrob/models/installed.json` (`REDROB_MODELS_DIR`
//! overrides the folder), to learn which local models exist, where their files are, and which of
//! them the launcher is serving right now. An app that finds no entry for the task it needs falls
//! back to the Console; the registry is how a local model gets used instead. The launcher is the
//! only writer; it rewrites the whole file after every change, atomically.

use serde::{Deserialize, Serialize};

use crate::models::{Model, Runtime};

pub const REGISTRY_VERSION: u32 = 1;
pub const REGISTRY_FILE: &str = "installed.json";

/// A served model's endpoint. OpenAI-compatible (`/v1/chat/completions`, `/v1/models`,
/// `/v1/embeddings`), no auth, loopback only.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Served {
    pub url: String,
    pub port: u16,
    pub pid: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstalledModel {
    pub id: String,
    /// `redrob` for a shared model, else the app id.
    pub app: String,
    pub task: String,
    pub runtime: Runtime,
    /// The model's folder, absolute.
    pub path: String,
    /// File names inside `path`, all present with their pinned size.
    pub files: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub served: Option<Served>,
}

impl InstalledModel {
    pub fn from_model(m: &Model, path: String, served: Option<Served>) -> Self {
        Self {
            id: m.id.clone(),
            app: m.app.clone(),
            task: m.task.clone(),
            runtime: m.runtime,
            path,
            files: m.files.iter().map(|f| f.name.clone()).collect(),
            served,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Registry {
    pub version: u32,
    /// The backend this computer uses (`cuda`, `metal`, `vulkan`, `cpu`), for apps that pick a
    /// code path by it.
    pub backend: String,
    pub models: Vec<InstalledModel>,
}

impl Registry {
    pub fn new(backend: &str, models: Vec<InstalledModel>) -> Self {
        Self { version: REGISTRY_VERSION, backend: backend.to_string(), models }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn parse(json: &str) -> Result<Self, String> {
        let r: Self = serde_json::from_str(json).map_err(|e| format!("unreadable registry ({e})"))?;
        if r.version != REGISTRY_VERSION {
            return Err(format!("registry version {} isn't supported", r.version));
        }
        Ok(r)
    }

    /// The first installed model for a task, served ones first: what an app asks.
    pub fn for_task<'a>(&'a self, task: &str) -> Option<&'a InstalledModel> {
        self.models.iter().filter(|m| m.task == task).max_by_key(|m| m.served.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ModelFile, Requires, Serve};

    fn model(id: &str, runtime: Runtime) -> Model {
        Model {
            id: id.into(),
            app: "redrob".into(),
            name: id.into(),
            description: String::new(),
            license: String::new(),
            source: "https://x".into(),
            task: "chat".into(),
            runtime,
            requires: Requires::default(),
            serve: runtime.served().then(|| Serve { model: "m.gguf".into(), mmproj: None }),
            files: vec![ModelFile { name: "m.gguf".into(), url: "https://x/m.gguf".into(), sha256: "0".repeat(64), size: 1 }],
        }
    }

    #[test]
    fn round_trips_and_prefers_a_served_model() {
        let served = Served { url: "http://127.0.0.1:4097".into(), port: 4097, pid: 7 };
        let r = Registry::new(
            "cuda",
            vec![
                InstalledModel::from_model(&model("a", Runtime::Files), "/m/a".into(), None),
                InstalledModel::from_model(&model("b", Runtime::LlamaServer), "/m/b".into(), Some(served.clone())),
            ],
        );
        let back = Registry::parse(&r.to_json()).unwrap();
        assert_eq!(back, r);
        assert_eq!(back.for_task("chat").map(|m| m.id.as_str()), Some("b"));
        assert_eq!(back.for_task("tts"), None);
        assert_eq!(back.models[1].served, Some(served));
        // A plain-files entry has no `served` key at all, so an app's "is it served" check is a key test.
        assert!(!r.to_json().contains(r#""served": null"#));
        assert!(Registry::parse(r#"{"version":9,"backend":"cpu","models":[]}"#).is_err());
    }
}
