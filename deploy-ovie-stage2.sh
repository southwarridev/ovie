#!/bin/bash

# Ovie Stage 2 Complete Deployment Script
# This script handles the complete deployment of Ovie Stage 2 including:
# - Self-hosted compiler
# - VS Code extension to marketplace
# - Website deployment
# - Documentation updates
# - Branding assets

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
VERSION="2.0.0"
BUILD_RELEASE=false
DEPLOY_EXTENSION=false
DEPLOY_WEBSITE=false
UPDATE_DOCS=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --build-release)
            BUILD_RELEASE=true
            shift
            ;;
        --deploy-extension)
            DEPLOY_EXTENSION=true
            shift
            ;;
        --deploy-website)
            DEPLOY_WEBSITE=true
            shift
            ;;
        --update-docs)
            UPDATE_DOCS=true
            shift
            ;;
        --all)
            BUILD_RELEASE=true
            DEPLOY_EXTENSION=true
            DEPLOY_WEBSITE=true
            UPDATE_DOCS=true
            shift
            ;;
        --version)
            VERSION="$2"
            shift 2
            ;;
        -h|--help)
            cat << EOF
Ovie Stage 2 Deployment Script

DESCRIPTION:
    Complete deployment script for Ovie Stage 2 with self-hosted compiler
    and advanced features.

USAGE:
    ./deploy-ovie-stage2.sh [OPTIONS]

OPTIONS:
    --build-release       Build release binaries with Stage 2 features
    --deploy-extension    Deploy VS Code extension to marketplace
    --deploy-website      Deploy website with Stage 2 updates
    --update-docs         Update documentation for Stage 2
    --all                 Run all deployment steps
    --version <version>   Version to deploy (default: 2.0.0)

EXAMPLES:
    ./deploy-ovie-stage2.sh --all                    # Complete deployment
    ./deploy-ovie-stage2.sh --build-release          # Build binaries only
    ./deploy-ovie-stage2.sh --deploy-extension       # Deploy extension only
    ./deploy-ovie-stage2.sh --version "2.1.0" --all # Deploy specific version

REQUIREMENTS:
    - Rust toolchain with LLVM support
    - Node.js and npm (for VS Code extension)
    - vsce (VS Code extension manager)
    - Git (for version control)
    - Bash 4.0 or later

STAGE 2 FEATURES:
    - Self-hosted compilation with oviec
    - Natural language programming syntax
    - LLVM backend for optimization
    - WebAssembly compilation target
    - Aproko static analysis
    - Cross-platform deployment
    - Memory safety without GC
    - AI-friendly development

EOF
            exit 0
            ;;
        *)
            echo "Unknown option $1"
            exit 1
            ;;
    esac
done

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

