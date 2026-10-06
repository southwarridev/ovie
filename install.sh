#!/bin/bash
# ============================================================================
#  Ovie Programming Language v2.3.0 — Linux/macOS Installer
#
#  Downloads prebuilt binaries from GitHub releases for your platform.
#  Supports: linux-x64, linux-arm64, macos-x64, macos-arm64
#
#  Usage:
#    bash install.sh
# ============================================================================

set -e

# ── colours ──────────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; NC='\033[0m'

ok()   { echo -e "  ${GREEN}[OK]${NC}    $*"; }
info() { echo -e "  ${CYAN}[>>]${NC}    $*"; }
warn() { echo -e "  ${YELLOW}[WARN]${NC}  $*"; }
fail() { echo -e "  ${RED}[ERROR]${NC} $*" >&2; exit 1; }

# ── banner ────────────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}  ============================================================================${NC}"
echo -e "${CYAN}  |              OVIE PROGRAMMING LANGUAGE v2.3.0                           |${NC}"
echo -e "${CYAN}  |              Publisher: Ovie Language Team  |  MIT License              |${NC}"
echo -e "${CYAN}  ============================================================================${NC}"
echo ""

INSTALL_DIR="$HOME/.local/ovie"
BIN_DIR="$HOME/.local/bin"
GITHUB_REPO="southwarridev/ovie"

# ── Detect platform ───────────────────────────────────────────────────────────
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)  PLATFORM="linux" ;;
    Darwin*) PLATFORM="macos" ;;
    *)       fail "Unsupported OS: $OS. Use the Windows installer for Windows." ;;
esac

case "$ARCH" in
    x86_64)          ARCH_SLUG="x64" ;;
    arm64|aarch64)   ARCH_SLUG="arm64" ;;
    *)               ARCH_SLUG="x64" ;;   # best-effort fallback
esac

# github asset names: ovie-linux-x64.tar.gz, ovie-linux-arm64.tar.gz, etc.
ASSET="ovie-${PLATFORM}-${ARCH_SLUG}.tar.gz"

info "Platform          : $OS ($ARCH)"
info "Asset             : $ASSET"
info "Install directory : $INSTALL_DIR"
info "Binaries added to : $BIN_DIR"
echo ""

# ── Step 1: Check git (optional, for fallback) ───────────────────────────────
info "[1/4] Checking requirements..."
if ! command -v curl >/dev/null 2>&1; then
    if [ "$PLATFORM" = "macos" ]; then
        fail "curl is not installed. Install Xcode Command Line Tools: xcode-select --install"
    else
        fail "curl is not installed. Run: sudo apt install curl (Ubuntu/Debian) or sudo dnf install curl (Fedora)"
    fi
fi
ok "curl found"

# ── Step 2: Download latest release ───────────────────────────────────────────
info "[2/4] Downloading latest Ovie release..."

API_URL="https://api.github.com/repos/$GITHUB_REPO/releases/latest"

# Fetch release info and extract download URL
info "   Fetching release information..."
RELEASE_JSON=$(curl -s "$API_URL" -H "Accept: application/vnd.github.v3+json")
DOWNLOAD_URL=$(echo "$RELEASE_JSON" | grep -o "\"browser_download_url\":[^,]*$ASSET[^,]*" | cut -d'"' -f4)

if [ -z "$DOWNLOAD_URL" ]; then
    # Fallback: parse with jq if available
    if command -v jq >/dev/null 2>&1; then
        DOWNLOAD_URL=$(echo "$RELEASE_JSON" | jq -r ".assets[] | select(.name==\"$ASSET\") | .browser_download_url")
    fi
fi

if [ -z "$DOWNLOAD_URL" ]; then
    echo "   Available assets in latest release:"
    echo "$RELEASE_JSON" | grep -o '"name":"[^"]*"' | head -10 | sed 's/"name":"//;s/"$//'
    fail "Could not find $ASSET in latest release"
fi

info "   Download URL: $DOWNLOAD_URL"

TEMP_FILE="/tmp/$ASSET"
info "   Downloading $ASSET..."
curl -L "$DOWNLOAD_URL" -o "$TEMP_FILE" -# 2>/dev/null || curl -L "$DOWNLOAD_URL" -o "$TEMP_FILE"

if [ ! -f "$TEMP_FILE" ]; then
    fail "Download failed. Check your internet connection."
fi
ok "Downloaded $ASSET"

# ── Step 3: Extract and install ───────────────────────────────────────────────
info "[3/4] Installing Ovie..."

# Create directories
mkdir -p "$INSTALL_DIR" "$BIN_DIR" \
         "$INSTALL_DIR/std" "$INSTALL_DIR/examples" "$INSTALL_DIR/docs"

# Extract the archive
EXTRACT_PATH="/tmp/ovie-installer-$$"
rm -rf "$EXTRACT_PATH"
mkdir -p "$EXTRACT_PATH"
tar -xzf "$TEMP_FILE" -C "$EXTRACT_PATH"

