<#
.SYNOPSIS
  Build and package Redrob Launcher for Windows.

.DESCRIPTION
  Produces, in $env:DIST (default: dist/release):
    redrob-launcher-<version>-windows-<arch>.msi            per-user installer with a setup wizard (WiX v5)
    redrob-launcher-<version>-windows-<arch>-portable.zip   redrob-launcher.exe + portable.txt

  The binary links the C runtime statically (+crt-static), so neither the MSI nor the portable
  zip needs the Visual C++ redistributable. This script does not sign: the release workflow signs
  the staged executable (after -BuildOnly) and the MSI (after -SkipBuild) with the shared
  redrob-labs/.github/actions/sign-windows action, so the portable zip carries the signed
  executable and the installer is signed last. A local run produces unsigned packages.

  Needs: Rust (MSVC toolchain + the target), the Windows SDK (rc.exe),
  and WiX v5: dotnet tool install --global wix --version 5.0.2
             wix extension add -g WixToolset.UI.wixext/5.0.2 WixToolset.Util.wixext/5.0.2

.EXAMPLE
  pwsh packaging/windows/package.ps1 -Arch x64
  pwsh packaging/windows/package.ps1 -Arch x86 -SkipBuild
  pwsh packaging/windows/package.ps1 -Arch x64 -BuildOnly  # CI: build and stage, sign the stage, then -SkipBuild
  pwsh packaging/windows/package.ps1 -Arch arm64     # cross-compiled; needs the MSVC ARM64 build tools
#>
param(
  [ValidateSet('x64', 'x86', 'arm64')] [string] $Arch = 'x64',
  [switch] $SkipBuild,
  [switch] $BuildOnly
)
if ($SkipBuild -and $BuildOnly) { throw '-SkipBuild and -BuildOnly exclude each other' }
$ErrorActionPreference = 'Stop'
$Root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path

# ---- app-specific values ----------------------------------------------------------------------
$Pkg = 'redrob-launcher'          # cargo package and binary
$AppName = 'Redrob Launcher'      # display name

function Invoke-Native([string] $What, [scriptblock] $Block) {
  Write-Output "==> $What"
  & $Block
  if ($LASTEXITCODE -ne 0) { throw "$What failed with exit code $LASTEXITCODE" }
}

# The version lives in one place: [workspace.package] version in the root Cargo.toml.
$Version = $env:REDROB_LAUNCHER_VERSION
if (-not $Version) {
  $inPkg = $false
  foreach ($line in Get-Content (Join-Path $Root 'Cargo.toml')) {
    if ($line -match '^\s*\[') { $inPkg = ($line.Trim() -eq '[workspace.package]'); continue }
    if ($inPkg -and $line -match '^\s*version\s*=\s*"([^"]+)"') { $Version = $Matches[1]; break }
  }
}
if (-not $Version) { throw 'could not read [workspace.package] version from Cargo.toml' }
# MSI ProductVersion is numeric (major.minor.build); pre-release tags are dropped there.
$MsiVersion = ($Version -split '-')[0]

$Target = switch ($Arch) { 'x64' { 'x86_64-pc-windows-msvc' } 'x86' { 'i686-pc-windows-msvc' } 'arm64' { 'aarch64-pc-windows-msvc' } }
$Dist = if ($env:DIST) { $env:DIST } else { Join-Path $Root 'dist\release' }
$TargetDir = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $Root 'target' }
New-Item -ItemType Directory -Force -Path $Dist | Out-Null

if (-not $env:REDROB_LAUNCHER_BUILD_SHA) { $env:REDROB_LAUNCHER_BUILD_SHA = (git -C $Root rev-parse HEAD 2>$null) }
if (-not $env:REDROB_LAUNCHER_BUILD_DATE) { $env:REDROB_LAUNCHER_BUILD_DATE = (Get-Date).ToUniversalTime().ToString('yyyy-MM-dd') }

Write-Output "$AppName $Version for Windows $Arch ($Target)"

if (-not $SkipBuild) {
  # Static CRT: no VC++ redistributable needed. Scoped to the target so host build scripts and
  # proc-macros are unaffected.
  $flagVar = 'CARGO_TARGET_' + ($Target.ToUpper() -replace '-', '_') + '_RUSTFLAGS'
  [Environment]::SetEnvironmentVariable($flagVar, '-C target-feature=+crt-static')
  # Fail the build (rather than warn) if the icon/VERSIONINFO can't be embedded.
  $env:REDROB_LAUNCHER_REQUIRE_WINRES = '1'
  Invoke-Native "cargo build ($Target)" { cargo build --release --locked -p $Pkg --target $Target }
}
$Bin = Join-Path $TargetDir "$Target\release"
$Exe = "$Pkg.exe"
$Stage = Join-Path $TargetDir "windows-package\$Arch"

