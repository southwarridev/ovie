# Chapter 2: Installation and Setup

## System Requirements

- OS: Windows 10+, macOS 11+, or Linux (any modern distro)
- Disk: ~100MB for full installation including source
- RAM: 256MB minimum
- Network: Only needed for initial download — everything after is offline

## One-line Install

**Windows (PowerShell as Administrator):**
```powershell
iwr -useb https://raw.githubusercontent.com/southwarridev/ovie/main/easy-windows-install.ps1 | iex
```

**Linux:**
```bash
curl -sSL https://raw.githubusercontent.com/southwarridev/ovie/main/easy-linux-install.sh | bash
```

**macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/southwarridev/ovie/main/easy-macos-install.sh | bash
```

## Build from Source (Recommended for Development)

```bash
git clone https://github.com/couragereward/ovie.git
cd ovie
cargo build --release --bin oviec
```

Requires Rust toolchain (see ust-toolchain.toml for the exact version).

After building, copy the binary to your PATH:

```powershell
# Windows
copy target\release\oviec.exe C:\Users\YourName\AppData\Local\Ovie\bin\
```

```bash
# Linux / macOS
sudo cp target/release/oviec /usr/local/bin/
```

## Verify

```
oviec --version
oviec --self-check
```

You should see:

```
Ovie Compiler (oviec) v2.3.0 - Stage 2.3
```

## Your First Program

Create hello.ov:

```ovie
oya greeting = "Hello from Ovie!"
seeAm greeting
```

Run it:

```
oviec run hello.ov
```

Output:

```
== Aproko =============================================
  OK: no issues found in hello.ov
=====================================================

Hello from Ovie!
```

You will see Aproko run before every program. If your code has no issues, it says so. If it finds something, it tells you before your program runs.

## What Gets Installed

| File | Purpose |
|------|---------|
| oviec.exe / oviec | Compiler and runner |
| ovie.exe / ovie | CLI alias |
| std/ | Standard library (11 modules) |
| examples/ | 22+ runnable example programs |
| docs/ | Full documentation |

## Editor Support

A VS Code extension is available in extensions/ovie-vscode/. It provides syntax highlighting and error underlining.

To install manually in VS Code:
1. Open VS Code
2. Press Ctrl+Shift+P
3. Type Extensions: Install from VSIX
4. Select the .vsix file from the extensions/ovie-vscode/ folder