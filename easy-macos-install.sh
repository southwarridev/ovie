#!/bin/bash
# ============================================================================
#                    OVIE PROGRAMMING LANGUAGE - MACOS INSTALLER
#                           macOS Optimized Easy Install
# ============================================================================

set -e

# Configuration
OVIE_VERSION="2.1.0"
INSTALL_DIR="$HOME/.local"
BIN_DIR="$HOME/.local/bin"
TEMP_DIR="/tmp/ovie-install-$$"
GITHUB_REPO="https://github.com/southwarridev/ovie"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ASCII Art
echo ""
echo -e "${CYAN}   ██████╗ ██╗   ██╗██╗███████╗${NC}"
echo -e "${CYAN}  ██╔═══██╗██║   ██║██║██╔════╝${NC}"
echo -e "${CYAN}  ██║   ██║██║   ██║██║█████╗  ${NC}"
echo -e "${CYAN}  ██║   ██║╚██╗ ██╔╝██║██╔══╝  ${NC}"
echo -e "${CYAN}  ╚██████╔╝ ╚████╔╝ ██║███████╗${NC}"
echo -e "${CYAN}   ╚═════╝   ╚═══╝  ╚═╝╚══════╝${NC}"
echo ""
echo -e "${GREEN}   🍎 MACOS OPTIMIZED INSTALLATION${NC}"
echo -e "${YELLOW}   📦 Easy macOS Install v$OVIE_VERSION${NC}"
echo ""
echo -e "${BLUE}============================================================================${NC}"

# Detect macOS version and architecture
MACOS_VERSION=$(sw_vers -productVersion)
ARCH="$(uname -m)"

case "$ARCH" in
    x86_64)     ARCH_NAME="Intel x64";;
    arm64)      ARCH_NAME="Apple Silicon (M1/M2/M3)";;
    *)          ARCH_NAME="Unknown ($ARCH)";;
esac

echo -e "${GREEN}🍎 Welcome to Ovie macOS Installer!${NC}"
echo ""
echo -e "${BLUE}[INFO]${NC} macOS Version: $MACOS_VERSION"
echo -e "${BLUE}[INFO]${NC} Architecture: $ARCH_NAME"
echo ""
echo -e "${WHITE}This installer will:${NC}"
echo -e "  ✅ Install Xcode Command Line Tools (if needed)"
echo -e "  ✅ Install Homebrew (if needed)"
echo -e "  ✅ Download and build Ovie v$OVIE_VERSION"
echo -e "  ✅ Install to: $INSTALL_DIR"
echo -e "  ✅ Configure Terminal and Zsh/Bash"
echo -e "  ✅ Set up VS Code integration"
echo -e "  ✅ Install examples and documentation"
echo ""

# Function to check for Xcode Command Line Tools
check_xcode_tools() {
    if ! xcode-select -p >/dev/null 2>&1; then
        echo -e "${YELLOW}[INFO]${NC} Installing Xcode Command Line Tools..."
        xcode-select --install
        echo -e "${BLUE}[INFO]${NC} Please complete the Xcode installation and run this script again."
        echo -e "${BLUE}[INFO]${NC} Press any key after Xcode installation is complete..."
        read -n 1 -s
    fi
    echo -e "${GREEN}✅ Xcode Command Line Tools available${NC}"
}

# Function to install Homebrew
install_homebrew() {
    if ! command -v brew >/dev/null 2>&1; then
        echo -e "${BLUE}[INFO]${NC} Installing Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        
        # Add Homebrew to PATH for Apple Silicon Macs
        if [[ "$ARCH" == "arm64" ]]; then
            echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
            eval "$(/opt/homebrew/bin/brew shellenv)"
        fi
        echo -e "${GREEN}✅ Homebrew installed${NC}"
    else
        echo -e "${GREEN}✅ Homebrew found${NC}"
    fi
}

# Check system requirements
echo -e "${BLUE}[1/8]${NC} Checking macOS system requirements..."

# Check for Xcode Command Line Tools
check_xcode_tools

