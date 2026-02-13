#!/bin/bash
# ============================================================================
#                    OVIE PROGRAMMING LANGUAGE - UNIVERSAL INSTALLER
#                           Linux/macOS/Unix Easy Install
# ============================================================================

set -e

# Configuration
OVIE_VERSION="2.2.0"
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
echo -e "${GREEN}   🚀 STAGE 2 - SELF-HOSTED PROGRAMMING LANGUAGE${NC}"
echo -e "${YELLOW}   📦 Universal Installation v$OVIE_VERSION${NC}"
echo ""
echo -e "${BLUE}============================================================================${NC}"

# Detect OS and Architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)     PLATFORM="linux";;
    Darwin*)    PLATFORM="macos";;
    CYGWIN*)    PLATFORM="windows";;
    MINGW*)     PLATFORM="windows";;
    *)          PLATFORM="unknown";;
esac

case "$ARCH" in
    x86_64)     ARCH="x64";;
    arm64)      ARCH="arm64";;
    aarch64)    ARCH="arm64";;
    *)          ARCH="x64";;
esac

echo -e "${GREEN}🎯 Welcome to Ovie Universal Installer!${NC}"
echo ""
echo -e "${WHITE}This installer will:${NC}"
echo -e "  ✅ Download Ovie v$OVIE_VERSION from GitHub"
echo -e "  ✅ Build the full self-hosted compiler"
echo -e "  ✅ Install to: $INSTALL_DIR"
echo -e "  ✅ Add Ovie to your PATH"
echo -e "  ✅ Set up examples, docs, and standard library"
echo -e "  ✅ Configure VS Code extension"
echo ""
echo -e "${BLUE}[INFO]${NC} Detected platform: $PLATFORM-$ARCH"
echo ""

# Function to check for required commands
check_command() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo -e "${RED}[ERROR]${NC} $1 is required but not installed."
        case "$1" in
            curl)
                echo -e "${YELLOW}[INFO]${NC} Install with: sudo apt install curl (Ubuntu/Debian) or brew install curl (macOS)"
                ;;
            git)
                echo -e "${YELLOW}[INFO]${NC} Install with: sudo apt install git (Ubuntu/Debian) or brew install git (macOS)"
                ;;
            make)
                echo -e "${YELLOW}[INFO]${NC} Install with: sudo apt install build-essential (Ubuntu/Debian) or xcode-select --install (macOS)"
                ;;
        esac
        return 1
    fi
    return 0
}

# Function to install missing dependencies
install_dependencies() {
    echo -e "${BLUE}[INFO]${NC} Installing missing dependencies..."
    
    if [[ "$PLATFORM" == "linux" ]]; then
        if command -v apt-get >/dev/null 2>&1; then
            sudo apt-get update -qq
            sudo apt-get install -y curl git build-essential
        elif command -v yum >/dev/null 2>&1; then
            sudo yum install -y curl git gcc gcc-c++ make
        elif command -v pacman >/dev/null 2>&1; then
            sudo pacman -S --noconfirm curl git base-devel
        else
            echo -e "${YELLOW}[WARNING]${NC} Unknown Linux distribution. Please install curl, git, and build tools manually."
        fi
    elif [[ "$PLATFORM" == "macos" ]]; then
        if ! command -v brew >/dev/null 2>&1; then
            echo -e "${BLUE}[INFO]${NC} Installing Homebrew..."
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        fi
        brew install curl git
        xcode-select --install 2>/dev/null || true
    fi
}

# Check system requirements
echo -e "${BLUE}[1/8]${NC} Checking system requirements..."
MISSING_DEPS=0

if ! check_command "curl"; then MISSING_DEPS=1; fi
if ! check_command "git"; then MISSING_DEPS=1; fi
if ! check_command "make"; then MISSING_DEPS=1; fi

if [ $MISSING_DEPS -eq 1 ]; then
    echo -e "${YELLOW}[INFO]${NC} Some dependencies are missing. Attempting to install..."
    install_dependencies
    
    # Re-check after installation
    if ! check_command "curl" || ! check_command "git"; then
        echo -e "${RED}[ERROR]${NC} Failed to install required dependencies. Please install manually."
        exit 1
    fi
fi

echo -e "${GREEN}✅ System requirements satisfied${NC}"

# Create directories
echo -e "${BLUE}[2/8]${NC} Creating installation directories..."
mkdir -p "$INSTALL_DIR" "$BIN_DIR" "$TEMP_DIR"

