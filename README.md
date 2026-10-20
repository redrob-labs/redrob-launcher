# Redrob Launcher

Redrob Launcher is one place to install, update, sign in and connect every Redrob app.

한국어: [README.ko.md](./README.ko.md)

> **Fork notice.** This repository is a hard fork of [storytold/craft-launcher](https://github.com/storytold/craft-launcher)
> at tag `v0.3.2`. Where it was cut, what it does not own and how upstream is absorbed are in
> [UPSTREAM.md](./UPSTREAM.md); the machine-readable pin is [`upstream-base.json`](./upstream-base.json).
> Upstream's copyright is retained in [NOTICE](./NOTICE) with ours added below it. Upstream's
> ArtCraft trademarks have been removed.

## What it does

- Lists every Redrob app: Cut, Shape, CAD, Canvas, Design, Browser, Code, Cowork
  and Office.
- Reads each app's GitHub releases, picks the right build for your OS and CPU (DMG, MSI, AppImage,
  .deb, .rpm and more), checks it against the release's SHA-256 checksums and installs it.
- Shows which apps you have and which version, tells you when an update is out, and updates them.
- Sits in the tray or menu bar. Native Rust and egui: no Electron, no web view.
- macOS, Windows and Linux, plus a web build that lists the apps with download links.
- English and Korean. The Windows setup wizard asks which on its first page; the launcher and
  every Redrob app then follow that choice, per app if wanted
  ([docs/shared-language.md](docs/shared-language.md)).

- Signs the computer in to a Redrob workspace once (the console's device flow), keeps the key where
  every Redrob app can read it, and hands it to apps that keep their own key
  ([docs/shared-account.md](docs/shared-account.md)).

- Shows the Redrob Code engine every app runs its AI through: whether it is up, which apps are
  attached, and Start / Stop / Restart. Uses `redrob-engine-client`, the Rust crate the apps use
  to reach that one daemon, pinned to the engine release it ships with ([docs/engine.md](docs/engine.md)).

- Channels: turns on a chat channel for that engine from Settings > Channels, so a person talks to
  it from WhatsApp, Telegram, Discord or Slack on their phone. One card per channel, a three-step
  wizard: a button opens the platform's "create a bot" page, the token goes in a field, and the
  first message sent to the bot makes its sender the owner (no ids to look up; the allowlist
  waits under Advanced). WhatsApp only shows a QR code to scan. A status line follows the running
  engine's `/channels/status`. Save writes the engine's `channels.json` and `channels/secrets.json`,
  turns the channel on through `redrob channels enable` (which registers the engine to start at
  login) and reloads the daemon. Settings > Apps lists the Redrob apps bridged to the engine right
  now, with their tools and last heartbeat.

- Downloads the local models every Redrob app can use: chat, coding and vision LLMs, Canvas's
  image models (our own Redrob Image, LaMa inpainting, background removal, upscaling, selection),
  Cut's speech models and voices and Shape's image-to-3D. Measures this computer
  first and says which of them run here. Resumable, pausable, every file checked against its
  pinned SHA-256. The list is `assets/models/models.json`.

- Serves a downloaded GGUF with `llama-server` on loopback, and tells the apps about every local
  model through one registry file, so an app uses the local model when there is one and the
  Console when there is not ([docs/local-models.md](docs/local-models.md)).

## Open from the web

Web pages open the launcher on one app with a `redrob://` link:

- `redrob://open/cad` starts Redrob CAD, or shows it with its Install button when it isn't
  installed;
- `redrob://install/cowork` shows Redrob Cowork and installs it.

The app is its catalog id with or without `redrob-` (`cad`, `cowork`, `office`, `design`…); any
other link just opens the launcher. The installers register the scheme, and the launcher checks it
on every start. Details: [docs/deep-links.md](docs/deep-links.md).

## Build from source

```sh
cargo run --release -p redrob-launcher     # the desktop app
cargo test --workspace                     # tests
python3 scripts/check_upstream_boundary.py # the fork boundary check
packaging/brand-assets.sh                  # regenerate icons from assets/brand/ (magick, resvg)
```

On Linux the build needs the egui/winit headers (`libxkbcommon-dev libwayland-dev libx11-dev
libxrandr-dev libxi-dev libgl1-mesa-dev`). Packaging scripts for every platform are in
[`packaging/`](packaging/).

## Branches and contributing

- Default branch `develop`, release branch `main`. Branch names take a
  `feat/ fix/ chore/ docs/ test/ refactor/ perf/ ci/ build/ sync/ hotfix/ release/` prefix, checked by
  the shared redrob-labs gitflow workflow (one type list for every repository).
- Upstream syncs land through `sync/upstream-<tag>` branches as merge commits.
- Organisation rules: CONTRIBUTING.md and FORKS.md in [redrob-labs/.github](https://github.com/redrob-labs/.github).
- Contributor guide for agents and people: [AGENTS.md](./AGENTS.md).

## License

MIT or Apache-2.0, at your option ([LICENSE-MIT](./LICENSE-MIT), [LICENSE-APACHE](./LICENSE-APACHE)).
Copyright notices are in [NOTICE](./NOTICE), upstream's first and ours below it:

```
Copyright (c) 2026 ArtCraft Team and the ArtCraft Launcher contributors.
Copyright (c) 2026-present Janghoon Lee (Redrob) and contributors
```

The Redrob name and marks are trademarks of Redrob, not covered by those licenses. Per-asset
attribution is in [ATTRIBUTION.md](./ATTRIBUTION.md).
