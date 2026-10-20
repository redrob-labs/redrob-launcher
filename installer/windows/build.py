#!/usr/bin/env python3
# ADDED-LINE-RATIONALE 8154f317daca4470:1: Makes `Python / standard / library` an explicit part of `build` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE 0165cc7cbcab0589:1: Keeps `The / art / is` in `build`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE 094eb847836bf059:1: Makes `assets / brand / symbol` an explicit part of `build` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE b3e7a804e76f8aee:1: Makes `packaging / brand / assets` an explicit part of `build` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE a839956742388a89:1: Makes `Latin / and / Wanted` an explicit part of `build` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE 74d54c43d9f0cd54:1: Makes `allow / system / fonts` an explicit part of `build` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE 095ad8e724217097:1: Supports `build` while preserving the nearby constraint: The brand panel's mark: the design system's white symbol, verbatim (assets/brand/README.md).
# ADDED-LINE-RATIONALE 7609073d0240bcc9:1: Supports `vs_dev_env` while preserving the nearby constraint: Committed Pretendard and Wanted Sans sources; subsets are generated build artifacts (ATTRIBUTION.md).
# ADDED-LINE-RATIONALE 4c245051d0f52fec:1: Freezes `FONTTOOLS_VERSION` for `vs_dev_env` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 17b42cc098764088:1: Freezes `SOURCE` for `vs_dev_env` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 160073300076ca08:1: Isolates `ui_literal_characters` as the owner of this behavior so callers can use and test it without duplicating policy.
# ADDED-LINE-RATIONALE df7fff4021126ba9:1: Makes `Extract / wide / string` an explicit part of `ui_literal_characters` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE 506379ab537a8b85:1: Freezes `literals` for `ui_literal_characters` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 0f5e91410fb48e88:1: Freezes `characters` for `ui_literal_characters` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE d3307c80d7f36890:1: Applies `ui_literal_characters` consistently to every `literal / in / literals` item rather than handling only the first match.
# ADDED-LINE-RATIONALE cc9770b1e3d693cf:1: Freezes `i` for `ui_literal_characters` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE d475a79a7ca8adea:1: Applies `ui_literal_characters` consistently to every `i / len / literal` item rather than handling only the first match.
# ADDED-LINE-RATIONALE 4c97765ce584c684:1: Freezes `if` for `ui_literal_characters` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE e8d6be30f6404908:1: Freezes `characters` for `ui_literal_characters` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 3be86e5e2ba9539f:1: Freezes `elif` for `ui_literal_characters` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE b7fb27615e7d22c5:1: Freezes `characters` for `ui_literal_characters` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE f41f3f17fd08a339:1: Prevents `ui_literal_characters` from applying its side effect outside the `elif / i / len` state.
# ADDED-LINE-RATIONALE bd251ac00b566d98:1: Freezes `characters` for `ui_literal_characters` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE fc98ea2784cac6c2:1: Prevents `ui_literal_characters` from applying its side effect outside the `characters / add / n` state.
# ADDED-LINE-RATIONALE 938160c8eab56ac7:1: Freezes `i` for `ui_literal_characters` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 1c185d7f4a210d2d:1: Stops `ui_literal_characters` at the `characters` outcome so no later branch can overwrite that result.
# ADDED-LINE-RATIONALE 5f9b23221ba397ec:1: Isolates `is_hangul` as the owner of this behavior so callers can use and test it without duplicating policy.
# ADDED-LINE-RATIONALE 820e723a11580a84:1: Freezes `codepoint` for `is_hangul` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE cdcd96b66d232778:1: Freezes `return` for `is_hangul` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 3e11dd9cf799368f:1: Isolates `require_fonttools_version` as the owner of this behavior so callers can use and test it without duplicating policy.
# ADDED-LINE-RATIONALE 7171744428a7dc02:1: Makes `Require / the / pinned` an explicit part of `require_fonttools_version` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE fc11aa687ef5ffd3:1: Gives `require_fonttools_version` direct access to `fontTools / type / ignore` instead of relying on an implicit platform facility.
# ADDED-LINE-RATIONALE 0185377b520422ad:1: Makes `except / ImportError / as` an explicit part of `require_fonttools_version` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE fba027188a6def7d:1: Makes `require_fonttools_version` fail visibly when `raise / RuntimeError / f` violates the behavior this path promises.
# ADDED-LINE-RATIONALE 3f99cc4f718ed0b8:1: Freezes `installed` for `require_fonttools_version` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE f301ba464d1e5d49:1: Freezes `if` for `require_fonttools_version` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 809c918d38b7020f:1: Makes `require_fonttools_version` fail visibly when `raise / RuntimeError / f` violates the behavior this path promises.
# ADDED-LINE-RATIONALE e845ee4d2c612b6d:1: Isolates `subset_font` as the owner of this behavior so callers can use and test it without duplicating policy.
# ADDED-LINE-RATIONALE 5f31db9bb981ba28:1: Makes `Create / one / deterministic` an explicit part of `subset_font` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE 9ffb2ad51f665398:3: Makes `try` an explicit part of `subset_font` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE 734f0ba423cc1377:1: Gives `subset_font` direct access to `fontTools / subset / type` instead of relying on an implicit platform facility.
# ADDED-LINE-RATIONALE 0185377b520422ad:2: Makes `except / ImportError / as` an explicit part of `subset_font` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE a55c4e8f303bee40:1: Makes `subset_font` fail visibly when `raise / RuntimeError / fontTools` violates the behavior this path promises.
# ADDED-LINE-RATIONALE 17762425594916cc:1: Prevents `subset_font` from applying its side effect outside the `not / source / is_file` state.
# ADDED-LINE-RATIONALE 49dfe1b1bd34bbfc:1: Makes `subset_font` fail visibly when `raise / RuntimeError / f` violates the behavior this path promises.
# ADDED-LINE-RATIONALE a5b55692d1a8db53:1: Freezes `options` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE f4a1ffe4e0336474:1: Freezes `options` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 6bdd12bade4fa364:1: Freezes `options` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 6e44830bada0342c:1: Freezes `options` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE d1d23afdd273ef49:1: Freezes `options` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 0f67f8eb7322ab66:1: Freezes `options` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 84ac1c63c29087d3:1: Freezes `options` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 53e89d0ff963001f:1: Freezes `font` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 6c0ae610092a653b:1: Freezes `subsetter` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 0289efb0c2eb576a:1: Freezes `subsetter` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 6d1d0f4ba9805316:1: Delegates `subset_font`'s `subsetter / subset / font` work to `subsetter.subset` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE c6d4191271b38607:1: Delegates `subset_font`'s `subset / save_font / font` work to `subset.save_font` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 8a8ccad764100d97:1: Supports `subset_font` while preserving the nearby constraint: Cmap validation prevents a malformed subset from becoming a silent system-font fallback.
# ADDED-LINE-RATIONALE 7676fcdeaa5906ca:1: Freezes `cmap` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 4da753a66a55f4aa:1: Freezes `missing` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 73ff907778f56d3a:1: Delegates `subset_font`'s `check / close` work to `check.close` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 1222ddc5ab151fc0:1: Prevents `subset_font` from applying its side effect outside the `missing` state.
# ADDED-LINE-RATIONALE c3c37b2dcb6850cc:1: Freezes `out_ttf` for `subset_font` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE b34f1ba80e203547:1: Makes `subset_font` fail visibly when `raise / RuntimeError / f` violates the behavior this path promises.
# ADDED-LINE-RATIONALE 79cc1fd5828e8d6c:1: Isolates `prepare_fonts` as the owner of this behavior so callers can use and test it without duplicating policy.
# ADDED-LINE-RATIONALE b9161e3e514ab52f:1: Makes `Build / required / Pretendard` an explicit part of `prepare_fonts` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE 729003f034ab5893:1: Delegates `prepare_fonts`'s `require_fonttools_version` work to `require_fonttools_version` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 82a135c1144d3f70:1: Freezes `ui` for `prepare_fonts` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE d98c3380c1de0860:1: Freezes `latin` for `prepare_fonts` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 26547e0dc7274f0f:1: Freezes `hangul` for `prepare_fonts` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 8c7b9a6b3efc9a52:1: Prevents `prepare_fonts` from applying its side effect outside the `not / hangul` state.
# ADDED-LINE-RATIONALE 3e1b4943ee5a87c4:1: Makes `prepare_fonts` fail visibly when `raise / RuntimeError / no` violates the behavior this path promises.
# ADDED-LINE-RATIONALE d241d106eee163b7:1: Freezes `outputs` for `prepare_fonts` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 017af601d4ab2f4c:1: Makes `out / WantedSans / Regular` an explicit part of `prepare_fonts` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE dcda66fc2735a1cc:1: Delegates `prepare_fonts`'s `subset_font / FONTS / Pretendard` work to `subset_font` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE fd6871e56a60d7ce:1: Delegates `prepare_fonts`'s `subset_font / FONTS / Pretendard` work to `subset_font` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 81ff31e70d79b19d:1: Delegates `prepare_fonts`'s `subset_font / FONTS / WantedSans` work to `subset_font` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 3edd9fe3c6d32f99:1: Stops `prepare_fonts` at the `outputs` outcome so no later branch can overwrite that result.
# ADDED-LINE-RATIONALE 65badf09f6ad4a58:1: Keeps `f / define / REDROB_SYMBOL_PATH` in `fwd`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE e42f0740280ee953:1: Delegates `fwd`'s `lines / extend` work to `lines.extend` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE e329bd941164cc20:1: Keeps `f / define / REDROB_FONT_SEMIBOLD_PATH` in `fwd`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE b749d971e964ce70:1: Keeps `f / define / REDROB_FONT_REGULAR_PATH` in `fwd`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE 00ee2c044404b02a:1: Keeps `f / define / REDROB_FONT_HANGUL_PATH` in `fwd`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE 79c118aea1b05f4a:1: Keeps `define / REDROB_REQUIRE_BUNDLED_FONTS` in `fwd`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE 41734d1d765923cf:1: Makes `define / REDROB_REQUIRE_BUNDLED_FONTS` an explicit part of `fwd` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE 6393745b2d2a4f62:1: Prevents `fwd` from applying its side effect outside the `Only / an / explicit` state.
# ADDED-LINE-RATIONALE 276c49d5d441f655:1: Supports `fwd` while preserving the nearby constraint: Only an explicit developer build may compile the documented system-font fallback.
# ADDED-LINE-RATIONALE 6a51f73181193d0a:1: Freezes `ap` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE bb89b085fab136ca:1: Freezes `ap` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE d1f956021eafb062:1: Freezes `help` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 50ede7cf8ec91877:1: Applies `main` consistently to every `f / in / SYMBOL_PNG` item rather than handling only the first match.
# ADDED-LINE-RATIONALE 91a4352cf82d784b:1: Delegates `main`'s `sys / exit / f` work to `sys.exit` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 9ffb2ad51f665398:4: Makes `try` an explicit part of `main` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE f5d8f669674262d7:1: Freezes `font` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE f94c231c28158ba2:1: Makes `except / RuntimeError / as` an explicit part of `main` because the native Windows installer remains accessible, deterministic, and warning-clean.
# ADDED-LINE-RATIONALE 619aaeb2bd1e0914:1: Delegates `main`'s `sys / exit / f` work to `sys.exit` so existing state and error handling remain centralized.
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Build the Redrob Launcher Windows setup (and its uninstaller).