# Install Homebrew if needed
install_homebrew

# Install required tools via Homebrew
echo -e "${BLUE}[2/8]${NC} Installing required tools..."
brew install curl git

echo -e "${GREEN}✅ System requirements satisfied${NC}"

# Create directories
echo -e "${BLUE}[3/8]${NC} Creating installation directories..."
mkdir -p "$INSTALL_DIR" "$BIN_DIR" "$TEMP_DIR"

# Download source code
echo -e "${BLUE}[4/8]${NC} Downloading Ovie source code..."

# Try pre-built binary first
BINARY_URL="$GITHUB_REPO/releases/download/v$OVIE_VERSION/ovie-v$OVIE_VERSION-macos-$ARCH.tar.gz"
SOURCE_URL="$GITHUB_REPO/archive/refs/heads/main.tar.gz"

echo -e "${BLUE}[INFO]${NC} Attempting to download pre-built binary for $ARCH_NAME..."
if curl -fsSL "$BINARY_URL" -o "$TEMP_DIR/ovie-binary.tar.gz" 2>/dev/null; then
    echo -e "${GREEN}✅ Pre-built binary downloaded${NC}"
    cd "$TEMP_DIR"
    tar -xzf ovie-binary.tar.gz
    DOWNLOADED_BINARY=1
    cd "ovie-v$OVIE_VERSION-macos-$ARCH" 2>/dev/null || cd "ovie-"*
else
    echo -e "${YELLOW}[WARNING]${NC} Pre-built binary not available. Downloading source..."
    if curl -fsSL "$SOURCE_URL" -o "$TEMP_DIR/ovie-source.tar.gz"; then
        echo -e "${GREEN}✅ Source code downloaded${NC}"
        cd "$TEMP_DIR"
        tar -xzf ovie-source.tar.gz
        DOWNLOADED_BINARY=0
        cd "ovie-main"
    else
        echo -e "${YELLOW}[WARNING]${NC} Source download failed. Cloning latest..."
        git clone "$GITHUB_REPO.git" "$TEMP_DIR/ovie-source"
        cd "$TEMP_DIR/ovie-source"
        DOWNLOADED_BINARY=0
    fi
fi

# Install Rust if needed
echo -e "${BLUE}[5/8]${NC} Setting up Rust build environment..."
if ! command -v cargo >/dev/null 2>&1; then
    echo -e "${BLUE}[INFO]${NC} Installing Rust toolchain optimized for macOS..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    source "$HOME/.cargo/env"
    
    # Add Rust to shell profiles
    echo 'source "$HOME/.cargo/env"' >> ~/.zshrc 2>/dev/null || true
    echo 'source "$HOME/.cargo/env"' >> ~/.bash_profile 2>/dev/null || true
    
    echo -e "${GREEN}✅ Rust toolchain installed${NC}"
else
    echo -e "${GREEN}✅ Rust toolchain found${NC}"
fi

# Build Ovie
echo -e "${BLUE}[6/8]${NC} Building Ovie self-hosted compiler..."

if [ $DOWNLOADED_BINARY -eq 1 ]; then
    echo -e "${GREEN}✅ Using pre-built binaries for $ARCH_NAME!${NC}"
    # Copy pre-built binaries
    if [ -f "ovie" ]; then
        cp "ovie" "$BIN_DIR/"
        chmod +x "$BIN_DIR/ovie"
    fi
    if [ -f "oviec" ]; then
        cp "oviec" "$BIN_DIR/"
        chmod +x "$BIN_DIR/oviec"
    fi
