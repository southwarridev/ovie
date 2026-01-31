# Ovie Programming Language - Windows Installer Script
# Stage 2: Self-Hosted Compiler with Advanced Features
# This script installs Ovie on Windows systems

param(
    [string]$InstallDir = "$env:USERPROFILE\.local\bin",
    [string]$Version = "2.0.0",
    [switch]$AddToPath = $true,
    [switch]$Force = $false,
    [switch]$IncludeExtension = $true
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
    Write-Host ""
    Write-Host "🎯 Ovie Programming Language - Stage 2" -ForegroundColor Magenta
    Write-Host "=======================================" -ForegroundColor Magenta
    Write-Host "Self-Hosted Compiler with Advanced Features" -ForegroundColor Cyan
    Write-Host "Version: $Version" -ForegroundColor Yellow
    Write-Host ""
    
    # Display Ovie logo (ASCII art)
    Write-Host "    ██████╗ ██╗   ██╗██╗███████╗" -ForegroundColor Yellow
    Write-Host "   ██╔═══██╗██║   ██║██║██╔════╝" -ForegroundColor Yellow  
    Write-Host "   ██║   ██║██║   ██║██║█████╗  " -ForegroundColor Yellow
    Write-Host "   ██║   ██║╚██╗ ██╔╝██║██╔══╝  " -ForegroundColor Yellow
    Write-Host "   ╚██████╔╝ ╚████╔╝ ██║███████╗" -ForegroundColor Yellow
    Write-Host "    ╚═════╝   ╚═══╝  ╚═╝╚══════╝" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "   Natural Language Programming" -ForegroundColor Cyan
    Write-Host "   AI-Friendly • Memory Safe • Self-Hosted" -ForegroundColor Green
    Write-Host ""
    
    Write-Status "🚀 Installing Ovie Programming Language v$Version"
    
    # Create install directory
    if (-not (Test-Path $InstallDir)) {
        Write-Status "Creating install directory: $InstallDir"
        New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    }
    
    # Try to download pre-built binaries first
    $DownloadUrl = "https://github.com/southwarridev/ovie/releases/download/v$Version/ovie-windows-x64.zip"
    $TempZip = "$env:TEMP\ovie-windows-x64.zip"
    
    Write-Status "Attempting to download pre-built binaries..."
    
    try {
        Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -ErrorAction Stop
        Write-Success "Downloaded pre-built binaries"
        
        # Extract binaries
        Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
        Remove-Item $TempZip -Force
        
    } catch {
        Write-Warning "Pre-built binaries not available. Building from source..."
        
        # Check if we're in the Ovie source directory
        if (-not (Test-Path "Cargo.toml") -and -not (Test-Path "ovie.toml")) {
            Write-Status "Cloning Ovie repository..."
            $TempDir = "$env:TEMP\ovie-install"
            if (Test-Path $TempDir) {
                Remove-Item $TempDir -Recurse -Force
            }
            git clone "https://github.com/southwarridev/ovie.git" $TempDir
            Set-Location $TempDir
        } else {
            Write-Status "Using current directory (detected Ovie source)"
        }
        
        # Check if we have oviec binary for self-hosted build
        if ((Test-Path "oviec.exe") -or (Test-Path "oviec")) {
            Write-Status "Building with self-hosted Ovie compiler..."
            & .\oviec.exe --build-all --output-dir="$InstallDir"
            Write-Success "Built with self-hosted compiler!"
        } else {
            Write-Warning "No oviec binary found. Attempting bootstrap build..."
            
            # Last resort: use Rust if available (for bootstrapping only)
            if (Test-Command "cargo") {
                Write-Status "Bootstrapping Ovie compiler with Rust (one-time only)..."
                cargo build --release --workspace
                
                if ($LASTEXITCODE -ne 0) {
                    Write-Error "Build failed. Please check the error messages above."
                    exit 1
                }
                
                # Copy binaries
                Copy-Item "target\release\ovie.exe" "$InstallDir\" -Force
                Copy-Item "target\release\oviec.exe" "$InstallDir\" -Force
            } else {
                Write-Error "Cannot build Ovie: No self-hosted compiler or Rust toolchain found"
                Write-Error "Please download a pre-built binary from:"
                Write-Error "https://github.com/southwarridev/ovie/releases"
                exit 1
            }
        }
    }
    
    # Copy Ovie logo
    if (Test-Path "ovie.png") {
        Copy-Item "ovie.png" "$InstallDir\" -Force
        Write-Status "Installed Ovie logo (ovie.png)"
    }
    
    # Install VS Code extension if requested
    if ($IncludeExtension -and (Test-Command "code")) {
        Install-VSCodeExtension
    }
    
    # Add to PATH if requested
    if ($AddToPath) {
        Add-ToPath $InstallDir
    }
    
    # Verify installation
    Test-Installation
    
    Write-Success "🎉 Ovie Programming Language Stage 2 installed successfully!"
    Write-Status "🎯 Features available:"
    Write-Host "  • Self-hosted compiler (oviec)" -ForegroundColor Green
    Write-Host "  • Natural language syntax" -ForegroundColor Green
    Write-Host "  • LLVM backend for optimization" -ForegroundColor Green
    Write-Host "  • WebAssembly compilation" -ForegroundColor Green
    Write-Host "  • Aproko code analyzer" -ForegroundColor Green
    Write-Host "  • Cross-platform support" -ForegroundColor Green
    Write-Host "  • Memory safety guarantees" -ForegroundColor Green
    Write-Host ""
    Write-Status "Get started with: ovie new my-first-project"
}

