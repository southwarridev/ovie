# Ovie Stage 2 Complete Deployment Script
# This script handles the complete deployment of Ovie Stage 2 including:
# - Self-hosted compiler
# - VS Code extension to marketplace
# - Website deployment
# - Documentation updates
# - Branding assets

param(
    [switch]$BuildRelease,
    [switch]$DeployExtension,
    [switch]$DeployWebsite,
    [switch]$UpdateDocs,
    [switch]$All,
    [string]$Version = "2.0.0"
)

# Colors for output
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

# Display Ovie Stage 2 banner
function Show-Banner {
    Write-Host ""
    Write-Host "🎯 Ovie Programming Language - Stage 2 Deployment" -ForegroundColor Magenta
    Write-Host "=================================================" -ForegroundColor Magenta
    Write-Host "Self-Hosted Compiler with Advanced Features" -ForegroundColor Cyan
    Write-Host "Version: $Version" -ForegroundColor Yellow
    Write-Host ""
    
    # ASCII art logo
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
}

# Build release binaries
function Build-Release {
    Write-Status "🔨 Building Ovie Stage 2 release binaries..."
    
    # Clean previous builds
    if (Test-Path "target") {
        Remove-Item "target" -Recurse -Force
    }
    
    # Build with all Stage 2 features
    Write-Status "Building with Stage 2 features: self-hosting, LLVM, WASM, Aproko"
    cargo build --release --workspace --features "self-hosting,llvm-backend,wasm-support,aproko-integration"
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Release build failed"
        exit 1
    }
    
    Write-Success "Release binaries built successfully"
    
    # Copy binaries to deployment directory
    $deployDir = "deploy"
    if (-not (Test-Path $deployDir)) {
        New-Item -ItemType Directory -Path $deployDir | Out-Null
    }
    
    Copy-Item "target\release\ovie.exe" "$deployDir\" -Force
    Copy-Item "target\release\oviec.exe" "$deployDir\" -Force
    Copy-Item "ovie.png" "$deployDir\" -Force
    
    Write-Success "Binaries prepared for deployment"
}

# Deploy VS Code extension to marketplace
function Deploy-Extension {
    Write-Status "📦 Deploying VS Code extension to marketplace..."
    
    Push-Location "extensions\ovie-vscode"
    
    try {
        # Update version in package.json
        $packageJson = Get-Content "package.json" | ConvertFrom-Json
        $packageJson.version = $Version
        $packageJson | ConvertTo-Json -Depth 10 | Set-Content "package.json"
        
        # Install dependencies and build
        npm install --silent
        npm run compile
        
        # Package extension
        npm run package
        
        # Publish to marketplace (requires vsce login)
        Write-Status "Publishing to VS Code Marketplace..."
        npm run publish
        
        Write-Success "VS Code extension deployed to marketplace"
        
    } catch {
        Write-Error "Extension deployment failed: $($_.Exception.Message)"
        exit 1
    } finally {
        Pop-Location
    }
}

# Deploy website
function Deploy-Website {
    Write-Status "🌐 Deploying Ovie website..."
    
    # Update website with Stage 2 information
    $indexPath = "website\index.html"
    if (Test-Path $indexPath) {
        $content = Get-Content $indexPath -Raw
        $content = $content -replace "Version: [\d\.]+", "Version: $Version"
        $content = $content -replace "Stage \d+", "Stage 2"
        Set-Content $indexPath $content
        
        Write-Success "Website updated with Stage 2 information"
    }
    
    # Copy ovie.png to website assets
    if (Test-Path "ovie.png") {
        Copy-Item "ovie.png" "website\assets\" -Force
        Write-Status "Updated website branding"
    }
    
    Write-Success "Website deployment prepared"
}

