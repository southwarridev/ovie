#Requires -Version 5.0
# ============================================================================
#  Ovie Programming Language v2.3.0 — Windows PowerShell Installer
#  Downloads prebuilt windows-x64 binaries from GitHub releases.
#  No Rust, no manual cloning needed.
# ============================================================================

param(
    [string]$InstallDir = "C:\Program Files\Ovie",
    [switch]$Force = $false
)

# ── helpers ──────────────────────────────────────────────────────────────────
function Write-Step   { param([string]$m) Write-Host "  [>>] $m" -ForegroundColor Cyan }
function Write-Ok     { param([string]$m) Write-Host "  [OK] $m" -ForegroundColor Green }
function Write-Fail   { param([string]$m) Write-Host "  [ERROR] $m" -ForegroundColor Red }
function Write-Warn   { param([string]$m) Write-Host "  [WARN] $m" -ForegroundColor Yellow }

function Require-Admin {
    $isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole(
        [Security.Principal.WindowsBuiltInRole]"Administrator")
    if (-not $isAdmin) {
        Write-Warn "Not running as Administrator — re-launching elevated..."
        Start-Process PowerShell -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
        exit
    }
}

# ── banner ────────────────────────────────────────────────────────────────────
Clear-Host
Write-Host ""
Write-Host "  ============================================================================" -ForegroundColor Cyan
Write-Host "  |                                                                          |" -ForegroundColor Cyan
Write-Host "  |              OVIE PROGRAMMING LANGUAGE v2.3.0                           |" -ForegroundColor Cyan
Write-Host "  |              Complete Module System - Full Package                       |" -ForegroundColor Cyan
Write-Host "  |              Publisher: Ovie Language Team  |  MIT License              |" -ForegroundColor Cyan
Write-Host "  |                                                                          |" -ForegroundColor Cyan
Write-Host "  ============================================================================" -ForegroundColor Cyan
Write-Host ""

# ── preflight ─────────────────────────────────────────────────────────────────
Require-Admin

$BinDir = "$InstallDir\bin"

Write-Host "  Install directory : $InstallDir" -ForegroundColor White
Write-Host "  Binaries          : $BinDir" -ForegroundColor White
Write-Host ""

$confirm = Read-Host "  Press ENTER to install or type 'cancel' to exit"
if ($confirm -eq "cancel") { exit 0 }

Write-Host ""

