# Ovie Programming Language - Error Model Specification

**Version:** 2.1.0  
**Status:** FROZEN (Stage 2)  
**Last Updated:** 2024-01-28  

## Overview

This document defines the complete error model for the Ovie Programming Language. The error model is **FROZEN** as part of Stage 2 development - no error model changes are permitted without formal RFC process.

The Ovie error model is designed to provide:
- **Predictable Error Handling**: Clear distinction between compile-time and runtime errors
- **Comprehensive Error Information**: Detailed error messages with context and suggestions
- **Deterministic Behavior**: Consistent error handling across platforms and runs
- **Recovery Mechanisms**: Graceful error recovery where possible

## Error Model Foundations

### Core Principles

1. **Explicit Error Handling**: No hidden exceptions or undefined behavior
2. **Fail-Fast Philosophy**: Errors are detected and reported as early as possible
3. **Comprehensive Diagnostics**: Every error includes context and suggestions
4. **Deterministic Reporting**: Same errors produce same messages consistently
5. **Structured Error Information**: Machine-readable error data for tooling

### Error Categories

```ebnf
error = compile_time_error | runtime_error | system_error ;

compile_time_error = syntax_error 
                   | type_error 
                   | name_resolution_error 
                   | borrow_check_error 
                   | const_eval_error 
                   ;

runtime_error = arithmetic_error 
              | memory_error 
              | io_error 
              | resource_error 
              | assertion_error 
              ;

system_error = platform_error 
             | environment_error 
             | resource_exhaustion_error 
             ;
```

## Compile-Time Errors

Compile-time errors are detected during the compilation process and prevent code generation.

### Syntax Errors

**Definition**: Violations of the formal grammar specification.

**Examples:**
```ovie
// Missing semicolon
x = 42          // ERROR: Expected ';' after expression

// Unmatched braces
fn test() {
    seeAm "hello";
// ERROR: Expected '}' to close function body

// Invalid operators
x = 5 @ 3;      // ERROR: Unknown operator '@'
```

**Error Structure:**
```rust
pub struct SyntaxError {
    pub location: SourceLocation,
    pub expected: Vec<String>,
    pub found: String,
    pub message: String,
    pub suggestions: Vec<ErrorSuggestion>,
}
```

**Recovery Strategy:**
- Continue parsing after inserting expected tokens
- Skip to next statement boundary
- Report multiple syntax errors in single pass

### Type Errors

**Definition**: Violations of the type system rules.

**Examples:**
```ovie
// Type mismatch
x = "hello" + 42;           // ERROR: Cannot add String and Number

// Undefined type
y: UnknownType = 5;         // ERROR: Type 'UnknownType' not found

// Function arity mismatch
fn add(a, b) { return a + b; }
result = add(1, 2, 3);      // ERROR: Function expects 2 arguments, found 3
```

**Error Structure:**
```rust
pub struct TypeError {
    pub location: SourceLocation,
    pub expected_type: Type,
    pub found_type: Type,
    pub context: TypeContext,
    pub message: String,
    pub suggestions: Vec<ErrorSuggestion>,
}
```

**Type Error Categories:**
1. **Mismatch**: Expected type A, found type B
2. **Undefined**: Reference to undefined type
3. **Inference Failure**: Cannot infer type from context
4. **Constraint Violation**: Type doesn't satisfy required constraints
5. **Arity Mismatch**: Wrong number of type parameters

### Name Resolution Errors

**Definition**: References to undefined identifiers or incorrect scoping.

**Examples:**
```ovie
// Undefined variable
seeAm undefined_var;        // ERROR: Variable 'undefined_var' not found

// Undefined function
result = unknown_func();    // ERROR: Function 'unknown_func' not found

// Out of scope
{
    local_var = 42;
}
seeAm local_var;           // ERROR: Variable 'local_var' not in scope
```

**Error Structure:**
```rust
pub struct NameResolutionError {
    pub location: SourceLocation,
    pub identifier: String,
    pub kind: IdentifierKind,
    pub available_names: Vec<String>,
    pub message: String,
    pub suggestions: Vec<ErrorSuggestion>,
}
```

### Borrow Check Errors

**Definition**: Violations of ownership and borrowing rules.

**Examples:**
```ovie
// Use after move
x = "hello";
y = x;
seeAm x;                   // ERROR: Use of moved value 'x'

// Multiple mutable borrows
mut z = "world";
ref1 = &mut z;
ref2 = &mut z;             // ERROR: Cannot borrow 'z' as mutable more than once

// Dangling reference
fn bad() -> &String {
    local = "temp";
    return &local;         // ERROR: Reference to local variable returned
}
```

