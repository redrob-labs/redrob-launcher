# AGENTS.md: guide for AI agents and contributors

Redrob Launcher installs, updates and launches the **Redrob apps** (Cut, Shape, CAD, Canvas,
Design, Studio, Browser, Code, IDE, Cowork, Office, Query, Recall: `crates/core/src/catalog.rs`).
It is a hard fork of an upstream launcher; `UPSTREAM.md` says which and what we do not own.
It is written in **Rust only** (no JavaScript or TypeScript): the desktop
app is native egui/eframe, and the web build is the same Rust compiled to WebAssembly (trunk +
wasm-bindgen). **No Tauri, Electron or webview shells**, ever.

The product name is always **Redrob Launcher** in user-facing text: UI, window titles, About,
tray menu, notifications, installers, release names, docs prose. The apps are the **Redrob
apps**. Machine names stay lowercase: crates and binaries (`redrob-launcher`,
`redrob-launcher-*`), file names, and the app id `ai.redrob.launcher`. Note the
GitHub repo is `redrob-labs/redrob-launcher`, while every file and package is `redrob-launcher-*`.

Organisation-wide rules live in `redrob-labs/.github` (CONTRIBUTING, FORKS).

## 1. Workspace map

```text
crates/
  core      redrob-launcher-core      pure and wasm-safe: the app catalog, versions, GitHub
                                        release parsing, picking the right asset per platform
  native    redrob-launcher-native    desktop only: HTTP, detecting installed apps, download,
                                        verify and install
  ui-egui   redrob-launcher-ui-egui   the shared egui UI (desktop and web)
apps/
  redrob-launcher                     desktop shell (eframe): window, tray icon, notifications
  redrob-launcher-web                 the same UI in the browser (trunk; catalog and downloads only)
xtask/                                  cargo xtask ci | wasm | version | ico
packaging/                              macOS, Windows, Linux and web packaging scripts
assets/brand/                           the design system's files, pinned by SHA256SUMS
assets/app-icon/                        the launcher's own icon and header lockup, derived from assets/brand/
assets/app-icons/                       each app's 128 px icon, derived from assets/brand/
assets/fonts/                           Pretendard and Wanted Sans (OFL-1.1): the design system's faces
```

Layering: `core` depends on no other workspace crate and must build for `wasm32-unknown-unknown`.
`native` may use `core`. `ui-egui` may use `core` (and `native` only behind
`cfg(not(target_arch = "wasm32"))` or through a trait the desktop shell implements). Nothing below
the apps opens windows, tray icons or notifications.

## 2. Golden rules

### Never crash (outranks feature work)

A bad network response, a malformed release, a half-downloaded file, a full disk or a corrupt
settings file must produce an error the user can act on, never a panic.

- **Non-test code never panics.** No `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!` or
  `unimplemented!`. Return the crate's error type and propagate with `?`.
