# Which apps use the launcher's sign-in

한국어: [app-integration.ko.md](./app-integration.ko.md)

Redrob Launcher signs a computer in once and keeps the key in the shared account file
([shared-account.md](./shared-account.md)). This page tracks, app by app, how far each one is from
using it. "Device flow" means the app already asks console.redrob.ai for its own key under its own
product id (the console's `DEVICE_PRODUCTS`); those apps are one small change away.

| App | Its own sign-in today | Reads the shared file | Launcher can hand it the key | What is left |
|---|---|---|---|---|
| Redrob CAD | Device flow (`cad`), `account.json` | Yes, released in v0.0.3 (redrob-cad#32) | Yes | Nothing |
| Redrob Cut | Device flow (`cut`), `account.json` | Yes, released in v0.0.1 (redrob-cut#6) | Yes | Nothing |
| Redrob Code | `REDROB_API_KEY`, its credential store, `auth.json` | Merged (redrob-code#88): fourth origin of `ConsoleKey.resolve`, reported as `source: "launcher"` | Yes (`auth.json`, other providers kept) | Release it |
| Redrob Shape | Device flow (`shape`) | Merged (redrob-shape#38) | No | Release it |
| Redrob Browser | Device flow (`redrob-browser`) | Merged (redrob-browser#110) | No | Release it |
| Redrob Cowork | Device flow (`work`), key kept by its Redrob Code engine | Through the engine (redrob-code#88); shows "Signed in with Redrob Launcher" (redrob-cowork#207) | No (the engine's `auth.json` is Redrob Code's row) | Release it with the new engine |
| Redrob Office | Device flow (`office`) | Merged (redrob-office#154) | No | Release it |
| Redrob Design | Device flow (`design`) | Merged (redrob-design#92) | No | Release it |
| Redrob Canvas | `REDROB_API_KEY`, saved `console-key`, device flow | Merged (redrob-canvas#160) | No | Release it |
| Redrob Query | `REDROB_API_KEY`, OS keyring | Merged (redrob-query#55) | No | Release it |
| Redrob Recall | OS keyring, pasted key | Merged (redrob-recall#49) | No | Release it |
| Redrob IDE | None: no AI feature, no console key | Not needed | Not needed | Nothing |
| Redrob Studio | None: runs every model locally | Not needed | Not needed | Nothing |

The launcher's account window reads this table through `SHARED_READERS` in `crates/core/src/account.rs`:
an app whose installed version is at or past the listed release shows "Uses this sign-in"; a merged but
unreleased one shows "Next release uses this sign-in"; IDE and Studio show "Needs no sign-in". When an app
releases, put its version into that table.

For an app that reads the shared file, the rules are the four in
[shared-account.md](./shared-account.md#rules-for-readers): read, never write; read it when the key is
needed; never log or show the key; the app's own key wins. Redrob CAD's and Redrob Cut's change is the
reference: `shared_account_path()` and `KeySource::current()` in their `device.rs`, and "Signed in
with Redrob Launcher" in place of Disconnect when the shared key is the one in use.

## Release asset names (audit 2026-10-13)

The launcher installs an asset only when its name reads `<stem>-<version>-<os>-<arch>[-portable].<ext>`
(`parse_asset_name` in `crates/core/src/platform.rs`). Latest release of each app:

| App | Latest | Example asset | Installable | To fix in that repo |
|---|---|---|---|---|
| Redrob CAD | v0.0.3 | `redrob-cad-0.0.3-linux-x86_64.tar.gz` | Yes, every platform | Nothing |
| Redrob Cut | v0.0.1 | `redrob-cut-0.0.1-linux-x86_64.tar.gz` | Yes, every platform | Rename `filmcraft-web-0.0.1.zip` (not installable anyway) |
| Redrob Canvas | v0.5.6 | `redrob-canvas-0.5.6-linux-x86_64.tar.gz` | Linux, macOS; Windows only via the `.zip` | `-setup.exe` carries a suffix the parser does not read |
| Redrob Shape | v0.1.0 | `redrob-shape-linux-x64-0.1.0.tar.xz` | No | Version before OS; `x64` -> `x86_64`; `.tar.xz` -> `.tar.gz` |
| Redrob Browser | v0.1.8 | `redrob-browser-macos-arm64-0.1.8.zip`, `redrob-browser-stable_0.1.8+155…_amd64.deb` | No | Version before OS; the Debian name is the packaging's own |
| Redrob Code | v0.5.0 | `redrob-linux-x64.tar.gz` | No | No version in the name; stem `redrob`, not `redrob-code`; `x64` -> `x86_64` |
| Redrob Cowork | v0.3.0 | `redrob-linux-x86_64-0.3.0.AppImage` | No | Version after arch; stem `redrob`; `mac` -> `macos` |
| Redrob Office | v0.9.0 | `Redrob-0.9.0-arm64.dmg`, `Redrob-0.9.0.AppImage` | No | electron-builder defaults: stem `Redrob`, no OS, arch missing on Linux |
| Redrob Design | v0.15.0 | `Redrob.Design_0.15.0_aarch64.dmg` | No | Tauri defaults: `Redrob.Design_<ver>_<arch>`; no Linux or Windows asset |
| Redrob Query | v0.2.0 | `Redrob.Query_0.2.0_amd64.AppImage` | No | Tauri defaults, as Design |
| Redrob Recall | v0.2.0 | `Redrob.Recall_0.2.0_amd64.AppImage` | No | Tauri defaults, as Design |
| Redrob Studio, Redrob IDE | none | -- | No | No release yet |

Two ways to close the gap: each repo renames its assets in its release workflow (one `mv` step before
upload, keeping the packagers' own names as extra copies), or the launcher learns the Tauri and
electron-builder spellings. The first keeps one rule for every app; the second ships sooner. Neither is
done yet.