# The release workflow builds and stages first, signs the staged executable, then packages in a
# second invocation, so the short-lived OpenID Connect login happens right before signing instead
# of expiring during a 12-minute build. The stage is therefore filled in the build phase and only
# read in the package phase.
if (-not $SkipBuild) {
  Remove-Item -Recurse -Force $Stage -ErrorAction SilentlyContinue
  New-Item -ItemType Directory -Force -Path $Stage | Out-Null
  Copy-Item (Join-Path $Bin $Exe) $Stage
}
if ($BuildOnly) { exit 0 }

# Check the PE header before packaging. Machine (COFF header) must match -Arch, so an x64 build
# can never ship labelled arm64. Subsystem (optional header) must be 2 = Windows GUI, so no
# console window opens with the launcher.
function Get-PeHeader([string] $Path) {
  $bytes = [System.IO.File]::ReadAllBytes($Path)
  $pe = [BitConverter]::ToInt32($bytes, 0x3C)
  return @{ Machine = [BitConverter]::ToUInt16($bytes, $pe + 4); Subsystem = [BitConverter]::ToUInt16($bytes, $pe + 0x5C) }
}
$Machine = switch ($Arch) { 'x64' { 0x8664 } 'x86' { 0x14C } 'arm64' { 0xAA64 } }
if (-not (Test-Path (Join-Path $Stage $Exe))) { throw "$Exe is not staged in $Stage; run with -BuildOnly first" }
$h = Get-PeHeader (Join-Path $Stage $Exe)
if ($h.Machine -ne $Machine) { throw "$Exe is for machine 0x$('{0:X}' -f $h.Machine), expected 0x$('{0:X}' -f $Machine) ($Arch)" }
if ($h.Subsystem -ne 2) { throw "$Exe has PE subsystem $($h.Subsystem), expected 2 (Windows GUI)" }
Write-Output "ok ${Exe}: $Arch, PE subsystem $($h.Subsystem)"

