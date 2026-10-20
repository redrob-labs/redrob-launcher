//! `cargo xtask tokens [--check] [--version X.Y.Z --sha256 HEX]`: generate the launcher's
//! colours from the integrity-pinned Redrob design-system package, `@redrob-labs/ui`.
//!
//! The design system ships its resolved colour tokens as `tokens.json` (one value per theme for
//! every token). With no source flags this command downloads the repository-pinned release and
//! verifies its SHA-256 before reading it. An explicit `--version` uses the built-in integrity
//! only for that pinned release; every other release requires an explicit `--sha256`. It writes
//! checked Rust, loading-page CSS, and Windows installer header views. The launcher role -> token
//! mapping is [`ROLES`] below: that is the launcher's own decision; the values are the design
//! system's. Approved platform and fixed brand-art exceptions also live here, so consumer
//! languages never spell colours independently.
//!
//! `--check` regenerates every output in memory and fails when any committed file differs.
//!
//! Needs `curl` and `tar` (the npm registry serves plain tarballs; no npm needed). The archive is
//! never unpacked onto the filesystem: only the expected token member is streamed with a size cap.

use std::fs::OpenOptions;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

const PACKAGE: &str = "@redrob-labs/ui";
const REGISTRY: &str = "https://registry.npmjs.org";
const PINNED_VERSION: &str = "1.7.0";
const PINNED_SHA256: &str = "e5433dc38abe67b39cfccb862847d5b5904f9a2772f0303d8015b0a4a188cdbd";
const MAX_TARBALL_BYTES: u64 = 4 * 1024 * 1024;
const MAX_TOKENS_JSON_BYTES: usize = 256 * 1024;
const TOKEN_PATHS: &[&str] = &["package/dist/styles/tokens.json", "package/dist/tokens.json", "package/tokens.json"];
const RUST_OUT: &str = "crates/ui-egui/src/theme/generated.rs";
const CSS_OUT: &str = "apps/redrob-launcher-web/generated_tokens.css";
const CPP_OUT: &str = "installer/windows/generated_tokens.h";

const USAGE: &str = "\
usage: cargo xtask tokens [--check] [--version X.Y.Z --sha256 HEX]\n\
\n\
Downloads integrity-pinned @redrob-labs/ui tokens and writes the generated Rust, CSS,\n\
and C++ views. With no source flags, version 1.7.0 and its repository-pinned SHA-256\n\
are used. A different --version requires its 64-digit SHA-256 via --sha256.\n\
\n\
options:\n\
  --check            fail instead of writing when any generated view is stale\n\
  --version X.Y.Z    use an explicit package version (pinned version is also allowed)\n\
  --sha256 HEX       required integrity for a non-pinned explicit version\n\
  -h, --help         print this help\n";

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
    ("primary_hover", "Primary action hover fill", "action-primary-hover", "action-primary-hover"),
    ("focus", "Keyboard focus ring", "focus-ring", "focus-ring"),
    ("brand_subtle", "Subtle brand-tinted surface", "surface-brand-subtle", "surface-brand-subtle"),
    ("on_brand", "Text and glyphs on brand and status fills", "ink-on-brand", "ink-on-brand"),
    ("invert_bg", "Primary buttons and hover inversion: the pill", "redrob-black", "redrob-white"),
    ("invert_fg", "Primary buttons and hover inversion: the text on the pill", "redrob-white", "redrob-black"),
    ("good", "Success", "status-success", "status-success"),
    ("warn", "Warning", "status-warning", "status-warning"),
    ("bad", "Danger", "status-danger", "status-danger"),
];

/// Windows owns this standard caption colour, so this named exception cannot drift between
/// renderers.
const PLATFORM_CAPTION_CLOSE_HOVER: u32 = 0xe81123;

/// These theme-independent installer gradient colours are fixed brand artwork, not UI roles.
const INSTALLER_BRAND_COLOURS: &[(&str, u32)] = &[("kPanelCore", 0x2e7eff), ("kPanelEdge", 0x2746ee), ("kPanelViolet", 0x8b82ff)];

