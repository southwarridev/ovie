# Ovie Programming Language - Local Development Script (Windows)
# This script sets up and runs Ovie completely offline

Write-Host "🏠 Ovie Programming Language - Offline Development Mode" -ForegroundColor Green
Write-Host "This script keeps everything local and offline-first!" -ForegroundColor Green
Write-Host ""

# Function to run a command and show status
function Invoke-Step {
    param(
        [string]$Description,
        [scriptblock]$Command
    )
    
    Write-Host "🔧 $Description..." -ForegroundColor Yellow
    try {
        & $Command
        Write-Host "✅ $Description complete!" -ForegroundColor Green
    } catch {
        Write-Host "❌ $Description failed!" -ForegroundColor Red
        Write-Host $_.Exception.Message -ForegroundColor Red
        exit 1
    }
    Write-Host ""
}

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml") -or -not (Test-Path "oviec")) {
    Write-Host "❌ Please run this script from the Ovie project root directory" -ForegroundColor Red
    exit 1
}

# Build everything locally
Invoke-Step "Building Ovie compiler (oviec)" { cargo build --release --package oviec }
Invoke-Step "Building Ovie CLI (ovie)" { cargo build --release --package ovie }
Invoke-Step "Building Aproko assistant" { cargo build --release --package aproko }

# Run tests locally
Invoke-Step "Running unit tests" { cargo test --lib --workspace }

# Create a local demo project
if (-not (Test-Path "local-demo")) {
    Invoke-Step "Creating local demo project" { & "./target/release/ovie.exe" new local-demo }
}

# Show what we built
Write-Host "🎉 Local build complete! Everything is offline and ready to use:" -ForegroundColor Green
Write-Host ""
Write-Host "📁 Built binaries:" -ForegroundColor Cyan
Write-Host "   ./target/release/ovie.exe    - Ovie CLI toolchain" -ForegroundColor White
Write-Host "   ./target/release/oviec.exe   - Ovie compiler" -ForegroundColor White
Write-Host ""
Write-Host "🚀 Try it out:" -ForegroundColor Cyan
Write-Host "   ./target/release/ovie.exe --help" -ForegroundColor White
Write-Host "   ./target/release/oviec.exe --help" -ForegroundColor White
Write-Host "   cd local-demo && ../target/release/ovie.exe run" -ForegroundColor White
Write-Host ""
Write-Host "🔒 Everything stays local - no network required!" -ForegroundColor Green
Write-Host "📖 See docs/ for offline documentation" -ForegroundColor Cyan
Write-Host "🎯 See examples/ for sample programs" -ForegroundColor Cyan