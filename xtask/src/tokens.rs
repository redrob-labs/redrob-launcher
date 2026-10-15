//! `cargo xtask tokens [--check] [--version X.Y.Z]`: the launcher's colours from the Redrob design
//! system package, `@redrob-labs/ui`.
//!
//! The design system ships its resolved colour tokens as `tokens.json` (one value per theme for
//! every token). This command downloads the newest published package (or `--version`), reads that
//! file and writes `crates/ui-egui/src/theme/generated.rs`: the launcher's colour ROLES
//! (`bg`, `ink`, `accent`, ...) bound to the design system's values, light and dark, plus the
//! package version they came from. The role -> token mapping is [`ROLES`] below: that is the
//! launcher's own decision; the values are the design system's. Nothing in the launcher spells a
//! colour by hand any more.
//!
//! `--check` regenerates to memory and fails when the committed file differs, so CI catches a
//! design-system release the launcher has not picked up (`cargo xtask tokens`, commit).
//!
//! Needs `curl` and `tar` (the npm registry serves plain tarballs; no npm needed).

use std::path::{Path, PathBuf};
use std::process::Command;

const PACKAGE: &str = "@redrob-labs/ui";
const REGISTRY: &str = "https://registry.npmjs.org";
const OUT: &str = "crates/ui-egui/src/theme/generated.rs";

/// The launcher's colour roles and the design-system token each one takes, per theme
/// (light, dark). A role bound to two tokens picks a different token per theme.
const ROLES: &[(&str, &str, &str, &str)] = &[
    // role, doc, light token, dark token
    ("bg", "Window and card background", "surface-base", "surface-base"),
    ("sunken", "Badges, wells and row hovers", "surface-sunken", "surface-sunken"),
    ("ink", "Body text", "ink-primary", "ink-primary"),
    ("ink_strong", "Headings", "redrob-black", "redrob-white"),
    ("muted", "Secondary text", "ink-secondary", "ink-secondary"),
    ("faint", "Labels and tertiary text", "ink-muted", "ink-muted"),
    ("line", "Hairlines", "border-subtle", "border-subtle"),
    ("line_strong", "Outlined buttons and stronger rules", "border-strong", "border-strong"),
    ("accent", "The brand blue, for surfaces", "action-primary", "action-primary"),
    ("accent_ink", "The brand blue, for text and icons", "ink-brand", "ink-brand"),
    ("invert_bg", "Primary buttons and hover inversion: the pill", "redrob-black", "redrob-white"),
    ("invert_fg", "Primary buttons and hover inversion: the text on the pill", "redrob-white", "redrob-black"),
    ("good", "Success", "status-success", "status-success"),
    ("warn", "Warning", "status-warning", "status-warning"),
    ("bad", "Danger", "status-danger", "status-danger"),
];

pub fn run(root: &Path, args: &[&str]) -> Result<(), String> {
    let mut check = false;
    let mut version: Option<String> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match *a {
            "--check" => check = true,
            "--version" => version = Some(it.next().ok_or("--version needs a value")?.to_string()),
            other => return Err(format!("unknown argument `{other}`")),
        }
    }
    let version = match version {
        Some(v) => v,
        None => latest_version()?,
    };
    let json = fetch_tokens_json(&version)?;
    let generated = generate(&version, &json)?;
    let out = root.join(OUT);
    if check {
        let current = std::fs::read_to_string(&out).map_err(|e| format!("{}: {e} (run `cargo xtask tokens`)", out.display()))?;
        if current != generated {
            return Err(format!("{} is out of date with {PACKAGE} {version}: run `cargo xtask tokens` and commit", out.display()));
        }
        println!("{} is current with {PACKAGE} {version}", out.display());
        return Ok(());
    }
    if let Some(dir) = out.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    std::fs::write(&out, generated).map_err(|e| format!("{}: {e}", out.display()))?;
    println!("wrote {} from {PACKAGE} {version}", out.display());
    Ok(())
}

fn curl(args: &[&str]) -> Result<Vec<u8>, String> {
    let output =
        Command::new("curl").args(["--fail", "--silent", "--show-error", "--location"]).args(args).output().map_err(|e| format!("curl: {e}"))?;
    if !output.status.success() {
        return Err(format!("curl {}: {}", args.join(" "), String::from_utf8_lossy(&output.stderr).trim()));
    }
    Ok(output.stdout)
}

/// The `latest` dist-tag of the package, from the registry.
fn latest_version() -> Result<String, String> {
    let body = curl(&[&format!("{REGISTRY}/{PACKAGE}/latest")])?;
    let v: serde_json::Value = serde_json::from_slice(&body).map_err(|e| format!("registry answer is not JSON: {e}"))?;
    v.get("version").and_then(|v| v.as_str()).map(str::to_string).ok_or_else(|| "registry answer has no version".to_string())
}

