# Chapter 2: Installation and Setup

## System Requirements

- OS: Windows 10+, macOS 11+, or Linux (any modern distro)
- Disk: ~50MB for the full installation
- RAM: 256MB minimum
- Network: Only needed for initial download — everything after is offline

## Installation

### Linux

```bash
curl -sSL https://raw.githubusercontent.com/southwarridev/ovie/main/easy-linux-install.sh | bash
```

Or download the binary directly:

```bash
wget https://github.com/southwarridev/ovie/releases/latest/download/ovie-linux-x64.tar.gz
tar -xzf ovie-linux-x64.tar.gz
sudo mv ovie /usr/local/bin/
sudo mv oviec /usr/local/bin/
```

### Windows

```powershell
iwr -useb https://raw.githubusercontent.com/southwarridev/ovie/main/easy-windows-install.ps1 | iex
```

Or run `easy-windows-install.bat` as Administrator.

### macOS

```bash
curl -sSL https://raw.githubusercontent.com/southwarridev/ovie/main/easy-macos-install.sh | bash
```

### Build from Source

```bash
git clone https://github.com/southwarridev/ovie.git
cd ovie
make build
make install
```

Requires Rust toolchain (see `rust-toolchain.toml` for the exact version).

## Verifying Installation

```bash
ovie --version       # CLI version
oviec --version      # Compiler version
oviec --self-check   # Full installation validation
oviec --env          # Show runtime environment
```

## Setting Up Your Editor

### VS Code

Install the Ovie extension from the marketplace or from the `.vsix` file in `releases/`:

```bash
code --install-extension ovie-lang-1.0.0.vsix
```

Features: syntax highlighting, snippets, inline error display, `seeAm` autocomplete.

### Other Editors

Syntax highlighting definitions are in `extensions/ovie-vscode/syntaxes/ovie.tmLanguage.json` — compatible with any editor that supports TextMate grammars.

## Your First Project

```bash
ovie new my-first-project
cd my-first-project
oviec run src/main.ov
```

This creates:

```
my-first-project/
├── ovie.toml       # Project manifest
└── src/
    └── main.ov     # Entry point
```

## Troubleshooting

**`ovie: command not found`** — Add the install directory to your PATH. The installer prints the path at the end.

**Permission denied on Linux** — Run `chmod +x /usr/local/bin/ovie /usr/local/bin/oviec`.

**Build fails from source** — Check `rust-toolchain.toml` and ensure you have the exact Rust version installed via `rustup`.
