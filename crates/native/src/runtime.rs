//! Serving models on this computer: installing `llama-server` (stock, or PrismML's fork for
//! ternary packs) and running one process per served model on loopback.
//!
//! The runtime binaries come from `assets/models/runtimes.json`, pinned by tag and SHA-256, and
//! are unpacked under `~/.redrob/runtimes/<runtime>/<tag>/`. A served model is a child process of
//! the launcher: `llama-server -m <model> --port <n> --host 127.0.0.1`, checked against `/health`
//! until it answers, then published in the installed registry and as a provider in Redrob Code's
//! config so the engine routes to it instead of the Console. Stopping kills the child and
//! withdraws both entries. Nothing here is reachable from another machine.

use std::path::{Path, PathBuf};
use std::process::{Child, Stdio};
use std::time::{Duration, Instant};

use redrob_launcher_core::device::Device;
use redrob_launcher_core::models::{Model, Runtime};
use redrob_launcher_core::runtimes::{self, RuntimeAsset, RuntimeRelease};

use crate::http::{Http, Stop};

/// Where runtimes are unpacked (`REDROB_RUNTIMES_DIR` overrides).
pub fn runtimes_dir() -> Option<PathBuf> {
    if let Some(d) = std::env::var_os("REDROB_RUNTIMES_DIR").filter(|v| !v.is_empty()) {
        return Some(PathBuf::from(d));
    }
    dirs::home_dir().map(|h| h.join(".redrob").join("runtimes"))
}

fn binary_name(release: &RuntimeRelease) -> String {
    if cfg!(windows) { format!("{}.exe", release.binary) } else { release.binary.clone() }
}

/// The folder a runtime's release is unpacked into.
pub fn release_dir(root: &Path, runtime: Runtime, release: &RuntimeRelease) -> PathBuf {
    root.join(runtime.id()).join(&release.tag)
}

/// The runtime's executable if it is unpacked here (searched one level deep: the archives put
/// the binaries either at the top or in one folder).
pub fn installed_binary(dir: &Path, release: &RuntimeRelease) -> Option<PathBuf> {
    let name = binary_name(release);
    let direct = dir.join(&name);
    if direct.is_file() {
        return Some(direct);
    }
    for entry in std::fs::read_dir(dir).ok()?.flatten() {
        let p = entry.path();
        if p.is_dir() {
            let candidate = p.join(&name);
            if candidate.is_file() {
                return Some(candidate);
            }
            let in_bin = p.join("bin").join(&name);
            if in_bin.is_file() {
                return Some(in_bin);
            }
        }
    }
    let in_bin = dir.join("bin").join(&name);
    in_bin.is_file().then_some(in_bin)
}

/// The asset this computer needs for a runtime, from the built-in manifest.
pub fn asset_for(device: &Device, runtime: Runtime) -> Option<(RuntimeRelease, String, RuntimeAsset)> {
    let release = runtimes::release(runtime)?;
    let (key, asset) = release.asset_for(device.os, device.arch, device.backend())?;
    let (key, asset) = (key.to_string(), asset.clone());
    Some((release, key, asset))
}

#[cfg(unix)]
fn unpack(archive: &Path, into: &Path) -> Result<(), String> {
    let gz = std::fs::File::open(archive).map_err(|e| format!("couldn't open {} ({e})", archive.display()))?;
    let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(gz));
    tar.set_preserve_permissions(true);
    // `unpack` refuses entries that would land outside `into` (`..`, absolute paths).
    tar.unpack(into).map_err(|e| format!("couldn't unpack the runtime ({e})"))
}

#[cfg(windows)]
fn unpack(archive: &Path, into: &Path) -> Result<(), String> {
    let ps = |p: &Path| format!("'{}'", p.display().to_string().replace('\'', "''"));
    let script = format!("Expand-Archive -LiteralPath {} -DestinationPath {} -Force", ps(archive), ps(into));
    crate::process::run("powershell", &["-NoProfile", "-NonInteractive", "-Command", &script], Duration::from_secs(600)).map(|_| ())
}

/// Downloads and unpacks a runtime's release for this computer. Returns the executable.
pub fn install(http: &Http, root: &Path, device: &Device, runtime: Runtime, stop: &Stop, progress: &mut dyn FnMut(u64, u64)) -> Result<PathBuf, String> {
    let (release, key, asset) = asset_for(device, runtime).ok_or_else(|| format!("no {} build for this computer", runtime.id()))?;
    let dir = release_dir(root, runtime, &release);
    if let Some(bin) = installed_binary(&dir, &release) {
        return Ok(bin);
    }
    std::fs::create_dir_all(&dir).map_err(|e| format!("couldn't create {} ({e})", dir.display()))?;
    let file_name = asset.url.rsplit('/').next().unwrap_or(&key).to_string();
    let archive = root.join(runtime.id()).join(&file_name);
    http.download(&asset.url, &archive, asset.size, Some(&asset.sha256), stop, &mut |t| {
        let (crate::http::Transfer::Bytes { done, .. } | crate::http::Transfer::Reconnecting { done, .. }) = t;
        progress(done.min(asset.size), asset.size);
    })?;
    unpack(&archive, &dir)?;
    let _ = std::fs::remove_file(&archive);
    let bin = installed_binary(&dir, &release).ok_or_else(|| format!("the {} archive doesn't contain {}", runtime.id(), release.binary))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(md) = std::fs::metadata(&bin) {
            let mut perm = md.permissions();
            perm.set_mode(perm.mode() | 0o755);
            let _ = std::fs::set_permissions(&bin, perm);
        }
    }
    Ok(bin)
}