pub fn run(root: &Path, args: &[&str]) -> Result<(), String> {
    let (check, source) = match parse_options(args)? {
        TokenCommand::Help => {
            print!("{USAGE}");
            return Ok(());
        }
        TokenCommand::Generate { check, source } => (check, source),
    };
    let json = fetch_tokens_json(&source)?;
    let version = &source.version;
    let outputs = [(RUST_OUT, generate(version, &json)?), (CSS_OUT, generate_css(version, &json)?), (CPP_OUT, generate_cpp(version, &json)?)];
    if check {
        let mut stale = Vec::new();
        for (relative, generated) in &outputs {
            let out = root.join(relative);
            let current = std::fs::read_to_string(&out).map_err(|e| format!("{}: {e} (run `cargo xtask tokens`)", out.display()))?;
            if current == *generated {
                println!("{} is current with {PACKAGE} {version}", out.display());
            } else {
                stale.push(out);
            }
        }
        if stale.is_empty() {
            return Ok(());
        }
        let paths = stale.iter().map(|path| path.display().to_string()).collect::<Vec<_>>().join(", ");
        return Err(format!("{paths} out of date with {PACKAGE} {version}: run `cargo xtask tokens` and commit"));
    }
    for (relative, generated) in outputs {
        let out = root.join(relative);
        if let Some(dir) = out.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        std::fs::write(&out, generated).map_err(|e| format!("{}: {e}", out.display()))?;
        println!("wrote {} from {PACKAGE} {version}", out.display());
    }
    Ok(())
}

struct TokenSource {
    version: String,
    sha256: String,
}

enum TokenCommand {
    Help,
    Generate { check: bool, source: TokenSource },
}

fn parse_options(args: &[&str]) -> Result<TokenCommand, String> {
    if matches!(args, ["-h"] | ["--help"]) {
        return Ok(TokenCommand::Help);
    }
    let mut check = false;
    let mut version: Option<String> = None;
    let mut sha256: Option<String> = None;
    let mut it = args.iter();
    while let Some(argument) = it.next() {
        match *argument {
            "--check" if !check => check = true,
            "--check" => return Err("--check may be specified only once".into()),
            "--version" if version.is_none() => {
                let value = it.next().ok_or("--version needs a value")?;
                if value.starts_with('-') {
                    return Err("--version needs a value".into());
                }
                version = Some((*value).to_string());
            }
            "--version" => return Err("--version may be specified only once".into()),
            "--sha256" if sha256.is_none() => {
                let value = it.next().ok_or("--sha256 needs a value")?;
                if value.starts_with('-') {
                    return Err("--sha256 needs a value".into());
                }
                sha256 = Some(validate_sha256(value)?);
            }
            "--sha256" => return Err("--sha256 may be specified only once".into()),
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    let source = select_source(version, sha256)?;
    Ok(TokenCommand::Generate { check, source })
}

fn select_source(version: Option<String>, sha256: Option<String>) -> Result<TokenSource, String> {
    match (version, sha256) {
        (None, None) => Ok(TokenSource { version: PINNED_VERSION.into(), sha256: PINNED_SHA256.into() }),
        (None, Some(_)) => Err("--sha256 requires --version; normal generation always uses the repository pin".into()),
        (Some(version), integrity) => {
            validate_version(&version)?;
            let sha256 = match integrity {
                Some(value) => value,
                None if version == PINNED_VERSION => PINNED_SHA256.into(),
                None => return Err(format!("--version {version} requires --sha256 HEX so an unpinned package cannot enter generated outputs")),
            };
            Ok(TokenSource { version, sha256 })
        }
    }
}

fn validate_version(version: &str) -> Result<(), String> {
    let core = version.split(['-', '+']).next().unwrap_or_default();
    let core_is_semver =
        core.split('.').count() == 3 && core.split('.').all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()));
    let safe = !version.is_empty()
        && version.len() <= 64
        && version.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'+'))
        && !version.contains("..");
    if safe && core_is_semver {
        Ok(())
    } else {
        Err(format!("invalid package version `{version}`; expected a path-safe X.Y.Z[-pre][+build] version"))
    }
}

fn validate_sha256(value: &str) -> Result<String, String> {
    if value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        Ok(value.to_ascii_lowercase())
    } else {
        Err("--sha256 must be exactly 64 hexadecimal digits".into())
    }
}

fn fetch_tokens_json(source: &TokenSource) -> Result<serde_json::Value, String> {
    let name = PACKAGE.rsplit('/').next().unwrap_or(PACKAGE);
    let url = format!("{REGISTRY}/{PACKAGE}/-/{name}-{}.tgz", source.version);
    let tgz = download_tarball(&url)?;
    let actual = sha256_hex(&tgz);
    if actual != source.sha256 {
        return Err(format!("{PACKAGE} {} tarball SHA-256 mismatch: expected {}, got {actual}", source.version, source.sha256));
    }
    let tarball = TempTarball::create(&tgz)?;
    let json = extract_tokens_json(&tarball.path)?;
    serde_json::from_slice(&json).map_err(|e| format!("tokens.json is not JSON: {e}"))
}

