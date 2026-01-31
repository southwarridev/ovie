# Minimal Ovie Compiler Subset Specification
## Stage 1 Self-Hosting Foundation

### Overview

This document defines the minimal subset of the Ovie programming language required to implement a self-hosting compiler. The minimal compiler must be capable of compiling itself while maintaining all correctness, performance, and security guarantees of the full Stage 0 Rust implementation.

### Design Principles

1. **Minimal Viable Subset**: Include only language features absolutely necessary for compiler implementation
2. **Bootstrap Compatibility**: Ensure seamless transition from Stage 0 (Rust) to Stage 1 (Partial Ovie)
3. **Deterministic Behavior**: Maintain identical output to the Rust compiler for all valid programs
4. **Offline-First**: No network dependencies during compilation or execution
5. **Property Preservation**: All existing property-based tests must pass

### Minimal Language Features

#### Core Data Types

**Primitive Types** (Required):
- `Number`: 64-bit signed integer and floating-point arithmetic
- `String`: UTF-8 encoded text with ownership semantics
- `Boolean`: True/false values for conditional logic

**Composite Types** (Required):
- `Array[T]`: Dynamic arrays with bounds checking
- `Struct`: Named field aggregation for data structures
- `Enum`: Sum types for token types and AST node variants

**Advanced Types** (Deferred to Stage 2):
- Generic types beyond basic `Array[T]`
- Trait system and implementations
- Advanced lifetime annotations

#### Control Flow Constructs

**Required for Compiler Implementation**:
```ovie
// Conditional execution
if condition {
    // then block
} else {
    // else block
}

// Iteration over collections
for item in collection {
    // loop body
}

// Conditional loops
while condition {
    // loop body
}

// Early returns
return value;
```

**Deferred to Stage 2**:
- Pattern matching with `match` expressions
- Loop labels and advanced control flow
- Exception handling beyond basic error propagation

#### Function System

**Required Features**:
```ovie
// Function definitions with parameters and return types
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // function body
    return result;
}

// Mutable parameters for state modification
fn modify_state(mut state: State) {
    state.field = new_value;
}

// Generic functions for collections
fn append(mut array: [T], item: T) {
    // append implementation
}
```

**Function Call Semantics**:
- Pass-by-value for primitive types
- Move semantics for owned types
- Mutable borrowing for state modification
- Immutable borrowing for read-only access

#### Memory Management

**Ownership System** (Simplified):
```ovie
// Ownership transfer
mut tokens = create_token_array();
process_tokens(tokens); // tokens moved

// Mutable borrowing
fn add_token(mut lexer: Lexer, token: Token) {
    append(lexer.tokens, token);
}

// Immutable borrowing
fn count_tokens(lexer: Lexer) -> Number {
    return array_length(lexer.tokens);
}
```

**Memory Safety Guarantees**:
- No null pointer dereferences
- No buffer overflows
- No use-after-free errors
- No data races (single-threaded execution)

#### Standard Library Subset

**String Operations** (Built-in):
```ovie
fn char_at(text: String, index: Number) -> String
fn substring(text: String, start: Number, end: Number) -> String
fn length(text: String) -> Number
```

**Array Operations** (Built-in):
```ovie
fn append(mut array: [T], item: T)
fn array_length(array: [T]) -> Number
fn array_get(array: [T], index: Number) -> T
```

**I/O Operations** (Built-in):
```ovie
fn print(value: T)
fn println(value: T)
fn eprint(message: String)  // Error output
```

**Character Classification** (Ovie Implementation):
```ovie
fn is_digit(c: String) -> Boolean
fn is_alpha(c: String) -> Boolean
fn is_alpha_numeric(c: String) -> Boolean
```

### Bootstrap Compiler Requirements

#### Stage 0 → Stage 1 Transition

**Phase 1: Lexer Replacement**
- Ovie lexer implementation compiled by Stage 0 Rust compiler
- Token-by-token verification against Rust lexer output
- Performance within 5x of Rust implementation
- Hash-based verification of identical tokenization

**Phase 2: Parser Integration**
- Ovie parser implementation using Ovie lexer output
- AST structure compatibility with existing Rust implementation
- Semantic preservation across the Rust/Ovie boundary
- Error handling and recovery parity

**Phase 3: Verification System**
- Automated equivalence testing between compilers
- Rollback capability for failed bootstrap attempts
- Reproducible build verification with cryptographic hashes
- Performance benchmarking and regression detection

#### Compilation Pipeline

**Stage 1 Hybrid Architecture**:
```
Source Code
    ↓
Ovie Lexer (compiled by Stage 0)
    ↓
Ovie Parser (compiled by Stage 0)
    ↓
Rust Semantic Analysis (Stage 0)
    ↓
Rust IR Generation (Stage 0)
    ↓
Rust Code Generation (Stage 0)
    ↓
Executable
```