else
    echo -e "${YELLOW}[INFO]${NC} Building for $ARCH_NAME - this may take a few minutes..."

    # Set macOS-specific build flags
    export MACOSX_DEPLOYMENT_TARGET="10.15"
    if [[ "$ARCH" == "arm64" ]]; then
        export CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="clang"
    else
        export CARGO_TARGET_X86_64_APPLE_DARWIN_LINKER="clang"
    fi

    # Try self-hosted build first if oviec exists
    if [ -f "oviec" ]; then
        echo -e "${BLUE}[INFO]${NC} Attempting self-hosted build..."
        if ./oviec --build-all --output-dir="$BIN_DIR" --target="macos-$ARCH" 2>/dev/null; then
            echo -e "${GREEN}✅ Self-hosted build successful!${NC}"
            BUILT_WITH_SELF_HOSTED=1
        else
            echo -e "${YELLOW}[WARNING]${NC} Self-hosted build failed, using Rust bootstrap..."
            BUILT_WITH_SELF_HOSTED=0
        fi
    else
        BUILT_WITH_SELF_HOSTED=0
    fi

    # Rust bootstrap build
    if [ $BUILT_WITH_SELF_HOSTED -eq 0 ]; then
        echo -e "${BLUE}[INFO]${NC} Building with Rust (bootstrap)..."
        cargo build --release --workspace
        
        # Copy binaries
        cp target/release/ovie "$BIN_DIR/"
        cp target/release/oviec "$BIN_DIR/"
        chmod +x "$BIN_DIR/ovie" "$BIN_DIR/oviec"
        echo -e "${GREEN}✅ Bootstrap build completed${NC}"
    fi
fi

# Install resources
echo -e "${BLUE}[7/8]${NC} Installing resources and documentation..."

# Copy standard library
if [ -d "std" ]; then
    cp -r std "$INSTALL_DIR/"
    echo -e "${GREEN}✅ Standard library installed${NC}"
fi

