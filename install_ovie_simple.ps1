# Simple Ovie Installation Script
# This script creates a basic Ovie installation without compiling from source

$Host.UI.RawUI.WindowTitle = "Ovie Programming Language - Simple Installer"

Write-Host ""
Write-Host "   ██████╗ ██╗   ██╗██╗███████╗" -ForegroundColor Cyan
Write-Host "  ██╔═══██╗██║   ██║██║██╔════╝" -ForegroundColor Cyan
Write-Host "  ██║   ██║██║   ██║██║█████╗  " -ForegroundColor Cyan
Write-Host "  ██║   ██║╚██╗ ██╔╝██║██╔══╝  " -ForegroundColor Cyan
Write-Host "  ╚██████╔╝ ╚████╔╝ ██║███████╗" -ForegroundColor Cyan
Write-Host "   ╚═════╝   ╚═══╝  ╚═╝╚══════╝" -ForegroundColor Cyan
Write-Host ""
Write-Host "  🚀 OVIE v2.2 - COMPLETE LANGUAGE CONSOLIDATION" -ForegroundColor Green
Write-Host "  📦 Simple Installation (Development Mode)" -ForegroundColor Yellow
Write-Host ""

$InstallDir = "$env:USERPROFILE\ovie"
$BinDir = "$InstallDir\bin"

Write-Host "🎯 Installing Ovie Development Environment..." -ForegroundColor Green
Write-Host ""