function Install-VSCodeExtension {
    Write-Status "Installing Ovie VS Code extension..."
    
    try {
        if (Test-Path "extensions\ovie-vscode") {
            Push-Location "extensions\ovie-vscode"
            
            # Install dependencies and build
            if (Test-Command "npm") {
                npm install --silent
                npm run compile
                
                # Package and install extension
                if (Test-Command "vsce") {
                    npm run package
                    $vsix = Get-ChildItem -Name "*.vsix" | Select-Object -First 1
                    if ($vsix) {
                        code --install-extension $vsix --force
                        Write-Success "VS Code extension installed successfully"
                    }
                } else {
                    Write-Warning "vsce not found. Install with: npm install -g vsce"
                }
            } else {
                Write-Warning "npm not found. Skipping VS Code extension installation"
            }
            
            Pop-Location
        }
    } catch {
        Write-Warning "Failed to install VS Code extension: $($_.Exception.Message)"
    }
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
        Write-Success "Ovie Stage 2 installed successfully!"
        Write-Status "Version information:"
        
        try {
            & $OviePath --version 2>$null
        } catch {
            Write-Status "  ovie: installed (CLI tool)"
        }
        
        try {
            & $OviecPath --version 2>$null
        } catch {
            Write-Status "  oviec: installed (self-hosted compiler)"
        }
        
        Write-Status ""
        Write-Status "🎯 Stage 2 Features Available:"
        Write-Host "  • Natural language programming syntax" -ForegroundColor Green
        Write-Host "  • Self-hosted compilation with oviec" -ForegroundColor Green  
        Write-Host "  • LLVM backend optimization" -ForegroundColor Green
        Write-Host "  • WebAssembly compilation target" -ForegroundColor Green
        Write-Host "  • Aproko static analysis" -ForegroundColor Green
        Write-Host "  • Cross-platform deployment" -ForegroundColor Green
        Write-Host "  • Memory safety guarantees" -ForegroundColor Green
        Write-Host "  • AI-friendly development" -ForegroundColor Green
        Write-Host ""
        Write-Status "Quick start:"
        Write-Host "  ovie new my-project    # Create a new project"
        Write-Host "  cd my-project"
        Write-Host "  ovie run               # Run your project"
        Write-Host "  ovie aproko            # Run code analysis"
        Write-Host "  ovie compile --wasm    # Compile to WebAssembly"
        
    } else {
        Write-Error "Installation verification failed"
        Write-Error "Binaries not found in $InstallDir"
        exit 1
    }
}

function Show-Help {
    Write-Host @"
Ovie Programming Language - Windows Installer (Stage 2)

DESCRIPTION:
    Installs Ovie Stage 2 with self-hosted compiler and advanced features.
    
    Stage 2 Features:
    • Self-hosted compiler (oviec)
    • Natural language programming syntax
    • LLVM backend optimization
    • WebAssembly compilation
    • Aproko static analysis
    • Cross-platform support
    • Memory safety guarantees
    • AI-friendly development

USAGE:
    .\install.ps1 [OPTIONS]

OPTIONS:
    -InstallDir <path>       Installation directory (default: $env:USERPROFILE\.local\bin)
    -Version <version>       Version to install (default: 2.0.0)
    -AddToPath              Add install directory to user PATH (default: true)
    -IncludeExtension       Install VS Code extension (default: true)
    -Force                  Force installation even if already installed
    -Help                   Show this help message

EXAMPLES:
    .\install.ps1                                    # Install with defaults
    .\install.ps1 -InstallDir "C:\Tools\Ovie"       # Install to custom directory
    .\install.ps1 -AddToPath:$false                 # Install without modifying PATH
    .\install.ps1 -IncludeExtension:$false          # Skip VS Code extension

REQUIREMENTS:
    - Windows 10 or later
    - PowerShell 5.1 or later
    - Git (for cloning repository if needed)
    - Node.js and npm (optional, for VS Code extension)
    - Visual Studio Code (optional, for extension)
    
    Note: Ovie is now fully self-hosted! No Rust installation required.

STAGE 2 CAPABILITIES:
    - Compile Ovie code with natural language syntax
    - Self-hosted compilation for bootstrap independence
    - Generate optimized native code via LLVM
    - Compile to WebAssembly for web deployment
    - Static analysis with Aproko for code quality
    - Cross-platform deployment (Windows, Linux, macOS)
    - Memory safety without garbage collection
    - AI-friendly syntax for LLM integration

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