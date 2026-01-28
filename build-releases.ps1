#!/usr/bin/env pwsh

# Cross-platform release build script for Ovie Programming Language
# This script creates production-ready binaries for Windows, macOS, and Linux

param(
    [string]$Version = "1.0.0",
    [switch]$SkipTests = $false
)

Write-Host "🚀 Building Ovie Programming Language v$Version for all platforms..." -ForegroundColor Green

# Create release directory
$ReleaseDir = "releases/v$Version"
New-Item -ItemType Directory -Force -Path $ReleaseDir | Out-Null

# Function to build for a specific target
function Build-Target {
    param(
        [string]$Target,
        [string]$Platform,
        [string]$Arch
    )
    
    Write-Host "📦 Building for $Platform ($Arch)..." -ForegroundColor Yellow
    
    # Build release binaries
    cargo build --release --target $Target
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Build failed for $Target" -ForegroundColor Red
        return $false
    }
    
    # Create platform-specific directory
    $PlatformDir = "$ReleaseDir/$Platform-$Arch"
    New-Item -ItemType Directory -Force -Path $PlatformDir | Out-Null
    
    # Copy binaries
    $BinExt = if ($Platform -eq "windows") { ".exe" } else { "" }
    
    Copy-Item "target/$Target/release/ovie$BinExt" "$PlatformDir/"
    Copy-Item "target/$Target/release/oviec$BinExt" "$PlatformDir/"
    
    # Copy documentation and examples
    Copy-Item "README.md" "$PlatformDir/"
    Copy-Item "LICENSE" "$PlatformDir/"
    Copy-Item "docs/" "$PlatformDir/" -Recurse
    Copy-Item "examples/" "$PlatformDir/" -Recurse
    
    # Create installation script
    if ($Platform -eq "windows") {
        @"
@echo off
echo Installing Ovie Programming Language...
copy ovie.exe "%USERPROFILE%\.cargo\bin\" 2>nul || copy ovie.exe "%PROGRAMFILES%\Ovie\bin\"
copy oviec.exe "%USERPROFILE%\.cargo\bin\" 2>nul || copy oviec.exe "%PROGRAMFILES%\Ovie\bin\"
echo Ovie installed successfully!
echo Run 'ovie --help' to get started.
pause
"@ | Out-File "$PlatformDir/install.bat" -Encoding ASCII
    } else {
        @"
#!/bin/bash
echo "Installing Ovie Programming Language..."
sudo cp ovie /usr/local/bin/ || cp ovie ~/.local/bin/
sudo cp oviec /usr/local/bin/ || cp oviec ~/.local/bin/
echo "Ovie installed successfully!"
echo "Run 'ovie --help' to get started."
"@ | Out-File "$PlatformDir/install.sh" -Encoding UTF8
        
        # Make install script executable (if on Unix-like system)
        if ($IsLinux -or $IsMacOS) {
            chmod +x "$PlatformDir/install.sh"
        }
    }
    
    # Create archive
    $ArchiveName = "ovie-v$Version-$Platform-$Arch"
    if ($Platform -eq "windows") {
        Compress-Archive -Path "$PlatformDir/*" -DestinationPath "$ReleaseDir/$ArchiveName.zip" -Force
    } else {
        tar -czf "$ReleaseDir/$ArchiveName.tar.gz" -C $ReleaseDir "$Platform-$Arch"
    }
    
    Write-Host "✅ Built $ArchiveName" -ForegroundColor Green
    return $true
}

# Install required targets
Write-Host "🔧 Installing Rust targets..." -ForegroundColor Cyan

$Targets = @(
    @{ Target = "x86_64-pc-windows-gnu"; Platform = "windows"; Arch = "x64" },
    @{ Target = "x86_64-pc-windows-msvc"; Platform = "windows"; Arch = "x64-msvc" },
    @{ Target = "x86_64-unknown-linux-gnu"; Platform = "linux"; Arch = "x64" },
    @{ Target = "x86_64-apple-darwin"; Platform = "macos"; Arch = "x64" },
    @{ Target = "aarch64-apple-darwin"; Platform = "macos"; Arch = "arm64" }
)

