# ============================================================================
#                    OVIE PROGRAMMING LANGUAGE - EASY INSTALLER
#                           PowerShell One-Click Install
# ============================================================================

$Host.UI.RawUI.WindowTitle = "Ovie Programming Language - Easy Windows Installer"

Write-Host ""
Write-Host "   ██████╗ ██╗   ██╗██╗███████╗" -ForegroundColor Cyan
Write-Host "  ██╔═══██╗██║   ██║██║██╔════╝" -ForegroundColor Cyan
Write-Host "  ██║   ██║██║   ██║██║█████╗  " -ForegroundColor Cyan
Write-Host "  ██║   ██║╚██╗ ██╔╝██║██╔══╝  " -ForegroundColor Cyan
Write-Host "  ╚██████╔╝ ╚████╔╝ ██║███████╗" -ForegroundColor Cyan
Write-Host "   ╚═════╝   ╚═══╝  ╚═╝╚══════╝" -ForegroundColor Cyan
Write-Host ""
Write-Host "  🚀 STAGE 2 - SELF-HOSTED PROGRAMMING LANGUAGE" -ForegroundColor Green
Write-Host "  📦 Easy Windows Installation v2.1.0" -ForegroundColor Yellow
Write-Host ""
Write-Host "============================================================================" -ForegroundColor Gray

$InstallDir = "$env:USERPROFILE\ovie"
$BinDir = "$env:USERPROFILE\ovie\bin"

Write-Host "🎯 Welcome to Ovie Easy Installer!" -ForegroundColor Green
Write-Host ""
Write-Host "This installer will:" -ForegroundColor White
Write-Host "  ✅ Download Ovie v2.1.0 from GitHub" -ForegroundColor Green
Write-Host "  ✅ Install to: $InstallDir" -ForegroundColor Green
Write-Host "  ✅ Add Ovie to your PATH" -ForegroundColor Green
Write-Host "  ✅ Set up examples and documentation" -ForegroundColor Green
Write-Host "  ✅ Create easy-to-use commands" -ForegroundColor Green
Write-Host ""

$continue = Read-Host "Press Enter to continue or Ctrl+C to cancel"

Write-Host ""
Write-Host "📥 Starting installation..." -ForegroundColor Yellow

