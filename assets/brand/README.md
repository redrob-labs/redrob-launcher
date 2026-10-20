# Brand sources

Verbatim copies of the Redrob design system's shipped files. `SHA256SUMS` pins them, and
`packaging/brand-assets.sh` derives everything under `assets/app-icon/` and `assets/app-icons/`
from these and nothing else.

| Files | From | Used for |
|---|---|---|
| `redrob-icon-web.svg` | `@redrob-labs/ui@1.8.0` `dist/brand/app-icons/redrob/` | The launcher's app icon and tray icon (the Redrob group icon, 2026, `icon-web` variant: superellipse, transparent corners) |
| `redrob-<app>-icon-web.svg` | `@redrob-labs/ui@1.8.0` `dist/brand/app-icons/redrob-<app>/` | Each app's icon in the launcher (2026, `icon-web` variant: superellipse, transparent corners) |
| `symbol-white-*.png` | design system `Favicons/` | The symbol alone on transparent, for a dark ground that is not Redrob Blue |
| `redrob-logo-primary.png`, `redrob-logo-primary-white.png` | design system `Logos/` | The header lockup: dark wordmark on light grounds, white wordmark on dark grounds |

Rules that bind here (design system `10-logo.md`, `Logos/README.md`, `app-icons/README.md`):

- Never redrawn, recolored, re-proportioned, given a shadow, border or second radius. Never a
  macOS template image: the system would recolor it.
- A new icon comes as a new `@redrob-labs/ui` delivery: copy the SVG byte for byte, check its
  sha256 against the package's `manifest.json`, update `SHA256SUMS` and rerun the script.
- The lockup variant is picked by ground; it is never tinted to fit one.
- Only apps with a 2026 icon are in the catalog (`crates/core/src/catalog.rs`).

The Film, Shape, CAD and Canvas icons are marked "not yet" approved in the 1.8.0 delivery.

| Source | sha256 (`manifest.json`) |
|---|---|
| `redrob-icon-web.svg` | `33a664b1f70f44006f19f684026728b003db5e72c6cb40b732cb43d9da229589` |
| `redrob-browser-icon-web.svg` | `1ce12193746cdeb767c058300dc9f3e7756ea187f53a8f2671480e620c91db8e` |
| `redrob-cad-icon-web.svg` | `664ff7cd83c8e1667dd7c41a7542a061c7d4e8d5196b676ad6f7072d598a85a2` |
| `redrob-canvas-icon-web.svg` | `1c0878b003927a459a365db32ba6e4c30959eb822d62335590f36ddbb83f07e2` |
| `redrob-code-icon-web.svg` | `0a9840bb44ac1307014d59b9e33e279b1568cbf51137745a437d7c4fbd755dcc` |
| `redrob-cowork-icon-web.svg` | `e2b985d0b238d6f33bfba9ad9c4ce84a7139939f448212c53305216635e5a7a0` |
| `redrob-design-icon-web.svg` | `97f6c7120409bcc9455475a3cd7e73af0d2171440433a81953cbaf1a65dac384` |
| `redrob-film-icon-web.svg` | `ebf56534f9ec6ad0c818b83ac91fb66a093acc3854187f444b78ef5dbff70bd1` |
| `redrob-office-icon-web.svg` | `5351591c18bc4e938558f0293f570da16da36c502bbfec63de557fc71f44c398` |
| `redrob-shape-icon-web.svg` | `219ac68e2c37a40fb3e63b93312ad239fe9a11def551ca3f86cc7b8d58d06e6a` |
