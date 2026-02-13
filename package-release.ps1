# Ovie v2.2.0 Release Packaging Script
# This script packages the complete Ovie distribution for all platforms

Write-Host "============================================================================" -ForegroundColor Cyan
Write-Host "                 OVIE v2.2.0 RELEASE PACKAGING" -ForegroundColor Cyan
Write-Host "============================================================================" -ForegroundColor Cyan
Write-Host ""

# Step 1: Build Windows executables
Write-Host "[1/6] Building Windows executables..." -ForegroundColor Yellow
cargo build --release --bin ovie --bin oviec 2>&1 | Out-Null

# Check if executables were created
if (Test-Path "target\release\ovie.exe") {
    $ovieSize = [math]::Round((Get-Item "target\release\ovie.exe").Length/1MB, 2)
    Write-Host "  ✓ ovie.exe created ($ovieSize MB)" -ForegroundColor Green
} else {
    Write-Host "  ✗ ovie.exe not found - checking deps folder..." -ForegroundColor Red
    $ovieExe = Get-ChildItem "target\release\deps" -Filter "ovie-*.exe" | Select-Object -First 1
    if ($ovieExe) {
        Copy-Item $ovieExe.FullName "target\release\ovie.exe"
        Write-Host "  ✓ Copied ovie.exe from deps" -ForegroundColor Green
    }
}

if (Test-Path "target\release\oviec.exe") {
    $oviecSize = [math]::Round((Get-Item "target\release\oviec.exe").Length/1MB, 2)
    Write-Host "  ✓ oviec.exe created ($oviecSize MB)" -ForegroundColor Green
} else {
    Write-Host "  ✗ oviec.exe not found - checking deps folder..." -ForegroundColor Red
    $oviecExe = Get-ChildItem "target\release\deps" -Filter "oviec-*.exe" | Select-Object -First 1
    if ($oviecExe) {
        Copy-Item $oviecExe.FullName "target\release\oviec.exe"
        Write-Host "  ✓ Copied oviec.exe from deps" -ForegroundColor Green
    }
}

# Step 2: Package Windows distribution
Write-Host ""
Write-Host "[2/6] Packaging Windows-x64 distribution..." -ForegroundColor Yellow

if (Test-Path "target\release\ovie.exe") {
    Copy-Item "target\release\ovie.exe" "windows-x64\" -Force
    Copy-Item "target\release\oviec.exe" "windows-x64\" -Force
    
    # Copy all necessary files
    Copy-Item "std" "windows-x64\" -Recurse -Force
    Copy-Item "examples" "windows-x64\" -Recurse -Force
    Copy-Item "docs" "windows-x64\" -Recurse -Force
    Copy-Item "LICENSE" "windows-x64\" -Force
    Copy-Item "README.md" "windows-x64\" -Force
    Copy-Item "ovie.png" "windows-x64\" -Force
    
    Write-Host "  ✓ Windows-x64 package complete" -ForegroundColor Green
    
    # Show sizes
    $ovieSize = [math]::Round((Get-Item "windows-x64\ovie.exe").Length/1MB, 2)
    $oviecSize = [math]::Round((Get-Item "windows-x64\oviec.exe").Length/1MB, 2)
    Write-Host "    - ovie.exe: $ovieSize MB" -ForegroundColor Cyan
    Write-Host "    - oviec.exe: $oviecSize MB" -ForegroundColor Cyan
} else {
    Write-Host "  ✗ Cannot package - executables not found" -ForegroundColor Red
}

# Step 3: Package Linux-x64 distribution
Write-Host ""
Write-Host "[3/6] Packaging Linux-x64 distribution..." -ForegroundColor Yellow

# Copy all necessary files for Linux
Copy-Item "std" "linux-x64\" -Recurse -Force
Copy-Item "examples" "linux-x64\" -Recurse -Force
Copy-Item "docs" "linux-x64\" -Recurse -Force
Copy-Item "LICENSE" "linux-x64\" -Force
Copy-Item "README.md" "linux-x64\" -Force
Copy-Item "ovie.png" "linux-x64\" -Force

Write-Host "  ✓ Linux-x64 package structure ready" -ForegroundColor Green
Write-Host "    Note: Linux binaries need to be built on Linux" -ForegroundColor Yellow

# Step 4: Package macOS-arm64 distribution
Write-Host ""
Write-Host "[4/6] Packaging macOS-arm64 distribution..." -ForegroundColor Yellow

# Copy all necessary files for macOS ARM64
Copy-Item "std" "macos-arm64\" -Recurse -Force
Copy-Item "examples" "macos-arm64\" -Recurse -Force
Copy-Item "docs" "macos-arm64\" -Recurse -Force
Copy-Item "LICENSE" "macos-arm64\" -Force
Copy-Item "README.md" "macos-arm64\" -Force
Copy-Item "ovie.png" "macos-arm64\" -Force

Write-Host "  ✓ macOS-arm64 package structure ready" -ForegroundColor Green
Write-Host "    Note: macOS binaries need to be built on macOS" -ForegroundColor Yellow

# Step 5: Package macOS-x64 distribution
Write-Host ""
Write-Host "[5/6] Packaging macOS-x64 distribution..." -ForegroundColor Yellow

# Copy all necessary files for macOS x64
Copy-Item "std" "macos-x64\" -Recurse -Force
Copy-Item "examples" "macos-x64\" -Recurse -Force
Copy-Item "docs" "macos-x64\" -Recurse -Force
Copy-Item "LICENSE" "macos-x64\" -Force
Copy-Item "README.md" "macos-x64\" -Force
Copy-Item "ovie.png" "macos-x64\" -Force

Write-Host "  ✓ macOS-x64 package structure ready" -ForegroundColor Green
Write-Host "    Note: macOS binaries need to be built on macOS" -ForegroundColor Yellow

# Step 6: Verification
Write-Host ""
Write-Host "[6/6] Verifying packages..." -ForegroundColor Yellow

$platforms = @("windows-x64", "linux-x64", "macos-arm64", "macos-x64")
foreach ($platform in $platforms) {
    $hasStd = Test-Path "$platform\std"
    $hasExamples = Test-Path "$platform\examples"
    $hasDocs = Test-Path "$platform\docs"
    $hasLicense = Test-Path "$platform\LICENSE"
    $hasReadme = Test-Path "$platform\README.md"
    $hasLogo = Test-Path "$platform\ovie.png"
    
    if ($hasStd -and $hasExamples -and $hasDocs -and $hasLicense -and $hasReadme -and $hasLogo) {
        Write-Host "  ✓ $platform - Complete" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $platform - Missing files" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "============================================================================" -ForegroundColor Cyan
Write-Host "                    PACKAGING COMPLETE" -ForegroundColor Cyan
Write-Host "============================================================================" -ForegroundColor Cyan
Write-Host ""

if (Test-Path "windows-x64\ovie.exe") {
    Write-Host "Windows executables are ready for distribution!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Yellow
    Write-Host "  1. Build Linux binaries on a Linux machine" -ForegroundColor White
    Write-Host "  2. Build macOS binaries on a Mac" -ForegroundColor White
    Write-Host "  3. Test all executables" -ForegroundColor White
    Write-Host "  4. Create release archives" -ForegroundColor White
} else {
    Write-Host "Warning: Windows executables were not generated." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Troubleshooting:" -ForegroundColor Yellow
    Write-Host "  1. Check if Cargo.toml has [[bin]] sections" -ForegroundColor White
    Write-Host "  2. Verify main.rs files have fn main()" -ForegroundColor White
    Write-Host "  3. Try: cargo build --release --verbose" -ForegroundColor White
}

Write-Host ""
