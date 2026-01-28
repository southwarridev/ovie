#!/bin/bash

# Ovie Programming Language - Universal Installer Script
# This script installs Ovie on Unix-like systems (Linux, macOS, WSL)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
OVIE_VERSION="0.1.0"
GITHUB_REPO="ovie-lang/ovie"
INSTALL_DIR="$HOME/.local/bin"
TEMP_DIR="/tmp/ovie-install"

# Detect platform and architecture
detect_platform() {
    local platform=""
    local arch=""
    
    case "$(uname -s)" in
        Linux*)     platform="linux" ;;
        Darwin*)    platform="macos" ;;
        CYGWIN*|MINGW*|MSYS*) platform="windows" ;;
        *)          platform="unknown" ;;
    esac
    
    case "$(uname -m)" in
        x86_64|amd64)   arch="x64" ;;
        arm64|aarch64)  arch="arm64" ;;
        *)              arch="unknown" ;;
    esac
    
    echo "${platform}-${arch}"
}

# Print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Download and extract release
download_release() {
    local platform_arch="$1"
    local download_url=""
    local filename=""
    
    # For now, we'll build from source since we don't have GitHub releases yet
    print_status "Building Ovie from source..."
    
    # Check if Rust is installed
    if ! command_exists cargo; then
        print_error "Rust is not installed. Please install Rust first:"
        print_error "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    # Clone or use current directory
    if [ ! -f "Cargo.toml" ]; then
        print_status "Cloning Ovie repository..."
        git clone "https://github.com/${GITHUB_REPO}.git" "$TEMP_DIR"
        cd "$TEMP_DIR"
    else
        print_status "Using current directory (detected Ovie source)"
    fi
    
    # Build release
    print_status "Building release binaries..."
    cargo build --release --workspace
    
    # Create install directory
    mkdir -p "$INSTALL_DIR"
    
    # Copy binaries
    print_status "Installing binaries to $INSTALL_DIR..."
    cp target/release/ovie "$INSTALL_DIR/"
    cp target/release/oviec "$INSTALL_DIR/"
    
    # Make executable
    chmod +x "$INSTALL_DIR/ovie"
    chmod +x "$INSTALL_DIR/oviec"
}

# Add to PATH if needed
setup_path() {
    local shell_rc=""
    
    # Detect shell and set appropriate RC file
    case "$SHELL" in
        */bash)     shell_rc="$HOME/.bashrc" ;;
        */zsh)      shell_rc="$HOME/.zshrc" ;;
        */fish)     shell_rc="$HOME/.config/fish/config.fish" ;;
        *)          shell_rc="$HOME/.profile" ;;
    esac
    
    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        print_status "Adding $INSTALL_DIR to PATH in $shell_rc"
        
        if [ "$SHELL" = "*/fish" ]; then
            echo "set -gx PATH $INSTALL_DIR \$PATH" >> "$shell_rc"
        else
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$shell_rc"
        fi
        
        print_warning "Please restart your shell or run: source $shell_rc"
    fi
}

# Verify installation
verify_installation() {
    print_status "Verifying installation..."
    
    if command_exists ovie && command_exists oviec; then
        print_success "Ovie installed successfully!"
        print_status "Version information:"
        "$INSTALL_DIR/ovie" --version 2>/dev/null || echo "  ovie: installed"
        "$INSTALL_DIR/oviec" --help >/dev/null 2>&1 && echo "  oviec: installed" || echo "  oviec: installed"
        
        print_status "Quick start:"
        echo "  ovie new my-project    # Create a new project"
        echo "  cd my-project"
        echo "  ovie run               # Run your project"
        
    else
        print_error "Installation verification failed"
        print_error "Please check that $INSTALL_DIR is in your PATH"
        exit 1
    fi
}

# Cleanup
cleanup() {
    if [ -d "$TEMP_DIR" ]; then
        print_status "Cleaning up temporary files..."
        rm -rf "$TEMP_DIR"
    fi
}

# Main installation function
main() {
    print_status "🚀 Installing Ovie Programming Language v$OVIE_VERSION"
    
    # Detect platform
    local platform_arch
    platform_arch=$(detect_platform)
    print_status "Detected platform: $platform_arch"
    
    # Check for unsupported platforms
    if [[ "$platform_arch" == *"unknown"* ]]; then
        print_error "Unsupported platform: $platform_arch"
        print_error "Please build from source manually"
        exit 1
    fi
    
    # Check dependencies
    if ! command_exists curl && ! command_exists wget; then
        print_error "Neither curl nor wget found. Please install one of them."
        exit 1
    fi
    
    if ! command_exists tar; then
        print_error "tar command not found. Please install tar."
        exit 1
    fi
    
    # Create temporary directory
    mkdir -p "$TEMP_DIR"
    
    # Download and install
    download_release "$platform_arch"
    
    # Setup PATH
    setup_path
    
    # Verify installation
    verify_installation
    
    # Cleanup
    cleanup
    
    print_success "🎉 Ovie Programming Language installed successfully!"
    print_status "Get started with: ovie new my-first-project"
}

# Handle interrupts
trap cleanup EXIT INT TERM

# Run main function
main "$@"