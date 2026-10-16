#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Build the committed art of the Windows setup window: installer/windows/res/lockup.png.

The lockup follows the design system's product-lockup rule (assets/Products/README.md): the Redrob
Primary lockup, white, untouched, with the product name "Launcher" after it in Pretendard SemiBold at
70% of the height, set 40% of the height after the wordmark, on the wordmark's baseline. Built at 400px
as the design system builds its lockups, then scaled to 68px: the window draws it 34 dip tall, so 68px
covers 200% displays. The same construction and constants as redrob-canvas's
scripts/build-brand-assets.py, so every Redrob setup window shows the same unit.

res/background.png is the deep ground with the brand wash-edge, res/background.html rasterised at
1320x800; it is shared with the other Redrob setups and committed as is.

Needs ImageMagick 7 (`magick`) and the design system delivery (default ../redrob-design-system).
"""
from __future__ import annotations

import argparse
import subprocess
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
DEFAULT_DS = REPO.parent / "redrob-design-system"
OUT = HERE / "res" / "lockup.png"
PRODUCT = "Launcher"
LOCKUP_HEIGHT = 400
SETUP_LOCKUP_HEIGHT = 68
NAME_BASELINE = 327 / 400
NAME_KERNING = -4.5 / 280
NAME_TAIL = 6


def magick(*args, capture=False):
    result = subprocess.run(["magick", *map(str, args)], check=True, capture_output=True)
    return result.stdout if capture else None


def compose_lockup(logo: Path, name: str, height: int, font: Path, out: Path, scratch: Path) -> None:
    mark = scratch / f"mark-{height}.png"
    magick(logo, "-filter", "Lanczos", "-resize", f"x{height}", "-strip",
           "-define", "png:exclude-chunks=date,time", f"PNG32:{mark}")
    mark_width = int(magick(mark, "-format", "%w", "info:", capture=True))
    size = 0.7 * height
    gap = round(0.4 * height)
    kerning = NAME_KERNING * size
    text = ["-font", font, "-pointsize", f"{size:g}", "-kerning", f"{kerning:g}", "-fill", "white"]
    canvas_width = mark_width + gap + int(size * len(name) * 1.2)
    magick(
        "-size", f"{canvas_width}x{height}", "xc:none",
        mark, "-geometry", "+0+0", "-composite",
        *text, "-annotate", f"+{mark_width + gap}+{round(NAME_BASELINE * height)}", name,
        "-define", "trim:edges=east", "-trim", "+repage",
        "-gravity", "east", "-background", "none", "-splice", f"{NAME_TAIL}x0", "+gravity",
        "-strip", "-define", "png:exclude-chunks=date,time", f"PNG32:{out}",
    )


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--design-system", default=str(DEFAULT_DS))
    args = ap.parse_args()
    design = Path(args.design_system)
    logo = design / "assets" / "Logos" / "redrob-logo-primary-white.png"
    font = design / "fonts" / "Pretendard-SemiBold.woff2"
    for f in (logo, font):
        if not f.exists():
            raise SystemExit(f"missing {f}; is the design system delivery at {design}?")
    with tempfile.TemporaryDirectory() as tmp:
        scratch = Path(tmp)
        full = scratch / "lockup-400.png"
        compose_lockup(logo, PRODUCT, LOCKUP_HEIGHT, font, full, scratch)
        OUT.parent.mkdir(parents=True, exist_ok=True)
        magick(full, "-filter", "Lanczos", "-resize", f"x{SETUP_LOCKUP_HEIGHT}", "-strip",
               "-define", "png:exclude-chunks=date,time", f"PNG32:{OUT}")
    print(f"wrote {OUT}")


if __name__ == "__main__":
    main()
