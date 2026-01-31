# Compiler Internals

This document provides a comprehensive overview of the Ovie compiler architecture, implementation details, and internal workings. It's designed for contributors, compiler engineers, and anyone interested in understanding how Ovie works under the hood.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Compilation Pipeline](#compilation-pipeline)
3. [Stage 0: Rust Implementation](#stage-0-rust-implementation)
4. [Self-Hosting Journey](#self-hosting-journey)
5. [Component Details](#component-details)
6. [Build System](#build-system)
7. [Testing Strategy](#testing-strategy)
8. [Performance Considerations](#performance-considerations)
9. [Contributing Guidelines](#contributing-guidelines)

## Architecture Overview

Ovie follows a multi-stage self-hosting architecture designed for gradual transition from Rust to Ovie implementation:

```
┌─────────────────────────────────────────────────────────────┐
│                    Ovie Ecosystem                           │
├─────────────────────────────────────────────────────────────┤
│  Stage 0 (Current)    │  Stage 1 (Future)  │  Stage 2       │
│  ┌─────────────────┐  │  ┌───────────────┐  │  ┌───────────┐ │
│  │ Rust Compiler   │  │  │ Hybrid        │  │  │ Full Ovie │ │
│  │ - Lexer         │  │  │ - Ovie Lexer  │  │  │ Compiler  │ │
│  │ - Parser        │  │  │ - Ovie Parser │  │  │           │ │
│  │ - Semantic      │  │  │ - Rust Core   │  │  │           │ │
│  │ - Codegen       │  │  │               │  │  │           │ │
│  └─────────────────┘  │  └───────────────┘  │  └───────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### Repository Structure

The Ovie ecosystem is organized across multiple repositories:

- **ovie**: CLI toolchain and project management
- **oviec**: Core compiler implementation
- **aproko**: Assistant engine and analysis framework
- **std**: Standard library (future)
- **docs**: Documentation and guides
- **spec**: Language specification and grammar
- **examples**: Sample programs and tutorials

## Compilation Pipeline

The Ovie compiler follows a traditional multi-pass architecture with modern enhancements:

```
Source Code (.ov)
       ↓
   [Lexer] ────────────→ Tokens
       ↓
   [Parser] ───────────→ AST (Abstract Syntax Tree)
       ↓
   [Normalizer] ───────→ Normalized AST
       ↓
   [Aproko Analysis] ──→ Analysis Results + Suggestions
       ↓
   [Semantic Analyzer] → Typed AST + Symbol Table
       ↓
   [IR Generator] ─────→ Intermediate Representation
       ↓
   [Optimizer] ────────→ Optimized IR
       ↓
   [Code Generator] ───→ Target Code (WASM/LLVM)
       ↓
   [Linker] ───────────→ Executable/Module
```

### Pipeline Characteristics

- **Deterministic**: Same input always produces same output
- **Incremental**: Only recompile changed components
- **Parallel**: Independent modules compile concurrently
- **Cached**: Intermediate results are cached for speed
- **Verifiable**: Each stage can be independently verified

## Stage 0: Rust Implementation

The current Stage 0 compiler is implemented entirely in Rust, providing a stable foundation for the self-hosting journey.

### Core Components

#### 1. Lexer (`oviec/src/lexer.rs`)

**Purpose**: Converts source code into tokens

**Implementation**:
- Uses the `logos` crate for efficient tokenization
- Supports all 13 Ovie keywords
- Handles pidgin English syntax patterns
- Provides detailed error locations

**Key Features**:
```rust
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("fn")]
    Fn,
    #[token("mut")]
    Mut,
    #[token("seeAm")]
    SeeAm,
    // ... other tokens
}
```

**Error Handling**:
- Precise error locations with line/column information
- Suggestions for common typos
- Integration with Aproko for auto-correction

#### 2. Parser (`oviec/src/parser.rs`)

**Purpose**: Builds Abstract Syntax Tree from tokens

**Implementation**:
- Recursive descent parser
- Precedence climbing for expressions
- Error recovery for better diagnostics
- AST nodes defined in `ast.rs`

**AST Structure**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Function(FunctionDecl),
    Variable(VariableDecl),
    Expression(Expression),
    Return(Option<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
}
```

#### 3. Normalizer (`oviec/src/normalizer.rs`)

**Purpose**: Safe auto-correction and syntax normalization

**Features**:
- Typo correction with confidence scoring
- Whitespace normalization
- Syntactic sugar expansion
- Change logging and user notification

**Safety Guarantees**:
- Never changes semantic meaning
- Requires user confirmation for ambiguous cases
- Maintains original source mapping

#### 4. Aproko Integration (`aproko/src/`)

**Purpose**: Real-time code analysis and guidance

**Analysis Categories**:
- **Syntax**: Grammar compliance, formatting
- **Logic**: Control flow, reachability analysis
- **Performance**: Complexity analysis, optimization suggestions
- **Security**: Unsafe operation detection, vulnerability scanning
- **Correctness**: Type safety, ownership validation
- **Style**: Naming conventions, best practices

**Implementation**:
```rust
pub trait Analyzer {
    fn analyze(&self, ast: &AST) -> AnalysisResult;
    fn category(&self) -> AnalysisCategory;
    fn severity(&self) -> Severity;
}
```

#### 5. Semantic Analyzer (`oviec/src/semantic.rs`)

**Purpose**: Type checking and semantic validation

**Responsibilities**:
- Type inference and checking
- Ownership rule enforcement
- Effect correctness verification
- Symbol table management
- Scope resolution

**Type System**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    Struct(StructType),
    Enum(EnumType),
    Function(FunctionType),
    Generic(GenericType),
}
```

#### 6. IR Generator (`oviec/src/ir.rs`)

**Purpose**: Generate platform-neutral intermediate representation

**IR Design**:
- SSA (Static Single Assignment) form
- Platform-neutral instructions
- Deterministic and serializable
- Optimization-friendly structure

**IR Structure**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub functions: Vec<Function>,
    pub globals: Vec<Global>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Load(Value),
    Store(Value, Value),
    BinaryOp(BinaryOperator, Value, Value),
    Call(String, Vec<Value>),
    Return(Option<Value>),
}
```

#### 7. Code Generators (`oviec/src/codegen/`)

**WASM Backend** (`wasm.rs`):
- Generates WebAssembly binary format
- Supports all Ovie language features
- Deterministic output generation
- Integration with `wasmtime` for execution

**LLVM Backend** (`llvm.rs`):
- Uses `inkwell` for LLVM integration
- Generates optimized native code
- Cross-compilation support
- Debug information generation

## Self-Hosting Journey

Ovie's path to self-hosting is carefully planned across three stages:

### Stage 0: Rust Foundation (Current)
- Complete Rust implementation
- Stable compilation pipeline
- Comprehensive testing
- Production-ready toolchain

### Stage 1: Partial Self-Hosting (Future)
- Lexer implemented in Ovie
- Parser implemented in Ovie
- Rust core for semantic analysis and codegen
- Bootstrap verification system

### Stage 2: Full Self-Hosting (Future)
- Entire compiler written in Ovie
- Minimal Rust runtime for bootstrapping
- Complete language sovereignty
- Self-compilation verification

### Bootstrap Verification

The bootstrap process includes rigorous verification:

```rust
pub fn verify_bootstrap(stage0_compiler: &Path, stage1_compiler: &Path) -> Result<(), BootstrapError> {
    // Compile Stage 1 compiler with Stage 0
    let stage1_binary = compile_with_stage0(stage0_compiler, stage1_source)?;
    
    // Compile Stage 1 compiler with itself
    let stage1_self_compiled = compile_with_stage1(&stage1_binary, stage1_source)?;
    
    // Verify binary equivalence
    verify_binary_equivalence(&stage1_binary, &stage1_self_compiled)?;
    
    Ok(())
}
```

## Component Details

### Error Handling System

Ovie uses a comprehensive error handling system:

```rust
#[derive(Debug, Clone)]
pub struct CompilerError {
    pub kind: ErrorKind,
    pub location: SourceLocation,
    pub message: String,
    pub suggestions: Vec<Suggestion>,
    pub help: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    LexError,
    ParseError,
    SemanticError,
    CodegenError,
    InternalError,
}
```

**Error Recovery**:
- Graceful degradation on errors
- Multiple error reporting
- Specific suggestions for fixes
- Integration with Aproko for guidance

### Memory Management

**Ownership System**:
- Rust-inspired ownership rules
- Compile-time memory safety
- No garbage collection overhead
- Clear ownership transfer semantics

**Safety Guarantees**:
- No null pointer dereferences
- No buffer overflows
- No use-after-free errors
- No data races in concurrent code

### Deterministic Builds

**Build Reproducibility**:
- Identical inputs produce identical outputs
- Deterministic dependency resolution
- Stable sort orders for all collections
- Consistent hash generation

**Implementation**:
```rust
pub fn ensure_deterministic_build() {
    // Sort all collections consistently
    dependencies.sort_by(|a, b| a.name.cmp(&b.name));
    
    // Use stable hash algorithms
    let hash = StableHasher::new().hash(&build_inputs);
    
    // Ensure consistent timestamps
    set_deterministic_timestamps(&output_files);
}
```

## Build System

### Dependency Management

**Offline-First Approach**:
- All dependencies stored locally in `vendor/`
- Cryptographic hash verification
- Immutable dependency caching
- No network calls during compilation

**Lock File Format** (`ovie.lock`):
```toml
[[dependency]]
name = "example-lib"
version = "1.0.0"
hash = "sha256:abc123..."
source = "vendor/example-lib-1.0.0"

[[dependency.dependencies]]
name = "sub-dep"
version = "0.5.0"
```

### Incremental Compilation

**Change Detection**:
- File modification timestamps
- Content hash comparison
- Dependency graph analysis
- Selective recompilation

**Caching Strategy**:
- Intermediate results cached
- Cross-compilation cache sharing
- Cache invalidation on changes
- Parallel cache access

## Testing Strategy

### Property-Based Testing

Ovie uses extensive property-based testing to ensure correctness:

```rust
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn lexer_roundtrip(source in any::<String>()) {
            let tokens = lexer::tokenize(&source)?;
            let reconstructed = tokens.to_source();
            prop_assert_eq!(normalize(&source), normalize(&reconstructed));
        }
    }
}
```

**Test Categories**:
- **Unit Tests**: Individual component testing
- **Integration Tests**: Cross-component interaction
- **Property Tests**: Universal correctness properties
- **Regression Tests**: Previously fixed bugs
- **Performance Tests**: Compilation speed and memory usage

### Continuous Integration

**Test Pipeline**:
1. Unit test execution
2. Property test runs (100+ iterations)
3. Integration test suite
4. Cross-platform compilation
5. Performance benchmarking
6. Security scanning

## Performance Considerations

### Compilation Speed

**Optimization Strategies**:
- Parallel compilation of independent modules
- Incremental compilation with smart caching
- Efficient data structures (arena allocation)
- Lazy evaluation where possible

**Benchmarking**:
```rust
#[bench]
fn bench_compilation_speed(b: &mut Bencher) {
    let source = include_str!("../examples/large_program.ov");
    b.iter(|| {
        compile_to_ir(source).unwrap()
    });
}
```

### Memory Usage

**Memory Management**:
- Arena allocation for AST nodes
- String interning for identifiers
- Efficient symbol table implementation
- Memory pool reuse across compilations

### Runtime Performance

**Generated Code Quality**:
- LLVM optimization passes
- Dead code elimination
- Constant folding and propagation
- Inlining of small functions

## Contributing Guidelines

### Development Setup

1. **Prerequisites**:
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install LLVM (optional, for native backend)
   # Ubuntu/Debian:
   sudo apt-get install llvm-dev libclang-dev
   # macOS:
   brew install llvm
   ```

2. **Clone and Build**:
   ```bash
   git clone https://github.com/ovie-lang/ovie.git
   cd ovie
   cargo build --release
   ```

3. **Run Tests**:
   ```bash
   cargo test
   cargo test --release  # Release mode tests
   ```

### Code Style

**Rust Guidelines**:
- Follow standard Rust formatting (`cargo fmt`)
- Use `clippy` for linting (`cargo clippy`)
- Comprehensive documentation comments
- Property-based tests for new features

**Ovie Guidelines**:
- Use `ovie fmt` for Ovie code formatting
- Run `ovie aproko` for code analysis
- Include both unit and property tests
- Update documentation for new features

### Testing Requirements

**New Features Must Include**:
- Unit tests for basic functionality
- Property tests for universal properties
- Integration tests for cross-component features
- Documentation updates
- Example programs demonstrating usage

### Review Process

1. **Code Review**: At least one maintainer approval
2. **Testing**: All tests must pass
3. **Documentation**: Updated as needed
4. **Performance**: No significant regressions
5. **Security**: Security implications reviewed

## Debugging and Diagnostics

### Compiler Debugging

**Debug Flags**:
```bash
# Enable verbose compilation output
ovie build --verbose

# Dump intermediate representations
ovie build --dump-ast --dump-ir

# Enable debug assertions
ovie build --debug-assertions
```

**Internal Diagnostics**:
```rust
#[cfg(debug_assertions)]
fn debug_dump_ast(ast: &AST) {
    eprintln!("AST: {:#?}", ast);
}
```

### Performance Profiling

**Profiling Tools**:
- `perf` for Linux profiling
- `Instruments` for macOS profiling
- `cargo flamegraph` for flame graphs
- Custom timing instrumentation

## Future Directions

### Planned Enhancements

1. **Stage 1 Self-Hosting**: Lexer and parser in Ovie
2. **Advanced Type System**: Generics, traits, higher-kinded types
3. **Concurrency**: Async/await, channels, actors
4. **Package System**: Distributed package registry
5. **IDE Integration**: Language server protocol support

### Research Areas

1. **Formal Verification**: Proving compiler correctness
2. **Advanced Optimizations**: Profile-guided optimization
3. **Incremental Type Checking**: Faster development cycles
4. **Distributed Compilation**: Cloud-based compilation

## Resources

### Documentation
- **Language Specification**: `spec/grammar.ebnf`
- **API Documentation**: Generated with `cargo doc`
- **Architecture Decisions**: `docs/adr/` directory

### Community
- **GitHub Discussions**: Design discussions and Q&A
- **Discord**: Real-time community chat
- **RFC Process**: Formal change proposals

### Tools
- **Compiler Explorer**: Online Ovie compilation
- **Playground**: Interactive Ovie environment
- **Benchmarking Suite**: Performance tracking

---

*This document covers the current Stage 0 implementation. It will be updated as Ovie progresses through its self-hosting journey.*