fn download_tarball(url: &str) -> Result<Vec<u8>, String> {
    let user_agent = format!("Redrob-Launcher/{}", env!("CARGO_PKG_VERSION"));
    let mut child = Command::new("curl")
        .args([
            "--fail",
            "--silent",
            "--show-error",
            "--location",
            "--proto",
            "=https",
            "--proto-redir",
            "=https",
            "--connect-timeout",
            "15",
            "--max-time",
            "60",
            "--max-filesize",
            &MAX_TARBALL_BYTES.to_string(),
            "--user-agent",
            &user_agent,
            url,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("curl: {e}"))?;
    let stdout = child.stdout.take().ok_or("curl stdout was not available")?;
    let mut bytes = Vec::new();
    stdout.take(MAX_TARBALL_BYTES + 1).read_to_end(&mut bytes).map_err(|e| format!("curl response: {e}"))?;
    if bytes.len() as u64 > MAX_TARBALL_BYTES {
        let _ = child.kill();
        let _ = child.wait();
        return Err(format!("package tarball exceeds the {MAX_TARBALL_BYTES}-byte download limit"));
    }
    let output = child.wait_with_output().map_err(|e| format!("curl: {e}"))?;
    if !output.status.success() {
        return Err(format!("curl {url}: {}", String::from_utf8_lossy(&output.stderr).trim()));
    }
    Ok(bytes)
}

struct TempTarball {
    path: PathBuf,
}

impl TempTarball {
    fn create(bytes: &[u8]) -> Result<Self, String> {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| format!("system clock: {e}"))?.as_nanos();
        for attempt in 0..100_u8 {
            let path = std::env::temp_dir().join(format!("redrob-ui-tokens-{}-{nonce}-{attempt}.tgz", std::process::id()));
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    let tarball = Self { path };
                    std::io::Write::write_all(&mut file, bytes).map_err(|e| format!("{}: {e}", tarball.path.display()))?;
                    return Ok(tarball);
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(format!("{}: {error}", path.display())),
            }
        }
        Err("could not reserve a unique temporary tarball path".into())
    }
}

impl Drop for TempTarball {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

fn extract_tokens_json(tarball: &Path) -> Result<Vec<u8>, String> {
    for member in TOKEN_PATHS {
        if let Some(json) = extract_member(tarball, member)? {
            return Ok(json);
        }
    }
    Err("the package has no supported tokens.json member".into())
}

fn extract_member(tarball: &Path, member: &str) -> Result<Option<Vec<u8>>, String> {
    let mut child = Command::new("tar")
        .arg("-xOzf")
        .arg(tarball)
        .arg("--")
        .arg(member)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("tar: {e}"))?;
    let stdout = child.stdout.take().ok_or("tar stdout was not available")?;
    let mut bytes = Vec::new();
    stdout.take(MAX_TOKENS_JSON_BYTES as u64 + 1).read_to_end(&mut bytes).map_err(|e| format!("tar output: {e}"))?;
    if bytes.len() > MAX_TOKENS_JSON_BYTES {
        let _ = child.kill();
        let _ = child.wait();
        return Err(format!("{member} exceeds the {MAX_TOKENS_JSON_BYTES}-byte extracted JSON limit"));
    }
    let status = child.wait().map_err(|e| format!("tar: {e}"))?;
    if status.success() { Ok(Some(bytes)) } else { Ok(None) }
}