Python standard library plus fontTools. It:
  1. locates Visual Studio (vswhere) and runs rc.exe / cl.exe under its developer environment for the
     requested --arch (x64, x86 or arm64; the x86 and arm64 compilers are cross compilers hosted on x64);
  2. compiles redrob_launcher_setup.rc + redrob_launcher_setup.cpp twice:
       uninstall/Uninstall Redrob Launcher.exe   no payload; this IS the uninstaller (--uninstall)
       <name>                                    embeds the release zip and the uninstaller above

Usage:
  build.py --out <dir> --arch x64 --payload <redrob-launcher-X-windows-x64-portable.zip> --version 0.4.0 [--name <exe>]
  build.py --out <dir> --version 0.4.0          # demo-only build (REDROB_NO_PAYLOAD): --demo flags only

The payload is the release's own -portable.zip (packaging/windows/package.ps1: redrob-launcher.exe and
portable.txt at the top level), already Authenticode-signed inside, so the setup must be built after signing
and then signed itself (release workflow, Windows jobs). The setup drops portable.txt when it installs.

The art is drawn by the window itself (the brand panel's gradient) around the design system's white symbol,
assets/brand/symbol-white-192.png, embedded as is; assets/app-icon/redrob-launcher.ico comes from
packaging/brand-assets.sh. The window embeds deterministic subsets from committed assets: Pretendard for
Latin and Wanted Sans for Hangul. Production builds fail when fontTools, a source font, a requested glyph, or a resource is missing;
`--allow-system-fonts` is an explicit developer-only escape hatch.

Adapted from redrob-labs/redrob-canvas installer/windows/build.py (our own code).
"""
from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
ICON = REPO / "assets" / "app-icon" / "redrob-launcher.ico"
# The brand panel's mark: the design system's white symbol, verbatim (assets/brand/README.md).
SYMBOL_PNG = REPO / "assets" / "brand" / "symbol-white-192.png"


def find_vswhere() -> Path:
    pf = os.environ.get("ProgramFiles(x86)", r"C:\Program Files (x86)")
    vswhere = Path(pf) / "Microsoft Visual Studio" / "Installer" / "vswhere.exe"
    if vswhere.exists():
        return vswhere
    found = shutil.which("vswhere")
    if found:
        return Path(found)
    sys.exit("vswhere.exe not found; install Visual Studio with the C++ x64 toolset.")


# vcvarsall.bat's name for each target, hosted on an x64 machine.
VCVARS_ARCH = {"x64": "x64", "x86": "x64_x86", "arm64": "x64_arm64"}


def vs_dev_env(arch: str) -> dict:
    """The environment of a developer command prompt for `arch` (x64, x86 or arm64; x64 host)."""
    vsinstall = subprocess.run(
        [str(find_vswhere()), "-latest", "-products", "*",
         "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
         "-property", "installationPath"],
        capture_output=True, text=True, check=True).stdout.strip()
    if not vsinstall:
        sys.exit("No Visual Studio install with the C++ x64 toolset found.")
    vcvars = Path(vsinstall) / "VC" / "Auxiliary" / "Build" / "vcvarsall.bat"
    if not vcvars.exists():
        sys.exit(f"vcvarsall.bat missing under {vsinstall}")
    # vcvars can leave a non-zero errorlevel on success, so key off a marker and a sane env instead.
    # A real batch file carries vcvars' variables into `set`; an inline `cmd /c "call ... & set"` does not.
    marker = "__REDROB_ENV__"
    bat = Path(tempfile.gettempdir()) / "_redrob_launcher_vcvars.bat"
    bat.write_text(f'@echo off\r\ncall "{vcvars}" {VCVARS_ARCH[arch]}\r\necho {marker}\r\nset\r\n', encoding="ascii")
    try:
        out = subprocess.run(["cmd", "/c", str(bat)], capture_output=True, text=True, errors="replace").stdout
    finally:
        bat.unlink(missing_ok=True)
    env, seen = {}, False
    for line in out.splitlines():
        if line.strip() == marker:
            seen = True
        elif seen and "=" in line:
            k, v = line.split("=", 1)
            env[k] = v
    keys = {k.upper() for k in env}
    if "PATH" not in keys or "INCLUDE" not in keys:
        sys.exit("vcvarsall did not produce a developer environment (no INCLUDE/PATH).")
    env.setdefault("SystemRoot", os.environ.get("SystemRoot", r"C:\Windows"))
    return env


# Committed Pretendard and Wanted Sans sources; subsets are generated build artifacts (ATTRIBUTION.md).
FONTS = REPO / "assets" / "fonts"
FONTTOOLS_VERSION = "4.67.0"
SOURCE = HERE / "redrob_launcher_setup.cpp"


def ui_literal_characters() -> set[str]:
    """Extract wide-string contents so comments cannot perturb deterministic subsets."""
    literals = re.findall(r'L"((?:\\.|[^"\\])*)"', SOURCE.read_text(encoding="utf-8"))
    characters: set[str] = set()
    for literal in literals:
        i = 0
        while i < len(literal):
            if literal[i] != "\\":
                characters.add(literal[i]); i += 1
            elif i + 5 < len(literal) and literal[i + 1] == "u":
                characters.add(chr(int(literal[i + 2:i + 6], 16))); i += 6
            elif i + 1 < len(literal):
                characters.add({"n": "\n", "r": "\r", "t": "\t"}.get(literal[i + 1], literal[i + 1])); i += 2
            else:
                i += 1
    return characters


def is_hangul(character: str) -> bool:
    codepoint = ord(character)
    return 0x1100 <= codepoint <= 0x11FF or 0x3130 <= codepoint <= 0x318F or 0xAC00 <= codepoint <= 0xD7A3


def require_fonttools_version() -> None:
    """Require the pinned release because fontTools changes can alter otherwise deterministic output."""
    try:
        import fontTools  # type: ignore
    except ImportError as exc:
        raise RuntimeError(f"fontTools {FONTTOOLS_VERSION} is required for bundled installer fonts") from exc
    installed = getattr(fontTools, "__version__", None)
    if installed != FONTTOOLS_VERSION:
        raise RuntimeError(f"fontTools {FONTTOOLS_VERSION} is required, found {installed or 'unknown'}")


def subset_font(source: Path, out_ttf: Path, characters: set[str]) -> None:
    """Create one deterministic subset and fail if any visible requested glyph was lost."""
    try:
        from fontTools import subset  # type: ignore
        from fontTools.ttLib import TTFont  # type: ignore
    except ImportError as exc:
        raise RuntimeError("fontTools is required for bundled installer fonts") from exc
    if not source.is_file():
        raise RuntimeError(f"missing committed font {source}")
    options = subset.Options()
    options.flavor = None
    options.layout_features = ["*"]
    options.name_IDs = ["*"]
    options.notdef_outline = True
    options.recalc_timestamp = False
    options.canonical_order = True
    font = subset.load_font(str(source), options)
    subsetter = subset.Subsetter(options)
    subsetter.populate(text="".join(sorted(characters)))
    subsetter.subset(font)
    subset.save_font(font, str(out_ttf), options)
    # Cmap validation prevents a malformed subset from becoming a silent system-font fallback.
    check = TTFont(str(out_ttf), recalcTimestamp=False)
    cmap = check.getBestCmap() or {}
    missing = sorted(ord(char) for char in characters if char >= " " and ord(char) not in cmap)
    check.close()
    if missing:
        out_ttf.unlink(missing_ok=True)
        raise RuntimeError(f"{source.name} is missing " + ", ".join(f"U+{value:04X}" for value in missing[:12]))


def prepare_fonts(out: Path) -> tuple[Path, Path, Path]:
    """Build required Pretendard Latin and Wanted Sans Hangul resources from committed assets."""
    require_fonttools_version()
    ui = ui_literal_characters()
    latin = {chr(codepoint) for codepoint in range(0x20, 0x7F)} | {char for char in ui if char >= " " and not is_hangul(char)}
    hangul = {char for char in ui if is_hangul(char)}
    if not hangul:
        raise RuntimeError("no Hangul UI characters found")
    outputs = (out / "Pretendard-SemiBold.subset.ttf", out / "Pretendard-Regular.subset.ttf",
               out / "WantedSans-Regular.subset.ttf")
    subset_font(FONTS / "Pretendard-SemiBold.ttf", outputs[0], latin)
    subset_font(FONTS / "Pretendard-Regular.ttf", outputs[1], latin)
    subset_font(FONTS / "WantedSans-Regular.ttf", outputs[2], hangul)
    return outputs


def run_in_env(cmd: list, env: dict, cwd: str) -> None:
    """CreateProcess resolves the program on the PARENT's PATH, so resolve it on the dev env's first."""
    path = next((v for k, v in env.items() if k.upper() == "PATH"), None)
    resolved = shutil.which(cmd[0], path=path)
    if resolved:
        cmd = [resolved, *cmd[1:]]
    print("$", " ".join(cmd), flush=True)
    subprocess.run(cmd, env=env, check=True, cwd=cwd)


def compile_app(env, out: Path, exe_name: str, font, ver: str, payload, uninstaller, machine: str = "X64") -> Path:
    """rc + cl + link into out/exe_name. Paths reach RC through a generated header with forward slashes:
    RC's preprocessor reads backslashes in string macros as escapes."""
    def fwd(p) -> str:
        return str(p).replace("\\", "/")

    out.mkdir(parents=True, exist_ok=True)
    parts = (ver.split(".") + ["0", "0", "0", "0"])[:4]
    lines = [
        "// generated by build.py -- do not edit",
        f'#define REDROB_SYMBOL_PATH "{fwd(SYMBOL_PNG)}"',
        f'#define REDROB_ICON_PATH "{fwd(ICON)}"',
        f"#define REDROB_VER_FILE {','.join(parts)}",
        f'#define REDROB_VER_STR "{ver}"',
        f'#define REDROB_VER_WSTR L"{ver}"',
        f'#define REDROB_SETUP_KIND "{"Uninstaller" if payload is None else "Setup"}"',
    ]
    if font:
        lines.extend([
            f'#define REDROB_FONT_SEMIBOLD_PATH "{fwd(font[0])}"',
            f'#define REDROB_FONT_REGULAR_PATH "{fwd(font[1])}"',
            f'#define REDROB_FONT_HANGUL_PATH "{fwd(font[2])}"',
            "#define REDROB_REQUIRE_BUNDLED_FONTS 1",
        ])
    else:
        # Only an explicit developer build may compile the documented system-font fallback.
        lines.append("#define REDROB_ALLOW_SYSTEM_FONTS 1")
    if payload:
        lines.append(f'#define REDROB_PAYLOAD_PATH "{fwd(payload)}"')
    else:
        lines.append("#define REDROB_NO_PAYLOAD 1")
    if uninstaller:
        lines.append(f'#define REDROB_UNINSTALLER_PATH "{fwd(uninstaller)}"')
    (out / "build_defs.h").write_text("\n".join(lines) + "\n", encoding="ascii")

    res = out / "redrob_launcher_setup.res"
    run_in_env(["rc.exe", "/nologo", f"/I{out}", f"/I{HERE}", f"/fo{res}", str(HERE / "redrob_launcher_setup.rc")],
               env, str(HERE))
    exe = out / exe_name
    obj = out / "redrob_launcher_setup.obj"
    run_in_env(["cl.exe", "/nologo", "/EHsc", "/O2", "/W3", "/WX", "/MT", "/std:c++17", "/utf-8",
                "/DUNICODE", "/D_UNICODE", f"/I{out}", f"/Fo{obj}", str(HERE / "redrob_launcher_setup.cpp"),
                "/link", "/SUBSYSTEM:WINDOWS", "/MANIFEST:NO", f"/MACHINE:{machine}", str(res), f"/OUT:{exe}"],
               env, str(out))
    if not exe.exists():
        sys.exit(f"link produced no {exe_name}")
    return exe


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--payload", help="the release's Windows zip (omit for a demo-only build)")
    ap.add_argument("--version", required=True, help="e.g. 0.4.0")
    ap.add_argument("--arch", default="x64", choices=sorted(VCVARS_ARCH), help="target architecture (default x64)")
    ap.add_argument("--name", default="Redrob Launcher Setup.exe", help="output file name of the setup")
    ap.add_argument("--allow-system-fonts", action="store_true",
                    help="developer-only escape hatch; production builds must embed the committed fonts")
    ap.add_argument("--uninstaller-only", action="store_true",
                    help="build only <out>/uninstall/Uninstall Redrob Launcher.exe, so it can be signed first")
    ap.add_argument("--uninstaller", help="an already built (and signed) uninstaller to embed, instead of "
                                          "building one")
    args = ap.parse_args()

    if not re.fullmatch(r"\d+\.\d+\.\d+", args.version):
        sys.exit(f"--version must be X.Y.Z, got {args.version!r}")
    for f in (SYMBOL_PNG, ICON):
        if not f.exists():
            sys.exit(f"missing committed asset {f}; see assets/brand/README.md / packaging/brand-assets.sh")

    out = Path(args.out).resolve()
    out.mkdir(parents=True, exist_ok=True)
    env = vs_dev_env(args.arch)
    machine = {"x64": "X64", "x86": "X86", "arm64": "ARM64"}[args.arch]
    try:
        font = None if args.allow_system_fonts else prepare_fonts(out)
    except RuntimeError as exc:
        sys.exit(f"font preparation failed: {exc}")

    if args.uninstaller_only:
        exe = compile_app(env, out / "uninstall", "Uninstall Redrob Launcher.exe", font, args.version,
                          payload=None, uninstaller=None, machine=machine)
    elif args.payload:
        payload = Path(args.payload).resolve()
        if not payload.is_file():
            sys.exit(f"payload not found: {payload}")
        if args.uninstaller:
            uninstaller = Path(args.uninstaller).resolve()
            if not uninstaller.is_file():
                sys.exit(f"uninstaller not found: {uninstaller}")
        else:
            uninstaller = compile_app(env, out / "uninstall", "Uninstall Redrob Launcher.exe", font, args.version,
                                      payload=None, uninstaller=None, machine=machine)
        exe = compile_app(env, out, args.name, font, args.version, payload=payload, uninstaller=uninstaller, machine=machine)
    else:
        exe = compile_app(env, out, args.name, font, args.version, payload=None, uninstaller=None, machine=machine)
    print(f"\nbuilt {exe}  {exe.stat().st_size // 1024} KB  (payload: {'yes' if args.payload else 'none'})", flush=True)


if __name__ == "__main__":
    main()
