#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Build the Redrob Launcher Windows setup (and its uninstaller).

Standard library only. It:
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

The art is committed, not rendered here: res/background.png is res/background.html rasterised at 1320x800
(shared with the other Redrob setups); res/lockup.png is built by build-art.py from the design system's
Redrob Primary lockup and the product name; assets/app-icon/redrob-launcher.ico comes from
packaging/brand-assets.sh. Pretendard is embedded when the design system delivery is present
(--design-system, default ../redrob-design-system) and fontTools is installed; otherwise the window uses
Segoe UI Variable (DirectWrite falls back to a system font for Hangul).

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
RES = HERE / "res"
REPO = HERE.parent.parent
DEFAULT_DS = REPO.parent / "redrob-design-system"
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


FONTS = REPO / "assets" / "fonts"  # the design system's Pretendard TTFs, byte for byte (ATTRIBUTION.md)
SOURCE = HERE / "redrob_launcher_setup.cpp"


def prepare_font(design_system: Path, out_ttf: Path, weight: str = "SemiBold") -> bool:
    """Pretendard `weight` as a TTF for AddFontMemResourceEx / DirectWrite, cut down to the characters the
    window can draw: printable ASCII plus every other character in redrob_launcher_setup.cpp (all of its
    text is in that file). The source is the repo's own assets/fonts TTF, else the design system's woff2.
    Needs fontTools; without it the window uses Segoe UI Variable."""
    source = FONTS / f"Pretendard-{weight}.ttf"
    if not source.exists():
        source = design_system / "fonts" / f"Pretendard-{weight}.woff2"
    if not source.exists():
        return False
    try:
        from fontTools import subset  # type: ignore
    except Exception:
        print("note: fontTools not available; the window uses Segoe UI Variable.")
        return False
    text = "".join(chr(c) for c in range(0x20, 0x7F))
    text += "".join(sorted({ch for ch in SOURCE.read_text(encoding="utf-8") if ord(ch) > 0x7E}))
    options = subset.Options()
    options.flavor = None
    options.layout_features = ["*"]
    options.name_IDs = ["*"]
    options.notdef_outline = True
    font = subset.load_font(str(source), options)
    subsetter = subset.Subsetter(options)
    subsetter.populate(text=text)
    subsetter.subset(font)
    subset.save_font(font, str(out_ttf), options)
    return out_ttf.exists()


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
        lines.append(f'#define REDROB_FONT_PATH "{fwd(font[0])}"')
        if font[1]:
            lines.append(f'#define REDROB_FONT_BODY_PATH "{fwd(font[1])}"')
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
    ap.add_argument("--payload", help="the release's windows zip (omit for a demo-only build)")
    ap.add_argument("--version", required=True, help="e.g. 0.4.0")
    ap.add_argument("--arch", default="x64", choices=sorted(VCVARS_ARCH), help="target architecture (default x64)")
    ap.add_argument("--name", default="Redrob Launcher Setup.exe", help="output file name of the setup")
    ap.add_argument("--design-system", default=str(DEFAULT_DS))
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
    semibold, regular = out / "Pretendard-SemiBold.ttf", out / "Pretendard-Regular.ttf"
    font = None
    if prepare_font(Path(args.design_system), semibold, "SemiBold"):
        font = (semibold, regular if prepare_font(Path(args.design_system), regular, "Regular") else None)

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
