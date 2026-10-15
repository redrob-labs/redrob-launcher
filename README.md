# Redrob Launcher

Redrob Launcher is one place to install, update, sign in and connect every Redrob app.

한국어: [README.ko.md](./README.ko.md)

> **Fork notice.** This repository is a hard fork of [storytold/craft-launcher](https://github.com/storytold/craft-launcher)
> at tag `v0.3.2`. Where it was cut, what it does not own and how upstream is absorbed are in
> [UPSTREAM.md](./UPSTREAM.md); the machine-readable pin is [`upstream-base.json`](./upstream-base.json).
> Upstream's copyright is retained in [NOTICE](./NOTICE) with ours added below it. Upstream's
> ArtCraft trademarks have been removed.

## What it does

- Lists every Redrob app: Cut, Shape, CAD, Canvas, Design, Browser, Code, Cowork,
  Office, Query and Recall.
- Reads each app's GitHub releases, picks the right build for your OS and CPU (DMG, MSI, AppImage,
  .deb, .rpm and more), checks it against the release's SHA-256 checksums and installs it.
- Shows which apps you have and which version, tells you when an update is out, and updates them.
- Sits in the tray or menu bar. Native Rust and egui: no Electron, no web view.
- macOS, Windows, Linux and FreeBSD, plus a web build that lists the apps with download links.
- English and Korean.

- Signs the computer in to a Redrob workspace once (the console's device flow), keeps the key where
  every Redrob app can read it, and hands it to apps that keep their own key
  ([docs/shared-account.md](docs/shared-account.md)).

- Shows the Redrob Code engine every app runs its AI through: whether it is up, which apps are
  attached, and Start / Stop / Restart. Uses `redrob-engine-client`, the Rust crate the apps use
  to reach that one daemon, pinned to the engine release it ships with ([docs/engine.md](docs/engine.md)).

- Downloads the models the apps use (today Redrob Cut's Whisper speech models and natural voices)
  straight into the folder each app reads them from: resumable, pausable, every file checked
  against its pinned SHA-256. The list is `assets/models/models.json`, regenerated from each app's
  own catalogue by `scripts/gen_models_manifest.py`.

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