# Copy binaries
if [ -f "$EXTRACT_PATH/ovie/bin/oviec" ]; then
    install -m 755 "$EXTRACT_PATH/ovie/bin/oviec" "$BIN_DIR/oviec"
    ok "oviec installed"
else
    fail "oviec binary not found in archive"
fi

if [ -f "$EXTRACT_PATH/ovie/bin/ovie" ]; then
    install -m 755 "$EXTRACT_PATH/ovie/bin/ovie" "$BIN_DIR/ovie"
    ok "ovie installed"
fi

# Copy stdlib, examples, docs
[ -d "$EXTRACT_PATH/ovie/std" ]      && cp -r "$EXTRACT_PATH/ovie/std/."      "$INSTALL_DIR/std/"
[ -d "$EXTRACT_PATH/ovie/examples" ] && cp -r "$EXTRACT_PATH/ovie/examples/." "$INSTALL_DIR/examples/"
[ -d "$EXTRACT_PATH/ovie/docs" ]     && cp -r "$EXTRACT_PATH/ovie/docs/."     "$INSTALL_DIR/docs/"

for f in README.md LICENSE; do
    [ -f "$EXTRACT_PATH/ovie/$f" ] && cp "$EXTRACT_PATH/ovie/$f" "$INSTALL_DIR/"
done

STD_COUNT=$(find "$INSTALL_DIR/std"      -type f 2>/dev/null | wc -l | tr -d ' ')
EX_COUNT=$(find  "$INSTALL_DIR/examples" -name "*.ov" 2>/dev/null | wc -l | tr -d ' ')
ok "Standard library ($STD_COUNT files, 11 modules)"
ok "Examples ($EX_COUNT .ov files)"

# ── Step 4: Add to PATH ───────────────────────────────────────────────────────
info "[4/4] Adding $BIN_DIR to PATH..."

# Pick shell rc files
if [ "$PLATFORM" = "macos" ]; then
    SHELL_RCS=("$HOME/.zshrc" "$HOME/.bash_profile")
elif [ -n "$ZSH_VERSION" ] || [ "$(basename "${SHELL:-bash}")" = "zsh" ]; then
    SHELL_RCS=("$HOME/.zshrc")
elif [ -f "$HOME/.bashrc" ]; then
    SHELL_RCS=("$HOME/.bashrc")
else
    SHELL_RCS=("$HOME/.profile")
fi

for rc in "${SHELL_RCS[@]}"; do
    if ! grep -q "$BIN_DIR" "$rc" 2>/dev/null; then
        {
            echo ""
            echo "# Ovie Programming Language"
            echo "export PATH=\"$BIN_DIR:\$PATH\""
        } >> "$rc"
        ok "Added to PATH in $rc"
    else
        ok "Already in PATH ($rc)"
    fi
done

export PATH="$BIN_DIR:$PATH"

# ── Cleanup ────────────────────────────────────────────────────────────────────
rm -rf "$EXTRACT_PATH" "$TEMP_FILE"

# ── Verify ────────────────────────────────────────────────────────────────────
echo ""
info "Verifying installation..."
VERSION_OUT=$("$BIN_DIR/oviec" --version 2>&1 | head -1)
ok "$VERSION_OUT"

# ── Done ──────────────────────────────────────────────────────────────────────
echo ""
echo -e "${GREEN}  ============================================================================${NC}"
echo -e "${GREEN}  |                    INSTALLATION COMPLETE!                               |${NC}"
echo -e "${GREEN}  ============================================================================${NC}"
echo ""
echo -e "  ${YELLOW}IMPORTANT:${NC} Reload your shell so PATH takes effect:"
for rc in "${SHELL_RCS[@]}"; do echo "    source $rc"; done
echo ""
echo "  Quick start:"
echo "    oviec --version               # Check version"
echo "    oviec --self-check            # Validate installation"
echo "    oviec run examples/hello.ov   # Run hello world"
echo "    oviec new my-project          # Create new project"
echo ""
echo "  Downloads for all platforms:"
echo "    Windows x64  : https://github.com/$GITHUB_REPO/releases"
echo "    macOS x64    : https://github.com/$GITHUB_REPO/releases"
echo "    macOS arm64  : https://github.com/$GITHUB_REPO/releases"
echo "    Linux x64    : https://github.com/$GITHUB_REPO/releases"
echo "    Linux arm64  : https://github.com/$GITHUB_REPO/releases"
echo ""
echo "  Resources:"
echo "    Website : https://ovie.nashedy.io"
echo "    GitHub  : https://github.com/southwarridev/ovie"
echo "    Book    : https://southwarridev.github.io/ovie/docs/book/index.html"
echo "    Discord : https://discord.gg/AuF4ubMyE"
echo ""