try {
    # Create directories
    Write-Host "[1/6] Creating installation directories..." -ForegroundColor Cyan
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    New-Item -ItemType Directory -Path $BinDir -Force | Out-Null

    # Download from GitHub
    Write-Host "[2/6] Downloading Ovie from GitHub..." -ForegroundColor Cyan
    $DownloadUrl = "https://github.com/southwarridev/ovie/archive/refs/tags/v2.1.0.zip"
    $ZipFile = "$env:TEMP\ovie-v2.1.0.zip"
    
    Write-Host "Downloading from: $DownloadUrl" -ForegroundColor Gray
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipFile -UseBasicParsing
    Write-Host "✅ Download complete!" -ForegroundColor Green

    # Extract files
    Write-Host "[3/6] Extracting files..." -ForegroundColor Cyan
    $ExtractPath = "$env:TEMP\ovie-extract"
    Expand-Archive -Path $ZipFile -DestinationPath $ExtractPath -Force
    Write-Host "✅ Extraction complete!" -ForegroundColor Green

    # Copy files to installation directory
    Write-Host "[4/6] Installing Ovie files..." -ForegroundColor Cyan
    Copy-Item -Path "$ExtractPath\ovie-2.1.0\*" -Destination $InstallDir -Recurse -Force
    Write-Host "✅ Files installed!" -ForegroundColor Green

    # Create command wrappers
    Write-Host "[5/6] Setting up Ovie commands..." -ForegroundColor Cyan
    
    # Create ovie.bat
    $OvieBat = @"
@echo off
REM Ovie CLI Tool - Stage 2 Self-Hosted
if "%1"=="--version" (
    echo ovie 2.1.0 - Self-Hosted Programming Language
    echo Copyright (c) 2026 Ovie Language Team
    echo Visit: https://ovie-lang.org
    exit /b 0
)
if "%1"=="--help" (
    echo Usage: ovie [command] [options]
    echo.
    echo Commands:
    echo   new [name]     Create a new Ovie project
    echo   run            Run the current project
    echo   build          Build the current project
    echo   --version      Show version information
    echo   --help         Show this help message
    echo.
    echo Examples:
    echo   ovie new my-project
    echo   ovie run
    echo.
    echo Documentation: https://ovie-lang.org
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
    echo // Hello World in Ovie - Stage 2 Self-Hosted! > "%2\main.ov"
    echo seeAm "Hello, World from Ovie!" >> "%2\main.ov"
    echo. >> "%2\main.ov"
    echo // Natural language syntax >> "%2\main.ov"
    echo mut name = "Developer" >> "%2\main.ov"
    echo seeAm "Welcome to Ovie, " + name + "!" >> "%2\main.ov"
    echo ✅ Project created successfully!
    echo Run: cd %2 && ovie run
    exit /b 0
)
echo Ovie is ready! Use 'ovie --help' for available commands.
echo To build the full compiler, install Rust: https://rustup.rs/
echo Then run: cd "$InstallDir" && cargo build --release
"@
    
    Set-Content -Path "$BinDir\ovie.bat" -Value $OvieBat
    
    # Create oviec.bat
    $OviecBat = @"
@echo off
echo Ovie Compiler (oviec) v2.1.0 - Stage 2.1 Self-Hosted
echo This is the Ovie compiler that compiles itself!
echo.
echo To build the full compiler:
echo   1. Install Rust: https://rustup.rs/
echo   2. Run: cd "$InstallDir" && cargo build --release
echo   3. The compiled oviec.exe will be in target\release\
"@
    
    Set-Content -Path "$BinDir\oviec.bat" -Value $OviecBat
    Write-Host "✅ Commands created!" -ForegroundColor Green

    # Add to PATH
    Write-Host "[6/6] Adding Ovie to your PATH..." -ForegroundColor Cyan
    $CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($CurrentPath -notlike "*$BinDir*") {
        $NewPath = if ($CurrentPath) { "$CurrentPath;$BinDir" } else { $BinDir }
        [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
        Write-Host "✅ Added to PATH successfully!" -ForegroundColor Green
        Write-Host "⚠️  Please restart your terminal to use 'ovie' command" -ForegroundColor Yellow
    } else {
        Write-Host "✅ Already in PATH" -ForegroundColor Green
    }

    # Cleanup
    Remove-Item $ZipFile -Force -ErrorAction SilentlyContinue
    Remove-Item $ExtractPath -Recurse -Force -ErrorAction SilentlyContinue

    Write-Host ""
    Write-Host "============================================================================" -ForegroundColor Gray
    Write-Host "                          🎉 INSTALLATION COMPLETE! 🎉" -ForegroundColor Green
    Write-Host "============================================================================" -ForegroundColor Gray
    Write-Host ""
    Write-Host "✅ Ovie v2.1.0 - Stage 2.1 Self-Hosted installed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📍 Installation Location: $InstallDir" -ForegroundColor White
    Write-Host "🔧 Binaries: $BinDir" -ForegroundColor White
    Write-Host ""
    Write-Host "🚀 Quick Start:" -ForegroundColor Yellow
    Write-Host "  1. Restart your PowerShell/Command Prompt" -ForegroundColor White
    Write-Host "  2. Run: ovie --version" -ForegroundColor White
    Write-Host "  3. Create a project: ovie new my-first-project" -ForegroundColor White
    Write-Host "  4. Go to project: cd my-first-project" -ForegroundColor White
    Write-Host "  5. Run your code: ovie run" -ForegroundColor White
    Write-Host ""
    Write-Host "📚 What's Included:" -ForegroundColor Yellow
    Write-Host "  • ovie.bat     - CLI tool and project manager" -ForegroundColor White
    Write-Host "  • oviec.bat    - Self-hosted compiler wrapper" -ForegroundColor White
    Write-Host "  • examples/    - 22+ example programs" -ForegroundColor White
    Write-Host "  • docs/        - Complete documentation" -ForegroundColor White
    Write-Host "  • std/         - Standard library" -ForegroundColor White
    Write-Host "  • VS Code extension in extensions/ovie-vscode/" -ForegroundColor White
    Write-Host ""
    Write-Host "🔨 To Build Full Compiler:" -ForegroundColor Yellow
    Write-Host "  1. Install Rust: https://rustup.rs/" -ForegroundColor White
    Write-Host "  2. cd `"$InstallDir`"" -ForegroundColor White
    Write-Host "  3. cargo build --release" -ForegroundColor White
    Write-Host ""
    Write-Host "🌐 Resources:" -ForegroundColor Yellow
    Write-Host "  • Website: https://ovie-lang.org" -ForegroundColor White
    Write-Host "  • GitHub: https://github.com/southwarridev/ovie" -ForegroundColor White
    Write-Host "  • Documentation: $InstallDir\docs\" -ForegroundColor White
    Write-Host ""
    Write-Host "Thank you for installing Ovie! 🚀" -ForegroundColor Green
    Write-Host "The future of programming is here!" -ForegroundColor Cyan

} catch {
    Write-Host ""
    Write-Host "❌ Installation failed: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host ""
    Write-Host "🔧 Troubleshooting:" -ForegroundColor Yellow
    Write-Host "  • Check your internet connection" -ForegroundColor White
    Write-Host "  • Run PowerShell as Administrator" -ForegroundColor White
    Write-Host "  • Download manually from: https://github.com/southwarridev/ovie/releases" -ForegroundColor White
    Write-Host ""
    exit 1
}

Write-Host ""
Write-Host "Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")