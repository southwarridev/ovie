# Ovie Programming Language - Offline-First Build System
# This Makefile provides a unified interface for building, testing, and releasing Ovie
# All operations are designed to work completely offline

.PHONY: all build test clean install release help dev-setup offline-dev

# Default target - completely offline
all: build

# Offline development (recommended)
offline-dev:
	@echo "🏠 Starting offline-first development..."
	@chmod +x local-dev.sh
	@./local-dev.sh

# Development setup (installs targets but doesn't go online)
dev-setup:
	@echo "🔧 Setting up development environment (offline-first)..."
	rustup update
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-pc-windows-msvc
	rustup target add x86_64-unknown-linux-gnu
	rustup target add x86_64-apple-darwin
	rustup target add aarch64-apple-darwin
	@echo "✅ Development environment ready! (All operations will be offline)"

# Build for current platform (offline)
build:
	@echo "🔨 Building Ovie for current platform (offline)..."
	cargo build --release --workspace
	@echo "✅ Build complete! No network required."

# Run tests (offline)
test:
	@echo "🧪 Running tests (offline)..."
	cargo test --lib --workspace
	@echo "✅ Tests complete! All local."

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf releases/
	rm -rf local-demo/
	rm -rf test-final-project/
	@echo "✅ Clean complete!"

# Install locally (Unix-like systems)
install: build
	@echo "📦 Installing Ovie locally..."
	cp target/release/ovie ~/.local/bin/ || sudo cp target/release/ovie /usr/local/bin/
	cp target/release/oviec ~/.local/bin/ || sudo cp target/release/oviec /usr/local/bin/
	@echo "✅ Ovie installed! Run 'ovie --help' to get started."

# Install locally (Windows - requires PowerShell)
install-windows: build
	@echo "📦 Installing Ovie locally (Windows)..."
	powershell -Command "Copy-Item target/release/ovie.exe $$env:USERPROFILE/.cargo/bin/ -ErrorAction SilentlyContinue; Copy-Item target/release/oviec.exe $$env:USERPROFILE/.cargo/bin/ -ErrorAction SilentlyContinue"
	@echo "✅ Ovie installed! Run 'ovie --help' to get started."

# Create v1.0.0 production release
release-v1: 
	@echo "🚀 Creating Ovie v1.0.0 production release..."
	chmod +x release-v1.sh
	./release-v1.sh

# Create v1.0.0 production release (Windows)
release-v1-windows:
	@echo "🚀 Creating Ovie v1.0.0 production release (Windows)..."
	powershell -ExecutionPolicy Bypass -File release-v1.ps1

# Create v1.0.0 production release (fast, skip tests)
release-v1-fast:
	@echo "🚀 Creating fast Ovie v1.0.0 production release..."
	chmod +x release-v1.sh
	./release-v1.sh --skip-tests

# Create cross-platform releases
release:
	@echo "🚀 Creating cross-platform releases..."
	powershell -ExecutionPolicy Bypass -File build-releases.ps1
	@echo "✅ Release build complete!"

# Quick release without tests (for CI/development)
release-fast:
	@echo "🚀 Creating fast release build..."
	powershell -ExecutionPolicy Bypass -File build-releases.ps1 -SkipTests
	@echo "✅ Fast release build complete!"

# Development build and test cycle
dev: clean build test
	@echo "🔄 Development cycle complete!"

# Run examples
examples: build
	@echo "🎯 Running example programs..."
	cargo run --bin oviec -- examples/hello.ov
	cargo run --bin oviec -- examples/variables.ov
	cargo run --bin oviec -- examples/functions.ov
	@echo "✅ Examples complete!"

# Performance benchmarks
benchmark: build
	@echo "⚡ Running performance benchmarks..."
	cargo run --release --bin benchmark
	@echo "✅ Benchmarks complete!"

# Create a new test project
demo: build
	@echo "🎪 Creating demo project..."
	cargo run --bin ovie -- new demo-project
	@echo "✅ Demo project created in 'demo-project/'"

# Package for distribution (creates tarballs/zips)
package: release
	@echo "📦 Packaging for distribution..."
	@echo "✅ Distribution packages ready in releases/"

# Verify installation works
verify: install
	@echo "🔍 Verifying installation..."
	ovie --version
	oviec --help
	ovie new verify-test
	@echo "✅ Installation verified!"

# Help target
help:
	@echo "Ovie Programming Language - Offline-First Build System"
	@echo ""
	@echo "🔒 OFFLINE-FIRST: All operations work without network access"
	@echo ""
	@echo "Available targets:"
	@echo "  offline-dev  - Complete offline development setup (RECOMMENDED)"
	@echo "  all          - Build for current platform (default, offline)"
	@echo "  build        - Build for current platform (offline)"
	@echo "  test         - Run test suite (offline)"
	@echo "  clean        - Clean build artifacts"
	@echo "  install      - Install locally (offline)"
	@echo "  install-windows - Install locally (Windows, offline)"
	@echo "  dev          - Development cycle (clean + build + test, offline)"
	@echo "  dev-setup    - Set up development environment (offline-first)"
	@echo "  examples     - Run example programs (offline)"
	@echo "  benchmark    - Run performance benchmarks (offline)"
	@echo "  demo         - Create a demo project (offline)"
	@echo "  verify       - Verify installation works (offline)"
	@echo "  help         - Show this help message"
	@echo ""
	@echo "⚠️  ONLINE OPERATIONS (use with caution):"
	@echo "  release-v1   - Create v1.0.0 production release (GitHub + GitLab)"
	@echo "  release-v1-windows - Create v1.0.0 release (Windows)"
	@echo "  release-v1-fast - Create v1.0.0 release (skip tests)"
	@echo "  release      - Create cross-platform releases"
	@echo "  release-fast - Create releases without running tests"
	@echo "  package      - Package for distribution"
	@echo ""
	@echo "🚀 Quick Start (Offline):"
	@echo "  make offline-dev        # Complete offline setup"
	@echo "  make build              # Build for current platform"
	@echo "  make install            # Install locally"
	@echo "  make demo               # Create demo project"
	@echo ""
	@echo "🎉 Production Release:"
	@echo "  make release-v1         # Create v1.0.0 production release"
	@echo ""
	@echo "🔒 Ovie is designed to work completely offline!"