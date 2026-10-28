#Requires -Version 5.0
# ============================================================================
#  Ovie Programming Language v2.3.0 — Easy One-Click Windows Installer
#
#  Can be run two ways:
#    1. From inside the cloned repo  — uses local windows-x64\ binaries (fast)
#    2. From a fresh PC via iwr | iex — clones the repo first, then installs
#
#  No Rust. No GitHub release zip. Just git + this script.
#
#  Usage (fresh PC, PowerShell as Admin):
#    iwr -useb https://raw.githubusercontent.com/southwarridev/ovie/main/easy-windows-install.ps1 | iex
#
#  Usage (already cloned):
#    powershell -ExecutionPolicy Bypass -File easy-windows-install.ps1
# ============================================================================

# Self-elevate to Administrator if needed
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole(
        [Security.Principal.WindowsBuiltInRole]"Administrator")) {
    Write-Host "  Requesting Administrator privileges..." -ForegroundColor Yellow
    if ($PSCommandPath) {
        # Running from a saved .ps1 file — re-launch it elevated
        Start-Process PowerShell -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    } else {
        # Running via iwr | iex — re-download and run elevated
        $relaunchCmd = "iwr -useb https://raw.githubusercontent.com/southwarridev/ovie/main/easy-windows-install.ps1 | iex"
        Start-Process PowerShell -ArgumentList "-NoProfile -ExecutionPolicy Bypass -Command `"$relaunchCmd`"" -Verb RunAs
    }
    exit
}

$Host.UI.RawUI.WindowTitle = "Ovie Programming Language v2.3.0 - Installer"

# ── Banner ────────────────────────────────────────────────────────────────────
Clear-Host
Write-Host ""
Write-Host "  ============================================================================" -ForegroundColor Cyan
Write-Host "  |                                                                          |" -ForegroundColor Cyan
Write-Host "  |              OVIE PROGRAMMING LANGUAGE v2.3.0                           |" -ForegroundColor Cyan
Write-Host "  |              Complete Module System - Full Package                       |" -ForegroundColor Cyan
Write-Host "  |              Publisher: Nashedy Ltd  |  MIT License              |" -ForegroundColor Cyan
Write-Host "  |                                                                          |" -ForegroundColor Cyan
Write-Host "  ============================================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "  Running as Administrator: YES" -ForegroundColor Green
Write-Host ""

$OvieVersion = "2.3.0"
$InstallDir  = "C:\Program Files\Ovie"
$BinDir      = "$InstallDir\bin"
$GithubRepo  = "https://github.com/southwarridev/ovie.git"

Write-Host "  This installer will set up Ovie v$OvieVersion with:" -ForegroundColor White
Write-Host "    [*] oviec.exe  - Ovie Compiler" -ForegroundColor Green
Write-Host "    [*] ovie.exe   - Ovie CLI and project manager" -ForegroundColor Green
Write-Host "    [*] std\       - Complete standard library (11 modules)" -ForegroundColor Green
Write-Host "    [*] examples\  - 22+ runnable example programs" -ForegroundColor Green
Write-Host "    [*] docs\      - Complete documentation" -ForegroundColor Green
Write-Host "    [*] PATH       - Added to system PATH" -ForegroundColor Green
Write-Host ""
Write-Host "  Installation directory: $InstallDir" -ForegroundColor White
Write-Host ""

$confirm = Read-Host "  Press ENTER to install or type 'cancel' to exit"
if ($confirm -eq "cancel") { exit 0 }

Write-Host ""

try {
    # ── Step 1: Find or clone the repo ───────────────────────────────────────
    Write-Host "  [1/6] Locating Ovie source..." -ForegroundColor Cyan

    $RepoRoot  = $null
    $TempClone = $null

    # If this script is inside a cloned repo, the prebuilt binary will be here
    $localCheck = if ($PSScriptRoot) { $PSScriptRoot } else { $PWD.Path }
    if (Test-Path (Join-Path $localCheck "windows-x64\bin\oviec.exe")) {
        $RepoRoot = $localCheck
        Write-Host "  [OK] Using local repo at $RepoRoot" -ForegroundColor Green
    } else {
        # Check if git is available before trying to clone
        if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
            Write-Host "  [ERROR] Git is not installed." -ForegroundColor Red
            Write-Host "  [ERROR] Install Git from https://git-scm.com/download/win then re-run." -ForegroundColor Red
            exit 1
        }

        $TempClone = "$env:TEMP\ovie-install-$(Get-Random)"
        Write-Host "  [>>] Cloning from GitHub (this takes ~30 seconds)..." -ForegroundColor Yellow
        git clone --depth 1 $GithubRepo $TempClone 2>&1 | Out-Null
        if ($LASTEXITCODE -ne 0) {
            Write-Host "  [ERROR] git clone failed. Check your internet connection." -ForegroundColor Red
            exit 1
        }
        $RepoRoot = $TempClone
        Write-Host "  [OK] Cloned to $TempClone" -ForegroundColor Green
    }

    $Src = Join-Path $RepoRoot "windows-x64"

    if (-not (Test-Path "$Src\bin\oviec.exe")) {
        Write-Host "  [ERROR] windows-x64\bin\oviec.exe not found in repo. Layout may have changed." -ForegroundColor Red
        exit 1
    }

    # ── Step 2: Create directories ───────────────────────────────────────────
    Write-Host ""
    Write-Host "  [2/6] Creating installation directories..." -ForegroundColor Cyan
    foreach ($d in @($InstallDir, $BinDir, "$InstallDir\std", "$InstallDir\examples", "$InstallDir\docs")) {
        New-Item -ItemType Directory -Path $d -Force | Out-Null
    }
    Write-Host "  [OK] Directories created at $InstallDir" -ForegroundColor Green

    # ── Step 3: Copy binaries ────────────────────────────────────────────────
    Write-Host ""
    Write-Host "  [3/6] Installing Ovie compiler (oviec.exe)..." -ForegroundColor Cyan
    Copy-Item "$Src\bin\oviec.exe" "$BinDir\oviec.exe" -Force
    Write-Host "  [OK] oviec.exe installed" -ForegroundColor Green

    Write-Host ""
    Write-Host "  [4/6] Installing Ovie CLI (ovie.exe)..." -ForegroundColor Cyan
    if (Test-Path "$Src\ovie.exe") {
        Copy-Item "$Src\ovie.exe" "$BinDir\ovie.exe" -Force
    } else {
        Copy-Item "$BinDir\oviec.exe" "$BinDir\ovie.exe" -Force
    }
    Write-Host "  [OK] ovie.exe installed" -ForegroundColor Green

    # ── Step 4: Copy stdlib, examples, docs ─────────────────────────────────
    Write-Host ""
    Write-Host "  [5/6] Installing standard library, examples and documentation..." -ForegroundColor Cyan

    if (Test-Path "$Src\std")      { Copy-Item "$Src\std"      "$InstallDir\std"      -Recurse -Force }
    if (Test-Path "$Src\examples") { Copy-Item "$Src\examples" "$InstallDir\examples" -Recurse -Force }
    if (Test-Path "$Src\docs")     { Copy-Item "$Src\docs"     "$InstallDir\docs"     -Recurse -Force }

    foreach ($f in @("README.md","LICENSE","RELEASE_NOTES_v2.3.md","ovie.png","ovie.svg","ovie.toml.template")) {
        $fp = Join-Path $Src $f
        if (Test-Path $fp) { Copy-Item $fp "$InstallDir\" -Force }
    }

    $stdCount  = (Get-ChildItem "$InstallDir\std"      -Recurse -File -ErrorAction SilentlyContinue).Count
    $exCount   = (Get-ChildItem "$InstallDir\examples" -Filter "*.ov" -ErrorAction SilentlyContinue).Count
    Write-Host "  [OK] Standard library installed ($stdCount files, 11 modules)" -ForegroundColor Green
    Write-Host "  [OK] Examples installed ($exCount .ov files)" -ForegroundColor Green

    # ── Step 5: Cleanup temp clone ───────────────────────────────────────────
    if ($TempClone -and (Test-Path $TempClone)) {
        Remove-Item $TempClone -Recurse -Force -ErrorAction SilentlyContinue
    }

    # ── Step 6: Add to system PATH ───────────────────────────────────────────
    Write-Host ""
    Write-Host "  [6/6] Adding Ovie to system PATH..." -ForegroundColor Cyan
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
    if ($currentPath -notlike "*$BinDir*") {
        [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$BinDir", "Machine")
        $env:PATH = "$env:PATH;$BinDir"
        Write-Host "  [OK] Added to system PATH (all users)" -ForegroundColor Green
    } else {
        Write-Host "  [OK] Already in system PATH" -ForegroundColor Green
    }

    # ── Verify ───────────────────────────────────────────────────────────────
    Write-Host ""
    Write-Host "  Verifying installation..." -ForegroundColor Cyan
    $version = & "$BinDir\oviec.exe" --version 2>&1 | Select-Object -First 1
    Write-Host "  [OK] $version" -ForegroundColor Green

    # ── Success ───────────────────────────────────────────────────────────────
    Write-Host ""
    Write-Host "  ============================================================================" -ForegroundColor Green
    Write-Host "  |                    INSTALLATION COMPLETE!                               |" -ForegroundColor Green
    Write-Host "  ============================================================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "  Ovie v$OvieVersion installed to: $InstallDir" -ForegroundColor White
    Write-Host ""
    Write-Host "  IMPORTANT: Restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  Quick Start:" -ForegroundColor Cyan
    Write-Host "    oviec --version              # Check version" -ForegroundColor White
    Write-Host "    oviec --self-check           # Validate installation" -ForegroundColor White
    Write-Host "    oviec run examples\hello.ov  # Run hello world" -ForegroundColor White
    Write-Host "    oviec new my-project         # Create new project" -ForegroundColor White
    Write-Host ""
    Write-Host "  Resources:" -ForegroundColor Cyan
    Write-Host "    Website:  https://ovie.nashedy.io" -ForegroundColor White
    Write-Host "    GitHub:   https://github.com/southwarridev/ovie" -ForegroundColor White
    Write-Host "    Book:     https://southwarridev.github.io/ovie/docs/book/index.html" -ForegroundColor White
    Write-Host "    Discord:  https://discord.gg/AuF4ubMyE" -ForegroundColor White
    Write-Host ""

} catch {
    Write-Host ""
    Write-Host "  [ERROR] Installation failed: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host ""
    Write-Host "  Troubleshooting:" -ForegroundColor Yellow
    Write-Host "    * Make sure you are running as Administrator" -ForegroundColor White
    Write-Host "    * Make sure Git is installed: https://git-scm.com/download/win" -ForegroundColor White
    Write-Host "    * Check your internet connection" -ForegroundColor White
    Write-Host "    * GitHub: https://github.com/southwarridev/ovie" -ForegroundColor White
    Write-Host ""
    exit 1
}

Write-Host "  Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
