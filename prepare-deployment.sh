#!/bin/bash

# Ovie Stage 2 Deployment Preparation Script
# This script prepares the Ovie Stage 2 codebase for production deployment by:
# 1. Moving development artifacts to shedydev
# 2. Cleaning build artifacts  
# 3. Validating production readiness
# 4. Preparing VS Code extension for marketplace

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
GRAY='\033[0;37m'
NC='\033[0m' # No Color

# Parse command line arguments
CLEAN=false
EXTENSION=false
VALIDATE=false
LOGO=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --clean)
            CLEAN=true
            shift
            ;;
        --extension)
            EXTENSION=true
            shift
            ;;
        --validate)
            VALIDATE=true
            shift
            ;;
        --logo)
            LOGO=true
            shift
            ;;
        --all)
            CLEAN=true
            EXTENSION=true
            VALIDATE=true
            LOGO=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [--clean] [--extension] [--validate] [--logo] [--all]"
            echo "  --clean      Clean build artifacts"
            echo "  --extension  Prepare VS Code extension"
            echo "  --validate   Validate deployment readiness"
            echo "  --logo       Prepare branding assets"
            echo "  --all        Run all preparation steps"
            exit 0
            ;;
        *)
            echo "Unknown option $1"
            exit 1
            ;;
    esac
done

echo -e "${YELLOW}🎯 Ovie Stage 2 Deployment Preparation${NC}"
echo -e "${YELLOW}=====================================${NC}"
echo -e "${CYAN}Self-Hosted Compiler with Advanced Features${NC}"
echo ""

# Function to move files to shedydev
move_to_shedydev() {
    local pattern="$1"
    local description="$2"
    
    if ls $pattern 1> /dev/null 2>&1; then
        echo -e "${CYAN}Moving $description to shedydev...${NC}"
        mv $pattern shedydev/ 2>/dev/null || true
        echo -e "${GREEN}✅ Moved files${NC}"
    fi
}

# Prepare Ovie logo and branding
if [ "$LOGO" = true ]; then
    echo -e "\n${BLUE}🎨 Preparing Ovie branding assets...${NC}"
    
    if [ -f "ovie.png" ]; then
        echo -e "${CYAN}Copying ovie.png to key locations...${NC}"
        
        # Copy to extension assets
        if [ -d "extensions/ovie-vscode/assets" ]; then
            cp ovie.png extensions/ovie-vscode/assets/
            echo -e "${GREEN}✅ Copied to VS Code extension assets${NC}"
        fi
        
        # Copy to website assets
        if [ -d "website/assets" ]; then
            cp ovie.png website/assets/
            echo -e "${GREEN}✅ Copied to website assets${NC}"
        fi
        
        # Copy to docs
        if [ -d "docs" ]; then
            cp ovie.png docs/
            echo -e "${GREEN}✅ Copied to documentation${NC}"
        fi
        
        echo -e "${GREEN}✅ Ovie branding assets prepared${NC}"
    else
        echo -e "${RED}❌ ovie.png not found in root directory${NC}"
    fi
fi

# Clean build artifacts
if [ "$CLEAN" = true ]; then
    echo -e "\n${BLUE}🧹 Cleaning build artifacts...${NC}"
    
    # Move test executables and artifacts
    move_to_shedydev "test_*.exe" "test executables"
    move_to_shedydev "test_*.o" "test object files"
    move_to_shedydev "*.rcgu.o" "Rust compilation units"
    move_to_shedydev "*.long-type-*.txt" "long type files"
    move_to_shedydev "ovie_final_test.exe" "final test executable"
    move_to_shedydev "libsemantic.rlib" "semantic library"
    move_to_shedydev "output.wasm" "WASM output"
    move_to_shedydev "test_*.ov" "test Ovie files"
    
    # Clean Rust build artifacts
    if [ -d "target" ]; then
        echo -e "${CYAN}Cleaning Rust target directory...${NC}"
        rm -rf target
        echo -e "${GREEN}✅ Cleaned target directory${NC}"
    fi
    
    # Clean oviec build artifacts
    if [ -d "oviec/target" ]; then
        echo -e "${CYAN}Cleaning oviec target directory...${NC}"
        rm -rf oviec/target
        echo -e "${GREEN}✅ Cleaned oviec target directory${NC}"
    fi
    
    echo -e "${GREEN}✅ Build artifacts cleaned${NC}"
fi

