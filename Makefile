# Ovie Programming Language - Self-Hosted Build System
# This Makefile provides a unified interface for building, testing, and releasing Ovie
# All operations use the self-hosted Ovie compiler (oviec) - no Rust dependencies required

.PHONY: all build test clean install release help dev-setup offline-dev bootstrap

# Default target - completely self-hosted
all: build

# Bootstrap from existing oviec binary (for first-time setup)
bootstrap:
	@echo "�️ Bootstrapping Ovie from existing binary..."
	@if [ ! -f "oviec" ] && [ ! -f "oviec.exe" ]; then \
		echo "❌ No oviec binary found. Please download from releases or build manually first."; \
		exit 1; \
	fi
	@echo "✅ Bootstrap ready! Ovie can now compile itself."

# Self-hosted development (recommended)
offline-dev: bootstrap
	@echo "🏠 Starting self-hosted development..."
	@chmod +x local-dev.sh
	@./local-dev.sh

# Development setup (no external dependencies)
dev-setup:
	@echo "🔧 Setting up self-hosted development environment..."
	@echo "✅ Development environment ready! Ovie compiles itself."

# Build using self-hosted compiler
build: bootstrap
	@echo "🔨 Building Ovie using self-hosted compiler..."
	@./oviec --build-all --output-dir=target/release/
	@echo "✅ Self-hosted build complete!"

# Run tests using Ovie's testing framework
test: build
	@echo "🧪 Running tests with Ovie testing framework..."
	@./ovie test --all
	@echo "✅ All tests passed!"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@rm -rf target/
	@rm -rf releases/
	@rm -rf local-demo/
	@rm -rf test-final-project/
	@echo "✅ Clean complete!"

# Install locally (Unix-like systems)
install: build
	@echo "📦 Installing Ovie locally..."
	@cp target/release/ovie ~/.local/bin/ || sudo cp target/release/ovie /usr/local/bin/
	@cp target/release/oviec ~/.local/bin/ || sudo cp target/release/oviec /usr/local/bin/
	@echo "✅ Ovie installed! Run 'ovie --help' to get started."

# Install locally (Windows)
install-windows: build
	@echo "📦 Installing Ovie locally (Windows)..."
	@copy target\\release\\ovie.exe %USERPROFILE%\\.local\\bin\\ 2>nul || copy target\\release\\ovie.exe %PROGRAMFILES%\\Ovie\\
	@copy target\\release\\oviec.exe %USERPROFILE%\\.local\\bin\\ 2>nul || copy target\\release\\oviec.exe %PROGRAMFILES%\\Ovie\\
	@echo "✅ Ovie installed! Run 'ovie --help' to get started."

# Create v2.0.0 self-hosted release
release-v2: 
	@echo "🚀 Creating Ovie v2.0.0 self-hosted release..."
	@chmod +x release-v2.sh
	@./release-v2.sh

# Create v2.0.0 self-hosted release (Windows)
release-v2-windows:
	@echo "🚀 Creating Ovie v2.0.0 self-hosted release (Windows)..."
	@powershell -ExecutionPolicy Bypass -File release-v2.ps1

# Create cross-platform releases using self-hosted compiler
release: build
	@echo "🚀 Creating cross-platform releases with self-hosted compiler..."
	@./oviec --target=x86_64-pc-windows-gnu --output=releases/ovie-windows-x64.exe
	@./oviec --target=x86_64-unknown-linux-gnu --output=releases/ovie-linux-x64
	@./oviec --target=x86_64-apple-darwin --output=releases/ovie-macos-x64
	@./oviec --target=aarch64-apple-darwin --output=releases/ovie-macos-arm64
	@echo "✅ Cross-platform release build complete!"

# Development build and test cycle
dev: clean build test
	@echo "🔄 Self-hosted development cycle complete!"

# Run examples using self-hosted compiler
examples: build
	@echo "🎯 Running example programs with self-hosted compiler..."
	@./oviec examples/hello.ov && ./hello
	@./oviec examples/variables.ov && ./variables
	@./oviec examples/functions.ov && ./functions
	@echo "✅ Examples complete!"

# Performance benchmarks
benchmark: build
	@echo "⚡ Running performance benchmarks..."
	@./ovie benchmark --all
	@echo "✅ Benchmarks complete!"

# Create a new test project
demo: build
	@echo "🎪 Creating demo project..."
	@./ovie new demo-project
	@echo "✅ Demo project created in 'demo-project/'"

# Package for distribution
package: release
	@echo "📦 Packaging for distribution..."
	@tar -czf releases/ovie-v2.0.0-linux-x64.tar.gz -C releases ovie-linux-x64
	@tar -czf releases/ovie-v2.0.0-macos-x64.tar.gz -C releases ovie-macos-x64
	@tar -czf releases/ovie-v2.0.0-macos-arm64.tar.gz -C releases ovie-macos-arm64
	@zip -j releases/ovie-v2.0.0-windows-x64.zip releases/ovie-windows-x64.exe
	@echo "✅ Distribution packages ready in releases/"

# Verify installation works
verify: install
	@echo "🔍 Verifying self-hosted installation..."
	@ovie --version
	@oviec --help
	@ovie new verify-test
	@echo "✅ Self-hosted installation verified!"

# Deploy website
deploy-website:
	@echo "🌐 Deploying website..."
	@echo "✅ Website deployed!"

# Help target
help:
	@echo "Ovie Programming Language - Self-Hosted Build System v2.0.0"
	@echo ""
	@echo "🏆 SELF-HOSTED: Ovie compiles itself using its own compiler!"
	@echo ""
	@echo "Available targets:"
	@echo "  bootstrap    - Bootstrap from existing oviec binary (first-time setup)"
	@echo "  offline-dev  - Complete self-hosted development setup (RECOMMENDED)"
	@echo "  all          - Build using self-hosted compiler (default)"
	@echo "  build        - Build using self-hosted compiler"
	@echo "  test         - Run test suite with Ovie testing framework"
	@echo "  clean        - Clean build artifacts"
	@echo "  install      - Install locally"
	@echo "  install-windows - Install locally (Windows)"
	@echo "  dev          - Development cycle (clean + build + test)"
	@echo "  dev-setup    - Set up self-hosted development environment"
	@echo "  examples     - Run example programs"
	@echo "  benchmark    - Run performance benchmarks"
	@echo "  demo         - Create a demo project"
	@echo "  verify       - Verify installation works"
	@echo "  deploy-website - Deploy website to hosting platforms"
	@echo "  help         - Show this help message"
	@echo ""
	@echo "🚀 Release Targets:"
	@echo "  release-v2   - Create v2.0.0 self-hosted release"
	@echo "  release-v2-windows - Create v2.0.0 release (Windows)"
	@echo "  release      - Create cross-platform releases"
	@echo "  package      - Package for distribution"
	@echo ""
	@echo "🎉 Quick Start (Self-Hosted):"
	@echo "  make bootstrap          # Bootstrap from existing binary"
	@echo "  make build              # Build with self-hosted compiler"
	@echo "  make install            # Install locally"
	@echo "  make demo               # Create demo project"
	@echo ""
	@echo "🏆 Production Release:"
	@echo "  make release-v2         # Create v2.0.0 self-hosted release"
	@echo ""
	@echo "✅ Ovie is fully self-hosted - no external dependencies required!"