#!/usr/bin/env bash
# ============================================================================
# Ovie Real Bootstrap Verification Script - v2.2.0
# Compares Rust lexer output to Ovie lexer output for real self-hosting
# ============================================================================

set -e  # Exit on any error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}============================================================================${NC}"
echo -e "${BLUE}                    OVIE REAL BOOTSTRAP VERIFICATION${NC}"
echo -e "${BLUE}                         v2.2.0 - Rust vs Ovie Lexer${NC}"
echo -e "${BLUE}============================================================================${NC}"
echo ""

# Check if Rust is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Error: Rust/Cargo not found${NC}"
    echo "Please install Rust: https://rustup.rs/"
    exit 1
fi

echo -e "${BLUE}[1/2] Building Ovie Compiler${NC}"
echo "Building Ovie compiler with Rust toolchain..."

# Build with Rust
cargo build --release --workspace
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Compiler build failed${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Ovie compiler built successfully${NC}"
echo ""

echo -e "${BLUE}[2/2] Running Bootstrap Verification${NC}"
echo "Using built-in bootstrap verification command..."
echo ""

# Run the real bootstrap verification using the CLI command
./target/release/ovie self-host bootstrap verify "$@"

exit $?
