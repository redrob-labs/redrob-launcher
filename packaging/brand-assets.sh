#!/usr/bin/env bash
# Derive every icon under assets/app-icon/ (the launcher's own) and assets/app-icons/ (each app's)
# from the pinned design-system sources in assets/brand/ and nothing else. Needs ImageMagick 7
# (`magick`), resvg (`cargo install resvg`) and cargo. Runs on any OS; the outputs are committed,
# so builds and packaging never need these tools.
#
#   packaging/brand-assets.sh
#
# What is derived, and from what (assets/brand/README.md has the rules):
#   app-icon/hicolor/<size>              redrob-icon-web.svg (the 2026 group icon, rounded) rendered at that size
#   app-icon/hicolor/scalable            redrob-icon-web.svg, as is
#   app-icon/redrob-launcher.ico         16, 32, 48, 64, 256
#   app-icon/redrob-launcher.icns        16, 32, 64, 128, 256, 512
#   app-icon/redrob-logo-{light,dark}.png  the lockups resized down to 64 px tall (header)
#   app-icons/<app>.png                  128 px: the app's 2026 icon-web SVG rendered
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
B="$ROOT/assets/brand"
A="$ROOT/assets/app-icon"
APPS="$ROOT/assets/app-icons"
ID="ai.redrob.launcher"
command -v magick >/dev/null || { echo "error: magick (ImageMagick 7) not found" >&2; exit 1; }
command -v resvg >/dev/null || { echo "error: resvg not found (cargo install resvg)" >&2; exit 1; }

(cd "$B" && sha256sum -c --quiet SHA256SUMS)

rm -rf "$A" "$APPS"
mkdir -p "$A" "$APPS"
for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$A/hicolor/${s}x${s}/apps"
  resvg -w "$s" -h "$s" "$B/redrob-icon-web.svg" "$A/hicolor/${s}x${s}/apps/$ID.png"
done
mkdir -p "$A/hicolor/scalable/apps"
cp "$B/redrob-icon-web.svg" "$A/hicolor/scalable/apps/$ID.svg"

H="$A/hicolor"
(cd "$ROOT" && cargo run -q -p xtask -- ico "$A/redrob-launcher.ico" \
  "$H/16x16/apps/$ID.png" "$H/32x32/apps/$ID.png" "$H/48x48/apps/$ID.png" "$H/64x64/apps/$ID.png" \
  "$H/256x256/apps/$ID.png")
(cd "$ROOT" && cargo run -q -p xtask -- icns "$A/redrob-launcher.icns" \
  "$H/16x16/apps/$ID.png" "$H/32x32/apps/$ID.png" "$H/64x64/apps/$ID.png" \
  "$H/128x128/apps/$ID.png" "$H/256x256/apps/$ID.png" "$H/512x512/apps/$ID.png")

# The header lockup, by ground. 64 px tall covers an 18 pt header at up to 3.5x.
magick "$B/redrob-logo-primary.png" -filter Lanczos -resize x64 "PNG32:$A/redrob-logo-light.png"
magick "$B/redrob-logo-primary-white.png" -filter Lanczos -resize x64 "PNG32:$A/redrob-logo-dark.png"

# Each app's 128 px icon, rendered from its 2026 icon-web SVG (superellipse, transparent corners).
apps="redrob-browser redrob-cad redrob-canvas redrob-code redrob-cowork redrob-design redrob-film redrob-office redrob-shape"
for id in $apps; do resvg -w 128 -h 128 "$B/$id-icon-web.svg" "$APPS/$id.png"; done

echo "brand assets written to $A and $APPS"