fn sha256_hex(bytes: &[u8]) -> String {
    sha256(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

fn sha256(bytes: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be,
        0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa,
        0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85,
        0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f,
        0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut padded = bytes.to_vec();
    let bit_len = (bytes.len() as u64).wrapping_mul(8);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    let mut state = [0x6a09e667_u32, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    for chunk in padded.as_chunks::<64>().0 {
        let mut schedule = [0_u32; 64];
        for (index, word) in chunk.as_chunks::<4>().0.iter().enumerate() {
            schedule[index] = u32::from_be_bytes(*word);
        }
        for index in 16..64 {
            let s0 = schedule[index - 15].rotate_right(7) ^ schedule[index - 15].rotate_right(18) ^ (schedule[index - 15] >> 3);
            let s1 = schedule[index - 2].rotate_right(17) ^ schedule[index - 2].rotate_right(19) ^ (schedule[index - 2] >> 10);
            schedule[index] = schedule[index - 16].wrapping_add(s0).wrapping_add(schedule[index - 7]).wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for index in 0..64 {
            let sum1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ ((!e) & g);
            let temp1 = h.wrapping_add(sum1).wrapping_add(choice).wrapping_add(K[index]).wrapping_add(schedule[index]);
            let sum0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = sum0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        for (slot, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
    let mut digest = [0_u8; 32];
    for (output, value) in digest.as_chunks_mut::<4>().0.iter_mut().zip(state) {
        *output = value.to_be_bytes();
    }
    digest
}

#[derive(Clone, Copy)]
enum GeneratedSyntax {
    Rust,
    Css,
    Cpp,
}

fn generated_line_is_comment(line: &str, syntax: GeneratedSyntax, in_block_comment: &mut bool) -> bool {
    let trimmed = line.trim();
    if *in_block_comment {
        if trimmed.contains("*/") {
            *in_block_comment = false;
        }
        return true;
    }
    match syntax {
        GeneratedSyntax::Rust | GeneratedSyntax::Cpp => trimmed.starts_with("//"),
        GeneratedSyntax::Css if trimmed.starts_with("/*") => {
            *in_block_comment = !trimmed.contains("*/");
            true
        }
        GeneratedSyntax::Css => false,
    }
}

fn generated_line_key(path: &str, line: &str) -> String {
    let mut digest = 0xcbf29ce484222325_u64;
    for byte in path.bytes().chain(std::iter::once(0)).chain(line.bytes()) {
        digest ^= u64::from(byte);
        digest = digest.wrapping_mul(0x100000001b3);
    }
    format!("{digest:016x}")
}

fn generated_line_rationale(line: &str) -> &'static str {
    let trimmed = line.trim();
    if trimmed.starts_with("pub const SOURCE") {
        "Records the package version so generated colours retain auditable provenance."
    } else if trimmed.starts_with("pub mod") {
        "Opens the generated role namespace for one explicitly selected appearance."
    } else if trimmed.starts_with("pub const") {
        "Exports one resolved design-system colour for renderer consumers."
    } else if trimmed == "}" {
        "Closes the generated appearance namespace before the next role group."
    } else if trimmed.starts_with(":root") {
        "Publishes the generated loading-page colour roles as CSS custom properties."
    } else if trimmed.starts_with("@media") {
        "Scopes dark token overrides to the user's requested colour scheme."
    } else if trimmed.starts_with("#pragma once") {
        "Prevents duplicate native token definitions within one installer translation unit."
    } else if trimmed.starts_with("constexpr Palette") {
        "Initializes the installer's ordered palette from resolved design-system roles."
    } else if trimmed.starts_with("constexpr D2D1_COLOR_F") {
        "Provides one immutable generated colour to the native installer renderer."
    } else {
        "Keeps this generated structural line explicit in the checked consumer view."
    }
}

fn with_generated_rationales(path: &str, syntax: GeneratedSyntax, source: String) -> String {
    let mut seen = Vec::<(String, usize)>::new();
    let mut rationales = Vec::new();
    let mut in_block_comment = false;
    for line in source.lines() {
        if line.trim().is_empty() || generated_line_is_comment(line, syntax, &mut in_block_comment) {
            continue;
        }
        let digest = generated_line_key(path, line);
        let occurrence = match seen.iter_mut().find(|(key, _)| *key == digest) {
            Some((_, count)) => {
                *count += 1;
                *count
            }
            None => {
                seen.push((digest.clone(), 1));
                1
            }
        };
        let body = format!("GENERATED-LINE-RATIONALE {digest}:{occurrence}: {}", generated_line_rationale(line));
        rationales.push(match syntax {
            GeneratedSyntax::Rust | GeneratedSyntax::Cpp => format!("// {body}"),
            GeneratedSyntax::Css => format!("/* {body} */"),
        });
    }
    format!("{}\n{source}", rationales.join("\n"))
}

/// The generated module text, deterministic for one (version, tokens.json) pair.
pub fn generate(version: &str, json: &serde_json::Value) -> Result<String, String> {
    let colour = |name: &str, theme: &str| token_colour(version, json, name, theme);
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
    // Cowork deliberately keeps dark secondary ink for caption symbols in both appearances.
    let symbol = colour("ink-secondary", "dark")?;
    out.push_str("\n/// Approved platform-caption roles that do not follow the active appearance.\npub mod platform_caption {\n");
    out.push_str(&format!(
        "    /// Redrob Cowork's fixed `titleBarOverlay.symbolColor` (dark `ink-secondary`).\n    pub const SYMBOL: u32 = 0x{symbol:06x};\n"
    ));
    out.push_str(&format!(
        "    /// Windows' standard close-button hover, as used by Electron.\n    pub const CLOSE_HOVER: u32 = 0x{PLATFORM_CAPTION_CLOSE_HOVER:06x};\n"
    ));
    out.push_str("}\n");
    Ok(with_generated_rationales(RUST_OUT, GeneratedSyntax::Rust, out))
}

fn token_colour(version: &str, json: &serde_json::Value, name: &str, theme: &str) -> Result<u32, String> {
    let tokens = json.get("tokens").and_then(|tokens| tokens.as_array()).ok_or("tokens.json has no `tokens` array")?;
    let token = tokens
        .iter()
        .find(|token| {
            token.get("name").and_then(|value| value.as_str()) == Some(name) && token.get("kind").and_then(|value| value.as_str()) == Some("color")
        })
        .ok_or_else(|| format!("{PACKAGE} {version} has no colour token `{name}`"))?;
    let hex = token.get(theme).and_then(|value| value.as_str()).ok_or_else(|| format!("token `{name}` has no {theme} value"))?;
    parse_hex(hex).ok_or_else(|| format!("token `{name}` {theme} value `{hex}` is not #rrggbb"))
}

fn role_colour(version: &str, json: &serde_json::Value, role_name: &str, dark: bool) -> Result<u32, String> {
    let (_, _, light_token, dark_token) =
        ROLES.iter().find(|(role, ..)| *role == role_name).ok_or_else(|| format!("generator has no launcher role `{role_name}`"))?;
    token_colour(version, json, if dark { dark_token } else { light_token }, if dark { "dark" } else { "light" })
}

fn generate_css(version: &str, json: &serde_json::Value) -> Result<String, String> {
    let light_bg = role_colour(version, json, "bg", false)?;
    let light_muted = role_colour(version, json, "muted", false)?;
    let dark_bg = role_colour(version, json, "bg", true)?;
    let dark_muted = role_colour(version, json, "muted", true)?;
    Ok(with_generated_rationales(
        CSS_OUT,
        GeneratedSyntax::Css,
        format!(
            "/* Generated by `cargo xtask tokens` from {PACKAGE} {version}. Do not edit;\n   launcher role bindings are defined in xtask/src/tokens.rs. */\n:root {{ --surface-base: #{light_bg:06x}; --ink-secondary: #{light_muted:06x}; }}\n@media (prefers-color-scheme: dark) {{\n  :root {{ --surface-base: #{dark_bg:06x}; --ink-secondary: #{dark_muted:06x}; }}\n}}\n"
        ),
    ))
}

fn generate_cpp(version: &str, json: &serde_json::Value) -> Result<String, String> {
    // Aggregate order is part of Palette's contract; role names prevent a second value mapping.
    const PALETTE_ROLES: &[&str] =
        &["bg", "sunken", "ink", "muted", "line", "line_strong", "accent", "primary_hover", "focus", "bad", "accent_ink", "brand_subtle"];
    let palette = |dark: bool| -> Result<String, String> {
        PALETTE_ROLES
            .iter()
            .map(|role| role_colour(version, json, role, dark).map(|value| format!("Hex(0x{value:06X})")))
            .collect::<Result<Vec<_>, _>>()
            .map(|values| values.join(", "))
    };
    let mut out = format!(
        "// Generated by `cargo xtask tokens` from {PACKAGE} {version}. Do not edit.\n// Include after Palette and Hex; launcher role bindings are in xtask/src/tokens.rs.\n#pragma once\n\nconstexpr Palette kLight = {{{}}};\nconstexpr Palette kDark = {{{}}};\n",
        palette(false)?,
        palette(true)?
    );
    let white = role_colour(version, json, "on_brand", false)?;
    let panel_mid = role_colour(version, json, "accent", false)?;
    out.push_str(&format!("constexpr D2D1_COLOR_F kWhite = Hex(0x{white:06X});\n"));
    out.push_str("// Fixed artwork colours preserve the installer's theme-independent brand gradient.\n");
    for (name, value) in INSTALLER_BRAND_COLOURS {
        out.push_str(&format!("constexpr D2D1_COLOR_F {name} = Hex(0x{value:06X});\n"));
    }
    out.push_str(&format!("constexpr D2D1_COLOR_F kPanelMid = Hex(0x{panel_mid:06X});\n"));
    Ok(with_generated_rationales(CPP_OUT, GeneratedSyntax::Cpp, out))
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