# Copy examples
if [ -d "examples" ]; then
    cp -r examples "$INSTALL_DIR/"
    EXAMPLE_COUNT=$(ls examples/*.ov 2>/dev/null | wc -l | tr -d ' ')
    echo -e "${GREEN}✅ Examples installed ($EXAMPLE_COUNT files)${NC}"
fi

# Copy documentation
if [ -d "docs" ]; then
    cp -r docs "$INSTALL_DIR/"
    echo -e "${GREEN}✅ Documentation installed${NC}"
fi

# Copy additional files
for file in ovie.png README.md LICENSE SELF_HOSTING_ACHIEVEMENT.md; do
    if [ -f "$file" ]; then
        cp "$file" "$INSTALL_DIR/"
    fi
done

# Copy VS Code extension
if [ -d "extensions/ovie-vscode" ]; then
    cp -r extensions "$INSTALL_DIR/"
    echo -e "${GREEN}✅ VS Code extension included${NC}"
fi

# Configure macOS-specific environment
echo -e "${BLUE}[8/8]${NC} Configuring macOS environment..."

# Add to both Zsh and Bash profiles
SHELL_CONFIGS=("$HOME/.zshrc" "$HOME/.bash_profile" "$HOME/.profile")

for config in "${SHELL_CONFIGS[@]}"; do
    if [ -f "$config" ] || [[ "$config" == *".zshrc" ]]; then
        if ! grep -q "$BIN_DIR" "$config" 2>/dev/null; then
            echo "" >> "$config"
            echo "# Ovie Programming Language - Stage 2 Self-Hosted" >> "$config"
            echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$config"
            echo -e "${GREEN}✅ Added to PATH in $(basename "$config")${NC}"
        fi
    fi
done

# Create macOS-optimized ovie command wrapper
cat > "$BIN_DIR/ovie" << 'EOF'
#!/bin/bash
# Ovie CLI Tool - Stage 2 Self-Hosted (macOS Optimized)
OVIE_HOME="$HOME/.local"

case "$1" in
    --version)
        echo "ovie 2.1.0 - Self-Hosted Programming Language (macOS)"
        echo "Copyright (c) 2026 Ovie Language Team"
        echo "Architecture: $(uname -m)"
        echo "Visit: https://ovie-lang.org"
        ;;
    --help)
        echo "Usage: ovie [command] [options]"
        echo ""
        echo "Commands:"
        echo "  new [name]     Create a new Ovie project"
        echo "  run            Run the current project"
        echo "  build          Build the current project"
        echo "  test           Run tests"
        echo "  aproko         Run code analysis"
        echo "  compile        Compile with options"
        echo "  vscode         Install VS Code extension"
        echo "  --version      Show version information"
        echo "  --help         Show this help message"
        echo ""
        echo "macOS Examples:"
        echo "  ovie new my-project"
        echo "  ovie run"
        echo "  ovie compile --target wasm"
        echo "  ovie vscode"
        echo ""
        echo "Documentation: https://ovie-lang.org"
        ;;
    new)
        if [ -z "$2" ]; then
            echo "Error: Project name required"
            echo "Usage: ovie new [project-name]"
            exit 1
        fi
        echo "Creating new Ovie project: $2"
        mkdir -p "$2"
        cat > "$2/main.ov" << 'OVIE_CODE'
// Hello World in Ovie - Stage 2 Self-Hosted!
// Running on macOS with native compilation

seeAm "Hello, World from Ovie on macOS!"
seeAm "Welcome to the future of programming!"

// Natural language syntax
mut name = "macOS Developer"
mut platform = "macOS"
fn greet(person, os) {
    seeAm "Hello, " + person + " on " + os + "!"
}

greet(name, platform)

// macOS-specific features available:
// - Native compilation for Apple Silicon and Intel
// - Optimized for macOS development
// - VS Code integration ready
OVIE_CODE
        
        cat > "$2/ovie.toml" << 'TOML_CONFIG'
[package]
name = "PROJECT_NAME"
version = "0.1.0"
description = "A new Ovie project for macOS"

[build]
target = "native"
optimization = "release"
macos_deployment_target = "10.15"

[dependencies]
# Add dependencies here
TOML_CONFIG
        sed -i '' "s/PROJECT_NAME/$2/g" "$2/ovie.toml"
        
        echo "✅ Project created successfully!"
        echo "Next steps:"
        echo "  cd $2"
        echo "  ovie run"
        ;;
    run)
        if [ -f "main.ov" ]; then
            echo "Running Ovie project on macOS..."
            "$OVIE_HOME/bin/oviec" main.ov -o main && ./main
        else
            echo "Error: No main.ov file found in current directory"
            echo "Create a new project with: ovie new [project-name]"
            exit 1
        fi
        ;;
    vscode)
        echo "Installing Ovie VS Code extension..."
        if command -v code >/dev/null 2>&1; then
            if [ -f "$OVIE_HOME/extensions/ovie-vscode/ovie-lang-1.0.0.vsix" ]; then
                code --install-extension "$OVIE_HOME/extensions/ovie-vscode/ovie-lang-1.0.0.vsix"
                echo "✅ VS Code extension installed!"
            else
                echo "Building VS Code extension..."
                cd "$OVIE_HOME/extensions/ovie-vscode"
                npm install && npm run package
                code --install-extension *.vsix
                echo "✅ VS Code extension built and installed!"
            fi
        else
            echo "VS Code not found. Please install VS Code first:"
            echo "https://code.visualstudio.com/download"
        fi
        ;;
    aproko)
        echo "Running Aproko code analysis..."
        if command -v "$OVIE_HOME/bin/aproko" >/dev/null 2>&1; then
            "$OVIE_HOME/bin/aproko" "${@:2}"
        else
            echo "Aproko analysis engine not found."
            echo "This feature will be available after full compilation."
        fi
        ;;
    compile)
        echo "Compiling with Ovie compiler..."
        "$OVIE_HOME/bin/oviec" "${@:2}"
        ;;
    *)
        echo "Ovie Programming Language v2.1.0 - Stage 2.1 Self-Hosted (macOS)"
        echo "Use 'ovie --help' for available commands."
        echo ""
        echo "Quick start:"
        echo "  ovie new my-project"
        echo "  cd my-project"
        echo "  ovie run"
        echo ""
        echo "macOS-specific:"
        echo "  ovie vscode    # Install VS Code extension"
        ;;
esac
EOF

chmod +x "$BIN_DIR/ovie"

# Create macOS app bundle (optional)
if command -v osascript >/dev/null 2>&1; then
    echo -e "${BLUE}[INFO]${NC} Creating macOS app shortcuts..."
    
    # Create Terminal shortcut for Ovie
    mkdir -p "$HOME/Applications/Ovie"
    cat > "$HOME/Applications/Ovie/Ovie Terminal.command" << 'EOF'
#!/bin/bash
cd "$HOME"
export PATH="$HOME/.local/bin:$PATH"
echo "Ovie Programming Language - Stage 2 Self-Hosted"
echo "Type 'ovie --help' for available commands"
echo ""
exec bash
EOF
    chmod +x "$HOME/Applications/Ovie/Ovie Terminal.command"
fi

# Cleanup
cd "$HOME"
rm -rf "$TEMP_DIR"

# Final verification
echo -e "${BLUE}[INFO]${NC} Verifying installation..."
if [ -x "$BIN_DIR/ovie" ] && [ -x "$BIN_DIR/oviec" ]; then
    echo ""
    echo -e "${BLUE}============================================================================${NC}"
    echo -e "${GREEN}                    🍎 MACOS INSTALLATION COMPLETE! 🎉${NC}"
    echo -e "${BLUE}============================================================================${NC}"
    echo ""
    echo -e "${GREEN}✅ Ovie v$OVIE_VERSION - Stage 2 Self-Hosted installed successfully!${NC}"
    echo ""
    echo -e "${YELLOW}📍 Installation Details:${NC}"
    echo -e "  • Location: $INSTALL_DIR"
    echo -e "  • Binaries: $BIN_DIR"
    echo -e "  • Architecture: $ARCH_NAME"
    echo -e "  • macOS Version: $MACOS_VERSION"
    echo -e "  • Standard Library: $INSTALL_DIR/std"
    echo -e "  • Examples: $INSTALL_DIR/examples"
    echo -e "  • Documentation: $INSTALL_DIR/docs"
    echo -e "  • VS Code Extension: $INSTALL_DIR/extensions/ovie-vscode"
    echo ""
    echo -e "${GREEN}🚀 Quick Start:${NC}"
    echo -e "  # Restart Terminal or run:"
    echo -e "  export PATH=\"$BIN_DIR:\$PATH\""
    echo -e ""
    echo -e "  # Verify installation:"
    echo -e "  ovie --version"
    echo -e ""
    echo -e "  # Create your first project:"
    echo -e "  ovie new hello-macos"
    echo -e "  cd hello-macos"
    echo -e "  ovie run"
    echo ""
    echo -e "${CYAN}🍎 macOS-Specific Features:${NC}"
    echo -e "  ✅ Optimized for Apple Silicon and Intel Macs"
    echo -e "  ✅ Native macOS compilation targets"
    echo -e "  ✅ Terminal integration"
    echo -e "  ✅ VS Code extension ready: ovie vscode"
    echo -e "  ✅ Homebrew integration"
    echo -e "  ✅ Xcode Command Line Tools integration"
    echo ""
    echo -e "${BLUE}🌐 Resources:${NC}"
    echo -e "  • Website: https://ovie-lang.org"
    echo -e "  • GitHub: https://github.com/southwarridev/ovie"
    echo -e "  • Documentation: $INSTALL_DIR/docs"
    echo -e "  • Examples: $INSTALL_DIR/examples"
    echo -e "  • Terminal Shortcut: ~/Applications/Ovie/"
    echo ""
    echo -e "${GREEN}Thank you for installing Ovie on macOS! 🚀${NC}"
    echo -e "${CYAN}The future of programming is here!${NC}"
    echo ""
else
    echo -e "${RED}[ERROR]${NC} Installation verification failed"
    echo -e "${YELLOW}[INFO]${NC} Please check the error messages above and try again"
    echo -e "${YELLOW}[INFO]${NC} For help, visit: https://github.com/southwarridev/ovie/issues"
    exit 1
fi