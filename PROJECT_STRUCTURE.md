<div align="center">
  <img src="ovie.png" alt="Ovie Programming Language" width="120" height="120">
  
  # Ovie Project Structure
  
  ### ✅ **SELF-HOSTED PROGRAMMING LANGUAGE**
</div>

This document describes the organization of the Ovie programming language project.

**🎉 Status: Clean, production-ready project structure for public distribution**

---

## 📁 Root Directory

```
ovie/
├── 📄 README.md                    # Main project documentation
├── 📄 LICENSE                      # MIT license
├── 📄 SPEC.md                      # Core language specification
├── 📄 ANNOUNCEMENT.md               # Self-hosting achievement announcement
├── 📄 CONTRIBUTING.md               # Contribution guidelines
├── 📄 SECURITY.md                  # Security policy
├── 📄 CODE_OF_CONDUCT.md           # Community guidelines
├── 📄 OFFLINE-FIRST.md             # Offline development guide
├── 🖼️ ovie.png                     # Official Ovie logo
├── ⚙️ Cargo.toml                   # Rust workspace configuration
├── ⚙️ Cargo.lock                   # Dependency lock file
├── ⚙️ rust-toolchain.toml          # Rust toolchain specification
├── ⚙️ Makefile                     # Build automation
├── 🔧 install.sh                   # Unix installation script
├── 🔧 install.ps1                  # Windows installation script
└── 📄 .gitignore                   # Git ignore rules
```

## 🏗️ Core Components

### 🔧 Compiler (`oviec/`)
The main Ovie compiler implementation:

```
oviec/
├── 📄 Cargo.toml                   # Compiler package configuration
├── 📁 src/                         # Compiler source code
│   ├── 📄 main.rs                  # Compiler entry point
│   ├── 📄 lib.rs                   # Library interface
│   ├── 📄 lexer.rs                 # Lexical analysis
│   ├── 📄 parser.rs                # Syntax analysis
│   ├── 📄 ast.rs                   # Abstract syntax tree
│   ├── 📄 semantic.rs              # Semantic analysis
│   ├── 📄 hir.rs                   # High-level IR
│   ├── 📄 mir.rs                   # Mid-level IR
│   ├── 📄 ir.rs                    # IR utilities
│   ├── 📄 interpreter.rs           # Interpreter
│   ├── 📄 error.rs                 # Error handling
│   ├── 📄 package.rs               # Package management
│   ├── 📄 security.rs              # Security analysis
│   ├── 📁 codegen/                 # Code generation
│   │   ├── 📄 mod.rs               # Code generation interface
│   │   ├── 📄 llvm.rs              # LLVM backend
│   │   └── 📄 wasm.rs              # WebAssembly backend
│   ├── 📁 self_hosting/            # Self-hosting implementation
│   │   ├── 📄 mod.rs               # Self-hosting interface
│   │   ├── 📄 minimal_compiler.ov  # Ovie compiler in Ovie!
│   │   ├── 📄 bootstrap_verification.rs # Bootstrap system
│   │   └── 📄 bootstrap_integration.rs  # Integration layer
│   └── 📁 bin/                     # Binary utilities
└── 📁 tests/                       # Comprehensive test suite
    ├── 📁 unit/                    # Unit tests
    ├── 📁 integration/             # Integration tests
    ├── 📁 property/                # Property-based tests
    ├── 📁 conformance/             # Language conformance tests
    └── 📁 performance/             # Performance tests
```

### 🤖 Assistant (`aproko/`)
The Aproko intelligent assistant system:

```
aproko/
├── 📄 Cargo.toml                   # Assistant package configuration
├── 📁 src/                         # Assistant source code
│   ├── 📄 lib.rs                   # Library interface
│   ├── 📄 diagnostic.rs            # Diagnostic engine
│   ├── 📄 explanation.rs           # Explanation system
│   └── 📁 analyzers/               # Analysis modules
│       ├── 📄 mod.rs               # Analyzer interface
│       ├── 📄 syntax.rs            # Syntax analysis
│       ├── 📄 logic.rs             # Logic analysis
│       ├── 📄 style.rs             # Style analysis
│       ├── 📄 performance.rs       # Performance analysis
│       ├── 📄 security.rs          # Security analysis
│       └── 📄 correctness.rs       # Correctness analysis
└── 📁 tests/                       # Assistant tests
```

### 🛠️ CLI Tools (`ovie/`)
Command-line interface and toolchain:

```
ovie/
├── 📄 Cargo.toml                   # CLI package configuration
├── 📁 src/                         # CLI source code
│   ├── 📄 main.rs                  # CLI entry point
│   └── 📄 tests.rs                 # CLI tests
└── 📁 test-project/                # Example project template
```

