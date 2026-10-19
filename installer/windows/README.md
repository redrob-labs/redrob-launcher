# Redrob Launcher: Windows setup

`redrob_launcher_setup.cpp` is the branded Windows setup and its uninstaller, in one small native Win32 app
(Direct2D, DirectWrite, WIC; nothing beyond the Windows SDK). It is the Redrob Canvas setup
(`redrob-labs/redrob-canvas`, `installer/windows/`, our own code, itself from the Redrob Browser setup) with
the launcher's own screens and install steps. It replaced the WiX MSI in 0.4.0: one wizard look across the
Redrob apps, a language page, an `.exe` that can carry the Redrob icon and build one SmartScreen reputation.

## What it does

- **Language first.** The first screen is bilingual and offers English / 한국어; every screen after it, the
  window title included, follows the choice. It is preselected from a previous install's answer, else
  Korean on a Korean Windows, else English (`--lang en|ko` overrides). The answer goes to
  `HKCU\Software\Redrob\Redrob Launcher\Language`; the launcher adopts it on its first run and publishes it
  for every Redrob app (`docs/shared-language.md`).
- **Install**, per user, no administrator rights, into `%LOCALAPPDATA%\Programs\Redrob Launcher` (the MSI's
  folder, so an update over an MSI install lands in place). The payload is the release's own signed
  `-portable.zip` (`redrob-launcher.exe` + `portable.txt`), unpacked with System32's `tar.exe` beside the old
  install and swapped in, so a failed update never leaves a half-updated folder; `portable.txt` is dropped. A
  running launcher is never closed for the person: the setup asks them to quit it, then Retry.
- **Shortcuts**: Start menu always, desktop when the welcome screen's box is ticked (the choice is
  remembered for the next update). Both carry the AppUserModelID `ai.redrob.launcher`, set through
  `IPropertyStore`, which Windows needs for toast notifications from an unpackaged app.
- **Registry**: `InstallDir` (read by the launcher's self-update), `Language`, `DesktopShortcutChoice` under
  `HKCU\Software\Redrob\Redrob Launcher`; the App Paths entry for Win+R `redrob-launcher`; and the
  **Settings > Apps** entry (`...\Uninstall\RedrobLauncher`: name, version, publisher, icon, size,
  `UninstallString`).
- **Uninstall**: the same app built without a payload, written into the install folder as
  `Uninstall Redrob Launcher.exe`. It asks first, with "Also delete its settings and downloads" off by
  default, then removes the shortcuts, the entries and the folder. With the box: `%APPDATA%\ai.redrob.launcher`
  (settings), `%LOCALAPPDATA%\Redrob Launcher` (downloads, release cache, the engine) and the registry key.
  `%APPDATA%\Redrob\Shared` (the sign-in and the language every Redrob app reads) and the shared models folder
  are never touched. The apps the launcher installed stay.

## Flags

| Flag | Does |
| --- | --- |
| (none) | the setup window; the uninstaller build opens its confirm screen |
| `--lang en` / `--lang ko` | preselect the language |
| `--uninstall` | the uninstall window (what Settings > Apps runs) |
| `--silent` | the same work with no window; the exit code is the result (0 = success, `0xE0000001` = the launcher is running). Never opens the launcher afterwards. The launcher's self-update runs this after quitting itself |
| `--delete-data` | with `--uninstall --silent`: also delete settings and downloads |
| `--demo`, `--demo-error`, `--demo-uninstall` | the window's states on a timer, touching nothing |
| `--render-stages <dir>` | draw every screen (both languages) to PNGs in `<dir>`, no window |
| `--theme light` / `--theme dark` | draw in that theme instead of Windows' own (mainly for `--render-stages`) |

## Build

Visual Studio with the C++ toolset (found with `vswhere`; the x86 and ARM64 compilers for those targets) and
Python 3:

```
py -3 installer\windows\build.py --out <dir> --arch x64 --payload redrob-launcher-X.Y.Z-windows-x64-portable.zip --version X.Y.Z
```

The release workflow builds it in two stages so every binary is signed before it is embedded:
`--uninstaller-only`, sign that, then `--payload … --uninstaller <signed exe> --name …-setup.exe`, sign that
(`.github/workflows/release.yml`, Windows jobs; signing is the shared `redrob-labs/.github/actions/sign-windows`
action through Azure Artifact Signing).

## Look

The window follows Windows' light or dark app theme and is two columns: a brand panel on the left, drawn in
Direct2D (Redrob Blue radial gradient `#2E7EFF` -> `#2B52FF` -> `#2746EE`, a white glint top-left and a
`#8B82FF` glint bottom-right) with the design system's white symbol, `assets/brand/symbol-white-192.png`,
embedded as is; and a left-aligned content column with the design system's button, checkbox, progress bar
and step dots. No new art file is committed for it.

Type is Pretendard SemiBold and Regular from `assets/fonts`, cut by `build.py` (fontTools) to the characters
the window draws (about 68 KB each). Without fontTools the build embeds no font and the window uses Segoe
UI Variable.

`res/background.png`, `res/background.html`, `res/lockup.png` and `build-art.py` belong to the previous
dark centred window; this build no longer embeds them. `assets/app-icon/redrob-launcher.ico` comes from
`packaging/brand-assets.sh`.

`--render-stages <dir>` draws every screen in both languages; add `--theme light` or `--theme dark` to pick
the theme (otherwise Windows' own).

## Verify

On a Windows machine: install from the setup, check the Start-menu shortcut, the Settings > Apps entry and
the signed exe; run the setup again over the install (an update); try it while the launcher runs (refused with
Retry); uninstall. Both languages: `--lang ko` and `--lang en`.
