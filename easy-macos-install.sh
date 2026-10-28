#!/bin/bash
# ============================================================================
#  Ovie Programming Language v2.3.0 — Easy macOS One-Click Installer
#
#  Supports both Apple Silicon (M1/M2/M3) and Intel Macs.
#
#  Two ways to run:
#    1. Already cloned the repo:
#       bash easy-macos-install.sh
#
#    2. Fresh machine (just needs git):
#       curl -sSL https://raw.githubusercontent.com/southwarridev/ovie/main/easy-macos-install.sh | bash
#
#  No Rust required. No release tarball needed.
#  Uses the prebuilt binaries inside macos-arm64/ or macos-x64/ from the repo.
# ============================================================================

set -e

# ── colours ──────────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; NC='\033[0m'

ok()   { echo -e "  ${GREEN}[OK]${NC}    $*"; }
info() { echo -e "  ${CYAN}[>>]${NC}    $*"; }
warn() { echo -e "  ${YELLOW}[WARN]${NC}  $*"; }
fail() { echo -e "  ${RED}[ERROR]${NC} $*" >&2; exit 1; }

# ── Detect architecture ───────────────────────────────────────────────────────
ARCH="$(uname -m)"
case "$ARCH" in
    arm64|aarch64)  ARCH_SLUG="arm64"; ARCH_NAME="Apple Silicon (M1/M2/M3)" ;;
    x86_64)         ARCH_SLUG="x64";   ARCH_NAME="Intel x64" ;;
    *)              ARCH_SLUG="x64";   ARCH_NAME="Unknown (defaulting to x64)" ;;
esac

PLATFORM_DIR="macos-${ARCH_SLUG}"
MACOS_VERSION="$(sw_vers -productVersion 2>/dev/null || echo unknown)"

# ── banner ────────────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}  ============================================================================${NC}"
echo -e "${CYAN}  |                                                                          |${NC}"
echo -e "${CYAN}  |              OVIE PROGRAMMING LANGUAGE v2.3.0                           |${NC}"
echo -e "${CYAN}  |              Complete Module System - Full Package                       |${NC}"
echo -e "${CYAN}  |              Publisher: Nashedy Ltd  |  MIT License              |${NC}"
echo -e "${CYAN}  |                                                                          |${NC}"
echo -e "${CYAN}  ============================================================================${NC}"
echo ""
echo "  macOS Version     : $MACOS_VERSION"
echo "  Architecture      : $ARCH_NAME"

OVIE_VERSION="2.3.0"
INSTALL_DIR="$HOME/.local/ovie"
BIN_DIR="$HOME/.local/bin"
GITHUB_REPO="https://github.com/southwarridev/ovie.git"

echo "  Install directory : $INSTALL_DIR"
echo "  Binaries added to : $BIN_DIR"
echo ""
read -rp "  Press ENTER to install or Ctrl+C to cancel..."
echo ""

# ── Step 1: Check git ────────────────────────────────────────────────────────
info "[1/5] Checking requirements..."
if ! command -v git >/dev/null 2>&1; then
    fail "Git is not installed. Run this first:
         xcode-select --install
         Then re-run this installer."
fi
ok "Git found"

# ── Step 2: Find or clone the repo ───────────────────────────────────────────
info "[2/5] Locating Ovie source ($PLATFORM_DIR/)..."

REPO_ROOT=""
TEMP_CLONE=""

if [ -n "${BASH_SOURCE[0]}" ] && [ "${BASH_SOURCE[0]}" != "bash" ]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
else
    SCRIPT_DIR="$(pwd)"
fi

if [ -f "$SCRIPT_DIR/$PLATFORM_DIR/oviec" ]; then
    REPO_ROOT="$SCRIPT_DIR"
    ok "Using local repo at $REPO_ROOT"
else
    TEMP_CLONE="/tmp/ovie-install-$$"
    info "   Cloning from GitHub (takes ~30 s on first run)..."
    git clone --depth 1 "$GITHUB_REPO" "$TEMP_CLONE" 2>&1 \
        | grep -E "Cloning|done\." || true
    REPO_ROOT="$TEMP_CLONE"
    ok "Cloned to $TEMP_CLONE"
fi

SRC="$REPO_ROOT/$PLATFORM_DIR"