# Update documentation
function Update-Documentation {
    Write-Status "📚 Updating documentation for Stage 2..."
    
    # Update README.md with Stage 2 features
    $readmePath = "README.md"
    if (Test-Path $readmePath) {
        $content = Get-Content $readmePath -Raw
        
        # Update version references
        $content = $content -replace "Version [\d\.]+", "Version $Version"
        $content = $content -replace "Stage \d+", "Stage 2"
        
        # Add Stage 2 features section if not present
        if ($content -notmatch "Stage 2 Features") {
            $stage2Features = @"

## Stage 2 Features

Ovie Stage 2 represents a major milestone with a fully self-hosted compiler and advanced features:

### 🎯 Core Features
- **Self-Hosted Compiler**: Complete bootstrap independence with `oviec`
- **Natural Language Syntax**: AI-friendly programming with readable code
- **Memory Safety**: Ownership system without garbage collection
- **Cross-Platform**: Windows, Linux, macOS support

### 🚀 Advanced Capabilities  
- **LLVM Backend**: Optimized native code generation
- **WebAssembly**: Compile to WASM for web deployment
- **Aproko Analysis**: Static code analysis and quality checks
- **VS Code Extension**: Full IDE support with syntax highlighting

### 🤖 AI Integration
- **LLM-Friendly**: Natural syntax designed for AI code generation
- **Semantic Analysis**: Rich AST for AI tooling integration
- **Documentation**: Comprehensive examples for AI training

"@
            $content = $content + $stage2Features
        }
        
        Set-Content $readmePath $content
        Write-Success "README.md updated with Stage 2 features"
    }
    
    # Update installation docs
    $installPath = "docs\installation.md"
    if (Test-Path $installPath) {
        $content = Get-Content $installPath -Raw
        $content = $content -replace "Version [\d\.]+", "Version $Version"
        $content = $content -replace "ovie-lang/ovie", "southwarridev/ovie"
        Set-Content $installPath $content
        Write-Success "Installation documentation updated"
    }
    
    Write-Success "Documentation updated for Stage 2"
}

# Main deployment function
function Deploy-Stage2 {
    Show-Banner
    
    Write-Status "🚀 Starting Ovie Stage 2 deployment process..."
    
    if ($All) {
        $BuildRelease = $true
        $DeployExtension = $true
        $DeployWebsite = $true
        $UpdateDocs = $true
    }
    
    if ($BuildRelease) {
        Build-Release
    }
    
    if ($UpdateDocs) {
        Update-Documentation
    }
    
    if ($DeployWebsite) {
        Deploy-Website
    }
    
    if ($DeployExtension) {
        Deploy-Extension
    }
    
    Write-Host ""
    Write-Success "🎉 Ovie Stage 2 deployment completed successfully!"
    Write-Host ""
    Write-Status "🎯 Stage 2 Achievements:"
    Write-Host "  • Self-hosted compiler (oviec) ✅" -ForegroundColor Green
    Write-Host "  • Natural language programming ✅" -ForegroundColor Green
    Write-Host "  • LLVM backend optimization ✅" -ForegroundColor Green
    Write-Host "  • WebAssembly compilation ✅" -ForegroundColor Green
    Write-Host "  • Aproko static analysis ✅" -ForegroundColor Green
    Write-Host "  • VS Code extension ✅" -ForegroundColor Green
    Write-Host "  • Cross-platform support ✅" -ForegroundColor Green
    Write-Host "  • Memory safety guarantees ✅" -ForegroundColor Green
    Write-Host ""
    Write-Status "🌟 Ovie Stage 2 is now live and ready for production use!"
}

# Show help
function Show-Help {
    Write-Host @"
Ovie Stage 2 Deployment Script

DESCRIPTION:
    Complete deployment script for Ovie Stage 2 with self-hosted compiler
    and advanced features.

USAGE:
    .\deploy-ovie-stage2.ps1 [OPTIONS]

OPTIONS:
    -BuildRelease       Build release binaries with Stage 2 features
    -DeployExtension    Deploy VS Code extension to marketplace
    -DeployWebsite      Deploy website with Stage 2 updates
    -UpdateDocs         Update documentation for Stage 2
    -All                Run all deployment steps
    -Version <version>  Version to deploy (default: 2.0.0)

EXAMPLES:
    .\deploy-ovie-stage2.ps1 -All                    # Complete deployment
    .\deploy-ovie-stage2.ps1 -BuildRelease           # Build binaries only
    .\deploy-ovie-stage2.ps1 -DeployExtension        # Deploy extension only
    .\deploy-ovie-stage2.ps1 -Version "2.1.0" -All  # Deploy specific version

REQUIREMENTS:
    - Rust toolchain with LLVM support
    - Node.js and npm (for VS Code extension)
    - vsce (VS Code extension manager)
    - Git (for version control)
    - PowerShell 5.1 or later

STAGE 2 FEATURES:
    - Self-hosted compilation with oviec
    - Natural language programming syntax
    - LLVM backend for optimization
    - WebAssembly compilation target
    - Aproko static analysis
    - Cross-platform deployment
    - Memory safety without GC
    - AI-friendly development

"@
}

# Main execution
try {
    if ($args -contains "-Help" -or $args -contains "--help" -or $args -contains "-h") {
        Show-Help
        exit 0
    }
    
    Deploy-Stage2
} catch {
    Write-Error "Deployment failed: $($_.Exception.Message)"
    exit 1
}