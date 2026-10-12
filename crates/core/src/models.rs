//! Models the launcher downloads for the apps, and what the UI is told about them.
//!
//! The list is data (`assets/models/models.json`), generated from each app's own pinned
//! catalogue, so a model the launcher fetches is exactly the file the app would have fetched: same
//! URL, same SHA-256, same size, in the folder the app reads it from. Every field is checked when
//! the manifest is read; a bad entry is dropped, never half-trusted.

use serde::Deserialize;

/// One file of a model.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ModelFile {
    /// File name inside the model's folder.
    pub name: String,
    pub url: String,
    pub sha256: String,
    pub size: u64,
}

/// A downloadable model for one app.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Model {
    /// Folder name under the app's models folder (`whisper-base`).
    pub id: String,
    /// The app that uses it (`redrob-cut`).
    pub app: String,
    pub name: String,
    pub description: String,
    pub license: String,
    /// The model's page, for its card and licence.
    pub source: String,
    pub files: Vec<ModelFile>,
}

impl Model {
    pub fn size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
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
    plain_name(&m.id)
        && crate::catalog::CraftApp::by_id(&m.app).is_some()
        && !m.name.is_empty()
        && m.source.starts_with("https://")
        && !m.files.is_empty()
        && m.files
            .iter()
            .all(|f| plain_name(&f.name) && f.url.starts_with("https://") && !f.url.contains(['\\', '\n']) && hex64(&f.sha256) && f.size > 0)
}

/// Reads a manifest, keeping only well-formed entries for known apps (ids unique).
pub fn parse(json: &str) -> Result<Vec<Model>, String> {
    let m: Manifest = serde_json::from_str(json).map_err(|e| format!("unreadable model manifest ({e})"))?;
    if m.version != 1 {
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

/// A model as the UI shows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelView {
    pub model: Model,
    pub state: ModelState,
    /// The folder it goes into, for the person to see (`~/.local/share/redrob-cut/models/whisper-base`).
    pub folder: Option<String>,
}

/// The desktop backend's model downloads. Every call returns at once.
pub trait ModelBackend {
    fn models(&self) -> Vec<ModelView>;
    /// Download (or resume) every missing file of a model.
    fn download(&mut self, app: &str, id: &str);
    /// Pause a running download, keeping what's downloaded.
    fn pause(&mut self, app: &str, id: &str);
    /// Delete a model's files (and any partial ones) from the app's folder.
    fn delete(&mut self, app: &str, id: &str) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_builtin_manifest_is_complete_and_pinned() {
        let models = builtin();
        let ids: Vec<&str> = models.iter().map(|m| m.id.as_str()).collect();
        assert_eq!(ids, ["whisper-tiny", "whisper-base", "whisper-small", "kokoro-82m"]);
        for m in &models {
            assert_eq!(m.app, "redrob-cut");
            for f in &m.files {
                // Pinned to a revision, never to a moving branch.
                assert!(!f.url.contains("/resolve/main/") && !f.url.contains("/master/"), "{}", f.url);
            }
        }
        // The narration package is 336 MB; Whisper base is the default speech model.
        assert_eq!(models[3].size(), 335_543_900);
        assert_eq!(models[1].files.len(), 4);
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
        assert!(parse(r#"{"version":2,"models":[]}"#).is_err());
        assert!(parse("<html>").is_err());
    }
}
