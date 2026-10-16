<#
.SYNOPSIS
  Build and package Redrob Launcher for Windows.

.DESCRIPTION
  Produces, in $env:DIST (default: dist/release):
    redrob-launcher-<version>-windows-<arch>-portable.zip   redrob-launcher.exe + portable.txt
    redrob-launcher-<version>-windows-<arch>-setup.exe      the branded per-user setup (installer/windows),
                                                            which embeds that zip and its uninstaller
  The binary links the C runtime statically (+crt-static), so nothing needs the Visual C++
  redistributable. This script does not sign. The release workflow runs it in three phases so every
  binary is signed before it is embedded (redrob-labs/.github/actions/sign-windows):
    -BuildOnly   build and stage redrob-launcher.exe              -> sign the staged exe
    -SkipBuild   portable zip + the uninstaller (no payload)      -> sign the uninstaller
    -SetupOnly   the setup, embedding the zip and the uninstaller -> sign the setup
  A local run without those switches does all of it, unsigned.
  Needs: Rust (MSVC toolchain + the target), the Windows SDK (rc.exe), Visual Studio's C++ toolset
  for the target (installer/windows/build.py finds it with vswhere) and Python 3.

.EXAMPLE
  pwsh packaging/windows/package.ps1 -Arch x64
  pwsh packaging/windows/package.ps1 -Arch x64 -BuildOnly  # CI phase 1
  pwsh packaging/windows/package.ps1 -Arch x64 -SkipBuild  # CI phase 2
  pwsh packaging/windows/package.ps1 -Arch x64 -SetupOnly  # CI phase 3
  pwsh packaging/windows/package.ps1 -Arch arm64     # cross-compiled; needs the MSVC ARM64 build tools
#>
param(
  [ValidateSet('x64', 'x86', 'arm64')] [string] $Arch = 'x64',
  [switch] $SkipBuild,
  [switch] $BuildOnly,
  [switch] $SetupOnly
)
if (($SkipBuild -and $BuildOnly) -or ($SetupOnly -and ($SkipBuild -or $BuildOnly))) { throw '-SkipBuild, -BuildOnly and -SetupOnly exclude each other' }
if ($SetupOnly) { $SkipBuild = $true }
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
# The setup's VERSIONINFO is numeric (major.minor.build); pre-release tags are dropped there.
$PlainVersion = ($Version -split '-')[0]

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

# ---- portable zip ------------------------------------------------------------------------------
$Portable = Join-Path $TargetDir "windows-package\$Pkg-$Version-windows-$Arch-portable"
$Zip = Join-Path $Dist "$Pkg-$Version-windows-$Arch-portable.zip"
if (-not $SetupOnly) {
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
  Remove-Item -Force $Zip -ErrorAction SilentlyContinue
  Compress-Archive -Path $Portable -DestinationPath $Zip

}
if (-not (Test-Path $Zip)) { throw "no portable zip at $Zip; run with -SkipBuild first" }

# ---- setup (installer/windows) -----------------------------------------------------------------
# Phase 2 (-SkipBuild) builds the uninstaller alone so the workflow can sign it; phase 3 (-SetupOnly)
# embeds the signed uninstaller and the zip into the setup. A plain local run does both at once.
$Installer = Join-Path $Root 'installer\windows\build.py'
$SetupWork = Join-Path $TargetDir "windows-package\$Arch-setup"
$Uninstaller = Join-Path $SetupWork 'uninstall\Uninstall Redrob Launcher.exe'
$SetupName = "$Pkg-$Version-windows-$Arch-setup.exe"
$Setup = Join-Path $Dist $SetupName
$Py = if (Get-Command py -ErrorAction SilentlyContinue) { 'py' } else { 'python' }
$PyArgs = if ($Py -eq 'py') { @('-3') } else { @() }
if (-not $SetupOnly) {
  Remove-Item -Recurse -Force $SetupWork -ErrorAction SilentlyContinue
  Invoke-Native 'setup: uninstaller' { & $Py @PyArgs $Installer --out $SetupWork --arch $Arch --version $PlainVersion --uninstaller-only }
}
if ($SetupOnly -or -not $SkipBuild) {
  if (-not (Test-Path $Uninstaller)) { throw "no uninstaller at $Uninstaller; run with -SkipBuild first" }
  Invoke-Native 'setup' { & $Py @PyArgs $Installer --out $SetupWork --arch $Arch --version $PlainVersion --payload $Zip --uninstaller $Uninstaller --name $SetupName }
  Move-Item -Force (Join-Path $SetupWork $SetupName) $Setup
  $h = Get-PeHeader $Setup
  if ($h.Machine -ne $Machine) { throw "setup is for machine 0x$('{0:X}' -f $h.Machine), expected 0x$('{0:X}' -f $Machine) ($Arch)" }
  Get-Item $Zip, $Setup | Format-Table Name, Length
} else {
  Get-Item $Zip, $Uninstaller | Format-Table Name, Length
}