**Bootstrap Verification Process**:
1. Compile Ovie lexer/parser with Stage 0 compiler
2. Run parallel compilation with both Rust and Ovie components
3. Compare intermediate representations at each stage
4. Verify identical final output with cryptographic hashes
5. Performance benchmark against baseline measurements

### Self-Hosting Roadmap

#### Milestone 1: Lexer Self-Hosting (Week 1-2)
- [ ] Implement required standard library functions
- [ ] Compile `lexer_spec.ov` with Stage 0 compiler
- [ ] Integrate Ovie lexer with Rust parser
- [ ] Achieve token-by-token verification
- [ ] Performance within acceptable bounds (5x slower)

#### Milestone 2: Parser Self-Hosting (Week 3-4)
- [ ] Compile `parser_spec.ov` with Stage 0 compiler
- [ ] Integrate Ovie parser with Rust semantic analysis
- [ ] Achieve AST-level verification
- [ ] Error handling parity with Rust implementation

#### Milestone 3: Bootstrap Verification (Week 5-6)
- [ ] Implement comprehensive verification system
- [ ] Automated testing of bootstrap process
- [ ] Performance optimization to within 2x of Rust
- [ ] Documentation and integration with CI/CD

#### Milestone 4: Stage 1 Production Ready (Week 7-8)
- [ ] Feature flag for Ovie lexer/parser usage
- [ ] Fallback mechanisms for error conditions
- [ ] Complete test suite passing
- [ ] Performance monitoring and alerting

### Language Feature Priorities

#### Tier 1: Essential (Required for Stage 1)
- Basic data types (Number, String, Boolean)
- Arrays with dynamic sizing
- Structs for data aggregation
- Enums for sum types
- Functions with parameters and return values
- Basic control flow (if/else, for, while)
- Ownership and borrowing (simplified)
- Standard library subset

#### Tier 2: Important (Target for Stage 2)
- Generic types beyond arrays
- Pattern matching with match expressions
- Advanced error handling
- Module system and imports
- Trait system for polymorphism
- Advanced lifetime annotations

#### Tier 3: Nice-to-Have (Future stages)
- Macro system
- Async/await functionality
- Advanced optimization hints
- Foreign function interface (FFI)
- Reflection and metaprogramming

### Implementation Strategy

#### Incremental Development
1. **Start with Lexer**: Smallest, most isolated component
2. **Add Parser**: Builds on lexer, produces AST
3. **Integrate Gradually**: Replace components one at a time
4. **Verify Continuously**: Test at each integration point
5. **Optimize Iteratively**: Improve performance after correctness

#### Risk Mitigation
- **Fallback Mechanisms**: Rust components remain available
- **Feature Flags**: Enable/disable Ovie components
- **Comprehensive Testing**: Property-based and unit tests
- **Performance Monitoring**: Detect regressions early
- **Rollback Capability**: Revert to known good states

#### Success Criteria
- **Functional**: All existing tests pass with Ovie components
- **Performance**: Within 2x of Rust implementation by Stage 1 completion
- **Correctness**: Identical output to Rust compiler (verified by hash)
- **Reliability**: Zero correctness regressions
- **Maintainability**: Clear separation between Rust and Ovie components

### Technical Specifications

#### Token Compatibility
The Ovie lexer must produce tokens identical to the Rust implementation:
```rust
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub location: SourceLocation,
}
```

#### AST Compatibility
The Ovie parser must produce AST nodes compatible with existing Rust structures:
```rust
pub enum AstNode {
    Program(Vec<Statement>),
    Statement(StatementKind),
    Expression(ExpressionKind),
}
```

#### Error Handling
Error messages and locations must be preserved across the Rust/Ovie boundary:
```rust
pub struct CompilerError {
    pub kind: ErrorKind,
    pub span: SourceSpan,
    pub message: String,
}
```

### Verification and Testing

#### Property-Based Testing
All existing properties must continue to pass:
- **Property 1**: Grammar Validation Completeness
- **Property 6**: IR Pipeline Integrity
- **Property 7**: Compiler Output Equivalence (new)
- **Property 8**: Bootstrap Process Reproducibility (new)

#### Unit Testing
- Port all existing Rust lexer tests to Ovie
- Port all existing Rust parser tests to Ovie
- Add new tests for bootstrap verification
- Cross-platform compatibility testing

#### Integration Testing
- End-to-end compilation with Ovie components
- Performance regression testing
- Memory usage monitoring
- Error handling verification

### Conclusion

This minimal compiler subset specification provides a clear path from the current Stage 0 Rust implementation to Stage 1 partial self-hosting. By focusing on the essential language features required for compiler implementation and maintaining strict compatibility with the existing system, we can achieve self-hosting while preserving all correctness and performance guarantees.

The incremental approach, comprehensive verification system, and clear success criteria ensure that the transition to self-hosting is both safe and measurable. The roadmap provides concrete milestones and timelines for achieving Stage 1 completion within 8 weeks.