## 📚 Documentation (`docs/`)

```
docs/
├── 📄 README.md                    # Documentation index
├── 📄 getting-started.md           # Getting started guide
├── 📄 installation.md              # Installation instructions
├── 📄 language-guide.md            # Complete language reference
├── 📄 cli.md                       # CLI reference
├── 📄 aproko.md                    # Assistant guide
├── 📄 internals.md                 # Compiler internals
├── 📄 ai-integration.md            # AI/LLM integration
├── 📄 engineering-overview.md      # Technical overview
└── 📄 examples.md                  # Example programs guide
```

## 🎯 Examples (`examples/`)

```
examples/
├── 📄 README.md                    # Examples index
├── 📄 hello.ov                     # Hello World
├── 📄 variables.ov                 # Variables and types
├── 📄 functions.ov                 # Functions
├── 📄 control_flow.ov              # Control structures
├── 📄 struct.ov                    # Data structures
├── 📄 enums.ov                     # Enumerations
├── 📄 errors.ov                    # Error handling
├── 📄 math.ov                      # Mathematics
├── 📄 calculator.ov                # Calculator app
├── 📄 bank_account.ov              # Banking system
├── 📄 employee_management.ov       # HR system
├── 📄 data_processing.ov           # Data analysis
├── 📄 cli_tool.ov                  # CLI application
├── 📄 testing.ov                   # Testing examples
├── 📄 memory_safety.ov             # Memory safety
├── 📄 natural_language.ov          # Natural patterns
├── 📄 ai_training_data.ov          # AI training
├── 📄 llm_friendly.ov              # LLM integration
├── 📄 lexer_demo.ov                # Lexer example
├── 📄 parser_demo.ov               # Parser example
└── 📄 grammar_showcase.ov          # Grammar features
```

## 📋 Language Specification (`spec/`)

```
spec/
├── 📄 grammar.ebnf                 # Formal BNF grammar
├── 📄 grammar.md                   # Grammar documentation
├── 📄 type-system.md               # Type system specification
├── 📄 memory-model.md              # Memory and ownership model
└── 📄 error-model.md               # Error handling specification
```

## 📦 Standard Library (`std/`)

```
std/
├── 📁 core/                        # Core types and functions
│   └── 📄 mod.ov
├── 📁 math/                        # Mathematical operations
│   └── 📄 mod.ov
├── 📁 io/                          # Input/output operations
│   └── 📄 mod.ov
├── 📁 fs/                          # File system operations
│   └── 📄 mod.ov
├── 📁 time/                        # Time and duration
│   └── 📄 mod.ov
├── 📁 cli/                         # Command-line utilities
│   └── 📄 mod.ov
├── 📁 testing/                     # Testing framework
│   └── 📄 mod.ov
├── 📁 log/                         # Logging system
│   └── 📄 mod.ov
└── 📁 env/                         # Environment access
    └── 📄 mod.ov
```

## 🔧 Configuration

### Project Configuration
- **📄 ovie.toml.template** - Project template configuration
- **📄 .ovie/aproko.toml** - Aproko assistant configuration

### CI/CD
- **📁 .github/workflows/** - GitHub Actions workflows
- **📄 .gitlab-ci.yml** - GitLab CI configuration

## 🚫 Excluded from Public Distribution

The following are kept in the private `shedydev/` directory:

- Internal development specifications (`.kiro/`)
- Development test files (`test_*.rs`)
- Build and release scripts
- Internal documentation and reports
- Competitive analysis and strategy
- Work-in-progress features

## 🎯 Key Features

### ✅ **Production Ready**
- Complete compiler implementation
- Comprehensive test suite
- Full documentation
- Example programs
- Standard library

### ✅ **Self-Hosted**
- Compiler written in Ovie itself
- Bootstrap verification system
- Complete development toolchain
- Production-quality implementation

### ✅ **Developer Friendly**
- Clear project organization
- Comprehensive documentation
- Easy installation process
- Rich example collection

### ✅ **Community Focused**
- Open source (MIT license)
- Contribution guidelines
- Code of conduct
- Security policy

---

## 🚀 Getting Started

1. **Clone the repository**
   ```bash
   git clone https://github.com/southwarridev/ovie.git
   cd ovie
   ```

2. **Install Ovie**
   ```bash
   ./install.sh    # Unix/Linux/macOS
   ./install.ps1   # Windows
   ```

3. **Try an example**
   ```bash
   ovie run examples/hello.ov
   ```

4. **Read the documentation**
   ```bash
   # Start with the getting started guide
   cat docs/getting-started.md
   ```

---

**This clean, organized structure makes Ovie accessible to developers while protecting sensitive development information.**

*Last updated: January 30, 2026 - Self-Hosting Achievement*