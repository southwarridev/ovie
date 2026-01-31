<div align="center">
  <img src="ovie.png" alt="Ovie Programming Language" width="200" height="200">
  
  # 🚀 Ovie Programming Language
  
  ### ✅ **OFFICIALLY SELF-HOSTED PROGRAMMING LANGUAGE**
  
  > A modern, safe, and AI-friendly **open-source** programming language designed for accessibility, deterministic builds, and AI/LLM integration.
  
  **🎉 Status: Production-Ready Self-Hosted Language (January 30, 2026)**
</div>

[![GitHub](https://img.shields.io/badge/GitHub-southwarridev%2Fovie-blue?logo=github)](https://github.com/southwarridev/ovie)
[![GitLab](https://img.shields.io/badge/GitLab-ovie1%2Fovie-orange?logo=gitlab)](https://gitlab.com/ovie1/ovie)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/southwarridev/ovie/workflows/CI/badge.svg)](https://github.com/southwarridev/ovie/actions)
[![GitLab CI](https://gitlab.com/ovie1/ovie/badges/main/pipeline.svg)](https://gitlab.com/ovie1/ovie/-/pipelines)
[![Security](https://img.shields.io/badge/security-policy-green.svg)](SECURITY.md)

## What is Ovie?

**Ovie is now officially a self-hosted programming language!** ✅

Ovie has achieved full programming language status with a complete compiler written in Ovie itself. It solves fundamental issues in modern software development through:

- **🔒 Offline-First**: Complete development environment with no network dependencies
- **🏆 Self-Hosted**: The compiler is written in Ovie itself (ACHIEVED!)
- **Natural Syntax**: Pidgin English keywords make programming accessible to everyone
- **Built-in Assistant**: Aproko provides real-time guidance and auto-correction
- **Deterministic Builds**: Identical inputs always produce identical outputs
- **Privacy-Focused**: No telemetry, no tracking, no data collection

> **✅ MILESTONE: Ovie achieved self-hosting capability on January 30, 2026**

> **See [OFFLINE-FIRST.md](OFFLINE-FIRST.md) for complete offline development guide**

## Quick Start

```ovie
// Hello World in Ovie
seeAm "Hello, World!"

// Variables and functions
mut name = "Ovie"
fn greet(person) {
    seeAm "Hello, " + person + "!"
}

greet(name)
```

## Key Features

### 🌍 Accessible to Everyone
- **Pidgin English syntax** for natural readability
- **Built-in guidance** through Aproko assistant
- **Clear error messages** with actionable suggestions
- **Comprehensive documentation** for all skill levels

### 🔒 Offline-First Development
- **Complete offline operation** - no network required for any development task
- **Local dependency storage** with cryptographic verification
- **Air-gapped environment support** for maximum security
- **Privacy-focused design** with no telemetry or tracking

### 🔐 Security and Safety
### 🔐 Security and Safety
- **Offline-first builds** with no hidden network calls
- **Cryptographic verification** of all dependencies
- **Deterministic compilation** for reproducible deployments
- **Memory safety** without garbage collection overhead

### 🤖 AI/LLM Integration
- **Natural language patterns** easily understood by AI
- **Structured feedback** suitable for machine learning
- **Code generation friendly** syntax and semantics
- **Training data generation** capabilities

### ⚡ Self-Hosting Architecture (COMPLETED!)
- **✅ Stage 0**: Bootstrap compiler written in Rust
- **✅ Stage 1**: Partial self-hosting (lexer/parser in Ovie)
- **✅ Stage 2**: Full self-hosting (entire compiler in Ovie)

**🎉 Achievement Unlocked: Ovie can now compile itself using a compiler written entirely in Ovie!**

## 🚀 Quick Installation (Offline-First)

### Offline Development (Recommended)
```bash
# Download/clone the source code, then:
./local-dev.sh     # Linux/macOS
./local-dev.ps1    # Windows

# Or use Make
make offline-dev
```

### Build from Source (Completely Offline)
```bash
git clone https://github.com/southwarridev/ovie.git
cd ovie
make build  # or: cargo build --release --workspace
make install  # or: cargo install --path ovie
```

### Quick Install Scripts (Optional Online)
> ⚠️ These scripts download from the internet. Use offline method above for air-gapped environments.

#### Linux/macOS/WSL
```bash
curl -sSL https://raw.githubusercontent.com/southwarridev/ovie/main/install.sh | bash
```

#### Windows (PowerShell)
```powershell
iwr -useb https://raw.githubusercontent.com/southwarridev/ovie/main/install.ps1 | iex
```

## 🔗 Repository Links

- **GitHub**: [https://github.com/southwarridev/ovie](https://github.com/southwarridev/ovie)
- **GitLab**: [https://gitlab.com/ovie1/ovie](https://gitlab.com/ovie1/ovie)

Both repositories are kept in sync and accept contributions.

## Usage

### Create a New Project
```bash
ovie new my-project
cd my-project
```

### Build and Run
```bash
ovie build
ovie run
```

### Other Commands
```bash
ovie test      # Run tests
ovie fmt       # Format code
ovie update    # Update dependencies
ovie vendor    # Vendor dependencies locally
```

## Project Structure

```
ovie/
├── oviec/          # Ovie compiler
├── aproko/         # Assistant engine  
├── ovie/           # CLI toolchain
├── docs/           # Documentation
├── examples/       # Example programs
├── spec/           # Language specification
└── SPEC.md         # Core immutable principles
```

## Core Principles

Ovie is built on [immutable core principles](SPEC.md) that ensure:

1. **🔒 Offline-first** operation (no network required for development)
2. **Deterministic builds** (reproducible compilation)
3. **Vendored dependencies** (local supply chain)
4. **No silent corrections** (explicit user consent)
5. **Minimal keywords** (13 core keywords only)
6. **Self-hosting target** (sovereignty goal)
7. **Open source** (MIT license)
8. **Aproko always-on** (built-in assistance)
9. **No telemetry** (complete privacy)
10. **Stable core spec** (RFC-based changes)

> **📖 Read [OFFLINE-FIRST.md](OFFLINE-FIRST.md) for detailed offline development guide**

## Language Examples

### Basic Syntax
```ovie
// Variables
name = "Ovie"
mut counter = 0

// Functions
fn add(a, b) {
    return a + b
}

// Control flow
if counter < 10 {
    seeAm "Counter is small"
} else {
    seeAm "Counter is big"
}

// Loops
for i in 0..10 {
    seeAm i
}
```

### Data Structures
```ovie
// Structs
struct Person {
    name: String,
    age: Number,
}

// Enums
enum Color {
    Red,
    Green,
    Blue,
}

// Usage
person = Person {
    name: "Alice",
    age: 30,
}

seeAm person.name
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
```bash
git clone https://github.com/southwarridev/ovie.git
cd ovie
make dev-setup  # Install all required Rust targets and tools
make dev        # Clean + build + test cycle
```

### Areas for Contribution
- Core language features
- Aproko assistant rules
- Documentation and examples
- Testing and benchmarks
- IDE integration
- Community building

## Documentation

- **[Engineering Overview](docs/engineering-overview.md)** - Complete Stage 0 → Stage 2 roadmap
- **[Language Guide](docs/language-guide.md)** - Complete language reference
- **[Getting Started](docs/getting-started.md)** - Tutorial for beginners
- **[Aproko Guide](docs/aproko.md)** - Assistant configuration
- **[Compiler Internals](docs/internals.md)** - Architecture documentation
- **[API Reference](https://docs.ovie-lang.org)** - Generated API docs

## 🤝 Community and Support

- **GitHub Issues**: [https://github.com/southwarridev/ovie/issues](https://github.com/southwarridev/ovie/issues)
- **GitLab Issues**: [https://gitlab.com/ovie1/ovie/-/issues](https://gitlab.com/ovie1/ovie/-/issues)
- **Discussions**: [GitHub Discussions](https://github.com/southwarridev/ovie/discussions)

## 📊 Roadmap

### ✅ Stage 0 (Complete) - Rust Bootstrap
- [x] Project foundation and structure
- [x] Lexer and parser implementation
- [x] Aproko assistant engine
- [x] Basic interpreter and IR system
- [x] CLI toolchain
- [x] Cross-platform build system
- [x] Comprehensive documentation

### ✅ Stage 1 (Complete) - Usable Language
- [x] Core syntax & keywords working
- [x] Basic type system operational
- [x] Offline-first philosophy implemented
- [x] Deterministic builds established
- [x] Professional repository structure
- [x] Production-ready v1.0.0 release

### ✅ Stage 2 (Complete) - Production Capable & Self-Hosted
- [x] Formal language specifications (BNF grammar, type system, memory model)
- [x] Multi-stage IR pipeline (AST → HIR → MIR → Backend)
- [x] **Self-hosting capability (Ovie compiles Ovie)** 🎉
- [x] Complete standard library (core, math, fs, io, time, cli, test)
- [x] Enhanced Aproko reasoning system
- [x] Multi-target code generation (Native + WASM)
- [x] Comprehensive testing framework
- [x] Production-grade tooling suite

**🏆 MILESTONE ACHIEVED: Ovie is now officially a self-hosted programming language!**

### Stage 3 (Future) - Ecosystem & Community
- [ ] Package registry and ecosystem
- [ ] IDE plugins and language server
- [ ] Advanced optimization passes
- [ ] Community-driven standard library expansion
- [ ] Educational curriculum and resources

> **📖 See [Engineering Overview](docs/engineering-overview.md) for complete Stage 0 → Stage 2 roadmap**

## Security

Security is a core principle of Ovie. Please see our [Security Policy](SECURITY.md) for:
- Vulnerability reporting process
- Security features and guarantees
- Supported versions and updates

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

- Inspired by the accessibility goals of natural language programming
- Built on the shoulders of giants in programming language design
- Grateful to the Rust community for excellent tooling and libraries
- Thanks to all contributors and early adopters

---

**"Making programming accessible to everyone, one line at a time."**