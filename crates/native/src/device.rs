//! Measuring the computer: CPU, RAM, GPU and NPU, through the OS's own tools, so the launcher
//! can say which models run here. Every probe is best-effort and short (a few seconds at most);
//! what it cannot read stays `None` and the UI says "not measured".

use std::time::Duration;

use redrob_launcher_core::device::{Device, Gpu, GpuVendor, Npu};
use redrob_launcher_core::platform::{Arch, Os};

use crate::process::capture;

const PROBE: Duration = Duration::from_secs(4);

fn os() -> Os {
    if cfg!(target_os = "macos") {
        Os::MacOs
    } else if cfg!(windows) {
        Os::Windows
    } else {
        Os::Linux
    }
}

fn arch() -> Arch {
    if cfg!(target_arch = "aarch64") {
        Arch::Arm64
    } else if cfg!(target_arch = "x86") {
        Arch::X86
    } else if cfg!(target_arch = "riscv64") {
        Arch::Riscv64
    } else {
        Arch::X64
    }
}

/// Picks the vendor from a GPU's name as the OS reports it.
pub fn vendor_of(name: &str) -> GpuVendor {
    let n = name.to_ascii_lowercase();
    if n.contains("nvidia") || n.contains("geforce") || n.contains("rtx") || n.contains("quadro") || n.contains("tesla") {
        GpuVendor::Nvidia
    } else if n.contains("apple") {
        GpuVendor::Apple
    } else if n.contains("amd") || n.contains("radeon") || n.contains("advanced micro") {
        GpuVendor::Amd
    } else if n.contains("intel") || n.contains("arc ") || n.contains("iris") || n.contains("uhd graphics") {
        GpuVendor::Intel
    } else {
        GpuVendor::Other
    }
}

/// `nvidia-smi --query-gpu=name,memory.total --format=csv,noheader,nounits` -> the first card.
pub fn parse_nvidia_smi(out: &str) -> Option<Gpu> {
    let line = out.lines().map(str::trim).find(|l| !l.is_empty())?;
    let (name, mem) = line.rsplit_once(',')?;
    let memory_mb = mem.trim().parse::<u64>().ok();
    Some(Gpu { name: name.trim().to_string(), vendor: GpuVendor::Nvidia, memory_mb, unified: false })
}

/// Windows `Get-CimInstance Win32_VideoController | Select Name,AdapterRAM | ConvertTo-Json`.
/// Picks the first discrete card (NVIDIA, then AMD, then anything), because the integrated one
/// is usually also listed. `AdapterRAM` is a 32-bit field and lies above 4 GB, so it is kept only
/// as a floor and the UI treats a 4095 MB reading as "not measured".
pub fn parse_win32_video(json: &str) -> Option<Gpu> {
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    let list: Vec<serde_json::Value> = match v {
        serde_json::Value::Array(a) => a,
        other => vec![other],
    };
    let mut cards: Vec<Gpu> = list
        .iter()
        .filter_map(|c| {
            let name = c.get("Name")?.as_str()?.trim().to_string();
            let ram = c.get("AdapterRAM").and_then(serde_json::Value::as_u64).map(|b| b / (1024 * 1024));
            // The field saturates at 4 GB; anything at or over that is unknown, not 4 GB.
            let memory_mb = ram.filter(|mb| *mb < 4095);
            Some(Gpu { vendor: vendor_of(&name), name, memory_mb, unified: false })
        })
        .collect();
    let rank = |g: &Gpu| match g.vendor {
        GpuVendor::Nvidia => 0,
        GpuVendor::Amd => 1,
        GpuVendor::Intel => 3,
        _ => 2,
    };
    cards.sort_by_key(rank);
    cards.into_iter().next()
}

/// Linux: the first VGA/3D controller line of `lspci`.
pub fn parse_lspci(out: &str) -> Option<Gpu> {
    let line = out.lines().find(|l| l.contains("VGA compatible controller") || l.contains("3D controller") || l.contains("Display controller"))?;
    let name = line.split_once(": ").map_or(line, |(_, n)| n).trim().to_string();
    Some(Gpu { vendor: vendor_of(&name), name, memory_mb: None, unified: false })
}

fn ram_mb() -> u64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(s) = std::fs::read_to_string("/proc/meminfo")
            && let Some(kb) =
                s.lines().find_map(|l| l.strip_prefix("MemTotal:")).and_then(|v| v.trim().trim_end_matches("kB").trim().parse::<u64>().ok())
        {
            return kb / 1024;
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(b) = capture("sysctl", &["-n", "hw.memsize"], PROBE).and_then(|s| s.trim().parse::<u64>().ok()) {
            return b / (1024 * 1024);
        }
    }
    #[cfg(windows)]
    {
        if let Some(b) = capture("powershell", &["-NoProfile", "-Command", "(Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory"], PROBE)
            .and_then(|s| s.trim().parse::<u64>().ok())
        {
            return b / (1024 * 1024);
        }
    }
    0
}

