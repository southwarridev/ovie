# Ovie Programming Language - Memory Model and Ownership Specification

**Version:** 2.1.0  
**Status:** FROZEN (Stage 2)  
**Last Updated:** 2024-01-28  

## Overview

This document defines the complete memory model and ownership system for the Ovie Programming Language. The memory model is **FROZEN** as part of Stage 2 development - no memory model changes are permitted without formal RFC process.

The Ovie memory model is designed to provide:
- **Memory Safety**: No buffer overflows, use-after-free, or double-free errors
- **Data Race Freedom**: No concurrent access to mutable data without synchronization
- **Predictable Performance**: Clear allocation and deallocation semantics
- **Deterministic Behavior**: Consistent memory layout and access patterns

## Memory Model Foundations

### Core Principles

1. **Ownership**: Every value has exactly one owner at any time
2. **Borrowing**: References allow temporary access without transferring ownership
3. **Lifetimes**: All references have a scope during which they are valid
4. **Move Semantics**: Values are moved by default, not copied
5. **Explicit Mutability**: Mutability must be declared explicitly

### Memory Layout

Ovie uses a stack-based memory model with explicit heap allocation:

```
Stack Frame Layout:
┌─────────────────┐ ← Stack Pointer (SP)
│ Local Variables │
├─────────────────┤
│ Function Args   │
├─────────────────┤
│ Return Address  │
├─────────────────┤
│ Previous Frame  │
└─────────────────┘ ← Frame Pointer (FP)

Heap Layout:
┌─────────────────┐
│ Object Header   │ ← Contains type info, size, ref count
├─────────────────┤
│ Object Data     │
└─────────────────┘
```

## Ownership System

### Ownership Rules

1. **Single Owner**: Each value has exactly one owner
2. **Transfer on Assignment**: Assignment transfers ownership
3. **Scope-based Cleanup**: Values are dropped when owner goes out of scope
4. **No Dangling Pointers**: References cannot outlive their referents

### Ownership Transfer

```ovie
// Ownership transfer example
x = "hello";        // x owns the string
y = x;              // Ownership transfers to y
// x is no longer valid here
seeAm y;            // OK: y owns the string
```

**Formal Rules:**
```
Γ ⊢ e : T
─────────────────────────
Γ ⊢ let x = e : Unit
Γ' = Γ[x ↦ T]    (x now owns the value)

Γ ⊢ x : T   x ∈ dom(Γ)
─────────────────────────
Γ ⊢ let y = x : Unit
Γ' = Γ[x ↦ ⊥, y ↦ T]    (ownership transferred from x to y)
```

### Move Semantics

By default, all assignments and function calls move values:

```ovie
fn consume(s: String) {
    seeAm s;
}

text = "hello";
consume(text);      // text is moved into function
// text is no longer accessible here
```

**Move Rules:**
- Primitive types (Number, Boolean) are copied, not moved
- Compound types (String, Struct, Enum) are moved by default
- Moved values become inaccessible in the original scope

## Borrowing System

### Reference Types

Ovie supports two types of references:
- **Shared References** (`&T`): Multiple immutable references allowed
- **Mutable References** (`&mut T`): Exactly one mutable reference allowed

```ovie
// Shared borrowing
x = "hello";
ref1 = &x;          // Shared reference
ref2 = &x;          // Multiple shared references OK
seeAm *ref1;        // Dereference to access value

// Mutable borrowing  
mut y = "world";
mut_ref = &mut y;   // Mutable reference
*mut_ref = "changed"; // Modify through reference
// Cannot have other references while mut_ref exists
```

### Borrowing Rules

1. **Aliasing XOR Mutability**: Either multiple shared references OR one mutable reference
2. **Lifetime Constraint**: References cannot outlive their referents
3. **No Dangling References**: All references must be valid when used
4. **Borrow Checker**: Compile-time verification of borrowing rules