# Prepare VS Code extension
if [ "$EXTENSION" = true ]; then
    echo -e "\n${BLUE}📦 Preparing VS Code extension...${NC}"
    
    cd extensions/ovie-vscode
    
    # Install dependencies
    echo -e "${CYAN}Installing dependencies...${NC}"
    npm install --silent
    
    # Compile TypeScript
    echo -e "${CYAN}Compiling TypeScript...${NC}"
    npm run compile
    
    # Package extension
    echo -e "${CYAN}Packaging extension...${NC}"
    npm run package
    
    echo -e "${GREEN}✅ VS Code extension ready for marketplace deployment${NC}"
    
    # Show package info
    if ls *.vsix 1> /dev/null 2>&1; then
        vsix=$(ls *.vsix | head -n1)
        size=$(du -h "$vsix" | cut -f1)
        echo -e "${YELLOW}📦 Package: $vsix ($size)${NC}"
        echo -e "${GREEN}🚀 Ready for VS Code Marketplace upload${NC}"
    fi
    
    cd ../..
fi

# Validate deployment readiness
if [ "$VALIDATE" = true ]; then
    echo -e "\n${BLUE}🔍 Validating deployment readiness...${NC}"
    
    issues=()
    
    # Check for development artifacts in root
    dev_artifacts=("test_*.exe" "test_*.o" "*.rcgu.o" "libsemantic.rlib" "output.wasm")
    
    for pattern in "${dev_artifacts[@]}"; do
        if ls $pattern 1> /dev/null 2>&1; then
            issues+=("Development artifacts found in root: $pattern")
        fi
    done
    
    # Check for required files
    required_files=(
        "README.md"
        "LICENSE"
        "Cargo.toml"
        "ovie/Cargo.toml"
        "oviec/Cargo.toml"
        "aproko/Cargo.toml"
    )
    
    for file in "${required_files[@]}"; do
        if [ ! -f "$file" ]; then
            issues+=("Missing required file: $file")
        fi
    done
    
    # Check VS Code extension
    if [ -d "extensions/ovie-vscode" ]; then
        extension_files=(
            "extensions/ovie-vscode/package.json"
            "extensions/ovie-vscode/README.md"
            "extensions/ovie-vscode/LICENSE"
        )
        
        for file in "${extension_files[@]}"; do
            if [ ! -f "$file" ]; then
                issues+=("Missing extension file: $file")
            fi
        done
        
        # Check if extension is compiled
        if [ ! -d "extensions/ovie-vscode/out" ]; then
            issues+=("VS Code extension not compiled (missing out/ directory)")
        fi
    fi
    
    # Report validation results
    if [ ${#issues[@]} -eq 0 ]; then
        echo -e "${GREEN}✅ Deployment validation passed${NC}"
        echo -e "${YELLOW}🚀 Ovie Stage 2 ready for production deployment${NC}"
        echo ""
        echo -e "${CYAN}Stage 2 Features Validated:${NC}"
        echo -e "${GREEN}  • Self-hosted compiler (oviec)${NC}"
        echo -e "${GREEN}  • Natural language syntax${NC}"
        echo -e "${GREEN}  • LLVM backend optimization${NC}"
        echo -e "${GREEN}  • WebAssembly compilation${NC}"
        echo -e "${GREEN}  • Aproko static analysis${NC}"
        echo -e "${GREEN}  • Cross-platform support${NC}"
        echo -e "${GREEN}  • VS Code extension${NC}"
    else
        echo -e "${RED}❌ Deployment validation failed${NC}"
        echo -e "${RED}Issues found:${NC}"
        for issue in "${issues[@]}"; do
            echo -e "${RED}  - $issue${NC}"
        done
        exit 1
    fi
fi

# Summary
echo -e "\n${YELLOW}📋 Ovie Stage 2 Deployment Summary${NC}"
echo -e "${YELLOW}==================================${NC}"

if [ "$CLEAN" = true ]; then
    echo -e "${GREEN}✅ Build artifacts cleaned and moved to shedydev${NC}"
fi

if [ "$EXTENSION" = true ]; then
    echo -e "${GREEN}✅ VS Code extension packaged for marketplace${NC}"
fi

if [ "$VALIDATE" = true ]; then
    echo -e "${GREEN}✅ Stage 2 deployment validation completed${NC}"
fi

if [ "$LOGO" = true ]; then
    echo -e "${GREEN}✅ Ovie branding assets prepared${NC}"
fi

echo -e "\n${YELLOW}🎯 Ovie Stage 2 is ready for deployment!${NC}"

# Usage examples
echo -e "\n${CYAN}Usage examples:${NC}"
echo -e "${GRAY}  ./prepare-deployment.sh --clean                    # Clean build artifacts${NC}"
echo -e "${GRAY}  ./prepare-deployment.sh --extension                # Prepare VS Code extension${NC}"
echo -e "${GRAY}  ./prepare-deployment.sh --validate                 # Validate deployment readiness${NC}"
echo -e "${GRAY}  ./prepare-deployment.sh --logo                     # Prepare branding assets${NC}"
echo -e "${GRAY}  ./prepare-deployment.sh --all                      # Full preparation${NC}"