fn cpu_name() -> String {
    #[cfg(target_os = "linux")]
    {
        if let Ok(s) = std::fs::read_to_string("/proc/cpuinfo")
            && let Some(n) = s.lines().find_map(|l| l.strip_prefix("model name")).and_then(|l| l.split_once(':')).map(|(_, v)| v.trim().to_string())
        {
            return n;
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(n) = capture("sysctl", &["-n", "machdep.cpu.brand_string"], PROBE) {
            return n.trim().to_string();
        }
    }
    #[cfg(windows)]
    {
        if let Some(n) = capture("powershell", &["-NoProfile", "-Command", "(Get-CimInstance Win32_Processor).Name"], PROBE) {
            return n.trim().to_string();
        }
    }
    String::new()
}

/// The GPU, and whether a probe ran at all.
fn gpu() -> (Option<Gpu>, bool) {
    if cfg!(target_os = "macos") {
        // Apple silicon: the GPU is the chip, and its memory is the unified memory.
        if cfg!(target_arch = "aarch64") {
            let chip =
                capture("sysctl", &["-n", "machdep.cpu.brand_string"], PROBE).map_or_else(|| "Apple silicon".to_string(), |s| s.trim().to_string());
            return (Some(Gpu { name: chip, vendor: GpuVendor::Apple, memory_mb: None, unified: true }), true);
        }
        return (None, true);
    }
    if let Some(g) =
        capture("nvidia-smi", &["--query-gpu=name,memory.total", "--format=csv,noheader,nounits"], PROBE).and_then(|o| parse_nvidia_smi(&o))
    {
        return (Some(g), true);
    }
    if cfg!(windows) {
        let out = capture(
            "powershell",
            &["-NoProfile", "-Command", "Get-CimInstance Win32_VideoController | Select-Object Name,AdapterRAM | ConvertTo-Json -Compress"],
            PROBE,
        );
        return match out {
            Some(o) => (parse_win32_video(&o), true),
            None => (None, false),
        };
    }
    match capture("lspci", &[], PROBE) {
        Some(o) => (parse_lspci(&o), true),
        None => (None, false),
    }
}

/// An NPU the OS names, if any. Best-effort: only what a device list says in so many words.
fn npu() -> Option<Npu> {
    let text = if cfg!(windows) {
        capture(
            "powershell",
            &[
                "-NoProfile",
                "-Command",
                "(Get-PnpDevice -PresentOnly | Where-Object { $_.FriendlyName -match 'NPU|Neural|AI Boost|Hexagon' }).FriendlyName",
            ],
            PROBE,
        )
    } else if cfg!(target_os = "linux") {
        capture("lspci", &[], PROBE)
            .map(|o| o.lines().filter(|l| l.contains("NPU") || l.contains("Neural") || l.contains("AI Boost")).collect::<Vec<_>>().join("\n"))
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        Some("Apple Neural Engine".to_string())
    } else {
        None
    }?;
    let name = text.lines().map(str::trim).find(|l| !l.is_empty())?;
    let name = name.split_once(": ").map_or(name, |(_, n)| n);
    Some(Npu { name: name.chars().take(80).collect() })
}

/// Measures this computer. Takes up to a few seconds; call it off the UI thread.
pub fn measure() -> Device {
    let (gpu, gpu_probed) = gpu();
    Device {
        os: os(),
        arch: arch(),
        cpu: cpu_name(),
        cores: std::thread::available_parallelism().map_or(0, |n| n.get() as u32),
        ram_mb: ram_mb(),
        gpu,
        npu: npu(),
        gpu_probed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nvidia_smi_output_is_read() {
        let g = parse_nvidia_smi("NVIDIA L40S, 46068\n").unwrap();
        assert_eq!((g.name.as_str(), g.vendor, g.memory_mb, g.unified), ("NVIDIA L40S", GpuVendor::Nvidia, Some(46068), false));
        let g = parse_nvidia_smi("NVIDIA GeForce RTX 4070 Ti SUPER, [N/A]").unwrap();
        assert_eq!(g.memory_mb, None);
        assert!(parse_nvidia_smi("").is_none());
    }

    #[test]
    fn windows_picks_the_discrete_card_and_distrusts_the_4gb_cap() {
        let json =
            r#"[{"Name":"Intel(R) UHD Graphics","AdapterRAM":1073741824},{"Name":"NVIDIA GeForce RTX 4060 Laptop GPU","AdapterRAM":4293918720}]"#;
        let g = parse_win32_video(json).unwrap();
        assert_eq!(g.vendor, GpuVendor::Nvidia);
        assert_eq!(g.memory_mb, None, "AdapterRAM saturates at 4 GB");
        let g = parse_win32_video(r#"{"Name":"AMD Radeon RX 6600","AdapterRAM":2147483648}"#).unwrap();
        assert_eq!((g.vendor, g.memory_mb), (GpuVendor::Amd, Some(2048)));
    }

    #[test]
    fn lspci_names_the_card_without_memory() {
        let g = parse_lspci("00:02.0 VGA compatible controller: Intel Corporation Alder Lake-P GT2 [Iris Xe Graphics] (rev 0c)\n").unwrap();
        assert_eq!(g.vendor, GpuVendor::Intel);
        assert_eq!(g.memory_mb, None);
        assert!(parse_lspci("00:1f.0 ISA bridge: Intel Corporation\n").is_none());
    }

    #[test]
    fn measure_runs_without_panicking() {
        let d = measure();
        assert!(d.cores > 0);
    }
}
