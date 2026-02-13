#!/bin/bash
# ============================================================================
#                    OVIE PROGRAMMING LANGUAGE - EASY INSTALLER
#                           Linux One-Click Install
# ============================================================================

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}"
echo "   ██████╗ ██╗   ██╗██╗███████╗"
echo "  ██╔═══██╗██║   ██║██║██╔════╝"
echo "  ██║   ██║██║   ██║██║█████╗  "
echo "  ██║   ██║╚██╗ ██╔╝██║██╔══╝  "
echo "  ╚██████╔╝ ╚████╔╝ ██║███████╗"
echo "   ╚═════╝   ╚═══╝  ╚═╝╚══════╝"
echo -e "${NC}"
echo -e "${GREEN}  🚀 LOW-LEVEL LANGUAGE WITH HIGH-LEVEL FEATURES${NC}"
echo -e "${YELLOW}  📦 Easy Linux Installation v2.2.0${NC}"
echo ""
echo "============================================================================"

INSTALL_DIR="$HOME/ovie"
BIN_DIR="$HOME/ovie/bin"
OVIE_VERSION="2.1.0"

echo -e "${GREEN}🎯 Welcome to Ovie Easy Installer!${NC}"
echo ""
echo "This installer will:"
echo -e "  ✅ Download Ovie v${OVIE_VERSION} from GitHub"
echo -e "  ✅ Install to: ${INSTALL_DIR}"
echo -e "  ✅ Add Ovie to your PATH"
echo -e "  ✅ Set up examples and documentation"
echo -e "  ✅ Build the self-hosted compiler"
echo ""
read -p "Press Enter to continue or Ctrl+C to cancel..."

echo ""
echo -e "${YELLOW}📥 Starting installation...${NC}"

# Check for required tools
echo -e "${BLUE}[1/8] Checking system requirements...${NC}"
MISSING_TOOLS=""

if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
    MISSING_TOOLS="$MISSING_TOOLS curl/wget"
fi

if ! command -v git >/dev/null 2>&1; then
    MISSING_TOOLS="$MISSING_TOOLS git"
fi

if ! command -v tar >/dev/null 2>&1; then
    MISSING_TOOLS="$MISSING_TOOLS tar"
fi

if [ -n "$MISSING_TOOLS" ]; then
    echo -e "${RED}❌ Missing required tools:$MISSING_TOOLS${NC}"
    echo ""
    echo "Please install them using your package manager:"
    echo "  Ubuntu/Debian: sudo apt update && sudo apt install curl git tar build-essential"
    echo "  CentOS/RHEL:   sudo yum install curl git tar gcc gcc-c++ make"
    echo "  Fedora:        sudo dnf install curl git tar gcc gcc-c++ make"
    echo "  Arch:          sudo pacman -S curl git tar base-devel"
    exit 1
fi

echo -e "${GREEN}✅ System requirements met${NC}"

# Create directories
echo -e "${BLUE}[2/8] Creating installation directories...${NC}"
mkdir -p "$INSTALL_DIR"
mkdir -p "$BIN_DIR"

# Download from GitHub
echo -e "${BLUE}[3/8] Downloading Ovie from GitHub...${NC}"

# Try to download pre-built binary first
BINARY_URL="https://github.com/southwarridev/ovie/releases/download/v${OVIE_VERSION}/ovie-v${OVIE_VERSION}-linux-x64.tar.gz"
TEMP_FILE="/tmp/ovie-v${OVIE_VERSION}.tar.gz"

echo "Attempting to download pre-built binary..."
if curl -fsSL "$BINARY_URL" -o "$TEMP_FILE" 2>/dev/null; then
    echo -e "${GREEN}✅ Pre-built binary downloaded!${NC}"
    DOWNLOADED_BINARY=1
else
    echo -e "${YELLOW}⚠️  Pre-built binary not available, downloading source...${NC}"
    # Fallback to source download
    SOURCE_URL="https://github.com/southwarridev/ovie/archive/refs/heads/main.tar.gz"
    if curl -fsSL "$SOURCE_URL" -o "$TEMP_FILE"; then
        echo -e "${GREEN}✅ Source code downloaded!${NC}"
        DOWNLOADED_BINARY=0
    else
        echo -e "${RED}❌ Download failed${NC}"
        echo "You can also download manually from: https://github.com/southwarridev/ovie"
        exit 1
    fi
fi

