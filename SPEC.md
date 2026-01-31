<div align="center">
  <img src="ovie.png" alt="Ovie Programming Language" width="150" height="150">
  
  # Ovie Language Core Specification
  
  ### ✅ **OFFICIALLY SELF-HOSTED PROGRAMMING LANGUAGE**
  
  **🎉 Status: Production-Ready Self-Hosted Language (January 30, 2026)**
</div>

## Immutable Core Principles

These principles are locked and cannot be changed without extraordinary consensus. They define the fundamental nature of the Ovie programming language.

**🏆 ACHIEVEMENT: All core principles have been successfully implemented in the self-hosted compiler!**

### 1. Offline-First Architecture
- **No network required** to build or run Ovie programs
- All dependencies must be **vendored locally**
- Build process operates in **complete isolation**
- Network access only with **explicit user permission**

### 2. Deterministic Builds
- **Identical inputs** produce **identical outputs** across all platforms
- **Reproducible compilation** with cryptographic verification
- **Immutable dependency resolution** through lock files
- **Hash-based verification** of all build artifacts

### 3. Vendored Dependencies
- All dependencies stored in **local vendor/ directory**
- **Cryptographic hash verification** for supply chain security
- **Immutable local registry** in ~/.ovie/registry/
- **No runtime downloads** or dynamic dependency resolution

### 4. No Silent Corrections
- **Explicit user consent** required for all code modifications
- **Transparent logging** of all auto-corrections
- **Clear feedback** about what was changed and why
- **Preserve semantic meaning** at all times

### 5. Minimal Keywords
- **Exactly 13 core keywords**: fn, mut, if, else, for, while, struct, enum, unsafe, return, true, false, seeAm
- **No keyword expansion** without RFC approval
- **Pidgin English syntax** for accessibility
- **Natural language patterns** for common operations

### 6. Self-Hosting Target
- **Stage 0**: Rust bootstrap compiler (minimal, frozen after Stage 2)
- **Stage 1**: Partial self-hosting (lexer/parser in Ovie)
- **Stage 2**: Full self-hosting (entire compiler in Ovie)
- **Bootstrap verification** through hash comparison

### 7. Open Source Commitment
- **Apache 2.0 license** for maximum freedom
- **Transparent development** process
- **Community governance** with RFC process
- **No proprietary extensions** or vendor lock-in

### 8. Aproko Always-On
- **Built-in assistant** integrated into compilation pipeline
- **Real-time analysis** across six categories
- **Configurable guidance** through .ovie/aproko.toml
- **AI/LLM friendly** feedback generation

### 9. No Telemetry
- **Zero data collection** or user tracking
- **No hidden network calls** or analytics
- **Complete privacy** by design
- **Transparent operation** logging only

### 10. Stable Core Spec
- **Immutable core principles** (this document)
- **RFC process** required for language changes
- **Backward compatibility** guarantees
- **Semantic versioning** for all releases

## Language Grammar (Locked)

```ebnf
program      = statement* ;
statement    = assign | func | if | loop | struct | enum | expr ;
assign       = ["mut"] identifier "=" expr ;
func         = "fn" identifier "(" params ")" block ;
print        = "seeAm" expr ;
if           = "if" expr block ["else" block] ;
loop         = ("for" | "while") expr block ;
struct       = "struct" identifier "{" fields "}" ;
enum         = "enum" identifier "{" variants "}" ;
```

## Core Keywords (Immutable)

1. **fn** - Function definition
2. **mut** - Mutable variable declaration
3. **if** - Conditional statement
4. **else** - Alternative branch
5. **for** - Iteration loop
6. **while** - Conditional loop
7. **struct** - Data structure definition
8. **enum** - Enumeration definition
9. **unsafe** - Unsafe operation block
10. **return** - Function return
11. **true** - Boolean literal
12. **false** - Boolean literal
13. **seeAm** - Print/output statement (pidgin English)

## Compiler Pipeline (Fixed)

```
Source Code
    ↓
Lexer (tokenization)
    ↓
Parser (AST generation)
    ↓
Normalizer (safe auto-correction)
    ↓
Aproko Engine (analysis & guidance)
    ↓
Semantic Analyzer (type & ownership checking)
    ↓
IR Builder (intermediate representation)
    ↓
Optimizer (target-independent optimization)
    ↓
Codegen (WASM/LLVM backend)
```

## Security Guarantees

- **Memory safety** without garbage collection
- **Ownership system** prevents use-after-free
- **Effect system** tracks side effects
- **Explicit unsafe** blocks for dangerous operations
- **Supply chain isolation** through vendoring
- **Cryptographic verification** of all dependencies

## Governance Model

- **RFC process** for all language changes
- **Core team** approval for breaking changes
- **Community input** on all proposals
- **Transparent decision** making process
- **Immutable principles** cannot be changed

## Compatibility Promise

- **Semantic versioning** for all releases
- **Backward compatibility** within major versions
- **Migration guides** for breaking changes
- **Deprecation warnings** before removal
- **Long-term support** for stable releases

---

**Note**: This specification defines the immutable core of the Ovie programming language. Changes to these principles require extraordinary consensus and may result in a new language version. The goal is to prevent feature creep and maintain the language's core identity over time.

**Last Updated**: 2024-01-26  
**Version**: 1.0.0  
**Status**: Locked