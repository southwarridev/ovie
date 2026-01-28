# Installation Guide

This guide covers installing the Ovie Programming Language toolchain on various platforms. Ovie is currently in Stage 0 (Rust implementation) and requires Rust to build from source.

## Quick Install

### Prerequisites

- **Rust 1.70+**: Required for building the Stage 0 compiler
- **Git**: For cloning the repository
- **LLVM 14+**: Optional, for native code generation

### One-Line Install (Unix/Linux/macOS)

```bash
curl -sSf https://install.ovie-lang.org | sh
```

### One-Line Install (Windows PowerShell)

```powershell
iwr -useb https://install.ovie-lang.org/install.ps1 | iex
```

## Manual Installation

### Step 1: Install Rust

If you don't have Rust installed:

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your shell or run:
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Step 2: Install LLVM (Optional)

LLVM is required for native code generation. Skip this step if you only need WASM output.

#### Ubuntu/Debian

```bash
sudo apt-get update
sudo apt-get install llvm-14-dev libclang-14-dev clang-14
```

#### CentOS/RHEL/Fedora

```bash
# Fedora
sudo dnf install llvm-devel clang-devel

# CentOS/RHEL (with EPEL)
sudo yum install llvm-devel clang-devel
```

#### macOS

```bash
# Using Homebrew
brew install llvm

# Add LLVM to PATH
echo 'export PATH="/opt/homebrew/opt/llvm/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

#### Windows

```powershell
# Using Chocolatey
choco install llvm

# Or download from https://releases.llvm.org/
# Add LLVM bin directory to PATH
```

### Step 3: Clone and Build Ovie

```bash
# Clone the repository
git clone https://github.com/ovie-lang/ovie.git
cd ovie

# Build the toolchain
cargo build --release

# Verify installation
./target/release/ovie --version
```

### Step 4: Add to PATH (Optional)

```bash
# Add Ovie to your PATH
echo 'export PATH="'$(pwd)'/target/release:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Or for zsh
echo 'export PATH="'$(pwd)'/target/release:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Verify
ovie --version
```

## Platform-Specific Instructions

### Linux

#### Ubuntu 20.04/22.04 LTS

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y curl git build-essential llvm-14-dev libclang-14-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build Ovie
git clone https://github.com/ovie-lang/ovie.git
cd ovie
cargo build --release

# Install globally (optional)
sudo cp target/release/ovie /usr/local/bin/
sudo cp target/release/oviec /usr/local/bin/
```

#### Arch Linux

```bash
# Install dependencies
sudo pacman -S rust git llvm clang

# Clone and build
git clone https://github.com/ovie-lang/ovie.git
cd ovie
cargo build --release
```

#### Alpine Linux

```bash
# Install dependencies
apk add --no-cache rust cargo git llvm14-dev clang-dev build-base

# Clone and build
git clone https://github.com/ovie-lang/ovie.git
cd ovie
cargo build --release
```

### macOS

#### macOS 11+ (Intel/Apple Silicon)

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install rust llvm git

# Clone and build
git clone https://github.com/ovie-lang/ovie.git
cd ovie
cargo build --release

# Add to PATH
echo 'export PATH="'$(pwd)'/target/release:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Windows

#### Windows 10/11

```powershell
# Install Rust via rustup-init.exe
# Download from: https://rustup.rs/
# Or use winget:
winget install Rustlang.Rustup

# Install Git
winget install Git.Git

# Install LLVM (optional)
winget install LLVM.LLVM

# Clone and build
git clone https://github.com/ovie-lang/ovie.git
cd ovie
cargo build --release

# Add to PATH via System Properties or:
$env:PATH += ";$(pwd)\target\release"
```

#### Windows Subsystem for Linux (WSL)

```bash
# Follow Linux instructions inside WSL
# Ubuntu WSL example:
sudo apt-get update
sudo apt-get install -y curl git build-essential llvm-14-dev libclang-14-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

git clone https://github.com/ovie-lang/ovie.git
cd ovie
cargo build --release
```

## Docker Installation

### Using Pre-built Image

```bash
# Pull the official Ovie image
docker pull ovielang/ovie:latest

# Run Ovie in a container
docker run -it --rm ovielang/ovie:latest ovie --version

# Mount your project directory
docker run -it --rm -v $(pwd):/workspace ovielang/ovie:latest
```

### Building from Dockerfile

```dockerfile
# Dockerfile
FROM rust:1.70-slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    git \
    llvm-14-dev \
    libclang-14-dev \
    && rm -rf /var/lib/apt/lists/*

# Clone and build Ovie
RUN git clone https://github.com/ovie-lang/ovie.git /ovie
WORKDIR /ovie
RUN cargo build --release

# Add to PATH
ENV PATH="/ovie/target/release:${PATH}"

# Set working directory
WORKDIR /workspace

# Default command
CMD ["ovie", "--help"]
```