**Formal Borrowing Rules:**
```
Γ ⊢ x : T   x ∈ dom(Γ)
─────────────────────────
Γ ⊢ &x : &T

Γ ⊢ x : mut T   x ∈ dom(Γ)   no_other_refs(x, Γ)
─────────────────────────────────────────────────
Γ ⊢ &mut x : &mut T

Γ ⊢ r : &T
─────────────
Γ ⊢ *r : T

Γ ⊢ r : &mut T
─────────────────
Γ ⊢ *r : T
```

### Lifetime System

Lifetimes ensure references remain valid:

```ovie
fn get_reference() -> &String {
    local = "temporary";
    return &local;      // ERROR: reference outlives referent
}

fn valid_reference(s: &String) -> &String {
    return s;           // OK: lifetime preserved
}
```

**Lifetime Rules:**
- References cannot outlive the data they reference
- Function parameters and return values have related lifetimes
- Local variables have lexical scope lifetimes
- Heap-allocated data has explicit lifetime management

## Mutability System

### Mutability Declaration

Mutability must be explicitly declared:

```ovie
// Immutable by default
x = 42;
// x = 43;          // ERROR: cannot mutate immutable variable

// Explicit mutability
mut y = 42;
y = 43;             // OK: y is mutable

// Mutable references
mut z = "hello";
ref = &mut z;
*ref = "world";     // OK: mutable reference allows mutation
```

### Mutability Rules

1. **Immutable by Default**: All bindings are immutable unless marked `mut`
2. **Transitive Mutability**: Mutability applies to the entire value
3. **Mutable References**: Require mutable binding and exclusive access
4. **Interior Mutability**: Not supported in Stage 2 (future extension)

**Formal Mutability Rules:**
```
Γ ⊢ e : T
─────────────────────────
Γ ⊢ let x = e : Unit
Γ' = Γ[x ↦ T]    (x is immutable)

Γ ⊢ e : T
─────────────────────────
Γ ⊢ let mut x = e : Unit
Γ' = Γ[x ↦ mut T]    (x is mutable)

Γ ⊢ x : mut T   Γ ⊢ e : T
─────────────────────────
Γ ⊢ x = e : Unit    (assignment to mutable variable)
```

## Memory Management

### Stack Allocation

Most values are allocated on the stack:
- Local variables
- Function parameters
- Small structs and enums
- Primitive types

**Stack Management:**
- Automatic allocation on variable declaration
- Automatic deallocation on scope exit
- LIFO (Last In, First Out) ordering
- No fragmentation or garbage collection needed

### Heap Allocation

Heap allocation is explicit and controlled:

```ovie
// Future syntax (not implemented in Stage 2)
boxed = Box::new("heap allocated");
vec = Vec::new();
vec.push(1);
vec.push(2);
```

**Heap Management (Future):**
- Reference counting for automatic deallocation
- No garbage collection (deterministic cleanup)
- Explicit allocation through standard library types
- Cycle detection for reference cycles

### Memory Safety Guarantees

1. **No Buffer Overflows**: Array bounds checking at runtime
2. **No Use-After-Free**: Ownership system prevents access to freed memory
3. **No Double-Free**: Each value is freed exactly once
4. **No Memory Leaks**: Automatic cleanup when owners go out of scope
5. **No Data Races**: Borrowing rules prevent concurrent mutable access

## Type-Specific Memory Behavior

### Primitive Types

```ovie
// Copy semantics for primitives
x = 42;
y = x;              // x is copied, both x and y are valid
seeAm x + y;        // OK: both values accessible
```

**Memory Layout:**
- `Number`: 8 bytes (64-bit float)
- `Boolean`: 1 byte
- `Unit`: 0 bytes (zero-sized type)

### String Type

```ovie
// Move semantics for strings
s1 = "hello";
s2 = s1;            // s1 is moved to s2
// seeAm s1;        // ERROR: s1 no longer valid
seeAm s2;           // OK: s2 owns the string
```

