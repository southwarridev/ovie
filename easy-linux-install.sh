#!/bin/bash
# ============================================================================
#  Ovie Programming Language v2.3.0 — Easy Linux One-Click Installer
#
#  Two ways to run:
#    1. Already cloned the repo:
#       bash easy-linux-install.sh
#
#    2. Fresh machine (just needs git):
#       curl -sSL https://raw.githubusercontent.com/southwarridev/ovie/main/easy-linux-install.sh | bash
#
#  No Rust required. No release tarball needed.
#  Uses the prebuilt binaries inside linux-x64/ from the repo.
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
echo -e "${CYAN}  |                                                                          |${NC}"
echo -e "${CYAN}  |              OVIE PROGRAMMING LANGUAGE v2.3.0                           |${NC}"
echo -e "${CYAN}  |              Complete Module System - Full Package                       |${NC}"
echo -e "${CYAN}  |              Publisher: Nashedy Ltd  |  MIT License              |${NC}"
echo -e "${CYAN}  |                                                                          |${NC}"
echo -e "${CYAN}  ============================================================================${NC}"
echo ""

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
    fail "Git is not installed. Please install it first:
         Ubuntu/Debian : sudo apt install git
         Fedora        : sudo dnf install git
         Arch          : sudo pacman -S git"
fi
ok "Git found"

# ── Step 2: Find or clone the repo ───────────────────────────────────────────
info "[2/5] Locating Ovie source..."

REPO_ROOT=""
TEMP_CLONE=""

# Work out where this script is — handles both file execution and curl | bash
if [ -n "${BASH_SOURCE[0]}" ] && [ "${BASH_SOURCE[0]}" != "bash" ]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
else
    SCRIPT_DIR="$(pwd)"
fi

# If linux-x64/oviec exists next to this script we are already inside the repo
if [ -f "$SCRIPT_DIR/linux-x64/oviec" ]; then
    REPO_ROOT="$SCRIPT_DIR"
    ok "Using local repo at $REPO_ROOT"
else
    # Clone fresh — works on any machine that has git
    TEMP_CLONE="/tmp/ovie-install-$$"
    info "   Cloning from GitHub (takes ~30 s on first run)..."
    git clone --depth 1 "$GITHUB_REPO" "$TEMP_CLONE" 2>&1 \
        | grep -E "Cloning|done\." || true
    REPO_ROOT="$TEMP_CLONE"
    ok "Cloned to $TEMP_CLONE"
fi

SRC="$REPO_ROOT/linux-x64"
[ -f "$SRC/oviec" ] || fail "linux-x64/oviec not found in repo. Please report this at https://github.com/southwarridev/ovie/issues"

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

if [ -n "$ZSH_VERSION" ] || [ "$(basename "${SHELL:-bash}")" = "zsh" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -f "$HOME/.bashrc" ]; then
    SHELL_RC="$HOME/.bashrc"
else
    SHELL_RC="$HOME/.profile"
fi

if ! grep -q "$BIN_DIR" "$SHELL_RC" 2>/dev/null; then
    {
        echo ""
        echo "# Ovie Programming Language"
        echo "export PATH=\"$BIN_DIR:\$PATH\""
    } >> "$SHELL_RC"
    ok "Added to PATH in $SHELL_RC"
else
    ok "Already in PATH"
fi

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
echo -e "${GREEN}  |                    INSTALLATION COMPLETE!                               |${NC}"
echo -e "${GREEN}  ============================================================================${NC}"
echo ""
echo -e "  ${YELLOW}IMPORTANT:${NC} Reload your shell so PATH takes effect:"
echo "    source $SHELL_RC"
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


# -- oviev self-hosted compiler ------------------------
info "Installing oviev self-hosted compiler..."
OVIEV_SRC="${REPO_ROOT}/oviev"
OVIEV_DST="${INSTALL_DIR}/oviev"
if [ -d "$OVIEV_SRC" ]; then
  cp -r "$OVIEV_SRC" "$OVIEV_DST"
  cat >"${BIN_DIR}/oviev" <<'EOF'
#!/bin/bash
oviec run "$OVIEV_DST/src/main.ov" "$@"
EOF
  chmod +x "${BIN_DIR}/oviev"
  ok "oviev installed"
else
  warn "oviev folder not found - skipping"
fi

# -- VS Code extension ----------------------------------
VSIX="${REPO_ROOT}/extensions/ovie-vscode/ovie-lang.vsix"
if [ -f "$VSIX" ]; then
  if command -v code &>/dev/null; then
    code --install-extension "$VSIX" --force &>/dev/null
    ok "VS Code extension installed"
  else
    cp "$VSIX" "${INSTALL_DIR}/"
    ok "Extension saved. Install with: code --install-extension ${INSTALL_DIR}/ovie-lang.vsix"
  fi
fi
