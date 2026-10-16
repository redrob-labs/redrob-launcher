# Brand sources

Verbatim copies of the Redrob design system's shipped files (`redrob-design-system/assets/`).
`SHA256SUMS` pins them, and `packaging/brand-assets.sh` derives everything under
`assets/app-icon/` and `assets/app-icons/` from these and nothing else.

| Files | From | Used for |
|---|---|---|
| `favicon-*.png`, `icon-192.png`, `icon-512.png`, `favicon.ico`, `favicon.svg` | `Favicons/` | The launcher's app icon and tray icon; the tile of every app without a product mark |
| `symbol-white-*.png` | `Favicons/` | The symbol alone on transparent, for a dark ground that is not Redrob Blue |
| `redrob-logo-primary.png`, `redrob-logo-primary-white.png` | `Logos/` | The header lockup: dark wordmark on light grounds, white wordmark on dark grounds |
| `redrob-{browser,code,design,office}-icon.svg` | `Products/` | The four products whose mark the design system ships |
| `redrob-{canvas,film,cad,shape,cowork,recall,query}-icon.svg` | here (see below) | The seven Labs products |

Rules that bind here (design system `10-logo.md`, `Logos/README.md`, `Products/README.md`):

- A white symbol on a Redrob Blue (`#2B52FF`) tile. One design, no dark-mode variant.
- Never redrawn, recolored, re-proportioned, given a shadow, border or second radius. Never a
  macOS template image: the system would recolor it.
- Each size is drawn for itself. Where the design system ships a size it is used as is; any other
  size is the next larger file resized down. Nothing is upscaled, so no icon here is larger than 512.
- The lockup variant is picked by ground; it is never tinted to fit one.

The Labs products (Canvas, Film, CAD, Shape, Cowork, Recall, Query) have no mark in the design
system yet. Their masters live here, built from the shipped files only: the `Products/` tile path
and the `Products/` white symbol, verbatim, on a tile color of their own. The doorway is left
empty; the tile color alone tells them apart. If the design system later ships a mark for one of
them, that file replaces the one here (update `SHA256SUMS`, rerun the script).

| File | Tile |
|---|---|
| `redrob-canvas-icon.svg` | #8944FF |
| `redrob-film-icon.svg` | #17181B |
| `redrob-cad-icon.svg` | #2F8DFF |
| `redrob-shape-icon.svg` | #C9A400 |
| `redrob-cowork-icon.svg` | #7FB800 |
| `redrob-recall-icon.svg` | #0F7F8F |
| `redrob-query-icon.svg` | #8C2F8C |