/// `tokens.json` out of the package tarball (`package/dist/styles/tokens.json`).
fn fetch_tokens_json(version: &str) -> Result<serde_json::Value, String> {
    let name = PACKAGE.rsplit('/').next().unwrap_or(PACKAGE);
    let url = format!("{REGISTRY}/{PACKAGE}/-/{name}-{version}.tgz");
    let tgz = curl(&[&url])?;
    let dir = std::env::temp_dir().join(format!("redrob-ui-tokens-{}-{version}", std::process::id()));
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let tarball = dir.join("ui.tgz");
    std::fs::write(&tarball, tgz).map_err(|e| e.to_string())?;
    let status = Command::new("tar").arg("-xzf").arg(&tarball).arg("-C").arg(&dir).status().map_err(|e| format!("tar: {e}"))?;
    if !status.success() {
        return Err("tar could not unpack the package".into());
    }
    let json_path = find_tokens_json(&dir).ok_or("the package has no tokens.json")?;
    let text = std::fs::read_to_string(&json_path).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_dir_all(&dir);
    serde_json::from_str(&text).map_err(|e| format!("tokens.json is not JSON: {e}"))
}

fn find_tokens_json(dir: &Path) -> Option<PathBuf> {
    for candidate in ["package/dist/styles/tokens.json", "package/dist/tokens.json", "package/tokens.json"] {
        let p = dir.join(candidate);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

/// The generated module text, deterministic for one (version, tokens.json) pair.
pub fn generate(version: &str, json: &serde_json::Value) -> Result<String, String> {
    let tokens = json.get("tokens").and_then(|t| t.as_array()).ok_or("tokens.json has no `tokens` array")?;
    let colour = |name: &str, theme: &str| -> Result<u32, String> {
        let t = tokens
            .iter()
            .find(|t| t.get("name").and_then(|n| n.as_str()) == Some(name) && t.get("kind").and_then(|k| k.as_str()) == Some("color"))
            .ok_or_else(|| format!("{PACKAGE} {version} has no colour token `{name}`"))?;
        let hex = t.get(theme).and_then(|v| v.as_str()).ok_or_else(|| format!("token `{name}` has no {theme} value"))?;
        parse_hex(hex).ok_or_else(|| format!("token `{name}` {theme} value `{hex}` is not #rrggbb"))
    };
    let mut out = String::new();
    out.push_str("// Generated by `cargo xtask tokens` from the Redrob design system package. Do not edit:\n");
    out.push_str("// the values are the design system's, the role -> token mapping is xtask/src/tokens.rs.\n\n");
    out.push_str(&format!("/// The design-system package these colours came from.\npub const SOURCE: &str = \"{PACKAGE} {version}\";\n\n"));
    for (theme, module) in [("light", "light"), ("dark", "dark")] {
        out.push_str(&format!("/// The {theme} appearance's colour roles, as 0xRRGGBB.\npub mod {module} {{\n"));
        for (role, doc, light_token, dark_token) in ROLES {
            let token = if theme == "light" { light_token } else { dark_token };
            let value = colour(token, theme)?;
            out.push_str(&format!("    /// {doc} (`{token}`).\n    pub const {}: u32 = 0x{value:06x};\n", role.to_uppercase()));
        }
        out.push_str("}\n");
        if theme == "light" {
            out.push('\n');
        }
    }
    Ok(out)
}

fn parse_hex(hex: &str) -> Option<u32> {
    let h = hex.strip_prefix('#')?;
    if h.len() != 6 {
        return None;
    }
    u32::from_str_radix(h, 16).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> serde_json::Value {
        let mut tokens = Vec::new();
        for (role_light, role_dark, _, _) in ROLES.iter().map(|(_, _, l, d)| (l, d, 0, 0)) {
            for name in [role_light, role_dark] {
                tokens.push(serde_json::json!({ "name": name, "kind": "color", "light": "#102030", "dark": "#405060" }));
            }
        }
        serde_json::json!({ "tokens": tokens })
    }

    #[test]
    fn every_role_binds_to_a_token_and_the_output_is_stable() {
        let a = generate("1.7.0", &sample()).unwrap();
        let b = generate("1.7.0", &sample()).unwrap();
        assert_eq!(a, b);
        assert!(a.contains("pub const SOURCE: &str = \"@redrob-labs/ui 1.7.0\";"));
        assert!(a.contains("pub const BG: u32 = 0x102030;"), "{a}");
        assert!(a.contains("pub mod dark {"));
        for (role, ..) in ROLES {
            assert!(a.contains(&format!("pub const {}: u32", role.to_uppercase())), "{role}");
        }
    }

    #[test]
    fn a_missing_token_is_an_error_not_a_default() {
        let json = serde_json::json!({ "tokens": [] });
        let err = generate("1.7.0", &json).unwrap_err();
        assert!(err.contains("has no colour token"), "{err}");
    }

    #[test]
    fn hex_parsing_accepts_only_rrggbb() {
        assert_eq!(parse_hex("#0a0b0c"), Some(0x0a0b0c));
        assert_eq!(parse_hex("0a0b0c"), None);
        assert_eq!(parse_hex("#fff"), None);
    }
}