**Memory Layout:**
```
String Structure:
┌─────────────────┐
│ Pointer to Data │ ← 8 bytes
├─────────────────┤
│ Length          │ ← 8 bytes  
├─────────────────┤
│ Capacity        │ ← 8 bytes
└─────────────────┘

String Data (heap):
┌─────────────────┐
│ UTF-8 Bytes     │ ← Variable length
└─────────────────┘
```

### Struct Types

```ovie
struct Person {
    name: String,
    age: Number,
}

// Move semantics for structs
p1 = Person { name: "Alice", age: 30 };
p2 = p1;            // Entire struct is moved
// seeAm p1.name;   // ERROR: p1 no longer valid
seeAm p2.name;      // OK: p2 owns the struct
```

**Memory Layout:**
```
Person Structure:
┌─────────────────┐
│ name: String    │ ← 24 bytes (String structure)
├─────────────────┤
│ age: Number     │ ← 8 bytes
└─────────────────┘
Total: 32 bytes + heap allocation for string data
```

### Enum Types

```ovie
enum Option {
    Some(Number),
    None,
}

// Move semantics for enums
opt1 = Option::Some(42);
opt2 = opt1;        // opt1 is moved to opt2
// Cannot access opt1 after move
```

**Memory Layout:**
```
Option Enum:
┌─────────────────┐
│ Discriminant    │ ← 1 byte (variant tag)
├─────────────────┤
│ Padding         │ ← 7 bytes (alignment)
├─────────────────┤
│ Data            │ ← 8 bytes (largest variant)
└─────────────────┘
Total: 16 bytes (aligned)
```

## Function Call Semantics

### Parameter Passing

```ovie
fn take_ownership(s: String) {
    seeAm s;
    // s is dropped here
}

fn borrow_immutable(s: &String) {
    seeAm s;
    // s reference expires here
}

fn borrow_mutable(s: &mut String) {
    s = "modified";
    // s reference expires here
}

text = "hello";
take_ownership(text);   // text is moved
// text is no longer valid

mut text2 = "world";
borrow_immutable(&text2);  // text2 is borrowed
seeAm text2;               // OK: text2 still valid

borrow_mutable(&mut text2); // text2 is mutably borrowed
seeAm text2;                // OK: text2 still valid (now "modified")
```

### Return Value Semantics

```ovie
fn create_string() -> String {
    return "created";   // Ownership transferred to caller
}

fn return_reference(s: &String) -> &String {
    return s;           // Reference lifetime preserved
}

owned = create_string();    // owned receives ownership
seeAm owned;               // OK: owned has the value

text = "original";
ref = return_reference(&text);  // ref borrows from text
seeAm ref;                     // OK: text still owns the data
```

## Error Conditions and Safety

### Compile-Time Errors

The borrow checker prevents these errors at compile time:

1. **Use After Move**:
```ovie
x = "hello";
y = x;              // x is moved
seeAm x;            // ERROR: use of moved value
```

2. **Multiple Mutable References**:
```ovie
mut x = "hello";
ref1 = &mut x;
ref2 = &mut x;      // ERROR: cannot borrow as mutable more than once
```

3. **Mutable and Immutable References**:
```ovie
mut x = "hello";
ref1 = &x;
ref2 = &mut x;      // ERROR: cannot borrow as mutable while immutable borrow exists
```

4. **Dangling References**:
```ovie
fn dangling() -> &String {
    local = "temp";
    return &local;  // ERROR: reference to local variable returned
}
```

### Runtime Safety

Even with compile-time checks, some safety is enforced at runtime:

1. **Array Bounds Checking**:
```ovie
// Future syntax
arr = [1, 2, 3];
value = arr[5];     // RUNTIME ERROR: index out of bounds
```

2. **Integer Overflow** (configurable):
```ovie
x = 9223372036854775807;  // Max i64
y = x + 1;                // RUNTIME ERROR: integer overflow (in debug mode)
```

## Memory Model Guarantees

### Safety Guarantees

