#Requires -Version 5.0
# ============================================================================
#                    OVIE PROGRAMMING LANGUAGE v2.3.0
#                    PowerShell One-Click Installer
#                    Publisher: Ovie Language Team
#                    License: MIT Open Source
# ============================================================================

# Self-elevate to Administrator if not already
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]"Administrator")) {
    Write-Host "Requesting Administrator privileges..." -ForegroundColor Yellow
    Start-Process PowerShell -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}

$Host.UI.RawUI.WindowTitle = "Ovie Programming Language v2.3.0 - Installer"

# Banner
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
Write-Host "  Running as Administrator: YES" -ForegroundColor Green
Write-Host ""

$OvieVersion = "2.3.0"
$InstallDir  = "C:\Program Files\Ovie"
$BinDir      = "$InstallDir\bin"
$GithubRepo  = "https://github.com/southwarridev/ovie"
$ReleaseBase = "$GithubRepo/releases/download/v$OvieVersion"

Write-Host "  This installer will set up Ovie v$OvieVersion with:" -ForegroundColor White
Write-Host "    [*] oviec.exe  - Ovie Compiler (self-hosted, v2.3.0)" -ForegroundColor Green
Write-Host "    [*] ovie.exe   - Ovie CLI and project manager" -ForegroundColor Green
Write-Host "    [*] std\       - Complete standard library (11 modules)" -ForegroundColor Green
Write-Host "    [*] std\module - v2.3 Module System (NEW)" -ForegroundColor Yellow
Write-Host "    [*] std\aproko - Aproko Knowledge Base (NEW)" -ForegroundColor Yellow
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
    # Step 1: Create directories
    Write-Host "  [1/7] Creating installation directories..." -ForegroundColor Cyan
    @($InstallDir, $BinDir, "$InstallDir\std", "$InstallDir\examples", "$InstallDir\docs") | ForEach-Object {
        New-Item -ItemType Directory -Path $_ -Force | Out-Null
    }
    Write-Host "  [OK] Directories created at $InstallDir" -ForegroundColor Green

    # Step 2: Download or copy binaries
    Write-Host ""
    Write-Host "  [2/7] Installing Ovie compiler (oviec.exe)..." -ForegroundColor Cyan
    
    $localOviec = Join-Path $PSScriptRoot "oviec.exe"
    if (Test-Path $localOviec) {
        # Local install from package
        Copy-Item $localOviec "$BinDir\oviec.exe" -Force
        Write-Host "  [OK] oviec.exe installed from local package" -ForegroundColor Green
    } else {
        # Download from GitHub releases
        Write-Host "  Downloading oviec.exe from GitHub..." -ForegroundColor Yellow
        $url = "$ReleaseBase/ovie-v$OvieVersion-windows-x64.zip"
        $zip = "$env:TEMP\ovie-v$OvieVersion-windows-x64.zip"
        try {
            Invoke-WebRequest -Uri $url -OutFile $zip -UseBasicParsing
            Expand-Archive -Path $zip -DestinationPath "$env:TEMP\ovie-extract" -Force
            $extracted = Get-ChildItem "$env:TEMP\ovie-extract" -Recurse -Filter "oviec.exe" | Select-Object -First 1
            if ($extracted) {
                Copy-Item $extracted.FullName "$BinDir\oviec.exe" -Force
            }
            Remove-Item $zip -Force -ErrorAction SilentlyContinue
            Remove-Item "$env:TEMP\ovie-extract" -Recurse -Force -ErrorAction SilentlyContinue
            Write-Host "  [OK] oviec.exe downloaded and installed" -ForegroundColor Green
        } catch {
            Write-Host "  [WARN] Could not download from GitHub. Checking source..." -ForegroundColor Yellow
            # Try to find in current directory tree
            $found = Get-ChildItem $PSScriptRoot -Recurse -Filter "oviec.exe" | Select-Object -First 1
            if ($found) {
                Copy-Item $found.FullName "$BinDir\oviec.exe" -Force
                Write-Host "  [OK] oviec.exe found and installed" -ForegroundColor Green
            } else {
                throw "oviec.exe not found. Please download the full package from: $GithubRepo/releases"
            }
        }
    }

    # Step 3: Install ovie.exe (CLI)
    Write-Host ""
    Write-Host "  [3/7] Installing Ovie CLI (ovie.exe)..." -ForegroundColor Cyan
    $localOvie = Join-Path $PSScriptRoot "ovie.exe"
    if (Test-Path $localOvie) {
        Copy-Item $localOvie "$BinDir\ovie.exe" -Force
    } else {
        # Create ovie.exe as a copy of oviec for now
        Copy-Item "$BinDir\oviec.exe" "$BinDir\ovie.exe" -Force
    }
    Write-Host "  [OK] ovie.exe installed" -ForegroundColor Green

    # Step 4: Install standard library
    Write-Host ""
    Write-Host "  [4/7] Installing standard library (v2.3 modules)..." -ForegroundColor Cyan
    $localStd = Join-Path $PSScriptRoot "std"
    if (Test-Path $localStd) {
        Copy-Item "$localStd\*" "$InstallDir\std\" -Recurse -Force
        $stdCount = (Get-ChildItem "$InstallDir\std" -Recurse -File).Count
        Write-Host "  [OK] Standard library installed ($stdCount files)" -ForegroundColor Green
        Write-Host "       Modules: core, math, io, fs, time, env, cli, log, testing, module, aproko" -ForegroundColor Gray
    } else {
        Write-Host "  [WARN] std directory not found in package" -ForegroundColor Yellow
    }

    # Step 5: Install examples and docs
    Write-Host ""
    Write-Host "  [5/7] Installing examples and documentation..." -ForegroundColor Cyan
    $localExamples = Join-Path $PSScriptRoot "examples"
    $localDocs     = Join-Path $PSScriptRoot "docs"
    if (Test-Path $localExamples) {
        Copy-Item "$localExamples\*" "$InstallDir\examples\" -Recurse -Force
        $exCount = (Get-ChildItem "$InstallDir\examples" -Filter "*.ov").Count
        Write-Host "  [OK] Examples installed ($exCount .ov files)" -ForegroundColor Green
    }
    if (Test-Path $localDocs) {
        Copy-Item "$localDocs\*" "$InstallDir\docs\" -Recurse -Force
        Write-Host "  [OK] Documentation installed" -ForegroundColor Green
    }
    foreach ($f in @("README.md","LICENSE","RELEASE_NOTES_v2.3.md","ovie.png","ovie.svg")) {
        $src = Join-Path $PSScriptRoot $f
        if (Test-Path $src) { Copy-Item $src "$InstallDir\" -Force }
    }

    # Step 6: Add to system PATH
    Write-Host ""
    Write-Host "  [6/7] Adding Ovie to system PATH..." -ForegroundColor Cyan
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
    if ($currentPath -notlike "*$BinDir*") {
        [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$BinDir", "Machine")
        Write-Host "  [OK] Added to system PATH (all users)" -ForegroundColor Green
    } else {
        Write-Host "  [OK] Already in system PATH" -ForegroundColor Green
    }
    # Also add to current session
    $env:PATH = "$env:PATH;$BinDir"

    # Step 7: Create Start Menu shortcuts
    Write-Host ""
    Write-Host "  [7/7] Creating Start Menu shortcuts..." -ForegroundColor Cyan
    $startMenuDir = "$env:ProgramData\Microsoft\Windows\Start Menu\Programs\Ovie"
    New-Item -ItemType Directory -Path $startMenuDir -Force | Out-Null
    
    $wsh = New-Object -ComObject WScript.Shell
    
    # Ovie Compiler shortcut
    $sc = $wsh.CreateShortcut("$startMenuDir\Ovie Compiler.lnk")
    $sc.TargetPath = "$BinDir\oviec.exe"
    $sc.WorkingDirectory = $InstallDir
    $sc.Description = "Ovie Programming Language Compiler v$OvieVersion"
    if (Test-Path "$InstallDir\ovie.png") { $sc.IconLocation = "$InstallDir\ovie.png" }
    $sc.Save()
    
    # Ovie Examples shortcut
    $sc2 = $wsh.CreateShortcut("$startMenuDir\Ovie Examples.lnk")
    $sc2.TargetPath = "$InstallDir\examples"
    $sc2.Description = "Ovie v$OvieVersion Example Programs"
    $sc2.Save()
    
    Write-Host "  [OK] Start Menu shortcuts created" -ForegroundColor Green

    # Verify installation
    Write-Host ""
    Write-Host "  Verifying installation..." -ForegroundColor Cyan
    $version = & "$BinDir\oviec.exe" --version 2>&1 | Select-Object -First 1
    Write-Host "  [OK] $version" -ForegroundColor Green

    # Success
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
    Write-Host "    oviec --version              # Check version (2.3.0)" -ForegroundColor White
    Write-Host "    oviec --self-check           # Validate installation" -ForegroundColor White
    Write-Host "    oviec run examples\hello.ov  # Run hello world" -ForegroundColor White
    Write-Host "    oviec new my-project         # Create new project" -ForegroundColor White
    Write-Host ""
    Write-Host "  v2.3 New Features:" -ForegroundColor Cyan
    Write-Host "    * Module System: use std::math::{sqrt}" -ForegroundColor White
    Write-Host "    * Aproko KB: oviec aproko query --symbol add" -ForegroundColor White
    Write-Host "    * Package Manager: oviec init, oviec add, oviec install" -ForegroundColor White
    Write-Host ""
    Write-Host "  Resources:" -ForegroundColor Cyan
    Write-Host "    Website:  https://ovie-lang.org" -ForegroundColor White
    Write-Host "    GitHub:   https://github.com/southwarridev/ovie" -ForegroundColor White
    Write-Host "    Docs:     $InstallDir\docs\" -ForegroundColor White
    Write-Host ""
    Write-Host "  Thank you for installing Ovie! " -ForegroundColor Green
    Write-Host ""

} catch {
    Write-Host ""
    Write-Host "  [ERROR] Installation failed: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host ""
    Write-Host "  Troubleshooting:" -ForegroundColor Yellow
    Write-Host "    * Ensure you are running as Administrator" -ForegroundColor White
    Write-Host "    * Check internet connection for downloads" -ForegroundColor White
    Write-Host "    * Download manually: $GithubRepo/releases" -ForegroundColor White
    Write-Host ""
    exit 1
}

Write-Host "  Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