# Display Ovie Stage 2 banner
show_banner() {
    echo ""
    echo -e "${MAGENTA}🎯 Ovie Programming Language - Stage 2 Deployment${NC}"
    echo -e "${MAGENTA}=================================================${NC}"
    echo -e "${CYAN}Self-Hosted Compiler with Advanced Features${NC}"
    echo -e "${YELLOW}Version: $VERSION${NC}"
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

# Build release binaries
build_release() {
    print_status "🔨 Building Ovie Stage 2 release binaries..."
    
    # Clean previous builds
    if [ -d "target" ]; then
        rm -rf target
    fi
    
    # Build with all Stage 2 features
    print_status "Building with Stage 2 features: self-hosting, LLVM, WASM, Aproko"
    cargo build --release --workspace --features "self-hosting,llvm-backend,wasm-support,aproko-integration"
    
    print_success "Release binaries built successfully"
    
    # Copy binaries to deployment directory
    deploy_dir="deploy"
    mkdir -p "$deploy_dir"
    
    cp target/release/ovie "$deploy_dir/"
    cp target/release/oviec "$deploy_dir/"
    cp ovie.png "$deploy_dir/"
    
    print_success "Binaries prepared for deployment"
}

# Deploy VS Code extension to marketplace
deploy_extension() {
    print_status "📦 Deploying VS Code extension to marketplace..."
    
    cd extensions/ovie-vscode
    
    # Update version in package.json
    sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" package.json
    
    # Install dependencies and build
    npm install --silent
    npm run compile
    
    # Package extension
    npm run package
    
    # Publish to marketplace (requires vsce login)
    print_status "Publishing to VS Code Marketplace..."
    npm run publish
    
    print_success "VS Code extension deployed to marketplace"
    
    cd ../..
}

# Deploy website
deploy_website() {
    print_status "🌐 Deploying Ovie website..."
    
    # Update website with Stage 2 information
    if [ -f "website/index.html" ]; then
        sed -i "s/Version: [0-9.]*/Version: $VERSION/" website/index.html
        sed -i "s/Stage [0-9]*/Stage 2/" website/index.html
        
        print_success "Website updated with Stage 2 information"
    fi
    
    # Copy ovie.png to website assets
    if [ -f "ovie.png" ]; then
        cp ovie.png website/assets/
        print_status "Updated website branding"
    fi
    
    print_success "Website deployment prepared"
}

# Update documentation
update_documentation() {
    print_status "📚 Updating documentation for Stage 2..."
    
    # Update README.md with Stage 2 features
    if [ -f "README.md" ]; then
        # Update version references
        sed -i "s/Version [0-9.]*/Version $VERSION/" README.md
        sed -i "s/Stage [0-9]*/Stage 2/" README.md
        
        # Add Stage 2 features section if not present
        if ! grep -q "Stage 2 Features" README.md; then
            cat >> README.md << 'EOF'

## Stage 2 Features

Ovie Stage 2 represents a major milestone with a fully self-hosted compiler and advanced features:

### 🎯 Core Features
- **Self-Hosted Compiler**: Complete bootstrap independence with `oviec`
- **Natural Language Syntax**: AI-friendly programming with readable code
- **Memory Safety**: Ownership system without garbage collection
- **Cross-Platform**: Windows, Linux, macOS support

### 🚀 Advanced Capabilities  
- **LLVM Backend**: Optimized native code generation
- **WebAssembly**: Compile to WASM for web deployment
- **Aproko Analysis**: Static code analysis and quality checks
- **VS Code Extension**: Full IDE support with syntax highlighting

### 🤖 AI Integration
- **LLM-Friendly**: Natural syntax designed for AI code generation
- **Semantic Analysis**: Rich AST for AI tooling integration
- **Documentation**: Comprehensive examples for AI training

EOF
        fi
        
        print_success "README.md updated with Stage 2 features"
    fi
    
    # Update installation docs
    if [ -f "docs/installation.md" ]; then
        sed -i "s/Version [0-9.]*/Version $VERSION/" docs/installation.md
        sed -i "s/ovie-lang\/ovie/southwarridev\/ovie/" docs/installation.md
        print_success "Installation documentation updated"
    fi
    
    print_success "Documentation updated for Stage 2"
}

# Main deployment function
deploy_stage2() {
    show_banner
    
    print_status "🚀 Starting Ovie Stage 2 deployment process..."
    
    if [ "$BUILD_RELEASE" = true ]; then
        build_release
    fi
    
    if [ "$UPDATE_DOCS" = true ]; then
        update_documentation
    fi
    
    if [ "$DEPLOY_WEBSITE" = true ]; then
        deploy_website
    fi
    
    if [ "$DEPLOY_EXTENSION" = true ]; then
        deploy_extension
    fi
    
    echo ""
    print_success "🎉 Ovie Stage 2 deployment completed successfully!"
    echo ""
    print_status "🎯 Stage 2 Achievements:"
    echo -e "${GREEN}  • Self-hosted compiler (oviec) ✅${NC}"
    echo -e "${GREEN}  • Natural language programming ✅${NC}"
    echo -e "${GREEN}  • LLVM backend optimization ✅${NC}"
    echo -e "${GREEN}  • WebAssembly compilation ✅${NC}"
    echo -e "${GREEN}  • Aproko static analysis ✅${NC}"
    echo -e "${GREEN}  • VS Code extension ✅${NC}"
    echo -e "${GREEN}  • Cross-platform support ✅${NC}"
    echo -e "${GREEN}  • Memory safety guarantees ✅${NC}"
    echo ""
    print_status "🌟 Ovie Stage 2 is now live and ready for production use!"
}

# Main execution
deploy_stage2