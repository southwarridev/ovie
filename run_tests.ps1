# Ovie Testing Script - Windows PowerShell
# Quick test runner for all Ovie features

Write-Host "🧪 Ovie Testing Suite" -ForegroundColor Cyan
Write-Host "=====================`n" -ForegroundColor Cyan

# Test 1: Bootstrap Compiler
Write-Host "✅ Test 1: Bootstrap Compiler (Ovie compiling Ovie)" -ForegroundColor Green
cargo run --bin oviec -- oviec/src/self_hosting/bootstrap_compiler_simple.ov
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Bootstrap compiler PASSED`n" -ForegroundColor Green
} else {
    Write-Host "❌ Bootstrap compiler FAILED`n" -ForegroundColor Red
}

# Test 2: Array Operations
Write-Host "✅ Test 2: Array Literals and Operations" -ForegroundColor Green
cargo run --bin oviec -- test_array_simple.ov
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Array operations PASSED`n" -ForegroundColor Green
} else {
    Write-Host "❌ Array operations FAILED`n" -ForegroundColor Red
}

# Test 3: Struct Operations
Write-Host "✅ Test 3: Struct Instantiation and Field Access" -ForegroundColor Green
cargo run --bin oviec -- test_struct_comprehensive.ov
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Struct operations PASSED`n" -ForegroundColor Green
} else {
    Write-Host "❌ Struct operations FAILED`n" -ForegroundColor Red
}

# Test 4: Hello World
Write-Host "✅ Test 4: Hello World Example" -ForegroundColor Green
cargo run --bin oviec -- examples/hello.ov
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Hello World PASSED`n" -ForegroundColor Green
} else {
    Write-Host "❌ Hello World FAILED`n" -ForegroundColor Red
}

# Test 5: Functions
Write-Host "✅ Test 5: Function Declarations and Calls" -ForegroundColor Green
cargo run --bin oviec -- examples/functions.ov
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Functions PASSED`n" -ForegroundColor Green
} else {
    Write-Host "❌ Functions FAILED`n" -ForegroundColor Red
}

Write-Host "`n🎉 Testing Complete!" -ForegroundColor Cyan
Write-Host "=====================" -ForegroundColor Cyan
