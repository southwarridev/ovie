#!/bin/bash

# Ovie Programming Language - Universal Installer Script
# Stage 2: Self-Hosted Compiler with Advanced Features
# This script installs Ovie on Unix-like systems (Linux, macOS, WSL)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
OVIE_VERSION="2.0.0"
GITHUB_REPO="southwarridev/ovie"
INSTALL_DIR="$HOME/.local/bin"
TEMP_DIR="/tmp/ovie-install"
INCLUDE_EXTENSION=true

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

# Display Ovie logo and info
show_logo() {
    echo ""
    echo -e "${MAGENTA}🎯 Ovie Programming Language - Stage 2${NC}"
    echo -e "${MAGENTA}=======================================${NC}"
    echo -e "${CYAN}Self-Hosted Compiler with Advanced Features${NC}"
    echo -e "${YELLOW}Version: $OVIE_VERSION${NC}"
    echo ""
    
    # ASCII art logo
    echo -e "${YELLOW}    ██████╗ ██╗   ██╗██╗███████╗${NC}"
    echo -e "${YELLOW}   ██╔═══██╗██║   ██║██║██╔════╝${NC}"
    echo -e "${YELLOW}   ██║   ██║██║   ██║██║█████╗  ${NC}"
    echo -e "${YELLOW}   ██║   ██║╚██╗ ██╔╝██║██╔══╝  ${NC}"
    echo -e "${YELLOW}   ╚██████╔╝ ╚████╔╝ ██║███████╗${NC}"
    echo -e "${YELLOW}    ╚═════╝   ╚═══╝  ╚═╝╚══════╝${NC}"
    echo ""
    echo -e "${CYAN}   Natural Language Programming${NC}"
    echo -e "${GREEN}   AI-Friendly • Memory Safe • Self-Hosted${NC}"
    echo ""
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
    
    print_status "Downloading Ovie Stage 2 self-hosted binaries..."
    
    # Try to download pre-built binaries first
    local download_url="https://github.com/${GITHUB_REPO}/releases/download/v${OVIE_VERSION}/ovie-${platform_arch}.tar.gz"
    
    print_status "Attempting to download from: $download_url"
    
    if command_exists curl; then
        if curl -fsSL "$download_url" -o "$TEMP_DIR/ovie.tar.gz" 2>/dev/null; then
            print_success "Downloaded pre-built binaries"
            cd "$TEMP_DIR"
            tar -xzf ovie.tar.gz
            
            # Create install directory
            mkdir -p "$INSTALL_DIR"
            
            # Copy binaries
            cp ovie "$INSTALL_DIR/" 2>/dev/null || cp ovie.exe "$INSTALL_DIR/" 2>/dev/null || true
            cp oviec "$INSTALL_DIR/" 2>/dev/null || cp oviec.exe "$INSTALL_DIR/" 2>/dev/null || true
            
            return 0
        fi
    fi
    
    # Fallback: Build from source using self-hosted compiler
    print_status "Pre-built binaries not available. Building from source..."
    
    # Clone or use current directory
    if [ ! -f "Cargo.toml" ] && [ ! -f "ovie.toml" ]; then
        print_status "Cloning Ovie repository..."
        git clone "https://github.com/${GITHUB_REPO}.git" "$TEMP_DIR"
        cd "$TEMP_DIR"
    else
        print_status "Using current directory (detected Ovie source)"
    fi
    
    # Check if we have oviec binary for self-hosted build
    if [ -f "oviec" ] || [ -f "oviec.exe" ]; then
        print_status "Building with self-hosted Ovie compiler..."
        ./oviec --build-all --output-dir="$INSTALL_DIR"
        print_success "Built with self-hosted compiler!"
    else
        print_warning "No oviec binary found. Attempting bootstrap build..."
        
        # Last resort: use Rust if available (for bootstrapping only)
        if command_exists cargo; then
            print_status "Bootstrapping Ovie compiler with Rust (one-time only)..."
            cargo build --release --workspace
            
            # Create install directory
            mkdir -p "$INSTALL_DIR"
            
            # Copy binaries
            cp target/release/ovie "$INSTALL_DIR/"
            cp target/release/oviec "$INSTALL_DIR/"
        else
            print_error "Cannot build Ovie: No self-hosted compiler or Rust toolchain found"
            print_error "Please download a pre-built binary from:"
            print_error "https://github.com/${GITHUB_REPO}/releases"
            exit 1
        fi
    fi
    
    # Copy Ovie logo if available
    if [ -f "ovie.png" ]; then
        cp ovie.png "$INSTALL_DIR/"
        print_status "Installed Ovie logo (ovie.png)"
    fi
    
    # Make executable
    chmod +x "$INSTALL_DIR/ovie" 2>/dev/null || true
    chmod +x "$INSTALL_DIR/oviec" 2>/dev/null || true
}