# Fallback: if arm64 folder is missing try x64
if [ ! -f "$SRC/oviec" ] && [ "$ARCH_SLUG" = "arm64" ]; then
    warn "macos-arm64/ not found — falling back to macos-x64/ (Rosetta 2 will be used)"
    SRC="$REPO_ROOT/macos-x64"
fi

[ -f "$SRC/oviec" ] || fail "Cannot find oviec in $SRC. Please report this at https://github.com/southwarridev/ovie/issues"

ok "Binaries found in $SRC"

# ── Step 3: Create directories ───────────────────────────────────────────────
info "[3/5] Creating install directories..."
mkdir -p "$INSTALL_DIR" "$BIN_DIR" \
         "$INSTALL_DIR/std" "$INSTALL_DIR/examples" "$INSTALL_DIR/docs"
ok "Directories created at $INSTALL_DIR"

# ── Step 4: Copy binaries and resources ──────────────────────────────────────
info "[4/5] Copying binaries..."

install -m 755 "$SRC/oviec" "$BIN_DIR/oviec"
ok "oviec installed"

if [ -f "$SRC/ovie" ]; then
    install -m 755 "$SRC/ovie" "$BIN_DIR/ovie"
else
    cp "$BIN_DIR/oviec" "$BIN_DIR/ovie"
    chmod +x "$BIN_DIR/ovie"
fi
ok "ovie installed"

# Remove macOS quarantine attribute so binaries run without Gatekeeper prompt
xattr -d com.apple.quarantine "$BIN_DIR/oviec" 2>/dev/null || true
xattr -d com.apple.quarantine "$BIN_DIR/ovie"  2>/dev/null || true

info "   Copying standard library, examples and docs..."
[ -d "$SRC/std" ]      && cp -r "$SRC/std/."      "$INSTALL_DIR/std/"
[ -d "$SRC/examples" ] && cp -r "$SRC/examples/." "$INSTALL_DIR/examples/"
[ -d "$SRC/docs" ]     && cp -r "$SRC/docs/."     "$INSTALL_DIR/docs/"

for f in README.md LICENSE RELEASE_NOTES_v2.3.md ovie.png ovie.svg ovie.toml.template; do
    [ -f "$SRC/$f" ] && cp "$SRC/$f" "$INSTALL_DIR/"
done

STD_COUNT=$(find "$INSTALL_DIR/std"      -type f 2>/dev/null | wc -l | tr -d ' ')
EX_COUNT=$(find  "$INSTALL_DIR/examples" -name "*.ov" 2>/dev/null | wc -l | tr -d ' ')
ok "Standard library ($STD_COUNT files, 11 modules)"
ok "Examples ($EX_COUNT .ov files)"

# ── Step 5: Add to PATH ───────────────────────────────────────────────────────
info "[5/5] Adding $BIN_DIR to PATH..."

# macOS uses zsh by default since Catalina; also update bash_profile for users on bash
for rc in "$HOME/.zshrc" "$HOME/.bash_profile"; do
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

# ── Cleanup temp clone ────────────────────────────────────────────────────────
if [ -n "$TEMP_CLONE" ] && [ -d "$TEMP_CLONE" ]; then
    rm -rf "$TEMP_CLONE"
fi

# ── Verify ────────────────────────────────────────────────────────────────────
echo ""
info "Verifying installation..."
VERSION_OUT=$("$BIN_DIR/oviec" --version 2>&1 | head -1)
ok "$VERSION_OUT"

# ── Done ──────────────────────────────────────────────────────────────────────
echo ""
echo -e "${GREEN}  ============================================================================${NC}"
echo -e "${GREEN}  |                  macOS INSTALLATION COMPLETE!                           |${NC}"
echo -e "${GREEN}  ============================================================================${NC}"
echo ""
echo -e "  ${YELLOW}IMPORTANT:${NC} Reload your shell so PATH takes effect:"
echo "    source ~/.zshrc          # zsh (default on macOS)"
echo "    source ~/.bash_profile   # bash"
echo ""
echo "  Quick start:"
echo "    oviec --version               # Check version"
echo "    oviec --self-check            # Validate installation"
echo "    oviec run examples/hello.ov   # Run hello world"
echo "    oviec new my-project          # Create new project"
echo ""
echo "  Resources:"
echo "    Website : https://ovie.nashedy.io"
echo "    GitHub  : https://github.com/southwarridev/ovie"
echo "    Book    : https://southwarridev.github.io/ovie/docs/book/index.html"
echo "    Discord : https://discord.gg/AuF4ubMyE"
echo ""
