# Ovie Programming Language - Windows Installer Script
# This script installs Ovie on Windows systems

param(
    [string]$InstallDir = "$env:USERPROFILE\.local\bin",
    [string]$Version = "0.1.0",
    [switch]$AddToPath = $true,
    [switch]$Force = $false
)

# Colors for output (if supported)
$Host.UI.RawUI.ForegroundColor = "White"

function Write-Status {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Test-Command {
    param([string]$Command)
    try {
        Get-Command $Command -ErrorAction Stop | Out-Null
        return $true
    } catch {
        return $false
    }
}

function Install-Ovie {
    Write-Status "🚀 Installing Ovie Programming Language v$Version"
    
    # Check if Rust is installed
    if (-not (Test-Command "cargo")) {
        Write-Error "Rust is not installed. Please install Rust first:"
        Write-Error "Visit: https://rustup.rs/"
        exit 1
    }
    
    # Create install directory
    if (-not (Test-Path $InstallDir)) {
        Write-Status "Creating install directory: $InstallDir"
        New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    }
    
    # Check if we're in the Ovie source directory
    if (-not (Test-Path "Cargo.toml")) {
        Write-Status "Cloning Ovie repository..."
        $TempDir = "$env:TEMP\ovie-install"
        if (Test-Path $TempDir) {
            Remove-Item $TempDir -Recurse -Force
        }
        git clone "https://github.com/ovie-lang/ovie.git" $TempDir
        Set-Location $TempDir
    } else {
        Write-Status "Using current directory (detected Ovie source)"
    }
    
    # Build release
    Write-Status "Building release binaries..."
    cargo build --release --workspace
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Build failed. Please check the error messages above."
        exit 1
    }
    
    # Copy binaries
    Write-Status "Installing binaries to $InstallDir..."
    Copy-Item "target\release\ovie.exe" "$InstallDir\" -Force
    Copy-Item "target\release\oviec.exe" "$InstallDir\" -Force
    
    # Add to PATH if requested
    if ($AddToPath) {
        Add-ToPath $InstallDir
    }
    
    # Verify installation
    Test-Installation
    
    Write-Success "🎉 Ovie Programming Language installed successfully!"
    Write-Status "Get started with: ovie new my-first-project"
}

function Add-ToPath {
    param([string]$Directory)
    
    $CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    
    if ($CurrentPath -notlike "*$Directory*") {
        Write-Status "Adding $Directory to user PATH..."
        
        $NewPath = if ($CurrentPath) { "$CurrentPath;$Directory" } else { $Directory }
        [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
        
        # Update current session PATH
        $env:PATH = "$env:PATH;$Directory"
        
        Write-Success "Added to PATH. You may need to restart your terminal."
    } else {
        Write-Status "Directory already in PATH"
    }
}

function Test-Installation {
    Write-Status "Verifying installation..."
    
    $OviePath = Join-Path $InstallDir "ovie.exe"
    $OviecPath = Join-Path $InstallDir "oviec.exe"
    
    if ((Test-Path $OviePath) -and (Test-Path $OviecPath)) {
        Write-Success "Ovie installed successfully!"
        Write-Status "Version information:"
        
        try {
            & $OviePath --version 2>$null
        } catch {
            Write-Status "  ovie: installed"
        }
        
        try {
            & $OviecPath --help >$null 2>&1
            Write-Status "  oviec: installed"
        } catch {
            Write-Status "  oviec: installed"
        }
        
        Write-Status "Quick start:"
        Write-Host "  ovie new my-project    # Create a new project"
        Write-Host "  cd my-project"
        Write-Host "  ovie run               # Run your project"
        
    } else {
        Write-Error "Installation verification failed"
        Write-Error "Binaries not found in $InstallDir"
        exit 1
    }
}

function Show-Help {
    Write-Host @"
Ovie Programming Language - Windows Installer

USAGE:
    .\install.ps1 [OPTIONS]

OPTIONS:
    -InstallDir <path>    Installation directory (default: $env:USERPROFILE\.local\bin)
    -Version <version>    Version to install (default: 0.1.0)
    -AddToPath           Add install directory to user PATH (default: true)
    -Force               Force installation even if already installed
    -Help                Show this help message

EXAMPLES:
    .\install.ps1                                    # Install with defaults
    .\install.ps1 -InstallDir "C:\Tools\Ovie"       # Install to custom directory
    .\install.ps1 -AddToPath:$false                 # Install without modifying PATH

REQUIREMENTS:
    - Windows 10 or later
    - PowerShell 5.1 or later
    - Rust toolchain (https://rustup.rs/)
    - Git (for cloning repository)

"@
}

# Main execution
try {
    if ($args -contains "-Help" -or $args -contains "--help" -or $args -contains "-h") {
        Show-Help
        exit 0
    }
    
    Install-Ovie
} catch {
    Write-Error "Installation failed: $($_.Exception.Message)"
    exit 1
}