1. **Memory Safety**: No undefined behavior related to memory access
2. **Type Safety**: No type confusion or invalid casts
3. **Thread Safety**: No data races (when threading is added)
4. **Resource Safety**: No resource leaks or double-free errors

### Performance Guarantees

1. **Zero-Cost Abstractions**: Ownership and borrowing have no runtime overhead
2. **Predictable Allocation**: Stack allocation is preferred and predictable
3. **No Garbage Collection**: Deterministic cleanup without GC pauses
4. **Minimal Runtime**: No hidden allocations or runtime type information

### Determinism Guarantees

1. **Consistent Layout**: Struct and enum layouts are deterministic
2. **Reproducible Behavior**: Same program produces same memory patterns
3. **Platform Independence**: Memory model behavior is consistent across platforms
4. **Version Stability**: Memory layout preserved across compiler versions

## Advanced Features (Future Extensions)

### Smart Pointers (Post-Stage 2)

```ovie
// Reference counted pointer
rc_ptr = Rc::new("shared data");
rc_ptr2 = rc_ptr.clone();

// Unique pointer
unique = Box::new("unique data");
```

### Interior Mutability (Post-Stage 2)

```ovie
// Cell for Copy types
cell = Cell::new(42);
cell.set(43);

// RefCell for non-Copy types
ref_cell = RefCell::new("mutable");
ref_cell.borrow_mut() = "changed";
```

### Weak References (Post-Stage 2)

```ovie
// Weak reference to break cycles
weak_ref = Rc::downgrade(&rc_ptr);
if let Some(strong) = weak_ref.upgrade() {
    seeAm strong;
}
```

## Implementation Notes

### Borrow Checker Implementation

The borrow checker uses these analyses:

1. **Liveness Analysis**: Determine when variables are live
2. **Loan Analysis**: Track when values are borrowed
3. **Region Analysis**: Compute reference lifetimes
4. **Move Analysis**: Track when values are moved

### Memory Layout Optimization

1. **Struct Packing**: Minimize padding in struct layouts
2. **Enum Optimization**: Use discriminant optimization for enums
3. **Zero-Sized Types**: Optimize away zero-sized types
4. **Alignment**: Respect platform alignment requirements

### Runtime Support

Minimal runtime support is required:

1. **Stack Overflow Detection**: Prevent stack overflow
2. **Bounds Checking**: Array and slice bounds checking
3. **Panic Handling**: Graceful error handling for runtime errors
4. **Memory Allocation**: Interface to system allocator (for heap types)

## Testing and Verification

### Property-Based Testing

Memory safety properties are verified through property-based testing:

```rust
// Property: No use-after-move
proptest!(|(program in arbitrary_program_with_moves())| {
    let result = compile_and_check_memory_safety(&program);
    prop_assert!(result.is_ok() || result.is_compile_error());
    prop_assert!(!result.is_runtime_memory_error());
});
```

### Static Analysis

The compiler performs static analysis to verify:

1. **Ownership Transfer**: All moves are valid
2. **Borrow Validity**: All borrows respect aliasing rules
3. **Lifetime Correctness**: All references are valid when used
4. **Memory Leak Prevention**: All allocated memory is eventually freed

### Runtime Verification

In debug mode, additional runtime checks verify:

1. **Double-Free Detection**: Prevent freeing the same memory twice
2. **Use-After-Free Detection**: Detect access to freed memory
3. **Buffer Overflow Detection**: Detect out-of-bounds access
4. **Memory Leak Detection**: Track unfreed allocations

## Compatibility and Migration

### Version Compatibility

- Memory model changes require major version bump
- Layout changes are breaking changes
- Safety improvements are non-breaking
- Performance optimizations preserve semantics

### Interoperability

- C FFI requires unsafe blocks for memory management
- WebAssembly linear memory model compatibility
- Platform-specific optimizations where safe
- Standard library provides safe abstractions

---

**This memory model specification is the authoritative definition of Ovie's memory management and ownership system and must be implemented exactly as specified.**