/// The command line for one served model. Public so the test and the UI's "how is it run"
/// hover agree with what actually runs.
pub fn args(model: &Model, dir: &Path, port: u16) -> Vec<String> {
    let serve = model.serve.as_ref();
    let mut a: Vec<String> = vec![
        "-m".into(),
        dir.join(serve.map_or("", |s| s.model.as_str())).display().to_string(),
        "--host".into(),
        "127.0.0.1".into(),
        "--port".into(),
        port.to_string(),
        "--alias".into(),
        model.id.clone(),
        // Every layer on the accelerator; llama.cpp spills to RAM on its own when it must.
        "-ngl".into(),
        "999".into(),
        "--flash-attn".into(),
        "on".into(),
    ];
    if let Some(mm) = serve.and_then(|s| s.mmproj.as_deref()) {
        a.push("--mmproj".into());
        a.push(dir.join(mm).display().to_string());
    }
    if model.task == "embedding" {
        a.push("--embedding".into());
    } else {
        a.push("--jinja".into());
    }
    a
}

/// A running server.
pub struct Server {
    pub child: Child,
    pub port: u16,
}

impl Server {
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    pub fn pid(&self) -> u32 {
        self.child.id()
    }

    /// Starts `llama-server` and waits until `/health` answers 200, or the process dies, or
    /// `wait` passes (a 27B model on a laptop can take a minute to map in).
    pub fn start(bin: &Path, model: &Model, dir: &Path, port: u16, http: &Http, wait: Duration) -> Result<Self, String> {
        let log = dir.join("llama-server.log");
        let out = std::fs::File::create(&log).ok();
        let mut cmd = crate::process::command(bin);
        cmd.args(args(model, dir, port)).stdin(Stdio::null());
        match out {
            Some(f) => {
                let err = f.try_clone().map(Stdio::from).unwrap_or_else(|_| Stdio::null());
                cmd.stdout(Stdio::from(f)).stderr(err);
            }
            None => {
                cmd.stdout(Stdio::null()).stderr(Stdio::null());
            }
        }
        if let Some(parent) = bin.parent() {
            // The archives ship their shared libraries beside the binary.
            cmd.current_dir(parent);
            #[cfg(target_os = "linux")]
            cmd.env("LD_LIBRARY_PATH", parent);
            #[cfg(target_os = "macos")]
            cmd.env("DYLD_LIBRARY_PATH", parent);
        }
        let child = cmd.spawn().map_err(|e| format!("couldn't start {} ({e})", bin.display()))?;
        let mut server = Server { child, port };
        let started = Instant::now();
        loop {
            if let Ok(Some(status)) = server.child.try_wait() {
                let tail = std::fs::read_to_string(&log).ok().map(|s| s.lines().rev().take(5).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n")).unwrap_or_default();
                return Err(format!("llama-server exited ({status}). {tail}").trim().to_string());
            }
            if http.get_text(&format!("{}/health", server.url())).is_ok_and(|t| t.is_some_and(|t| t.contains("ok"))) {
                return Ok(server);
            }
            if started.elapsed() > wait {
                server.stop();
                return Err("llama-server didn't answer in time".into());
            }
            std::thread::sleep(Duration::from_millis(400));
        }
    }

    pub fn stop(mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// A free loopback port at or above `from`, so several models can be served at once.
pub fn free_port(from: u16, taken: &[u16]) -> Option<u16> {
    (from..from.saturating_add(200)).find(|p| !taken.contains(p) && std::net::TcpListener::bind(("127.0.0.1", *p)).is_ok())
}

/// Redrob Code's user config file (`$XDG_CONFIG_HOME/redrob/redrob.json`, `REDROB_CONFIG_DIR`
/// overrides the folder): where the engine learns about a local provider.
pub fn code_config_path() -> Option<PathBuf> {
    let dir = std::env::var_os("REDROB_CONFIG_DIR")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()).map(|c| PathBuf::from(c).join("redrob")))
        .or_else(|| dirs::home_dir().map(|h| h.join(".config").join("redrob")))?;
    Some(dir.join("redrob.json"))
}

/// Provider id for a served model, as Redrob Code sees it (`redrob-local-qwen3.8-27b-q4`).
pub fn provider_id(model_id: &str) -> String {
    format!("redrob-local-{model_id}")
}

/// Adds or removes the provider entry for a served model in Redrob Code's config. The engine
/// only accepts a local provider built on `@ai-sdk/openai-compatible` at a loopback URL, which is
/// exactly what is written; everything else in the file is left as it was.
pub fn set_code_provider(path: &Path, model: &Model, url: Option<&str>) -> Result<(), String> {
    let mut root: serde_json::Value = match std::fs::read_to_string(path) {
        Ok(s) if !s.trim().is_empty() => serde_json::from_str(&s).map_err(|e| format!("{} isn't valid JSON ({e}); not touching it", path.display()))?,
        _ => serde_json::json!({ "$schema": "https://code.redrob.ai/config.json" }),
    };
    let obj = root.as_object_mut().ok_or("config isn't an object")?;
    let providers = obj.entry("provider").or_insert_with(|| serde_json::json!({}));
    let providers = providers.as_object_mut().ok_or("config.provider isn't an object")?;
    let id = provider_id(&model.id);
    match url {
        Some(url) => {
            providers.insert(
                id,
                serde_json::json!({
                    "name": format!("{} (local)", model.name),
                    "api": { "type": "aisdk", "package": "@ai-sdk/openai-compatible", "url": format!("{url}/v1") },
                    "models": { model.id.clone(): { "name": model.name.clone() } }
                }),
            );
        }
        None => {
            providers.remove(&id);
        }
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("couldn't create {} ({e})", dir.display()))?;
    }
    let text = serde_json::to_string_pretty(&root).map_err(|e| e.to_string())?;
    crate::paths::write_atomic(path, text.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use redrob_launcher_core::models::{ModelFile, Requires, Serve};

    fn model(task: &str, mmproj: bool) -> Model {
        Model {
            id: "m-1".into(),
            app: "redrob".into(),
            name: "M".into(),
            description: String::new(),
            license: String::new(),
            source: "https://x".into(),
            task: task.into(),
            runtime: Runtime::LlamaServer,
            requires: Requires::default(),
            serve: Some(Serve { model: "m.gguf".into(), mmproj: mmproj.then(|| "mm.gguf".into()) }),
            files: vec![ModelFile { name: "m.gguf".into(), url: "https://x/m".into(), sha256: "0".repeat(64), size: 1 }],
        }
    }

    #[test]
    fn the_command_line_names_model_projector_and_port() {
        let a = args(&model("vision", true), Path::new("/m"), 4101);
        let s = a.join(" ");
        assert!(s.contains("-m /m/m.gguf") && s.contains("--mmproj /m/mm.gguf") && s.contains("--port 4101") && s.contains("--host 127.0.0.1"));
        assert!(s.contains("--jinja") && !s.contains("--embedding"));
        let e = args(&model("embedding", false), Path::new("/m"), 4102).join(" ");
        assert!(e.contains("--embedding") && !e.contains("--mmproj"));
    }

    #[test]
    fn code_provider_is_added_and_removed_without_touching_the_rest() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("redrob.json");
        std::fs::write(&path, r#"{"theme":"dark","provider":{"other":{"api":{"type":"aisdk"}}}}"#).unwrap();
        set_code_provider(&path, &model("chat", false), Some("http://127.0.0.1:4101")).unwrap();
        let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(v["theme"], "dark");
        assert!(v["provider"]["other"].is_object());
        assert_eq!(v["provider"]["redrob-local-m-1"]["api"]["url"], "http://127.0.0.1:4101/v1");
        assert_eq!(v["provider"]["redrob-local-m-1"]["api"]["package"], "@ai-sdk/openai-compatible");
        set_code_provider(&path, &model("chat", false), None).unwrap();
        let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert!(v["provider"].get("redrob-local-m-1").is_none());
        assert!(v["provider"]["other"].is_object());
        // A fresh file gets the schema line Redrob Code writes itself.
        let fresh = dir.path().join("new").join("redrob.json");
        set_code_provider(&fresh, &model("chat", false), Some("http://127.0.0.1:4101")).unwrap();
        assert!(std::fs::read_to_string(&fresh).unwrap().contains("code.redrob.ai/config.json"));
        // Broken JSON is left alone rather than overwritten.
        std::fs::write(&path, "{ not json").unwrap();
        assert!(set_code_provider(&path, &model("chat", false), Some("http://127.0.0.1:1")).is_err());
    }

    #[test]
    fn a_free_port_is_found_above_the_floor_and_skips_taken_ones() {
        let p = free_port(4100, &[4100, 4101]).unwrap();
        assert!(p >= 4102);
    }

    #[test]
    fn the_installed_binary_is_found_at_the_top_or_one_folder_down() {
        let dir = tempfile::tempdir().unwrap();
        let release = RuntimeRelease { release: String::new(), tag: "b1".into(), binary: "llama-server".into(), assets: Default::default(), note: None };
        assert!(installed_binary(dir.path(), &release).is_none());
        let nested = dir.path().join("build").join("bin");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(nested.join(binary_name(&release)), b"x").unwrap();
        assert!(installed_binary(dir.path(), &release).unwrap().ends_with(binary_name(&release)));
    }
}