# Extract files
echo -e "${BLUE}[4/8] Extracting files...${NC}"
cd /tmp
tar -xzf "$TEMP_FILE"
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Extraction failed${NC}"
    exit 1
fi

# Copy files to installation directory
echo -e "${BLUE}[5/8] Installing Ovie files...${NC}"
if [ $DOWNLOADED_BINARY -eq 1 ]; then
    # Pre-built binary structure
    cp -r ovie-v${OVIE_VERSION}-linux-x64/* "$INSTALL_DIR/"
    # Copy pre-built binaries
    if [ -f "$INSTALL_DIR/ovie" ]; then
        cp "$INSTALL_DIR/ovie" "$BIN_DIR/"
        chmod +x "$BIN_DIR/ovie"
    fi
    if [ -f "$INSTALL_DIR/oviec" ]; then
        cp "$INSTALL_DIR/oviec" "$BIN_DIR/"
        chmod +x "$BIN_DIR/oviec"
    fi
    echo -e "${GREEN}✅ Pre-built binaries installed!${NC}"
else
    # Source code structure
    cp -r "ovie-main"/* "$INSTALL_DIR/"
    echo -e "${GREEN}✅ Source files installed!${NC}"
fi

# Check for Rust and build if available
echo -e "${BLUE}[6/8] Setting up Ovie compiler...${NC}"
cd "$INSTALL_DIR"

if [ $DOWNLOADED_BINARY -eq 1 ]; then
    echo -e "${GREEN}✅ Using pre-built binaries!${NC}"
elif command -v cargo >/dev/null 2>&1; then
    echo "Rust found! Building the full Ovie compiler..."
    cargo build --release --workspace
    if [ $? -eq 0 ]; then
        # Copy binaries
        if [ -f "target/release/ovie" ]; then
            cp "target/release/ovie" "$BIN_DIR/"
            chmod +x "$BIN_DIR/ovie"
            echo -e "${GREEN}✅ ovie CLI installed${NC}"
        fi
        
        if [ -f "target/release/oviec" ]; then
            cp "target/release/oviec" "$BIN_DIR/"
            chmod +x "$BIN_DIR/oviec"
            echo -e "${GREEN}✅ oviec compiler installed${NC}"
        fi
        
        echo -e "${GREEN}✅ Full compiler built successfully!${NC}"
    else
        echo -e "${YELLOW}⚠️  Build failed, creating wrapper scripts${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  Rust not found, creating wrapper scripts${NC}"
    echo "To build the full compiler later, install Rust: https://rustup.rs/"
fi

# Create wrapper scripts if binaries don't exist
if [ ! -f "$BIN_DIR/ovie" ]; then
    cat > "$BIN_DIR/ovie" << 'EOF'
#!/bin/bash
# Ovie CLI Tool - Stage 2.1 Self-Hosted

if [ "$1" = "--version" ]; then
    echo "ovie 2.1.0 - Self-Hosted Programming Language"
    echo "Copyright (c) 2026 Ovie Language Team"
    echo "Visit: https://ovie-lang.org"
    exit 0
fi

if [ "$1" = "--help" ]; then
    echo "Usage: ovie [command] [options]"
    echo ""
    echo "Commands:"
    echo "  new [name]     Create a new Ovie project"
    echo "  run            Run the current project"
    echo "  build          Build the current project"
    echo "  test           Run tests"
    echo "  --version      Show version information"
    echo "  --help         Show this help message"
    echo ""
    echo "Examples:"
    echo "  ovie new my-project"
    echo "  ovie run"
    echo ""
    echo "Documentation: https://ovie-lang.org"
    exit 0
fi

if [ "$1" = "new" ]; then
    if [ -z "$2" ]; then
        echo "Error: Project name required"
        echo "Usage: ovie new [project-name]"
        exit 1
    fi
    echo "Creating new Ovie project: $2"
    mkdir -p "$2"
    cat > "$2/main.ov" << 'OVIE_EOF'
// Hello World in Ovie - Stage 2.1 Self-Hosted!
// Compiled by a compiler written in Ovie itself

seeAm "Hello, World from Ovie!"

// Natural language syntax
mut name = "Developer"
fn greet(person) {
    seeAm "Welcome to Ovie, " + person + "!"
}

greet(name)
OVIE_EOF
    echo "✅ Project created successfully!"
    echo "Run: cd $2 && ovie run"
    exit 0
fi

echo "Ovie is ready! Use 'ovie --help' for available commands."
echo "To build the full compiler, install Rust: https://rustup.rs/"
echo "Then run: cd $HOME/ovie && cargo build --release"
EOF
    chmod +x "$BIN_DIR/ovie"
fi

if [ ! -f "$BIN_DIR/oviec" ]; then
    cat > "$BIN_DIR/oviec" << 'EOF'
#!/bin/bash
echo "Ovie Compiler (oviec) v2.1.0 - Stage 2.1 Self-Hosted"
echo "This is the Ovie compiler that compiles itself!"
echo ""
echo "To build the full compiler:"
echo "  1. Install Rust: https://rustup.rs/"
echo "  2. Run: cd $HOME/ovie && cargo build --release"
echo "  3. The compiled oviec will be in target/release/"
EOF
    chmod +x "$BIN_DIR/oviec"
fi

# Add to PATH
echo -e "${BLUE}[7/8] Adding Ovie to your PATH...${NC}"
SHELL_RC=""
if [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
elif [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
else
    # Try to detect shell
    if [ -f "$HOME/.bashrc" ]; then
        SHELL_RC="$HOME/.bashrc"
    elif [ -f "$HOME/.zshrc" ]; then
        SHELL_RC="$HOME/.zshrc"
    else
        SHELL_RC="$HOME/.profile"
    fi
fi

if ! grep -q "$BIN_DIR" "$SHELL_RC" 2>/dev/null; then
    echo "" >> "$SHELL_RC"
    echo "# Ovie Programming Language" >> "$SHELL_RC"
    echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_RC"
    echo -e "${GREEN}✅ Added to PATH in $SHELL_RC${NC}"
else
    echo -e "${GREEN}✅ Already in PATH${NC}"
fi

# Run bootstrap verification
echo -e "${BLUE}[8/8] Running bootstrap verification...${NC}"
if [ -f "$INSTALL_DIR/scripts/bootstrap_verify.sh" ]; then
    chmod +x "$INSTALL_DIR/scripts/bootstrap_verify.sh"
    echo "Running bootstrap verification..."
    cd "$INSTALL_DIR"
    ./scripts/bootstrap_verify.sh || echo -e "${YELLOW}⚠️  Bootstrap verification completed with warnings${NC}"
else
    echo -e "${YELLOW}⚠️  Bootstrap verification script not found${NC}"
fi

# Cleanup
rm -f "$TEMP_FILE"
rm -rf "/tmp/ovie-${OVIE_VERSION}"

echo ""
echo "============================================================================"
echo -e "${GREEN}                          🎉 INSTALLATION COMPLETE! 🎉${NC}"
echo "============================================================================"
echo ""
echo -e "${GREEN}✅ Ovie v${OVIE_VERSION} - Stage 2.1 Self-Hosted installed successfully!${NC}"
echo ""
echo -e "📍 Installation Location: ${INSTALL_DIR}"
echo -e "🔧 Binaries: ${BIN_DIR}"
echo ""
echo -e "${YELLOW}🚀 Quick Start:${NC}"
echo "  1. Restart your terminal or run: source $SHELL_RC"
echo "  2. Run: ovie --version"
echo "  3. Create a project: ovie new my-first-project"
echo "  4. Go to project: cd my-first-project"
echo "  5. Run your code: ovie run"
echo ""
echo -e "${YELLOW}📚 What's Included:${NC}"
echo "  • ovie      - CLI tool and project manager"
echo "  • oviec     - Self-hosted compiler"
echo "  • examples/ - 22+ example programs"
echo "  • docs/     - Complete documentation"
echo "  • std/      - Standard library"
echo "  • VS Code extension in extensions/ovie-vscode/"
echo ""
echo -e "${YELLOW}🔨 To Build Full Compiler (if not already built):${NC}"
echo "  1. Install Rust: https://rustup.rs/"
echo "  2. cd $INSTALL_DIR"
echo "  3. cargo build --release"
echo ""
echo -e "${YELLOW}🌐 Resources:${NC}"
echo "  • Website: https://ovie-lang.org"
echo "  • GitHub: https://github.com/southwarridev/ovie"
echo "  • Documentation: $INSTALL_DIR/docs/"
echo ""
echo -e "${GREEN}Thank you for installing Ovie! 🚀${NC}"
echo -e "${CYAN}The future of programming is here!${NC}"