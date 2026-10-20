# Attribution

Every non-code asset in this repository, with its author, source and license. Redrob Launcher's
code is MIT OR Apache-2.0 (see [`LICENSE-MIT`](LICENSE-MIT), [`LICENSE-APACHE`](LICENSE-APACHE)
and [`NOTICE`](NOTICE)). When you add an asset, add a row here in the same change; third-party
assets must be permissively licensed and keep their license file next to them.

## Bundled in the app

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `assets/brand/` (all files) | The 2026 Redrob group icon and app icons, symbol and primary lockup | Redrob | The Redrob design system (`@redrob-labs/ui@1.8.0` `dist/brand/app-icons/`, `Favicons/`, `Logos/`), copied unmodified and pinned by `assets/brand/SHA256SUMS`; see [`assets/brand/README.md`](assets/brand/README.md) | Redrob trademarks; not MIT or Apache-2.0 |
| `assets/app-icon/` (all files) | Redrob Launcher app icon (the Redrob group icon) and the header lockup | Redrob | Derived from `assets/brand/` by `packaging/brand-assets.sh` (SVG rendered per size; lockup resized down) | Redrob trademarks; not MIT or Apache-2.0 |
| `assets/app-icons/*.png` | Each Redrob app's 128 px icon, rendered from its 2026 icon | Redrob | Derived from `assets/brand/` by `packaging/brand-assets.sh` | Redrob trademarks; not MIT or Apache-2.0 |
| `assets/fonts/Pretendard-Regular.ttf`, `-Medium.ttf`, `-SemiBold.ttf` | Pretendard (Latin UI text) | Kil Hyung-jin | The Redrob design system's `assets/Fonts/`, byte for byte (<https://github.com/orioncactus/pretendard>) | SIL OFL 1.1, [`assets/fonts/OFL-Pretendard.txt`](assets/fonts/OFL-Pretendard.txt) |
| `assets/fonts/WantedSans-Regular.ttf` | Wanted Sans (Korean UI text) | The Wanted Sans Project Authors | The Redrob design system's `assets/Fonts/`, byte for byte (<https://github.com/wanteddev/wanted-sans>) | SIL OFL 1.1, [`assets/fonts/WantedSans-OFL.txt`](assets/fonts/WantedSans-OFL.txt) |