- Every production crate root (`lib.rs`, each binary's `main.rs`, and `xtask`) carries
  `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`.
- **No `unsafe`.** The workspace sets `unsafe_code = "forbid"`. The one exception is the macOS
  Apple Event handler for `redrob://` links (`apps/redrob-launcher/src/url_event.rs`): that crate
  denies `unsafe_code` instead, and only that module allows it.
- **Everything from the network is hostile.** Use `get()` instead of indexing, checked arithmetic
  for sizes and offsets, and cap downloads and JSON sizes.

### Installing other people's software safely

- Download only from `https://github.com/storytold/<app>/releases` (and GitHub's asset CDN).
  The one exception is a developer's own loopback mirror (`REDROB_LAUNCHER_DOWNLOAD_BASE`,
  `cargo xtask local`); keep it loopback-only and behind the `local-mirror` cargo feature, which
  release builds never enable.
- Verify each download against the release's `SHA256SUMS.txt` before running or installing it.
- Never run an installer with elevated rights silently; tell the user what is about to happen.
- Never delete a user's files outside what the launcher itself installed.

### Look and feel

The UI follows the Redrob design system: hard square corners, 1 px hairlines instead of shadows,
Pretendard for Latin and Wanted Sans for Hangul, sentence-case labels (never capitals, no second
typeface), and the system's colour roles (`theme::Tokens`, each value commented with its token).
Blue acts; a product's colour marks its tile, never text. Build new UI from `widgets.rs`; never hard-code a colour.

### Languages

Every user-facing string goes through `crates/ui-egui/src/i18n` (`tl!("…")` for literals, `fmt`
for `{placeholders}`, `tn` for counts). English in the code is the lookup key; adding a string
means adding its row to every catalog marked `complete_menus` (`ko.tsv`), which the i18n tests
check. Product and app names (Redrob Launcher, Redrob CAD…) stay in Latin script. Hangul is drawn
with the bundled Wanted Sans, so Korean never depends on the system's fonts.

### Other rules

1. **Assets need attribution.** Every bundled asset gets a row in `ATTRIBUTION.md` (path, title,
   author, source, license) in the same change. The Redrob logo, tile and product marks in
   `assets/brand/` are used exactly as the design system ships them: never redrawn, recoloured,
   tinted or used as a macOS template image (`assets/brand/README.md`).
2. **Icons are generated.** Everything under `assets/app-icon/` and `assets/app-icons/` comes from
   `assets/brand/` through `packaging/brand-assets.sh` (needs `magick` and `resvg`); never edit the
   outputs by hand, and never upscale.
3. **Never break wasm.** `cargo xtask wasm` must pass. File-system, process and OS code is
   `cfg(not(target_arch = "wasm32"))`.
4. **Tests are the gate.** Release parsing and asset picking in `core` get unit tests with real
   GitHub release JSON fixtures (small, committed under `crates/core/tests/fixtures/`).
5. When fetching anything, use a generic `Redrob-Launcher/<version>` User-Agent and never put
   a person's name, email or other personal details in requests.

## 3. Before you finish a task

```sh
cargo xtask ci              # fmt --check, clippy -D warnings, test, wasm check
```

Or piecemeal: `cargo test -p <crate>`, `cargo clippy -p <crate> --all-targets -- -D warnings`,
`cargo xtask wasm`. If you touched packaging: `shellcheck packaging/*.sh packaging/*/*.sh
packaging/linux/AppRun` and `actionlint`.

## 4. Releasing

The version lives only in `[workspace.package] version` in the root `Cargo.toml`
(`cargo xtask version`, `cargo xtask version set X.Y.Z[-pre]`). Upstream's release workflow was
removed (it used upstream's signing secrets, see `UPSTREAM.md`); ours follows the organisation's
gitflow (`release/x.y.z` from `develop`, a pull request to `main`, tag `vX.Y.Z`; the shared gitflow
workflow opens the back-merge pull request `main` -> `develop`). The tag runs
`.github/workflows/release.yml`, which builds every platform, signs Windows executables and MSIs with
the shared `redrob-labs/.github/actions/sign-windows` action (Azure Artifact Signing over the job's
OpenID Connect login; `packaging/windows/package.ps1` itself never signs), and publishes the GitHub
Release **Redrob Launcher v<version>** with `SHA256SUMS.txt` through the shared
`redrob-labs/.github` release-publish workflow (no `redrob-launcher.json`: this repository is the
launcher). A manual dispatch is a dry run that builds and signs but publishes nothing.

Release assets (keep these names exact; the launcher's own self-update and the README rely on them):

| Platform | Files |
|---|---|
| Windows | `redrob-launcher-<ver>-windows-{x64,x86,arm64}-setup.exe`, `…-windows-{x64,x86,arm64}-portable.zip` |
| macOS | `redrob-launcher-<ver>-macos-universal.dmg` |
| Linux | `redrob-launcher-<ver>-linux-{x86_64,aarch64}.{AppImage,AppImage.zsync,deb,rpm,tar.gz}`, `redrob-launcher-<ver>-linux-riscv64.{deb,rpm,tar.gz}` |
| Web | `redrob-launcher-web-<ver>.zip` |

Packaging facts the code relies on:

- The Windows setup (`installer/windows`, a native Direct2D app ported from Redrob Canvas's; the
  WiX MSI it replaced was dropped in 0.4.0) is **per-user** (no UAC prompt), into
  `%LOCALAPPDATA%\Programs\Redrob Launcher`. Its first screen asks the language (English / 한국어)
  and every later screen follows it; then a welcome screen with a "Create a desktop shortcut" box,
  progress, done. `--silent` installs with no window: the launcher's self-update runs the new setup
  that way after quitting. It embeds the signed portable zip and its own payload-less build as the
  uninstaller. It records `InstallDir`, `Language` and `DesktopShortcutChoice` under
  `HKCU\Software\Redrob\Redrob Launcher`; `self_update.rs` reads `InstallDir` to know the running
  copy came from the setup, `language.rs` reads `Language` on the first run. Its shortcuts set
  AppUserModelID `ai.redrob.launcher` through `IPropertyStore`, which toast notifications need.
  `portable.txt` beside the exe switches on portable mode (`RedrobLauncherData\` next to the exe);
  the setup deletes it from what it installs.
- The macOS bundle is `Redrob Launcher.app` (executable `Contents/MacOS/Redrob Launcher`), a
  normal Dock app that can also sit in the menu bar: `LSUIElement` must never be set.
- Linux: the tray uses StatusNotifierItem over D-Bus and notifications use
  `org.freedesktop.Notifications`, both in pure Rust (no GTK, appindicator or libdbus).
- `build.rs` reads `REDROB_LAUNCHER_BUILD_SHA` / `REDROB_LAUNCHER_BUILD_DATE` and, on Windows,
  embeds `assets/app-icon/redrob-launcher.ico`, failing the build when
  `REDROB_LAUNCHER_REQUIRE_WINRES=1` and it can't.

## 5. Parallel agents

- Use your own target dir (`CARGO_TARGET_DIR=target/agent-<name>`) and edit only the files you own.
- Keep every `Cargo.toml` valid at all times: the `crates/*` and `apps/*` globs mean one broken
  manifest breaks everyone's build. Write manifests to a temp file and `mv` them into place.
- If someone else's in-progress edit breaks the build, wait and retry; don't fix their files.