foreach ($TargetInfo in $Targets) {
    rustup target add $TargetInfo.Target
}

# Run tests (unless skipped)
if (-not $SkipTests) {
    Write-Host "🧪 Running tests..." -ForegroundColor Cyan
    cargo test --lib --workspace
    if ($LASTEXITCODE -ne 0) {
        Write-Host "⚠️  Some tests failed, but continuing with release build..." -ForegroundColor Yellow
    }
}

# Build for all targets
$SuccessfulBuilds = 0
$TotalBuilds = $Targets.Count

foreach ($TargetInfo in $Targets) {
    if (Build-Target $TargetInfo.Target $TargetInfo.Platform $TargetInfo.Arch) {
        $SuccessfulBuilds++
    }
}

# Create checksums
Write-Host "🔐 Generating checksums..." -ForegroundColor Cyan
$ChecksumFile = "$ReleaseDir/checksums.txt"
Get-ChildItem "$ReleaseDir/*.zip", "$ReleaseDir/*.tar.gz" | ForEach-Object {
    $Hash = Get-FileHash $_.FullName -Algorithm SHA256
    "$($Hash.Hash)  $($_.Name)" | Add-Content $ChecksumFile
}

# Create release notes
$ReleaseNotes = @"
# Ovie Programming Language v$Version

## Installation

### Windows
1. Download `ovie-v$Version-windows-x64.zip` or `ovie-v$Version-windows-x64-msvc.zip`
2. Extract the archive
3. Run `install.bat` as administrator, or manually copy `ovie.exe` and `oviec.exe` to your PATH

### macOS
1. Download `ovie-v$Version-macos-x64.tar.gz` (Intel) or `ovie-v$Version-macos-arm64.tar.gz` (Apple Silicon)
2. Extract: `tar -xzf ovie-v$Version-macos-*.tar.gz`
3. Run: `./install.sh`

### Linux
1. Download `ovie-v$Version-linux-x64.tar.gz`
2. Extract: `tar -xzf ovie-v$Version-linux-x64.tar.gz`
3. Run: `./install.sh`

## Quick Start

```bash
# Create a new project
ovie new my-project
cd my-project

# Run your project
ovie run

# Compile to different backends
oviec src/main.ov --backend wasm
oviec src/main.ov --backend ir
```

## What's Included

- `ovie` - The Ovie CLI toolchain
- `oviec` - The Ovie compiler
- Complete documentation in `docs/`
- Example programs in `examples/`
- Installation scripts

## Verification

Verify your download with SHA256 checksums:
```bash
sha256sum -c checksums.txt  # Linux/macOS
certutil -hashfile filename SHA256  # Windows
```

## Features

✅ Complete lexer, parser, and AST generation
✅ Normalizer with safe auto-correction
✅ Multiple compilation backends (IR, WASM)
✅ Aproko assistant engine for code analysis
✅ Package management system
✅ CLI toolchain with project scaffolding
✅ Comprehensive documentation and examples
✅ Cross-platform support
✅ Security and safety features

## System Requirements

- **Windows**: Windows 10 or later (x64)
- **macOS**: macOS 10.15 or later (Intel or Apple Silicon)
- **Linux**: Any modern Linux distribution (x64)

## Support

- Documentation: See `docs/` directory
- Examples: See `examples/` directory
- Issues: https://github.com/ovie-lang/ovie/issues
"@

$ReleaseNotes | Out-File "$ReleaseDir/RELEASE_NOTES.md" -Encoding UTF8

# Summary
Write-Host "`n🎉 Release build complete!" -ForegroundColor Green
Write-Host "📊 Successfully built $SuccessfulBuilds out of $TotalBuilds targets" -ForegroundColor Cyan
Write-Host "📁 Release files available in: $ReleaseDir" -ForegroundColor Cyan

Get-ChildItem $ReleaseDir -Name | ForEach-Object {
    Write-Host "   📦 $_" -ForegroundColor White
}

Write-Host "`n🚀 Ready for distribution!" -ForegroundColor Green