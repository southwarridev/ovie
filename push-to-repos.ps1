# PowerShell script to push Ovie Programming Language to both GitHub and GitLab repositories
# This script sets up both remotes and pushes the code
#
# ⚠️  WARNING: This script goes ONLINE and pushes to remote repositories
# Ovie is designed to be offline-first. Only run this when you explicitly
# want to publish your changes to GitHub and GitLab.

param(
    [string]$CommitMessage = "Complete Ovie Programming Language implementation

- ✅ Full compiler pipeline (lexer, parser, IR, interpreter)
- ✅ Cross-platform CLI toolchain (ovie, oviec)
- ✅ Aproko assistant engine for code analysis
- ✅ Package management with cryptographic verification
- ✅ Multiple compilation backends (IR, WASM)
- ✅ Comprehensive documentation and examples
- ✅ Cross-platform build system and installers
- ✅ CI/CD pipelines for GitHub and GitLab
- ✅ Production-ready release system

Ready for Stage 0 production use!",
    [switch]$Force = $false
)

if (-not $Force) {
    Write-Host "⚠️  WARNING: This will push code to online repositories!" -ForegroundColor Yellow
    Write-Host "Ovie is designed to work offline-first. Continue? (y/N): " -NoNewline -ForegroundColor Yellow
    $response = Read-Host
    if ($response -notmatch '^[Yy]$') {
        Write-Host "❌ Aborted. Staying offline as intended." -ForegroundColor Red
        exit 0
    }
}

Write-Host "🚀 Setting up Ovie Programming Language repositories..." -ForegroundColor Green

# Check if we're in a git repository
if (-not (Test-Path ".git")) {
    Write-Host "📁 Initializing git repository..." -ForegroundColor Yellow
    git init
    git branch -M main
}

# Function to check if remote exists
function Test-GitRemote {
    param([string]$RemoteName)
    try {
        git remote get-url $RemoteName 2>$null | Out-Null
        return $true
    } catch {
        return $false
    }
}

# Add GitHub remote
Write-Host "🐙 Adding GitHub remote..." -ForegroundColor Cyan
if (Test-GitRemote "github") {
    Write-Host "   GitHub remote already exists, updating URL..." -ForegroundColor Yellow
    git remote set-url github https://github.com/southwarridev/ovie.git
} else {
    git remote add github https://github.com/southwarridev/ovie.git
}

# Add GitLab remote
Write-Host "🦊 Adding GitLab remote..." -ForegroundColor Cyan
if (Test-GitRemote "gitlab") {
    Write-Host "   GitLab remote already exists, updating URL..." -ForegroundColor Yellow
    git remote set-url gitlab https://gitlab.com/ovie1/ovie.git
} else {
    git remote add gitlab https://gitlab.com/ovie1/ovie.git
}

# Set origin to GitHub (primary)
Write-Host "🔗 Setting GitHub as primary origin..." -ForegroundColor Cyan
if (Test-GitRemote "origin") {
    git remote set-url origin https://github.com/southwarridev/ovie.git
} else {
    git remote add origin https://github.com/southwarridev/ovie.git
}

# Show current remotes
Write-Host "📋 Current remotes:" -ForegroundColor Cyan
git remote -v

# Stage all files
Write-Host "📦 Staging files..." -ForegroundColor Yellow
git add .

# Check if there are changes to commit
$changes = git diff --staged --name-only
if ($changes) {
    # Commit changes
    Write-Host "💾 Committing changes..." -ForegroundColor Yellow
    git commit -m $CommitMessage
} else {
    Write-Host "ℹ️  No changes to commit" -ForegroundColor Blue
}

# Push to both repositories
Write-Host "🚀 Pushing to GitHub..." -ForegroundColor Green
try {
    git push -u origin main
    Write-Host "✅ Successfully pushed to GitHub!" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to push to GitHub: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "🚀 Pushing to GitLab..." -ForegroundColor Green
try {
    git push -u gitlab main
    Write-Host "✅ Successfully pushed to GitLab!" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to push to GitLab: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "🔗 Repository URLs:" -ForegroundColor Cyan
Write-Host "   GitHub: https://github.com/southwarridev/ovie" -ForegroundColor White
Write-Host "   GitLab: https://gitlab.com/ovie1/ovie" -ForegroundColor White
Write-Host ""
Write-Host "🎉 Ovie Programming Language is now live on both platforms!" -ForegroundColor Green