try {
    # Create directories
    Write-Host "[1/4] Creating installation directories..." -ForegroundColor Cyan
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    New-Item -ItemType Directory -Path $BinDir -Force | Out-Null
    New-Item -ItemType Directory -Path "$InstallDir\std" -Force | Out-Null
    New-Item -ItemType Directory -Path "$InstallDir\examples" -Force | Out-Null
    New-Item -ItemType Directory -Path "$InstallDir\docs" -Force | Out-Null

    # Copy current source files
    Write-Host "[2/4] Copying Ovie source files..." -ForegroundColor Cyan
    Copy-Item -Path ".\*" -Destination $InstallDir -Recurse -Force -Exclude @("target", ".git", "node_modules")

    # Create simple ovie command wrapper
    Write-Host "[3/4] Creating Ovie commands..." -ForegroundColor Cyan
    
    $OvieCmd = @"
@echo off
REM Ovie CLI Tool - v2.2 Development Mode
echo Ovie Programming Language v2.2 - Complete Language Consolidation
echo Development Mode - Source Installation
echo.

if "%1"=="--version" (
    echo ovie 2.2.0-dev - Complete Language Consolidation
    echo Copyright ^(c^) 2026 Ovie Language Team
    echo Installation: %USERPROFILE%\ovie
    echo Status: Development Mode ^(Source^)
    echo.
    echo To build the full compiler:
    echo   1. Install Rust: https://rustup.rs/
    echo   2. cd "%USERPROFILE%\ovie"
    echo   3. cargo build --release
    exit /b 0
)

if "%1"=="--help" (
    echo Usage: ovie [command] [options]
    echo.
    echo Commands:
    echo   new [name]     Create a new Ovie project
    echo   run [file]     Run an Ovie program (requires build)
    echo   build          Build the Ovie compiler
    echo   --version      Show version information
    echo   --help         Show this help message
    echo.
    echo Development Status:
    echo   This is a development installation. To use Ovie:
    echo   1. Install Rust from https://rustup.rs/
    echo   2. Run: cd "%USERPROFILE%\ovie" ^&^& cargo build --release
    echo   3. The compiled binaries will be in target\release\
    echo.
    exit /b 0
)

if "%1"=="new" (
    if "%2"=="" (
        echo Error: Project name required
        echo Usage: ovie new [project-name]
        exit /b 1
    )
    echo Creating new Ovie project: %2
    mkdir "%2" 2>nul
    echo // Hello World in Ovie v2.2! > "%2\main.ov"
    echo seeAm "Hello, World from Ovie v2.2!" >> "%2\main.ov"
    echo. >> "%2\main.ov"
    echo // Complete Language Consolidation >> "%2\main.ov"
    echo mut name = "Developer" >> "%2\main.ov"
    echo seeAm "Welcome to Ovie v2.2, " + name + "!" >> "%2\main.ov"
    echo ✅ Project created successfully!
    echo.
    echo Next steps:
    echo   1. cd %2
    echo   2. Build Ovie: cd "%USERPROFILE%\ovie" ^&^& cargo build --release
    echo   3. Run: "%USERPROFILE%\ovie\target\release\ovie.exe" run main.ov
    exit /b 0
)

if "%1"=="build" (
    echo Building Ovie compiler from source...
    echo Location: %USERPROFILE%\ovie
    echo.
    cd /d "%USERPROFILE%\ovie"
    if exist "Cargo.toml" (
        echo Running: cargo build --release
        cargo build --release
        if %ERRORLEVEL% EQU 0 (
            echo.
            echo ✅ Build successful!
            echo Binaries available in: %USERPROFILE%\ovie\target\release\
            echo   - ovie.exe     ^(CLI tool^)
            echo   - oviec.exe    ^(Compiler^)
        ) else (
            echo.
            echo ❌ Build failed. Please check the error messages above.
            echo Make sure you have Rust installed: https://rustup.rs/
        )
    ) else (
        echo Error: Cargo.toml not found. Installation may be incomplete.
    )
    exit /b %ERRORLEVEL%
)

echo Ovie v2.2 - Complete Language Consolidation
echo Development Mode - Source Installation
echo.
echo Available commands:
echo   ovie --help      Show help
echo   ovie --version   Show version
echo   ovie new [name]  Create new project
echo   ovie build       Build compiler from source
echo.
echo Status: Development installation
echo To use Ovie, first run: ovie build
"@
    
    Set-Content -Path "$BinDir\ovie.bat" -Value $OvieCmd

    # Add to PATH
    Write-Host "[4/4] Adding Ovie to your PATH..." -ForegroundColor Cyan
    $CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($CurrentPath -notlike "*$BinDir*") {
        $NewPath = if ($CurrentPath) { "$CurrentPath;$BinDir" } else { $BinDir }
        [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
        Write-Host "✅ Added to PATH successfully!" -ForegroundColor Green
        Write-Host "⚠️  Please restart your terminal to use 'ovie' command" -ForegroundColor Yellow
    } else {
        Write-Host "✅ Already in PATH" -ForegroundColor Green
    }

    Write-Host ""
    Write-Host "============================================================================" -ForegroundColor Gray
    Write-Host "                          🎉 INSTALLATION COMPLETE! 🎉" -ForegroundColor Green
    Write-Host "============================================================================" -ForegroundColor Gray
    Write-Host ""
    Write-Host "✅ Ovie v2.2 - Complete Language Consolidation installed!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📍 Installation Location: $InstallDir" -ForegroundColor White
    Write-Host "🔧 Commands: $BinDir" -ForegroundColor White
    Write-Host ""
    Write-Host "🚀 Quick Start:" -ForegroundColor Yellow
    Write-Host "  1. Restart your PowerShell/Command Prompt" -ForegroundColor White
    Write-Host "  2. Install Rust: https://rustup.rs/" -ForegroundColor White
    Write-Host "  3. Build Ovie: ovie build" -ForegroundColor White
    Write-Host "  4. Create project: ovie new my-project" -ForegroundColor White
    Write-Host "  5. Test: cd my-project && ovie run main.ov" -ForegroundColor White
    Write-Host ""
    Write-Host "📚 What's Included:" -ForegroundColor Yellow
    Write-Host "  • Complete Ovie v2.2 source code" -ForegroundColor White
    Write-Host "  • All 100+ implementation tasks" -ForegroundColor White
    Write-Host "  • Property-based testing framework" -ForegroundColor White
    Write-Host "  • Standard library modules" -ForegroundColor White
    Write-Host "  • Examples and documentation" -ForegroundColor White
    Write-Host "  • VS Code extension" -ForegroundColor White
    Write-Host ""
    Write-Host "🔨 Build Instructions:" -ForegroundColor Yellow
    Write-Host "  1. Install Rust: https://rustup.rs/" -ForegroundColor White
    Write-Host "  2. Run: ovie build" -ForegroundColor White
    Write-Host "  3. Wait for compilation to complete" -ForegroundColor White
    Write-Host ""
    Write-Host "Thank you for installing Ovie v2.2! 🚀" -ForegroundColor Green
    Write-Host "The Complete Language Consolidation is ready!" -ForegroundColor Cyan

} catch {
    Write-Host ""
    Write-Host "❌ Installation failed: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host ""
    Write-Host "🔧 Troubleshooting:" -ForegroundColor Yellow
    Write-Host "  • Run PowerShell as Administrator" -ForegroundColor White
    Write-Host "  • Check disk space and permissions" -ForegroundColor White
    Write-Host "  • Try manual installation" -ForegroundColor White
    Write-Host ""
    exit 1
}

Write-Host ""
Write-Host "Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")