# Manual binary build script for Ovie v2.2.0
Write-Host "Building Ovie binaries manually..." -ForegroundColor Cyan

# Build oviec binary
Write-Host "Building oviec..." -ForegroundColor Yellow
cargo rustc --release --bin oviec -- --crate-type bin

# Build ovie binary  
Write-Host "Building ovie..." -ForegroundColor Yellow
cargo rustc --release --bin ovie -- --crate-type bin

# Check results
Write-Host ""
Write-Host "Checking for executables..." -ForegroundColor Yellow

if (Test-Path "target\release\oviec.exe") {
    $size = [math]::Round((Get-Item "target\release\oviec.exe").Length/1MB, 2)
    Write-Host "✓ oviec.exe: $size MB" -ForegroundColor Green
} else {
    Write-Host "✗ oviec.exe not found" -ForegroundColor Red
}

if (Test-Path "target\release\ovie.exe") {
    $size = [math]::Round((Get-Item "target\release\ovie.exe").Length/1MB, 2)
    Write-Host "✓ ovie.exe: $size MB" -ForegroundColor Green
} else {
    Write-Host "✗ ovie.exe not found" -ForegroundColor Red
}