# Install VS Code extension
install_vscode_extension() {
    if [ "$INCLUDE_EXTENSION" = true ] && command_exists code; then
        print_status "Installing Ovie VS Code extension..."
        
        if [ -d "extensions/ovie-vscode" ]; then
            cd extensions/ovie-vscode
            
            if command_exists npm; then
                npm install --silent
                npm run compile
                
                if command_exists vsce; then
                    npm run package
                    vsix=$(ls *.vsix 2>/dev/null | head -n1)
                    if [ -n "$vsix" ]; then
                        code --install-extension "$vsix" --force
                        print_success "VS Code extension installed successfully"
                    fi
                else
                    print_warning "vsce not found. Install with: npm install -g vsce"
                fi
            else
                print_warning "npm not found. Skipping VS Code extension installation"
            fi
            
            cd ../..
        fi
    fi
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
        print_success "Ovie Stage 2 installed successfully!"
        print_status "Version information:"
        "$INSTALL_DIR/ovie" --version 2>/dev/null || echo "  ovie: installed (CLI tool)"
        "$INSTALL_DIR/oviec" --version 2>/dev/null || echo "  oviec: installed (self-hosted compiler)"
        
        echo ""
        print_status "🎯 Stage 2 Features Available:"
        echo -e "${GREEN}  • Natural language programming syntax${NC}"
        echo -e "${GREEN}  • Self-hosted compilation with oviec${NC}"
        echo -e "${GREEN}  • LLVM backend optimization${NC}"
        echo -e "${GREEN}  • WebAssembly compilation target${NC}"
        echo -e "${GREEN}  • Aproko static analysis${NC}"
        echo -e "${GREEN}  • Cross-platform deployment${NC}"
        echo -e "${GREEN}  • Memory safety guarantees${NC}"
        echo -e "${GREEN}  • AI-friendly development${NC}"
        echo ""
        print_status "Quick start:"
        echo "  ovie new my-project    # Create a new project"
        echo "  cd my-project"
        echo "  ovie run               # Run your project"
        echo "  ovie aproko            # Run code analysis"
        echo "  ovie compile --wasm    # Compile to WebAssembly"
        
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
    show_logo
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
    
    # Install VS Code extension
    install_vscode_extension
    
    # Setup PATH
    setup_path
    
    # Verify installation
    verify_installation
    
    # Cleanup
    cleanup
    
    print_success "🎉 Ovie Programming Language Stage 2 installed successfully!"
    echo ""
    print_status "🎯 Features available:"
    echo -e "${GREEN}  • Self-hosted compiler (oviec)${NC}"
    echo -e "${GREEN}  • Natural language syntax${NC}"
    echo -e "${GREEN}  • LLVM backend for optimization${NC}"
    echo -e "${GREEN}  • WebAssembly compilation${NC}"
    echo -e "${GREEN}  • Aproko code analyzer${NC}"
    echo -e "${GREEN}  • Cross-platform support${NC}"
    echo -e "${GREEN}  • Memory safety guarantees${NC}"
    echo ""
    print_status "Get started with: ovie new my-first-project"
}

# Handle interrupts
trap cleanup EXIT INT TERM

# Run main function
main "$@"