# ---- MSI ---------------------------------------------------------------------------------------
# The setup wizard's art, drawn here (System.Drawing) in the launcher's own style so the MSI shows
# Redrob rather than WiX's stock bitmaps. Both must be 24-bit BMPs:
#   - dialog, 493x312: the Welcome and Finish pages' backdrop. Their text sits on the right, so
#     that stays white; the left panel is ours: the Redrob tile, the white lockup, a
#     "Launcher" label and the Redrob apps' icons in a hairline grid.
#   - banner, 493x58: across the top of the inner pages. The page title is drawn over its left
#     side, so that stays white; the right end is a dark cell with the mark.
# The Redrob tile and lockup are drawn as the design system ships them (assets/brand/README.md).
function New-InstallerArt([string] $BannerOut, [string] $DialogOut) {
  Add-Type -AssemblyName System.Drawing
  $ink = [System.Drawing.ColorTranslator]::FromHtml('#0a0b0c')
  $accent = [System.Drawing.ColorTranslator]::FromHtml('#2b52ff')
  $muted = [System.Drawing.ColorTranslator]::FromHtml('#aab0bb')
  $line = [System.Drawing.ColorTranslator]::FromHtml('#292e37')
  # The Redrob apps, in the launcher's order (crates/core/src/catalog.rs).
  $apps = 'redrob-cut', 'redrob-shape', 'redrob-cad', 'redrob-canvas', 'redrob-design', 'redrob-studio',
    'redrob-browser', 'redrob-code', 'redrob-cowork', 'redrob-office', 'redrob-query'
  $mark = [System.Drawing.Image]::FromFile((Join-Path $Root 'assets\app-icon\hicolor\256x256\apps\ai.redrob.launcher.png'))
  $wordmark = [System.Drawing.Image]::FromFile((Join-Path $Root 'assets\app-icon\redrob-logo-dark.png'))
  $icons = $apps | ForEach-Object { [System.Drawing.Image]::FromFile((Join-Path $Root "assets\app-icons\$_.png")) }
  # The design system's label face.
  $fonts = New-Object System.Drawing.Text.PrivateFontCollection
  $fonts.AddFontFile((Join-Path $Root 'assets\fonts\Pretendard-Medium.ttf'))
  $labelFont = New-Object System.Drawing.Font $fonts.Families[0], 9, ([System.Drawing.FontStyle]::Regular), ([System.Drawing.GraphicsUnit]::Pixel)
  try {
    foreach ($spec in @(@($BannerOut, 493, 58), @($DialogOut, 493, 312))) {
      $bmp = New-Object System.Drawing.Bitmap $spec[1], $spec[2], ([System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
      $g = [System.Drawing.Graphics]::FromImage($bmp)
      $panel = New-Object System.Drawing.SolidBrush $ink
      $stripe = New-Object System.Drawing.SolidBrush $accent
      try {
        $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
        $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
        $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
        $g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit
        $g.Clear([System.Drawing.Color]::White)
        if ($spec[2] -eq 58) {
          $g.FillRectangle($panel, 405, 0, 88, 58)
          $g.FillRectangle($stripe, 405, 0, 2, 58)
          $g.DrawImage($mark, 431, 11, 36, 36)
        } else {
          $g.FillRectangle($panel, 0, 0, 164, 312)
          $g.FillRectangle($stripe, 162, 0, 2, 312)
          $g.DrawImage($mark, 16, 18, 44, 44)
          $g.DrawImage($wordmark, 22, 74, 120, [int](120 * $wordmark.Height / $wordmark.Width))
          # "LAUNCHER" in mono capitals, spaced like the site's labels (0.15 em).
          $brush = New-Object System.Drawing.SolidBrush $muted
          $x = 22.0
          foreach ($ch in 'LAUNCHER'.ToCharArray()) {
            $g.DrawString([string]$ch, $labelFont, $brush, $x, 94, [System.Drawing.StringFormat]::GenericTypographic)
            $x += $g.MeasureString([string]$ch, $labelFont, 100, [System.Drawing.StringFormat]::GenericTypographic).Width + 1.35
          }
          $brush.Dispose()
          # The apps, three to a row, in 40 px cells divided by hairlines.
          $pen = New-Object System.Drawing.Pen $line, 1
          $left = 22; $top = 126; $cell = 40
          for ($i = 0; $i -lt $icons.Count; $i++) {
            $cx = $left + ($i % 3) * $cell
            $cy = $top + [math]::Floor($i / 3) * $cell
            $g.DrawRectangle($pen, $cx, $cy, $cell, $cell)
            $g.DrawImage($icons[$i], $cx + 6, $cy + 6, $cell - 12, $cell - 12)
          }
          $pen.Dispose()
        }
      } finally { $panel.Dispose(); $stripe.Dispose(); $g.Dispose() }
      $bmp.Save($spec[0], [System.Drawing.Imaging.ImageFormat]::Bmp)
      $bmp.Dispose()
    }
  } finally {
    $labelFont.Dispose(); $fonts.Dispose(); $mark.Dispose(); $wordmark.Dispose()
    $icons | ForEach-Object { $_.Dispose() }
  }
}
$Banner = Join-Path $Stage 'installer-banner.bmp'
$Dialog = Join-Path $Stage 'installer-dialog.bmp'
New-InstallerArt $Banner $Dialog

$Msi = Join-Path $Dist "$Pkg-$Version-windows-$Arch.msi"
& (Join-Path $PSScriptRoot 'check-icons.ps1')
Invoke-Native 'wix build' {
  wix build (Join-Path $PSScriptRoot "$Pkg.wxs") -arch $Arch `
    -ext WixToolset.UI.wixext -ext WixToolset.Util.wixext `
    -culture en-US -loc (Join-Path $PSScriptRoot "$Pkg.en-us.wxl") `
    -d "Version=$MsiVersion" -d "DisplayVersion=$Version" -d "BinDir=$Stage" -d "IconPath=$(Join-Path $Root "assets\app-icon\$Pkg.ico")" `
    -d "BannerBmp=$Banner" -d "DialogBmp=$Dialog" `
    -o $Msi
}
Invoke-Native 'MSI shortcut icon validation (ICE50)' {
  wix msi validate $Msi -ice ICE50 -intermediateFolder (Join-Path $Stage 'msi-validation')
}
# wix writes its debug symbols (.wixpdb) next to the MSI; keep them out of the release assets.
Remove-Item -Force -ErrorAction SilentlyContinue ([IO.Path]::ChangeExtension($Msi, '.wixpdb'))

# ---- portable zip ------------------------------------------------------------------------------
$Portable = Join-Path $TargetDir "windows-package\$Pkg-$Version-windows-$Arch-portable"
Remove-Item -Recurse -Force $Portable -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $Portable | Out-Null
Copy-Item (Join-Path $Stage $Exe) $Portable
foreach ($f in 'README.md', 'LICENSE-MIT', 'LICENSE-APACHE', 'NOTICE', 'ATTRIBUTION.md') {
  $p = Join-Path $Root $f
  if (Test-Path $p) { Copy-Item $p $Portable }
}
# portable.txt beside redrob-launcher.exe switches on portable mode: settings and the download
# cache go to RedrobLauncherData\ next to the exe instead of %APPDATA%.
Copy-Item (Join-Path $PSScriptRoot 'portable.txt') $Portable
$Zip = Join-Path $Dist "$Pkg-$Version-windows-$Arch-portable.zip"
Remove-Item -Force $Zip -ErrorAction SilentlyContinue
Compress-Archive -Path $Portable -DestinationPath $Zip

Get-Item $Msi, $Zip | Format-Table Name, Length
