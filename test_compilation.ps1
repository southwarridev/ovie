# Simple Rust Compilation Test for Ovie
# Tests core compilation without running complex tests

Write-Host "🦀 Testing Rust compilation for Ovie codebase..." -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Continue"
$success = $true

# Test 1: Basic syntax check
Write-Host "1️⃣ Running cargo check on main workspace..." -ForegroundColor Yellow
try {
    $result = cargo check --workspace --all-targets 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Main workspace check passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Main workspace check failed:" -ForegroundColor Red
        Write-Host $result -ForegroundColor Red
        $success = $false
    }
} catch {
    Write-Host "❌ Failed to run cargo check: $_" -ForegroundColor Red
    $success = $false
}

# Test 2: Check oviec crate specifically  
Write-Host ""
Write-Host "2️⃣ Running cargo check on oviec crate..." -ForegroundColor Yellow
try {
    $result = cargo check --manifest-path oviec/Cargo.toml 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Oviec crate check passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Oviec crate check failed:" -ForegroundColor Red
        Write-Host $result -ForegroundColor Red
        $success = $false
    }
} catch {
    Write-Host "❌ Failed to run cargo check on oviec: $_" -ForegroundColor Red
    $success = $false
}

# Test 3: Check aproko crate
Write-Host ""
Write-Host "3️⃣ Running cargo check on aproko crate..." -ForegroundColor Yellow
try {
    $result = cargo check --manifest-path aproko/Cargo.toml 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Aproko crate check passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Aproko crate check failed:" -ForegroundColor Red
        Write-Host $result -ForegroundColor Red
        $success = $false
    }
} catch {
    Write-Host "❌ Failed to run cargo check on aproko: $_" -ForegroundColor Red
    $success = $false
}

# Test 4: Check ovie CLI crate
Write-Host ""
Write-Host "4️⃣ Running cargo check on ovie CLI crate..." -ForegroundColor Yellow
try {
    $result = cargo check --manifest-path ovie/Cargo.toml 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Ovie CLI crate check passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Ovie CLI crate check failed:" -ForegroundColor Red
        Write-Host $result -ForegroundColor Red
        $success = $false
    }
} catch {
    Write-Host "❌ Failed to run cargo check on ovie CLI: $_" -ForegroundColor Red
    $success = $false
}

# Test 5: Try basic lib compilation
Write-Host ""
Write-Host "5️⃣ Testing library compilation..." -ForegroundColor Yellow
try {
    $result = cargo build --lib --manifest-path oviec/Cargo.toml 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Library compilation passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Library compilation failed:" -ForegroundColor Red
        Write-Host $result -ForegroundColor Red
        $success = $false
    }
} catch {
    Write-Host "❌ Failed to compile library: $_" -ForegroundColor Red
    $success = $false
}

Write-Host ""
if ($success) {
    Write-Host "🎉 All Rust compilation tests passed!" -ForegroundColor Green
    Write-Host "✨ The codebase appears to be error-free and ready for use." -ForegroundColor Green
    exit 0
} else {
    Write-Host "💥 Some compilation tests failed!" -ForegroundColor Red
    Write-Host "🔧 Please review the errors above and fix them." -ForegroundColor Yellow
    exit 1
}