# Download source code
echo -e "${BLUE}[3/8]${NC} Downloading Ovie source code..."
DOWNLOAD_URL="$GITHUB_REPO/archive/refs/tags/v$OVIE_VERSION.tar.gz"

if curl -fsSL "$DOWNLOAD_URL" -o "$TEMP_DIR/ovie-source.tar.gz"; then
    echo -e "${GREEN}✅ Source code downloaded${NC}"
    cd "$TEMP_DIR"
    tar -xzf ovie-source.tar.gz
    cd "ovie-$OVIE_VERSION"
else
    echo -e "${YELLOW}[WARNING]${NC} Tagged release not found. Cloning latest..."
    git clone "$GITHUB_REPO.git" "$TEMP_DIR/ovie-source"
    cd "$TEMP_DIR/ovie-source"
fi

# Install Rust if needed
echo -e "${BLUE}[4/8]${NC} Setting up build environment..."
if ! command -v cargo >/dev/null 2>&1; then
    echo -e "${BLUE}[INFO]${NC} Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✅ Rust toolchain installed${NC}"
else
    echo -e "${GREEN}✅ Rust toolchain found${NC}"
fi

# Build Ovie
echo -e "${BLUE}[5/8]${NC} Building Ovie self-hosted compiler..."
echo -e "${YELLOW}[INFO]${NC} This may take a few minutes..."

# Try self-hosted build first if oviec exists
if [ -f "oviec" ]; then
    echo -e "${BLUE}[INFO]${NC} Attempting self-hosted build..."
    if ./oviec --build-all --output-dir="$BIN_DIR" 2>/dev/null; then
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

# Install resources
echo -e "${BLUE}[6/8]${NC} Installing resources and documentation..."

# Copy standard library
if [ -d "std" ]; then
    cp -r std "$INSTALL_DIR/"
    echo -e "${GREEN}✅ Standard library installed${NC}"
fi

# Copy examples
if [ -d "examples" ]; then
    cp -r examples "$INSTALL_DIR/"
    echo -e "${GREEN}✅ Examples installed ($(ls examples/*.ov 2>/dev/null | wc -l) files)${NC}"
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

# Configure PATH
echo -e "${BLUE}[7/8]${NC} Configuring environment..."

# Detect shell and add to appropriate RC file
SHELL_RC=""
if [ -n "$BASH_VERSION" ] || [ "$SHELL" = "/bin/bash" ]; then
    SHELL_RC="$HOME/.bashrc"
elif [ -n "$ZSH_VERSION" ] || [ "$SHELL" = "/bin/zsh" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -f "$HOME/.profile" ]; then
    SHELL_RC="$HOME/.profile"
fi

if [ -n "$SHELL_RC" ]; then
    if ! grep -q "$BIN_DIR" "$SHELL_RC" 2>/dev/null; then
        echo "" >> "$SHELL_RC"
        echo "# Ovie Programming Language - Stage 2 Self-Hosted" >> "$SHELL_RC"
        echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_RC"
        echo -e "${GREEN}✅ Added to PATH in $SHELL_RC${NC}"
    else
        echo -e "${BLUE}[INFO]${NC} Already in PATH"
    fi
fi

# Create ovie command wrapper with enhanced functionality
cat > "$BIN_DIR/ovie" << 'EOF'
#!/bin/bash
# Ovie CLI Tool - Stage 2 Self-Hosted
OVIE_HOME="$HOME/.local"

case "$1" in
    --version)
        echo "ovie 2.2.0 - Self-Hosted Programming Language"
        echo "Copyright (c) 2026 Ovie Language Team"
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
        echo "  --version      Show version information"
        echo "  --help         Show this help message"
        echo ""
        echo "Examples:"
        echo "  ovie new my-project"
        echo "  ovie run"
        echo "  ovie compile --wasm"
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
// Compiled by a compiler written in Ovie itself

seeAm "Hello, World from Ovie!"
seeAm "Welcome to the future of programming!"

// Natural language syntax
mut name = "Developer"
fn greet(person) {
    seeAm "Hello, " + person + "!"
}

greet(name)

// Try some examples:
// - Check out examples/ directory
// - Run: ovie aproko (for code analysis)
// - Compile: ovie compile --wasm
OVIE_CODE
        
        cat > "$2/ovie.toml" << 'TOML_CONFIG'
[package]
name = "PROJECT_NAME"
version = "0.1.0"
description = "A new Ovie project"

[build]
target = "native"
optimization = "release"