try {
    # ── Step 1: Download latest release ───────────────────────────────────────
    Write-Step "[1/3] Downloading latest Ovie release..."

    $GITHUB_REPO = "southwarridev/ovie"
    $AssetName = "ovie-windows-x64.zip"
    $ApiUrl = "https://api.github.com/repos/$GITHUB_REPO/releases/latest"

    Write-Step "   Fetching release information..."

    $Release = Invoke-RestMethod -Uri $ApiUrl -Headers @{"Accept"="application/vnd.github.v3+json"}
    $DownloadUrl = $Release.assets | Where-Object { $_.name -eq $AssetName } | Select-Object -ExpandProperty browser_download_url

    if (-not $DownloadUrl) {
        Write-Fail "Could not find $AssetName in latest release"
        Write-Fail "Check your internet connection or GitHub availability."
        exit 1
    }

    Write-Ok "Download URL: $DownloadUrl"

    Write-Step "   Downloading $AssetName..."
    $TempFile = Join-Path $env:TEMP $AssetName
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempFile -UseBasicParsing

    if ($LASTEXITCODE -ne 0) {
        Write-Fail "Download failed. Check your internet connection."
        exit 1
    }

    Write-Ok "Downloaded $AssetName"

    # ── Step 2: Extract and copy binaries ──────────────────────────────────────
    Write-Step "[2/3] Installing Ovie..."

    # Create directories
    foreach ($d in @($InstallDir, $BinDir, "$InstallDir\std", "$InstallDir\examples", "$InstallDir\docs")) {
        New-Item -ItemType Directory -Path $d -Force | Out-Null
    }

    # Extract the archive
    $ExtractPath = Join-Path $env:TEMP "ovie-installer"
    if (Test-Path $ExtractPath) { Remove-Item $ExtractPath -Recurse -Force }
    Expand-Archive -Path $TempFile -DestinationPath $ExtractPath -Force

    # Copy binaries
    Copy-Item "$ExtractPath\ovie\bin\oviec.exe" "$BinDir\oviec.exe" -Force
    if (Test-Path "$ExtractPath\ovie\bin\ovie.exe") {
        Copy-Item "$ExtractPath\ovie\bin\ovie.exe" "$BinDir\ovie.exe" -Force
    }

    Write-Ok "Binaries installed"

    # Copy stdlib, examples, docs
    if (Test-Path "$ExtractPath\ovie\std") {
        Copy-Item "$ExtractPath\ovie\std" "$InstallDir\std" -Recurse -Force
    }
    if (Test-Path "$ExtractPath\ovie\examples") {
        Copy-Item "$ExtractPath\ovie\examples" "$InstallDir\examples" -Recurse -Force
    }
    if (Test-Path "$ExtractPath\ovie\docs") {
        Copy-Item "$ExtractPath\ovie\docs" "$InstallDir\docs" -Recurse -Force
    }

    # Copy root files
    foreach ($f in @("README.md", "LICENSE")) {
        $fp = Join-Path "$ExtractPath\ovie" $f
        if (Test-Path $fp) { Copy-Item $fp "$InstallDir\" -Force }
    }

    Write-Ok "Standard library and examples copied"

    # ── Step 3: Add to PATH ─────────────────────────────────────────────────────
    Write-Step "[3/3] Adding $BinDir to system PATH..."
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
    if ($currentPath -notlike "*$BinDir*") {
        [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$BinDir", "Machine")
        $env:PATH = "$env:PATH;$BinDir"
        Write-Ok "Added to PATH"
    } else {
        Write-Ok "Already in PATH"
    }

    # ── Cleanup ────────────────────────────────────────────────────────────────
    Write-Step "Cleaning up temporary files..."
    if (Test-Path $ExtractPath) { Remove-Item $ExtractPath -Recurse -Force -ErrorAction SilentlyContinue }
    if (Test-Path $TempFile) { Remove-Item $TempFile -Force -ErrorAction SilentlyContinue }

    # ── Verify ─────────────────────────────────────────────────────────────────
    Write-Host ""
    Write-Step "Verifying..."
    $ver = & "$BinDir\oviec.exe" --version 2>&1 | Select-Object -First 1
    Write-Ok $ver

    # ── Done ───────────────────────────────────────────────────────────────────
    Write-Host ""
    Write-Host "  ============================================================================" -ForegroundColor Green
    Write-Host "  |                  INSTALLATION COMPLETE!                                 |" -ForegroundColor Green
    Write-Host "  ============================================================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "  IMPORTANT: Restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  Quick start:" -ForegroundColor Cyan
    Write-Host "    oviec --version              # Check version" -ForegroundColor White
    Write-Host "    oviec --self-check           # Validate installation" -ForegroundColor White
    Write-Host "    oviec run examples\hello.ov  # Run hello world" -ForegroundColor White
    Write-Host "    oviec new my-project         # Create new project" -ForegroundColor White
    Write-Host ""
    Write-Host "  Downloads for all platforms:" -ForegroundColor Cyan
    Write-Host "    Windows x64  : https://github.com/$GITHUB_REPO/releases" -ForegroundColor White
    Write-Host "    macOS x64    : https://github.com/$GITHUB_REPO/releases" -ForegroundColor White
    Write-Host "    macOS arm64  : https://github.com/$GITHUB_REPO/releases" -ForegroundColor White
    Write-Host "    Linux x64    : https://github.com/$GITHUB_REPO/releases" -ForegroundColor White
    Write-Host "    Linux arm64  : https://github.com/$GITHUB_REPO/releases" -ForegroundColor White
    Write-Host ""
    Write-Host "  Docs: https://southwarridev.github.io/ovie/docs/book/index.html" -ForegroundColor White
    Write-Host ""

} catch {
    Write-Fail "Installation failed: $($_.Exception.Message)"
    exit 1
}

Write-Host "  Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")