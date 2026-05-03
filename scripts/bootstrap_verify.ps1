# ============================================================================
# Ovie Real Bootstrap Verification Script - v2.2.0 (Windows PowerShell)
# Runs the real bootstrap verification CLI command
# ============================================================================

param(
    [string]$File = "",
    [switch]$Verbose = $false,
    [string]$Report = "",
    [switch]$Benchmark = $false
)

$ErrorActionPreference = "Stop"

# Get script directory and project root
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir
Set-Location $ProjectRoot

Write-Host "============================================================================" -ForegroundColor Blue
Write-Host "                    OVIE REAL BOOTSTRAP VERIFICATION" -ForegroundColor Blue
Write-Host "                         v2.2.0 - Rust vs Ovie Lexer" -ForegroundColor Blue
Write-Host "============================================================================" -ForegroundColor Blue
Write-Host ""

# Check if Rust is available
try {
    $null = Get-Command cargo -ErrorAction Stop
} catch {
    Write-Host "❌ Error: Rust/Cargo not found" -ForegroundColor Red
    Write-Host "Please install Rust: https://rustup.rs/"
    exit 1
}

# Build the project
Write-Host "[1/2] Building Ovie compiler..." -ForegroundColor Blue
Write-Host "Running: cargo build --release --workspace"
$buildResult = Start-Process -FilePath "cargo" -ArgumentList "build", "--release", "--workspace" -Wait -PassThru -NoNewWindow
if ($buildResult.ExitCode -ne 0) {
    Write-Host "❌ Compiler build failed" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Ovie compiler built successfully" -ForegroundColor Green
Write-Host ""

# Run the real bootstrap verification command
Write-Host "[2/2] Running bootstrap verification..." -ForegroundColor Blue

# Build command arguments
$args = @("self-host", "bootstrap", "verify")

if ($File -ne "") {
    $args += "--file"
    $args += $File
}

if ($Verbose) {
    $args += "--verbose"
}

if ($Report -ne "") {
    $args += "--report"
    $args += $Report
}

if ($Benchmark) {
    $args += "--benchmark"
}

# Run the verification
& ".\target\release\ovie.exe" @args

# Exit with the same code as the verification command
exit $LASTEXITCODE