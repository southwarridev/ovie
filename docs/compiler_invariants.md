# Ovie Compiler Invariants

**Version:** 2.1.0  
**Status:** MANDATORY - These are contracts between compiler stages  
**Rule:** If an invariant breaks → compiler bug → panic loudly

## Overview

Invariants are contracts between compiler stages that must ALWAYS hold true. They prevent silent corruption and ensure compiler correctness.

## AST Invariants

**Purpose:** Raw syntax tree from parsing - no semantic analysis

### MUST BE TRUE:
- ✅ AST contains no resolved types
- ✅ AST contains no symbol IDs  
- ✅ AST nodes preserve exact source spans
- ✅ No semantic validation occurs in AST
- ✅ All syntax is valid (parser succeeded)
- ✅ Comments and whitespace are preserved for tooling

### MUST NOT BE TRUE:
- ❌ No type information attached to nodes
- ❌ No symbol resolution
- ❌ No control flow analysis
- ❌ No semantic errors reported

### Validation Method:
```rust
impl AST {
    pub fn validate(&self) -> Result<(), InvariantError> {
        for node in &self.nodes {
            assert!(node.type_info.is_none(), "AST must not contain type info");
            assert!(node.symbol_id.is_none(), "AST must not contain symbol IDs");
            assert!(node.source_span.is_valid(), "AST must preserve source spans");
        }
        Ok(())
    }
}
```

## HIR Invariants (High-level IR)

**Purpose:** Semantically analyzed, typed, but still high-level

### MUST BE TRUE:
- ✅ All identifiers are resolved to symbols
- ✅ No unresolved names exist
- ✅ Every expression has a known type
- ✅ Type inference is complete
- ✅ Symbol table is fully populated
- ✅ Semantic errors have been caught
- ✅ Function signatures are resolved
- ✅ Struct/enum definitions are complete

### MUST NOT BE TRUE:
- ❌ No control-flow lowering has occurred
- ❌ No basic blocks exist yet
- ❌ No explicit temporaries
- ❌ No ABI-specific transformations

### Validation Method:
```rust
impl HIR {
    pub fn validate(&self) -> Result<(), InvariantError> {
        for node in &self.nodes {
            assert!(node.type_info.is_known(), "HIR must have complete type info");
            assert!(node.symbol.is_resolved(), "HIR must have resolved symbols");
            assert!(!node.is_lowered(), "HIR must not be lowered to control flow");
        }
        Ok(())
    }
}
```

## MIR Invariants (Mid-level IR)

**Purpose:** Control flow explicit, ready for optimization

### MUST BE TRUE:
- ✅ Control flow is explicit (basic blocks only)
- ✅ No high-level expressions exist
- ✅ No unresolved symbols
- ✅ No implicit temporaries
- ✅ All memory operations are explicit
- ✅ Borrow/ownership rules validated
- ✅ Function calls are explicit
- ✅ All variables have explicit lifetimes

### MUST NOT BE TRUE:
- ❌ No high-level control structures (if/while/for)
- ❌ No implicit conversions
- ❌ No unresolved function calls
- ❌ No dynamic dispatch unless explicitly marked

### Validation Method:
```rust
impl MIR {
    pub fn validate(&self) -> Result<(), InvariantError> {
        for block in &self.basic_blocks {
            assert!(block.is_well_formed(), "MIR blocks must be well-formed");
            assert!(block.has_terminator(), "MIR blocks must have terminators");
        }
        for instruction in &self.instructions {
            assert!(instruction.operands_resolved(), "MIR operands must be resolved");
            assert!(!instruction.is_high_level(), "MIR must not contain high-level constructs");
        }
        Ok(())
    }
}
```

## Backend Invariants

**Purpose:** Ready for code generation

### MUST BE TRUE:
- ✅ MIR contains no unreachable blocks
- ✅ All symbols are resolved
- ✅ ABI is fully known
- ✅ Register allocation is possible
- ✅ All function signatures match calling convention
- ✅ Memory layout is determined
- ✅ All external dependencies are resolved

### MUST NOT BE TRUE:
- ❌ No dynamic dispatch unless explicitly marked
- ❌ No unresolved external symbols
- ❌ No impossible register constraints
- ❌ No ABI mismatches

### Validation Method:
```rust
impl Backend {
    pub fn validate(&self) -> Result<(), InvariantError> {
        assert!(self.mir.is_optimized(), "Backend MIR must be optimized");
        assert!(self.abi.is_complete(), "Backend ABI must be complete");
        assert!(self.symbols.all_resolved(), "Backend symbols must be resolved");
        Ok(())
    }
}
```

## Cross-Stage Invariants

### AST → HIR Transition:
- ✅ No information loss (source spans preserved)
- ✅ All syntax errors caught before HIR
- ✅ Symbol table created
- ✅ Type information added

### HIR → MIR Transition:
- ✅ All high-level constructs lowered
- ✅ Control flow made explicit
- ✅ Memory operations explicit
- ✅ No semantic information lost

### MIR → Backend Transition:
- ✅ All optimizations applied
- ✅ ABI transformations complete
- ✅ Ready for code generation
- ✅ No MIR-specific constructs remain

## Invariant Testing

### Required Tests:
```rust
#[test]
fn ast_has_no_type_info() {
    let ast = parse("valid.ov");
    ast.validate().unwrap();
}

#[test]
fn hir_has_complete_types() {
    let hir = compile_to_hir("valid.ov");
    hir.validate().unwrap();
}

#[test]
fn mir_has_explicit_control_flow() {
    let mir = compile_to_mir("valid.ov");
    mir.validate().unwrap();
}

#[test]
fn backend_is_ready_for_codegen() {
    let backend = compile_to_backend("valid.ov");
    backend.validate().unwrap();
}
```

## Error Handling

### When Invariants Break:
1. **Panic immediately** - Don't continue with corrupted state
2. **Log detailed information** - What invariant, where, why
3. **Provide debugging context** - Source location, compiler stage
4. **Exit with error code 2** - Distinguish from user errors (code 1)

### Example Error:
```
COMPILER INVARIANT VIOLATION
Stage: HIR → MIR
Invariant: All symbols must be resolved
Location: src/example.ov:15:8
Symbol: 'undefined_function'
Context: Function call resolution

This is a compiler bug. Please report it.
```

## Enforcement

### In Development:
- All invariant violations are fatal
- CI fails if any invariant test fails
- Debug builds include extra invariant checks

### In Production:
- Invariant violations still fatal (better than silent corruption)
- Release builds may skip expensive checks for performance
- Users can enable full checking with `--debug-invariants`

## Future Extensions

### Planned for v2.2:
- Performance invariants (compilation time bounds)
- Memory usage invariants (no unbounded allocation)
- Determinism invariants (same input → same output)

### Planned for v3.0:
- Formal verification of invariants
- Automated invariant generation
- Cross-compilation invariants