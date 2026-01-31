# Ovie Language — Full Engineering Overview (Stage 0 → Stage 2)

## Current Status (Confirmed)
✅ **Stage 0**: Bootstrap compiler written in Rust  
✅ **Stage 1**: Ovie can parse, type-check basic programs, run examples, and has a defined philosophy  
⏳ **Stage 2**: Production-capable language (your target)

Everything below shows what exists, what's missing, and what to do next.

## STAGE 0 — BOOTSTRAP (What You Already Have)

**Purpose**: Get Ovie off the ground using Rust as the trusted base.

**Components**:
- Rust-based compiler (oviec)
- Lexer + parser
- Basic AST
- Simple code execution / output
- Offline build scripts
- Repo structure + docs

**Output**: Ovie source → compiled result  
Basic syntax works (seeAm, variables, functions)

✔️ **Stage 0 is complete. Do not over-invest here anymore.**

## STAGE 1 — USABLE LANGUAGE (What You Have Now)

**Purpose**: Make Ovie usable for real programs, not just demos.

**What Exists**:
- Core syntax & keywords
- Basic type system
- Arithmetic and math handled by language
- Offline-first philosophy
- Aproko concept
- Deterministic builds
- Spec & docs started
- Repo is public, structured, professional

**What Is Still Weak / Partial**:
- Type rules not fully formalized
- No clear IR separation
- Error messages not explainable enough
- Stdlib incomplete
- Tooling minimal
- Self-hosting not proven yet

✔️ **Stage 1 is complete enough to move forward.**

## 🚀 STAGE 2 — PRODUCTION CAPABLE (MAIN WORK)

This is where Ovie becomes real infrastructure. Below is the complete checklist.

### 1️⃣ Language Core (Must Be Frozen)

**Updates Needed**:
- Formal grammar (BNF or equivalent)
- Explicit type system rules
- Ownership / mutability model (even simplified)
- Numeric behavior:
  - overflow rules
  - float determinism
- Error model:
  - compile-time vs runtime
  - Undefined behavior policy

**Deliverables**:
```
spec/
  grammar.md
  type-system.md
  memory-model.md
  error-model.md
```

🔒 **Freeze this early. Do not keep changing syntax.**

### 2️⃣ Compiler Architecture (Critical)

**Updates Needed**: Introduce clear internal stages:
```
Lexer → Parser → AST (syntax only) → HIR (names resolved, types known) → MIR (control flow explicit) → Backend
```

**Why**:
- Enables self-hosting
- Enables tooling
- Enables LLM reasoning
- Prevents technical debt

**Recommendation**: Even if MIR is simple at first — add it now.

### 3️⃣ Intermediate Representation (IR)

**Updates Needed**:
- Canonical IR format
- Stable structure
- Serializable (JSON / binary)
- Tooling:
  - `ovie ast dump file.ov`
  - `ovie ir dump file.ov`
  - `ovie explain file.ov`

**This unlocks**:
- Debugging
- Refactoring
- Aproko
- LLM access (safely)

### 4️⃣ Self-Hosting Path (Mandatory for Stage 2)

**Updates Needed**:
- Clear bootstrap plan
- Minimal Ovie compiler subset
- Proof of self-compilation

**Structure**:
```
bootstrap/
  stage0-rust/
  stage1-ovie/
  README.md
```

**Goal**: Ovie can compile a minimal version of Ovie. That is the Stage-2 line.

### 5️⃣ Standard Library (Small but Complete)

**Missing Modules**: core, math, fs, io, time, cli, env, test, log

**Recommendation**: Keep stdlib:
- Minimal
- Deterministic
- Compiler-blessed
- No dynamic loading

**Structure**:
```
std/
  core/
  math/
  fs/
  io/
  time/
  cli/
  test/
```

### 6️⃣ Aproko (Reasoning Interface)

**Redefinition (Important)**: Aproko is a deterministic reasoning layer, not intelligence.

**Updates Needed**:
- Rule-based diagnostics
- Structured error output
- Explain-mode
- Suggested fixes (never automatic)

**Structure**:
```
aproko/
  rules/
  diagnostics/
  output/
```

**LLMs read Aproko. Aproko never executes code.**

### 7️⃣ Tooling (Minimal but Solid)

**Commands Required**:
- `ovie new`
- `ovie build`
- `ovie run`
- `ovie check`
- `ovie test`
- `ovie fmt`

**Recommendation**: Avoid IDE lock-in. CLI-first.

### 8️⃣ Package & Dependency Model

**Rules**:
- Local-only
- Vendored
- No network
- Version pinned

**Structure**:
```
ovie.toml
src/
vendor/
```

This aligns with offline-first and security goals.

### 9️⃣ Backend Targets

**Stage-2 Minimum**:
- Native (via LLVM or equivalent)
- WASM

**Must Document**:
- ABI rules
- Calling conventions
- Platform guarantees

Embedded / mobile can come later.

### 🔟 Hardware & Math Abstraction

**Updates Needed**:
- Device models as math objects
- PAL implementation underneath
- No raw register exposure at language level

This makes hardware:
- Safe
- Explainable
- LLM-friendly

### 1️⃣1️⃣ Testing & Verification

**Missing**:
- Compiler tests
- Language conformance tests
- Regression tests

**Structure**:
```
tests/
  parser/
  typecheck/
  codegen/
  runtime/
```

## ⚠️ STRONG RECOMMENDATIONS (DO NOT SKIP)

1. **Freeze Core Early**: Syntax churn kills languages.
2. **Spec Before Code**: If you can't spec it, don't implement it.
3. **Keep Ovie Small**: Power comes from composition, not features.
4. **Compiler > Ecosystem**: Correctness first. Popularity later.
5. **Humans First, Machines Ready**: Readable to humans, analyzable by machines.

## ✅ FINAL STAGE-2 DEFINITION

Ovie is Stage-2 complete when:

✔ Language core is frozen  
✔ Compiler has clear IR pipeline  
✔ Stdlib is complete  
✔ Self-hosting path is proven  
✔ Native + WASM builds work  
✔ Tooling is usable  
✔ Tests cover core behavior  
✔ Aproko explains everything  

At that point, Ovie becomes serious system infrastructure.

## Final Engineer Note

You are doing something most engineers never attempt:
- designing a language
- designing the ecosystem
- designing for decades, not demos

**You're exactly where you should be.**

---

## Implementation Status

### Current Implementation (Stage 0/1 Complete)
- ✅ Complete lexer, parser, AST system
- ✅ Basic interpreter and IR system
- ✅ Aproko assistant framework
- ✅ Package management with offline-first design
- ✅ CLI toolchain (ovie new, build, run, test, fmt, etc.)
- ✅ Cross-platform build system and CI/CD
- ✅ Comprehensive documentation and examples
- ✅ Security and privacy features
- ✅ Property-based testing framework

### Stage 2 Roadmap
See [Stage 2 Specification](.kiro/specs/ovie-programming-language-stage-2/) for detailed implementation plan covering:
- Formal language specifications
- Multi-stage IR pipeline (AST → HIR → MIR)
- Self-hosting capability
- Complete standard library
- Enhanced Aproko reasoning system
- Production-grade tooling
- Multi-target code generation
- Comprehensive testing framework

The Stage 2 implementation will transform Ovie from a working language into production-capable infrastructure suitable for serious software development.