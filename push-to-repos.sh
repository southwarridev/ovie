#!/bin/bash

# Script to push Ovie Programming Language to both GitHub and GitLab repositories
# This script sets up both remotes and pushes the code
# 
# ⚠️  WARNING: This script goes ONLINE and pushes to remote repositories
# Ovie is designed to be offline-first. Only run this when you explicitly
# want to publish your changes to GitHub and GitLab.

set -e

echo "⚠️  WARNING: This will push code to online repositories!"
echo "Ovie is designed to work offline-first. Continue? (y/N)"
read -r response
if [[ ! "$response" =~ ^[Yy]$ ]]; then
    echo "❌ Aborted. Staying offline as intended."
    exit 0
fi

echo "🚀 Setting up Ovie Programming Language repositories..."

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "📁 Initializing git repository..."
    git init
    git branch -M main
fi

# Add GitHub remote
echo "🐙 Adding GitHub remote..."
if git remote get-url github >/dev/null 2>&1; then
    echo "   GitHub remote already exists, updating URL..."
    git remote set-url github https://github.com/southwarridev/ovie.git
else
    git remote add github https://github.com/southwarridev/ovie.git
fi

# Add GitLab remote
echo "🦊 Adding GitLab remote..."
if git remote get-url gitlab >/dev/null 2>&1; then
    echo "   GitLab remote already exists, updating URL..."
    git remote set-url gitlab https://gitlab.com/ovie1/ovie.git
else
    git remote add gitlab https://gitlab.com/ovie1/ovie.git
fi

# Set origin to GitHub (primary)
echo "🔗 Setting GitHub as primary origin..."
if git remote get-url origin >/dev/null 2>&1; then
    git remote set-url origin https://github.com/southwarridev/ovie.git
else
    git remote add origin https://github.com/southwarridev/ovie.git
fi

# Show current remotes
echo "📋 Current remotes:"
git remote -v

# Stage all files
echo "📦 Staging files..."
git add .

# Check if there are changes to commit
if git diff --staged --quiet; then
    echo "ℹ️  No changes to commit"
else
    # Commit changes
    echo "💾 Committing changes..."
    if [ -z "$1" ]; then
        git commit -m "Complete Ovie Programming Language implementation

- ✅ Full compiler pipeline (lexer, parser, IR, interpreter)
- ✅ Cross-platform CLI toolchain (ovie, oviec)
- ✅ Aproko assistant engine for code analysis
- ✅ Package management with cryptographic verification
- ✅ Multiple compilation backends (IR, WASM)
- ✅ Comprehensive documentation and examples
- ✅ Cross-platform build system and installers
- ✅ CI/CD pipelines for GitHub and GitLab
- ✅ Production-ready release system

Ready for Stage 0 production use!"
    else
        git commit -m "$1"
    fi
fi

# Push to both repositories
echo "🚀 Pushing to GitHub..."
git push -u origin main

echo "🚀 Pushing to GitLab..."
git push -u gitlab main

echo "✅ Successfully pushed to both repositories!"
echo ""
echo "🔗 Repository URLs:"
echo "   GitHub: https://github.com/southwarridev/ovie"
echo "   GitLab: https://gitlab.com/ovie1/ovie"
echo ""
echo "🎉 Ovie Programming Language is now live on both platforms!"