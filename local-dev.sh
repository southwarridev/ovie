#!/bin/bash

# Ovie Programming Language - Local Development Script
# This script sets up and runs Ovie completely offline

set -e

echo "🏠 Ovie Programming Language - Offline Development Mode"
echo "This script keeps everything local and offline-first!"
echo ""

# Function to run a command and show status
run_step() {
    echo "🔧 $1..."
    if eval "$2"; then
        echo "✅ $1 complete!"
    else
        echo "❌ $1 failed!"
        exit 1
    fi
    echo ""
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "oviec" ]; then
    echo "❌ Please run this script from the Ovie project root directory"
    exit 1
fi

# Build everything locally
run_step "Building Ovie compiler (oviec)" "cargo build --release --package oviec"
run_step "Building Ovie CLI (ovie)" "cargo build --release --package ovie"
run_step "Building Aproko assistant" "cargo build --release --package aproko"

# Run tests locally
run_step "Running unit tests" "cargo test --lib --workspace"

# Create a local demo project
if [ ! -d "local-demo" ]; then
    run_step "Creating local demo project" "./target/release/ovie new local-demo"
fi

# Show what we built
echo "🎉 Local build complete! Everything is offline and ready to use:"
echo ""
echo "📁 Built binaries:"
echo "   ./target/release/ovie    - Ovie CLI toolchain"
echo "   ./target/release/oviec   - Ovie compiler"
echo ""
echo "🚀 Try it out:"
echo "   ./target/release/ovie --help"
echo "   ./target/release/oviec --help"
echo "   cd local-demo && ../target/release/ovie run"
echo ""
echo "🔒 Everything stays local - no network required!"
echo "📖 See docs/ for offline documentation"
echo "🎯 See examples/ for sample programs"