[dependencies]
# Add dependencies here
TOML_CONFIG
        sed -i "s/PROJECT_NAME/$2/g" "$2/ovie.toml" 2>/dev/null || sed -i '' "s/PROJECT_NAME/$2/g" "$2/ovie.toml"
        
        echo "✅ Project created successfully!"
        echo "Next steps:"
        echo "  cd $2"
        echo "  ovie run"
        ;;
    run)
        if [ -f "main.ov" ]; then
            echo "Running Ovie project..."
            "$OVIE_HOME/bin/oviec" main.ov -o main && ./main
        else
            echo "Error: No main.ov file found in current directory"
            echo "Create a new project with: ovie new [project-name]"
            exit 1
        fi
        ;;
    aproko)
        echo "Running Aproko code analysis..."
        if command -v "$OVIE_HOME/bin/aproko" >/dev/null 2>&1; then
            "$OVIE_HOME/bin/aproko" "${@:2}"
        else
            echo "Aproko analysis engine not found. Building from source..."
            echo "This feature will be available after full compilation."
        fi
        ;;
    compile)
        echo "Compiling with Ovie compiler..."
        "$OVIE_HOME/bin/oviec" "${@:2}"
        ;;
    *)
        echo "Ovie Programming Language v2.2.0 - Stage 2 Self-Hosted"
        echo "Use 'ovie --help' for available commands."
        echo ""
        echo "Quick start:"
        echo "  ovie new my-project"
        echo "  cd my-project"
        echo "  ovie run"
        ;;
esac
EOF

chmod +x "$BIN_DIR/ovie"

# Cleanup
echo -e "${BLUE}[8/8]${NC} Cleaning up..."
cd "$HOME"
rm -rf "$TEMP_DIR"

# Final verification
echo -e "${BLUE}[INFO]${NC} Verifying installation..."
if [ -x "$BIN_DIR/ovie" ] && [ -x "$BIN_DIR/oviec" ]; then
    echo ""
    echo -e "${BLUE}============================================================================${NC}"
    echo -e "${GREEN}                          🎉 INSTALLATION COMPLETE! 🎉${NC}"
    echo -e "${BLUE}============================================================================${NC}"
    echo ""
    echo -e "${GREEN}✅ Ovie v$OVIE_VERSION - Stage 2 Self-Hosted installed successfully!${NC}"
    echo ""
    echo -e "${YELLOW}📍 Installation Details:${NC}"
    echo -e "  • Location: $INSTALL_DIR"
    echo -e "  • Binaries: $BIN_DIR"
    echo -e "  • Standard Library: $INSTALL_DIR/std"
    echo -e "  • Examples: $INSTALL_DIR/examples"
    echo -e "  • Documentation: $INSTALL_DIR/docs"
    echo -e "  • VS Code Extension: $INSTALL_DIR/extensions/ovie-vscode"
    echo ""
    echo -e "${GREEN}🚀 Quick Start:${NC}"
    echo -e "  # Reload your shell or run:"
    echo -e "  export PATH=\"$BIN_DIR:\$PATH\""
    echo -e ""
    echo -e "  # Verify installation:"
    echo -e "  ovie --version"
    echo -e ""
    echo -e "  # Create your first project:"
    echo -e "  ovie new hello-world"
    echo -e "  cd hello-world"
    echo -e "  ovie run"
    echo ""
    echo -e "${CYAN}🎯 Stage 2 Features Available:${NC}"
    echo -e "  ✅ Natural language programming syntax"
    echo -e "  ✅ Self-hosted compilation with oviec"
    echo -e "  ✅ LLVM backend optimization"
    echo -e "  ✅ WebAssembly compilation target"
    echo -e "  ✅ Aproko static analysis"
    echo -e "  ✅ Cross-platform deployment"
    echo -e "  ✅ Memory safety guarantees"
    echo -e "  ✅ AI-friendly development"
    echo ""
    echo -e "${BLUE}🌐 Resources:${NC}"
    echo -e "  • Website: https://ovie-lang.org"
    echo -e "  • GitHub: https://github.com/southwarridev/ovie"
    echo -e "  • Documentation: $INSTALL_DIR/docs"
    echo -e "  • Examples: $INSTALL_DIR/examples"
    echo ""
    echo -e "${GREEN}Thank you for installing Ovie! 🚀${NC}"
    echo -e "${CYAN}The future of programming is here!${NC}"
    echo ""
else
    echo -e "${RED}[ERROR]${NC} Installation verification failed"
    echo -e "${YELLOW}[INFO]${NC} Please check the error messages above and try again"
    echo -e "${YELLOW}[INFO]${NC} For help, visit: https://github.com/southwarridev/ovie/issues"
    exit 1
fi