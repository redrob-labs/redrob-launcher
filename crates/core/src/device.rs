//! What this computer is, and whether a model runs on it.
//!
//! The measuring is the desktop crate's job (`nvidia-smi`, `sysctl`, WMI); this is the record it
//! fills in and the one judgement everything else reads: [`Device::fit`]. A value that was not
//! measured is `None`, never a guess. A machine with a card we could not see is told "not
//! measured", not "no GPU", because the second is a false statement about the person's own
//! computer.

use serde::{Deserialize, Serialize};

use crate::models::{Backend, Requires};
use crate::platform::{Arch, Os};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GpuVendor {
    Nvidia,
    Apple,
    Amd,
    Intel,
    Other,
}

/// One accelerator. On Apple silicon `memory_mb` is the unified memory, shared with the CPU.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gpu {
    pub name: String,
    pub vendor: GpuVendor,
    /// Dedicated (or unified) memory in MiB; `None` when the probe could not read it.
    pub memory_mb: Option<u64>,
    pub unified: bool,
}

/// A neural accelerator the OS reports (Intel AI Boost, Qualcomm Hexagon, Apple Neural Engine).
/// Shown to the person; nothing in this release runs on it yet.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Npu {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
    pub os: Os,
    pub arch: Arch,
    pub cpu: String,
    pub cores: u32,
    pub ram_mb: u64,
    /// The accelerator the backend would use, if one was found.
    pub gpu: Option<Gpu>,
    pub npu: Option<Npu>,
    /// Whether the GPU probe ran at all. `false` with `gpu == None` means "not measured".
    pub gpu_probed: bool,
}

/// The one verdict the UI shows next to a model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Fit {
    /// Fits in the backend's memory.
    Runs,
    /// Runs, but only on the CPU or by spilling into RAM; expect it to be slow.
    Slow,
    WontRun,
    /// The computer's memory could not be measured, so no verdict.
    Unknown,
}

impl Device {
    /// The backend `llama-server` and the apps should use on this computer.
    pub fn backend(&self) -> Backend {
        match self.gpu.as_ref().map(|g| g.vendor) {
            Some(GpuVendor::Nvidia) => Backend::Cuda,
            Some(GpuVendor::Apple) => Backend::Metal,
            Some(GpuVendor::Amd | GpuVendor::Intel | GpuVendor::Other) => Backend::Vulkan,
            None => Backend::Cpu,
        }
    }

    /// Memory the accelerator backend can use, in MiB. On Apple silicon that is the unified
    /// memory; on the CPU it is RAM.
    pub fn accel_memory_mb(&self) -> Option<u64> {
        match &self.gpu {
            Some(g) if g.unified => Some(g.memory_mb.unwrap_or(self.ram_mb)),
            Some(g) => g.memory_mb,
            None => Some(self.ram_mb),
        }
    }

    /// The largest `memory_gb` a model may ask for and still get [`Fit::Runs`], rounded down.
    pub fn runs_up_to_gb(&self) -> Option<u64> {
        self.accel_memory_mb().map(|mb| mb / 1024)
    }

    pub fn fit(&self, r: &Requires) -> Fit {
        let need = u64::from(r.memory_mb);
        let backend = self.backend();
        let on_backend = r.backends.contains(&backend);
        let on_cpu = r.backends.contains(&Backend::Cpu);
        if !on_backend && !on_cpu {
            return Fit::WontRun;
        }
        if !on_backend {
            // Only the CPU is left for it.
            return if self.ram_mb >= need { Fit::Slow } else { Fit::WontRun };
        }
        match self.accel_memory_mb() {
            None => Fit::Unknown,
            Some(mem) if mem >= need => {
                if backend == Backend::Cpu {
                    Fit::Slow
                } else {
                    Fit::Runs
                }
            }
            // Too big for the card: llama.cpp spills layers into RAM when the model allows CPU.
            Some(_) if on_cpu && self.ram_mb >= need => Fit::Slow,
            Some(_) => Fit::WontRun,
        }
    }

    /// A one-line summary for the device card, in English (the UI translates the pattern).
    pub fn gpu_label(&self) -> String {
        match &self.gpu {
            Some(g) => match g.memory_mb {
                Some(mb) => format!("{} ({} GB)", g.name, mb.div_ceil(1024)),
                None => g.name.clone(),
            },
            None if self.gpu_probed => "No GPU found".to_string(),
            None => "GPU not measured".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req(gb: u32, backends: &[Backend]) -> Requires {
        Requires { memory_mb: gb * 1024, backends: backends.to_vec() }
    }
    const ALL: [Backend; 4] = [Backend::Cuda, Backend::Metal, Backend::Vulkan, Backend::Cpu];

    fn pc(gpu: Option<Gpu>) -> Device {
        Device { os: Os::Linux, arch: Arch::X64, cpu: "x".into(), cores: 8, ram_mb: 32 * 1024, gpu, gpu_probed: true, npu: None }
    }

    #[test]
    fn nvidia_card_fits_by_vram_and_spills_to_ram() {
        let d = pc(Some(Gpu { name: "RTX 4070".into(), vendor: GpuVendor::Nvidia, memory_mb: Some(12 * 1024), unified: false }));
        assert_eq!(d.backend(), Backend::Cuda);
        assert_eq!(d.fit(&req(10, &ALL)), Fit::Runs);
        assert_eq!(d.fit(&req(19, &ALL)), Fit::Slow, "spills into 32 GB RAM");
        assert_eq!(d.fit(&req(19, &[Backend::Cuda, Backend::Metal])), Fit::WontRun, "no CPU path");
        assert_eq!(d.fit(&req(40, &ALL)), Fit::WontRun);
        assert_eq!(d.runs_up_to_gb(), Some(12));
    }

    #[test]
    fn apple_silicon_uses_unified_memory() {
        let mut d = pc(Some(Gpu { name: "Apple M3".into(), vendor: GpuVendor::Apple, memory_mb: None, unified: true }));
        d.os = Os::MacOs;
        d.arch = Arch::Arm64;
        assert_eq!(d.backend(), Backend::Metal);
        assert_eq!(d.fit(&req(19, &ALL)), Fit::Runs, "32 GB unified");
        assert_eq!(d.fit(&req(14, &[Backend::Cuda, Backend::Metal])), Fit::Runs);
        assert_eq!(d.fit(&req(14, &[Backend::Cuda])), Fit::WontRun);
    }

    #[test]
    fn no_gpu_means_cpu_and_slow() {
        let d = pc(None);
        assert_eq!(d.backend(), Backend::Cpu);
        assert_eq!(d.fit(&req(2, &ALL)), Fit::Slow);
        assert_eq!(d.fit(&req(2, &[Backend::Cuda, Backend::Metal])), Fit::WontRun);
        assert_eq!(d.gpu_label(), "No GPU found");
    }

    #[test]
    fn an_unmeasured_card_gives_no_verdict() {
        let d = pc(Some(Gpu { name: "Some GPU".into(), vendor: GpuVendor::Amd, memory_mb: None, unified: false }));
        assert_eq!(d.backend(), Backend::Vulkan);
        assert_eq!(d.fit(&req(10, &ALL)), Fit::Unknown);
        let mut unprobed = pc(None);
        unprobed.gpu_probed = false;
        assert_eq!(unprobed.gpu_label(), "GPU not measured");
    }
}
