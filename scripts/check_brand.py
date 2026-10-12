#!/usr/bin/env python3
"""Brand gate: the shipped brand is Redrob's, exactly as the design system ships it.

1. Every file in assets/brand/ is listed in SHA256SUMS and matches it (no edited or extra copies).
2. No upstream name, link or service appears on a user-facing surface: the UI and backend sources
   outside their test modules, the translation catalogs, and the packaging files.

Usage: python3 scripts/check_brand.py   (exit 0 = clean)
"""

from __future__ import annotations

import hashlib
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BRAND = os.path.join(ROOT, "assets", "brand")
# Upstream's product, company, site, services and community. Matched case-insensitively.
FORBIDDEN = re.compile(r"artcraft|getartcraft|storyteller|storytold|craft app|craft-common-actions|discord\.gg|learning machines")
SURFACES = ("apps/", "crates/", "packaging/")


def brand_errors() -> list[str]:
    errors = []
    sums = {}
    with open(os.path.join(BRAND, "SHA256SUMS"), encoding="utf-8") as fh:
        for line in fh:
            digest, name = line.split()
            sums[name] = digest
    for name in sorted(os.listdir(BRAND)):
        if name in ("SHA256SUMS", "README.md"):
            continue
        if name not in sums:
            errors.append(f"assets/brand/{name} is not pinned in SHA256SUMS")
            continue
        with open(os.path.join(BRAND, name), "rb") as fh:
            if hashlib.sha256(fh.read()).hexdigest() != sums[name]:
                errors.append(f"assets/brand/{name} differs from its pinned design-system file")
    for name in sums:
        if not os.path.exists(os.path.join(BRAND, name)):
            errors.append(f"assets/brand/{name} is pinned but missing")
    return errors


def surface_text(path: str, text: str) -> str:
    """The part of a file a user can see: Rust files lose their `#[cfg(test)]` module and comments."""
    if not path.endswith(".rs"):
        return text
    cut = text.find("#[cfg(test)]\nmod tests")
    if cut >= 0:
        text = text[:cut]
    return "\n".join(line for line in text.splitlines() if not line.lstrip().startswith("//"))


def surface_errors() -> list[str]:
    files = subprocess.run(["git", "ls-files", "-z", "--", *SURFACES], cwd=ROOT, check=True, capture_output=True).stdout.decode().split("\0")
    errors = []
    for path in filter(None, files):
        if "/tests/" in path or path.endswith("i18n/tests.rs"):
            continue
        try:
            text = open(os.path.join(ROOT, path), encoding="utf-8").read()
        except (UnicodeDecodeError, FileNotFoundError):
            continue
        for n, line in enumerate(surface_text(path, text).splitlines(), 1):
            if FORBIDDEN.search(line.lower()):
                errors.append(f"{path}: upstream name on a user-facing surface: {line.strip()[:120]}")
    return errors


def main() -> int:
    errors = brand_errors() + surface_errors()
    for e in errors:
        print(f"error: {e}", file=sys.stderr)
    if errors:
        return 1
    print("brand ok: assets/brand matches SHA256SUMS, no upstream names on user-facing surfaces")
    return 0


if __name__ == "__main__":
    sys.exit(main())