**Error Structure:**
```rust
pub struct BorrowCheckError {
    pub location: SourceLocation,
    pub error_kind: BorrowErrorKind,
    pub moved_location: Option<SourceLocation>,
    pub borrowed_location: Option<SourceLocation>,
    pub message: String,
    pub suggestions: Vec<ErrorSuggestion>,
}

pub enum BorrowErrorKind {
    UseAfterMove,
    MultipleMutableBorrows,
    MutableBorrowWhileImmutableBorrowExists,
    DanglingReference,
    BorrowOfMovedValue,
}
```

### Constant Evaluation Errors

**Definition**: Errors during compile-time constant evaluation.

**Examples:**
```ovie
// Division by zero in constant
const INVALID = 10 / 0;    // ERROR: Division by zero in constant expression

// Overflow in constant
const TOO_BIG = 9223372036854775807 + 1;  // ERROR: Integer overflow in constant

// Invalid constant operation
const BAD = "hello"[100];  // ERROR: Index out of bounds in constant
```

**Error Structure:**
```rust
pub struct ConstEvalError {
    pub location: SourceLocation,
    pub operation: String,
    pub operands: Vec<String>,
    pub error_kind: ConstEvalErrorKind,
    pub message: String,
    pub suggestions: Vec<ErrorSuggestion>,
}
```

## Runtime Errors

Runtime errors occur during program execution and can potentially be handled by the program.

### Arithmetic Errors

**Definition**: Mathematical operations that cannot be completed.

**Examples:**
```ovie
// Division by zero
x = 10 / 0;                // RUNTIME ERROR: Division by zero

// Integer overflow (in debug mode)
y = 9223372036854775807 + 1;  // RUNTIME ERROR: Integer overflow

// Invalid floating-point operation
z = 0.0 / 0.0;             // Result: NaN (not an error, but special value)
```

**Error Structure:**
```rust
pub struct ArithmeticError {
    pub location: SourceLocation,
    pub operation: ArithmeticOperation,
    pub operands: Vec<Value>,
    pub error_kind: ArithmeticErrorKind,
    pub message: String,
}

pub enum ArithmeticErrorKind {
    DivisionByZero,
    IntegerOverflow,
    IntegerUnderflow,
    InvalidOperation,
}
```

**Handling Strategy:**
- Division by zero: Panic with detailed message
- Integer overflow: Panic in debug mode, wrap in release mode
- Floating-point errors: Return special values (NaN, Infinity)

### Memory Errors

**Definition**: Invalid memory access or management errors.

**Examples:**
```ovie
// Array bounds violation (future)
arr = [1, 2, 3];
value = arr[10];           // RUNTIME ERROR: Index 10 out of bounds for array of length 3

// Stack overflow
fn infinite_recursion() {
    infinite_recursion();  // RUNTIME ERROR: Stack overflow
}
```

**Error Structure:**
```rust
pub struct MemoryError {
    pub location: SourceLocation,
    pub error_kind: MemoryErrorKind,
    pub address: Option<usize>,
    pub size: Option<usize>,
    pub message: String,
}

pub enum MemoryErrorKind {
    IndexOutOfBounds,
    StackOverflow,
    HeapExhaustion,
    InvalidAccess,
}
```

### I/O Errors

**Definition**: Input/output operations that fail.

**Examples:**
```ovie
// File not found (future)
content = read_file("nonexistent.txt");  // RUNTIME ERROR: File not found

// Permission denied (future)
write_file("/root/file.txt", "data");    // RUNTIME ERROR: Permission denied
```

**Error Structure:**
```rust
pub struct IoError {
    pub location: SourceLocation,
    pub operation: String,
    pub path: Option<String>,
    pub error_kind: IoErrorKind,
    pub system_error: Option<i32>,
    pub message: String,
}
```

### Resource Errors

**Definition**: System resource exhaustion or unavailability.

**Examples:**
```ovie
// Out of memory
large_array = allocate_array(1000000000);  // RUNTIME ERROR: Out of memory

// Too many open files
for i in 0..10000 {
    file = open_file("temp.txt");          // RUNTIME ERROR: Too many open files
}
```

### Assertion Errors

**Definition**: Failed assertions and debug checks.

**Examples:**
```ovie
// Debug assertion (future)
debug_assert!(x > 0);      // RUNTIME ERROR: Assertion failed: x > 0

// Unreachable code (future)
unreachable!();            // RUNTIME ERROR: Reached unreachable code
```