```bash
# Build the image
docker build -t ovie-local .

# Run
docker run -it --rm -v $(pwd):/workspace ovie-local
```

## Development Installation

For contributors and developers who want to work on Ovie itself:

### Additional Dependencies

```bash
# Install additional development tools
cargo install cargo-watch cargo-tarpaulin cargo-audit

# Install pre-commit hooks (optional)
pip install pre-commit
pre-commit install
```

### Development Build

```bash
# Clone with all submodules
git clone --recursive https://github.com/ovie-lang/ovie.git
cd ovie

# Build in development mode
cargo build

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug ./target/debug/ovie --version

# Watch for changes during development
cargo watch -x "build" -x "test"
```

### IDE Setup

#### VS Code

```bash
# Install Rust extension
code --install-extension rust-lang.rust-analyzer

# Install Ovie extension (when available)
code --install-extension ovie-lang.ovie-vscode
```

#### Vim/Neovim

```vim
" Add to your .vimrc or init.vim
Plug 'rust-lang/rust.vim'
Plug 'ovie-lang/ovie.vim'  " When available
```

## Verification

### Test Installation

```bash
# Check version
ovie --version
# Expected: ovie 0.1.0 (Stage 0 - Rust Implementation)

# Check compiler
oviec --version
# Expected: oviec 0.1.0 (Ovie Compiler)

# Create test project
ovie new test-project
cd test-project

# Build and run
ovie build
ovie run
# Expected: Hello, Ovie World!

# Run tests
ovie test
# Expected: All tests passed

# Check Aproko
ovie aproko src/main.ov
# Expected: Analysis complete, no issues found
```

### Performance Test

```bash
# Run benchmark suite
ovie bench

# Expected output:
# Compilation speed: ~1000 lines/second
# Memory usage: <100MB for typical projects
# Binary size: <10MB for hello world
```

## Troubleshooting

### Common Issues

#### Rust Not Found

```bash
# Error: command 'rustc' not found
# Solution: Install Rust and restart shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### LLVM Not Found

```bash
# Error: Could not find LLVM
# Solution: Install LLVM development packages
# Ubuntu:
sudo apt-get install llvm-14-dev libclang-14-dev

# macOS:
brew install llvm
export LLVM_SYS_140_PREFIX=$(brew --prefix llvm)
```

#### Permission Denied

```bash
# Error: Permission denied when installing globally
# Solution: Use sudo or install to user directory
sudo cp target/release/ovie /usr/local/bin/

# Or install to user directory
mkdir -p ~/.local/bin
cp target/release/ovie ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

#### Build Failures

```bash
# Error: Build failed with linker errors
# Solution: Install build tools
# Ubuntu:
sudo apt-get install build-essential

# macOS:
xcode-select --install

# Windows:
# Install Visual Studio Build Tools
```

### Getting Help

If you encounter issues:

1. **Check the logs**: Run with `RUST_LOG=debug` for detailed output
2. **Search issues**: Check [GitHub Issues](https://github.com/ovie-lang/ovie/issues)
3. **Ask for help**: Join our [Discord](https://discord.gg/ovie-lang)
4. **Report bugs**: Create a new issue with system details

### System Requirements

#### Minimum Requirements

- **RAM**: 2GB available memory
- **Storage**: 1GB free space
- **CPU**: Any 64-bit processor
- **OS**: Linux, macOS, Windows 10+

#### Recommended Requirements

- **RAM**: 4GB+ available memory
- **Storage**: 5GB+ free space (for development)
- **CPU**: Multi-core processor for parallel builds
- **OS**: Latest stable versions

## Uninstallation

### Remove Ovie

```bash
# If installed globally
sudo rm /usr/local/bin/ovie /usr/local/bin/oviec

# If installed to user directory
rm ~/.local/bin/ovie ~/.local/bin/oviec

# Remove from PATH (edit your shell config file)
# Remove the Ovie PATH export line

# Clean up project directory
rm -rf ~/path/to/ovie-source
```

### Clean Up

```bash
# Remove Rust (if only installed for Ovie)
rustup self uninstall

# Remove LLVM (if only installed for Ovie)
# Ubuntu:
sudo apt-get remove llvm-14-dev libclang-14-dev

# macOS:
brew uninstall llvm
```

## Next Steps

After installation:

1. **[Getting Started](getting-started.md)**: Write your first Ovie program
2. **[Language Guide](language-guide.md)**: Learn Ovie syntax and features
3. **[CLI Reference](cli.md)**: Master the command-line tools
4. **[Examples](examples.md)**: Explore sample programs

---

*Installation issues? Join our [Discord community](https://discord.gg/ovie-lang) for help!*