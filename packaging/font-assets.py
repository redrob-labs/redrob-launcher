#!/usr/bin/env python3
"""Generate the committed Latin-only Pretendard faces used by egui.

Requires fontTools 4.67.0 only when regenerating or checking assets; normal Rust
builds embed the committed outputs and have no generator dependency.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.metadata
import shutil
import tempfile
import urllib.request
from pathlib import Path

FONTTOOLS_VERSION = "4.67.0"
PRETENDARD_VERSION = "1.3.9"
ROOT = Path(__file__).resolve().parents[1]
FONT_DIR = ROOT / "assets" / "fonts"
SOURCE_BASE = (
    "https://raw.githubusercontent.com/orioncactus/pretendard/"
    f"v{PRETENDARD_VERSION}/packages/pretendard/dist/public/static/alternative"
)
SOURCES = {
    "Pretendard-Regular.ttf": "6d0af5258997aec7354a6e340fc2325ba321c410ca48b3af858c8c3d6e92a324",
    "Pretendard-Medium.ttf": "3bae579377eb8e9ac412cb4809ebc3de1d956ed75995c1e346f0c1311053f4e2",
    "Pretendard-SemiBold.ttf": "5e1c548732af70873103066c16e1369b9a8a871f0b38c321a1d5bc73e43cea2d",
}
EXPECTED_WEIGHTS = {
    "Pretendard-Regular.ttf": 400,
    "Pretendard-Medium.ttf": 500,
    "Pretendard-SemiBold.ttf": 600,
}
# Keep broad Latin language coverage and shared typography without retaining the source fonts'
# Hangul cmap. Shared punctuation stays in Pretendard so mixed Latin text is visually coherent.
LATIN_RANGES = (
    (0x0020, 0x02FF),  # Basic Latin through spacing modifier letters
    (0x0300, 0x036F),  # Combining diacritical marks
    (0x1D00, 0x1DFF),  # Phonetic extensions and combining supplement
    (0x1E00, 0x1EFF),  # Latin Extended Additional
    (0x2000, 0x218F),  # Punctuation, currency, letterlike symbols, and number forms
    (0x2460, 0x24FF),  # Enclosed alphanumerics
    (0x2C60, 0x2C7F),  # Latin Extended-C
    (0x2E00, 0x2E7F),  # Supplemental punctuation
    (0xA720, 0xA7FF),  # Latin Extended-D
    (0xAB30, 0xAB6F),  # Latin Extended-E
    (0xFF01, 0xFF5E),  # Fullwidth ASCII forms (not halfwidth Hangul)
    (0x10780, 0x107BF),  # Latin Extended-F
    (0x1DF00, 0x1DFFF),  # Latin Extended-G
)
HANGUL_RANGES = ((0x1100, 0x11FF), (0x3130, 0x318F), (0xAC00, 0xD7A3))


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def codepoints(ranges: tuple[tuple[int, int], ...]) -> set[int]:
    return {value for start, end in ranges for value in range(start, end + 1)}


def fetch_sources(destination: Path) -> None:
    destination.mkdir()
    for name, expected_hash in SOURCES.items():
        request = urllib.request.Request(
            f"{SOURCE_BASE}/{name}",
            headers={"User-Agent": "Redrob-Launcher/font-assets"},
        )
        with urllib.request.urlopen(request) as response:
            data = response.read()
        actual_hash = sha256(data)
        if actual_hash != expected_hash:
            raise RuntimeError(
                f"{name}: source SHA-256 is {actual_hash}, expected {expected_hash}"
            )
        (destination / name).write_bytes(data)


def generate(source_dir: Path, destination: Path) -> None:
    from fontTools import subset  # type: ignore[import-untyped]
    from fontTools.ttLib import TTFont  # type: ignore[import-untyped]

    destination.mkdir()
    allowed = codepoints(LATIN_RANGES)
    forbidden = codepoints(HANGUL_RANGES)
    for name, expected_weight in EXPECTED_WEIGHTS.items():
        source_path = source_dir / name
        source = TTFont(source_path, lazy=True)
        source_cmap = source.getBestCmap() or {}
        weight = source["OS/2"].usWeightClass
        source.close()
        if weight != expected_weight:
            raise RuntimeError(
                f"{name}: source weight is {weight}, expected {expected_weight}"
            )
        retained = allowed.intersection(source_cmap)
        if not set(range(0x20, 0x7F)).issubset(retained):
            raise RuntimeError(f"{name}: source does not contain printable ASCII")

        options = subset.Options()
        options.flavor = None
        options.layout_features = ["*"]
        options.name_IDs = ["*"]
        options.notdef_outline = True
        options.recalc_timestamp = False
        options.canonical_order = True
        font = subset.load_font(str(source_path), options)
        subsetter = subset.Subsetter(options)
        subsetter.populate(unicodes=retained)
        subsetter.subset(font)
        output_path = destination / name
        subset.save_font(font, str(output_path), options)

        check = TTFont(output_path, lazy=True, recalcTimestamp=False)
        output_cmap = check.getBestCmap() or {}
        output_weight = check["OS/2"].usWeightClass
        check.close()
        missing = sorted(retained.difference(output_cmap))
        claimed_hangul = sorted(forbidden.intersection(output_cmap))
        if missing:
            raise RuntimeError(f"{name}: generated face lost U+{missing[0]:04X}")
        if claimed_hangul:
            raise RuntimeError(
                f"{name}: generated face still claims U+{claimed_hangul[0]:04X}"
            )
        if output_weight != expected_weight:
            raise RuntimeError(
                f"{name}: generated weight is {output_weight}, expected {expected_weight}"
            )


def compare(left: Path, right: Path, description: str) -> None:
    for name in SOURCES:
        left_bytes = (left / name).read_bytes()
        right_bytes = (right / name).read_bytes()
        if left_bytes != right_bytes:
            raise RuntimeError(f"{name}: {description}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check", action="store_true", help="verify committed outputs and idempotence"
    )
    args = parser.parse_args()

    installed = importlib.metadata.version("fonttools")
    if installed != FONTTOOLS_VERSION:
        raise RuntimeError(
            f"fontTools {FONTTOOLS_VERSION} is required for deterministic output; found {installed}"
        )

    with tempfile.TemporaryDirectory(prefix="redrob-fonts-") as temporary:
        temp = Path(temporary)
        source_dir = temp / "source"
        first = temp / "first"
        fetch_sources(source_dir)
        generate(source_dir, first)
        if args.check:
            second = temp / "second"
            generate(source_dir, second)
            compare(first, second, "two generations differ")
            compare(first, FONT_DIR, "committed output is stale")
            print("Pretendard subsets are current and byte-for-byte reproducible")
        else:
            for name in SOURCES:
                shutil.copyfile(first / name, FONT_DIR / name)
                data = (FONT_DIR / name).read_bytes()
                print(
                    f"wrote assets/fonts/{name} ({len(data)} bytes, sha256 {sha256(data)})"
                )


if __name__ == "__main__":
    main()