## System Errors

System errors are caused by the environment or platform and are generally not recoverable.

### Platform Errors

**Definition**: Platform-specific errors that prevent execution.

**Examples:**
- Unsupported instruction set
- Missing system libraries
- Incompatible operating system version

### Environment Errors

**Definition**: Environment configuration errors.

**Examples:**
- Missing environment variables
- Invalid locale settings
- Insufficient permissions

### Resource Exhaustion Errors

**Definition**: System resource limits exceeded.

**Examples:**
- Out of virtual memory
- Process limit exceeded
- Disk space exhausted

## Error Reporting and Diagnostics

### Error Message Format

All errors follow a consistent format:

```
error[E0001]: type mismatch
  --> src/main.ov:5:13
   |
5  |     let x = "hello" + 42;
   |             ^^^^^^^^^^^^
   |             |       |
   |             |       expected `String`, found `Number`
   |             expected `String` because of this
   |
help: convert the number to a string
   |
5  |     let x = "hello" + 42.to_string();
   |                       ~~~~~~~~~~~~~~

note: the `+` operator is defined for string concatenation
  --> src/main.ov:5:21
   |
5  |     let x = "hello" + 42;
   |                     ^
```

### Error Code System

Each error type has a unique error code:

- **E0001-E0999**: Syntax errors
- **E1000-E1999**: Type errors  
- **E2000-E2999**: Name resolution errors
- **E3000-E3999**: Borrow check errors
- **E4000-E4999**: Constant evaluation errors
- **E5000-E5999**: Runtime errors
- **E6000-E6999**: System errors

### Diagnostic Structure

```rust
pub struct Diagnostic {
    pub code: ErrorCode,
    pub severity: Severity,
    pub message: String,
    pub location: SourceLocation,
    pub spans: Vec<DiagnosticSpan>,
    pub suggestions: Vec<ErrorSuggestion>,
    pub notes: Vec<String>,
    pub help: Option<String>,
}

pub enum Severity {
    Error,      // Compilation must stop
    Warning,    // Compilation continues, but issue should be addressed
    Note,       // Additional information
    Help,       // Suggestion for fixing the issue
}

pub struct DiagnosticSpan {
    pub location: SourceLocation,
    pub length: usize,
    pub label: Option<String>,
    pub style: SpanStyle,
}

pub enum SpanStyle {
    Primary,    // Main error location
    Secondary,  // Related location
    Note,       // Additional context
}
```

### Error Suggestions

The compiler provides actionable suggestions for fixing errors:

```rust
pub struct ErrorSuggestion {
    pub message: String,
    pub fixes: Vec<CodeFix>,
    pub confidence: SuggestionConfidence,
}

pub enum SuggestionConfidence {
    High,       // Very likely to fix the issue
    Medium,     // Might fix the issue
    Low,        // Speculative suggestion
}

pub struct CodeFix {
    pub location: SourceLocation,
    pub replacement: String,
    pub description: String,
}
```

## Error Handling Mechanisms

### Panic System

For unrecoverable errors, Ovie uses a panic system:

```ovie
// Explicit panic (future)
if critical_condition {
    panic!("Critical error occurred");
}

// Automatic panic on runtime errors
x = 10 / 0;  // Panics with "Division by zero"
```

**Panic Behavior:**
1. Print error message and stack trace
2. Unwind the stack (cleanup local variables)
3. Exit the program with non-zero status
4. Optional panic hook for custom handling

### Result Types (Future)

For recoverable errors, Ovie will use Result types:

```ovie
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Function that can fail
fn divide(a: Number, b: Number) -> Result<Number, String> {
    if b == 0 {
        return Result::Err("Division by zero");
    }
    return Result::Ok(a / b);
}

// Error handling
match divide(10, 0) {
    Result::Ok(value) => seeAm value,
    Result::Err(error) => seeAm "Error: " + error,
}
```

### Option Types (Future)

For values that might not exist:

```ovie
enum Option<T> {
    Some(T),
    None,
}

// Function that might not return a value
fn find_item(items: Array<String>, target: String) -> Option<String> {
    // Implementation
}

// Handling optional values
match find_item(items, "target") {
    Option::Some(item) => seeAm "Found: " + item,
    Option::None => seeAm "Not found",
}
```

## Undefined Behavior

Ovie aims to eliminate undefined behavior through:

### Compile-Time Prevention

