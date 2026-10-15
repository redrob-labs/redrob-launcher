//! The runtime binaries that serve models (`llama-server`, stock and PrismML's fork), from
//! `assets/models/runtimes.json`: one release per runtime, one pinned asset per platform and
//! backend. Pure data; the desktop crate downloads and unpacks.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::models::{Backend, Runtime};
use crate::platform::{Arch, Os};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct RuntimeAsset {
    pub url: String,
    pub sha256: String,
    pub size: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct RuntimeRelease {
    pub release: String,
    pub tag: String,
    /// The executable's name inside the archive, without `.exe`.
    pub binary: String,
    /// Keyed `<os>-<arch>[-<backend>]`: `macos-arm64`, `linux-x64-cuda`, `windows-x64-vulkan`.
    #[serde(default)]
    pub assets: BTreeMap<String, RuntimeAsset>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Deserialize)]
struct Manifest {
    version: u32,
    runtimes: BTreeMap<String, RuntimeRelease>,
}

pub const MANIFEST: &str = include_str!("../../../assets/models/runtimes.json");

pub fn parse(json: &str) -> Result<BTreeMap<String, RuntimeRelease>, String> {
    let m: Manifest = serde_json::from_str(json).map_err(|e| format!("unreadable runtimes manifest ({e})"))?;
    if m.version != 1 {
        return Err(format!("runtimes manifest version {} isn't supported", m.version));
    }
    let hex64 = |s: &str| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit());
    let mut out = m.runtimes;
    for r in out.values_mut() {
        r.assets.retain(|_, a| a.url.starts_with("https://") && hex64(&a.sha256) && a.size > 0);
    }
    Ok(out)
}

pub fn builtin() -> BTreeMap<String, RuntimeRelease> {
    parse(MANIFEST).unwrap_or_default()
}

fn os_key(os: Os) -> Option<&'static str> {
    match os {
        Os::MacOs => Some("macos"),
        Os::Linux => Some("linux"),
        Os::Windows => Some("windows"),
        Os::FreeBsd => None,
    }
}

fn arch_key(arch: Arch) -> Option<&'static str> {
    match arch {
        Arch::X64 => Some("x64"),
        Arch::Arm64 => Some("arm64"),
        Arch::X86 | Arch::Riscv64 => None,
    }
}

/// The asset keys to try for a platform, best backend first. On macOS one build does Metal and
/// CPU; elsewhere a CUDA build is first, then Vulkan, then CPU, so a model that needs the CPU
/// path still gets a server when the GPU build is missing.
pub fn asset_keys(os: Os, arch: Arch, backend: Backend) -> Vec<String> {
    let (Some(os), Some(arch)) = (os_key(os), arch_key(arch)) else { return Vec::new() };
    if os == "macos" {
        return vec![format!("{os}-{arch}")];
    }
    let order: &[&str] = match backend {
        Backend::Cuda => &["cuda", "vulkan", "cpu"],
        Backend::Vulkan | Backend::Metal => &["vulkan", "cpu"],
        Backend::Cpu => &["cpu", "vulkan"],
    };
    order.iter().map(|b| format!("{os}-{arch}-{b}")).collect()
}

impl RuntimeRelease {
    /// The asset for this platform, and the key it was found under.
    pub fn asset_for(&self, os: Os, arch: Arch, backend: Backend) -> Option<(&str, &RuntimeAsset)> {
        asset_keys(os, arch, backend).into_iter().find_map(|k| self.assets.get_key_value(&k).map(|(k, a)| (k.as_str(), a)))
    }
}

/// The release for a runtime, from the built-in manifest.
pub fn release(runtime: Runtime) -> Option<RuntimeRelease> {
    builtin().remove(runtime.id())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_builtin_manifest_covers_the_three_desktop_platforms() {
        let r = builtin();
        let stock = r.get("llama-server").expect("stock llama-server");
        for key in ["macos-arm64", "linux-x64-cuda", "linux-x64-vulkan", "windows-x64-cuda", "windows-x64-vulkan"] {
            assert!(stock.assets.contains_key(key), "{key}");
        }
        assert!(stock.tag.starts_with('b'), "{}", stock.tag);
        let prism = r.get("llama-server-prism").expect("prism fork");
        assert!(prism.assets.contains_key("macos-arm64") && prism.assets.contains_key("windows-x64-cuda"));
    }

    #[test]
    fn asset_choice_falls_back_from_cuda_to_vulkan_to_cpu() {
        let r = builtin();
        let stock = &r["llama-server"];
        assert_eq!(stock.asset_for(Os::Linux, Arch::X64, Backend::Cuda).map(|(k, _)| k), Some("linux-x64-cuda"));
        assert_eq!(stock.asset_for(Os::Linux, Arch::X64, Backend::Cpu).map(|(k, _)| k), Some("linux-x64-vulkan"), "no cpu build: vulkan runs on cpu-less boxes too");
        assert_eq!(stock.asset_for(Os::MacOs, Arch::Arm64, Backend::Metal).map(|(k, _)| k), Some("macos-arm64"));
        assert_eq!(stock.asset_for(Os::MacOs, Arch::X64, Backend::Cpu), None);
        assert_eq!(stock.asset_for(Os::FreeBsd, Arch::X64, Backend::Cpu), None);
        // The fork ships CPU builds where it has no Vulkan.
        let prism = &r["llama-server-prism"];
        assert_eq!(prism.asset_for(Os::Windows, Arch::X64, Backend::Vulkan).map(|(k, _)| k), Some("windows-x64-cpu"));
    }
}
