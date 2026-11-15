# Ovie Lang 1.0

A Systems Programming Language


---

Ovie is designed to combine low-level programming capabilities with high-level language features, providing developers with direct control, expressive syntax and a structured development environment.
## A Language with Its Own Identity

Ovie introduces its own declaration and output vocabulary:

    oya name = "Shedrack"
    oya language = "Ovie"

    mut age = 31

    seeAm "Hello, " + name
    seeAm language
    seeAm age
- oya � immutable declaration.
- mut � mutable declaration.
- seeAm � output with a newline.
- print � output without a newline.

## Compiler Architecture
Ovie has two compiler implementations:

- oviec � The Rust-based compiler implementation.
- oviev � The self-hosted compiler written in Ovie (.ov).

Both run the same .ov programs:

    oviec run hello.ov   # Rust compiler
    oviev run hello.ov   # Ovie self-hosted compiler

## Aproko � Built-in Analysis

Aproko (an Urhobo word) runs on every oviec run. It catches bad keywords, explains runtime errors, and teaches best practices as you code.

## Installation

Windows (PowerShell as Admin):

    iwr -useb https://raw.githubusercontent.com/Ovie-Lang/ovie/main/easy-windows-install.ps1 | iex

Linux:

    curl -sSL https://raw.githubusercontent.com/Ovie-Lang/ovie/main/easy-linux-install.sh | bash

macOS:

    curl -sSL https://raw.githubusercontent.com/Ovie-Lang/ovie/main/easy-macos-install.sh | bash

Build from source:

    git clone https://github.com/Ovie-Lang/ovie.git
    cargo build --release --bin oviec

Verify:
    oviec --version    # Ovie Lang 1.0
    oviev --version    # Ovie Lang 1.0

## Documentation
- [Getting Started](docs/getting-started.md)
- [Language Guide](docs/language-guide.md)
- [The Ovie Book](docs/book/ovie-in-southern-kaduna/INDEX.md)
- [Aproko Guide](docs/aproko.md)
- [Tutorials](docs/tutorials/)

## The Official Project
Ovie Lang is the official organization for the programming language, its source code, documentation, releases and developer community.

The project is developed under Nashedy Ltd.
## Links

- Website repository: https://github.com/Ovie-Lang/oviewebsite
- X (Twitter): https://x.com/Ovielang
- GitHub Organization: https://github.com/Ovie-Lang
## License

MIT License. Copyright (c) 2026 Nashedy Ltd.