1. **Type Safety**: Type system prevents type confusion
2. **Memory Safety**: Ownership system prevents memory errors
3. **Bounds Checking**: Array access is bounds-checked
4. **Initialization**: All variables must be initialized before use

### Runtime Detection

1. **Integer Overflow**: Detected in debug mode
2. **Array Bounds**: Checked at runtime
3. **Stack Overflow**: Detected and reported
4. **Null Pointer Dereference**: Prevented by type system (no null pointers)

### Explicitly Undefined Behavior

The following operations have explicitly undefined behavior:

1. **Unsafe Blocks**: Operations in `unsafe` blocks may have undefined behavior
2. **FFI Calls**: Foreign function interface calls may have undefined behavior
3. **Inline Assembly**: Assembly code may have undefined behavior
4. **Transmute Operations**: Type transmutation may have undefined behavior

## Error Recovery and Resilience

### Compiler Error Recovery

The compiler attempts to recover from errors to find additional issues:

1. **Syntax Error Recovery**: Insert missing tokens and continue parsing
2. **Type Error Recovery**: Assume reasonable types and continue checking
3. **Name Resolution Recovery**: Suggest similar names and continue
4. **Partial Compilation**: Generate partial results when possible

### Runtime Error Recovery

Runtime errors can be handled through:

1. **Panic Handling**: Custom panic hooks for cleanup
2. **Error Propagation**: Result types for recoverable errors
3. **Graceful Degradation**: Continue with reduced functionality
4. **Restart Mechanisms**: Restart failed components

### Testing Error Conditions

Error conditions are thoroughly tested:

```rust
#[test]
fn test_division_by_zero() {
    let result = std::panic::catch_unwind(|| {
        let x = 10.0 / 0.0;
    });
    assert!(result.is_err());
}

#[test]
fn test_type_error_message() {
    let source = r#"x = "hello" + 42;"#;
    let error = compile_and_get_error(source);
    assert_eq!(error.code, ErrorCode::E1001);
    assert!(error.message.contains("type mismatch"));
}
```

## Deterministic Error Behavior

### Consistent Error Messages

- Same errors produce identical messages across platforms
- Error messages are deterministic and reproducible
- No randomization in error reporting or suggestions
- Consistent ordering of multiple errors

### Platform Independence

- Error codes are consistent across platforms
- Error messages don't include platform-specific details
- Stack traces use portable format
- File paths use canonical representation

### Version Stability

- Error codes remain stable across compiler versions
- Error message format is preserved
- New error categories get new code ranges
- Deprecated errors are marked but not removed

## Integration with Aproko

The error system integrates with the Aproko reasoning system:

### Enhanced Diagnostics

Aproko provides additional context for errors:

```
error[E1001]: type mismatch
  --> src/main.ov:5:13
   |
5  |     let x = "hello" + 42;
   |             ^^^^^^^^^^^^
   |
   = note: The `+` operator for strings performs concatenation
   = help: Convert the number to a string: 42.to_string()
   = aproko: This is a common mistake when mixing strings and numbers
   = aproko: Consider using string interpolation: "hello{42}"
```

### Learning from Errors

Aproko tracks common error patterns:

1. **Frequency Analysis**: Most common errors in codebase
2. **Pattern Recognition**: Similar errors across files
3. **Suggestion Improvement**: Better suggestions based on context
4. **Educational Content**: Links to relevant documentation

## Performance Considerations

### Error Reporting Performance

- Error collection is lazy (only computed when needed)
- Error messages are cached to avoid recomputation
- Diagnostic spans are computed efficiently
- Suggestion generation is optimized for common cases

### Runtime Error Performance

- Error paths are optimized for the non-error case
- Panic unwinding is efficient but not zero-cost
- Stack trace generation is optimized
- Error propagation has minimal overhead

## Future Extensions

### Advanced Error Types

1. **Structured Errors**: Machine-readable error data
2. **Error Chains**: Linked error causation
3. **Context Preservation**: Rich error context
4. **Error Recovery**: Automatic error correction

### IDE Integration

1. **Real-time Error Checking**: Errors as you type
2. **Quick Fixes**: One-click error resolution
3. **Error Navigation**: Jump between related errors
4. **Error Explanation**: Detailed error explanations

### Debugging Support

1. **Error Breakpoints**: Break on specific error types
2. **Error History**: Track error patterns over time
3. **Error Visualization**: Graphical error representation
4. **Error Replay**: Reproduce error conditions

---

**This error model specification is the authoritative definition of Ovie's error handling system and must be implemented exactly as specified.**