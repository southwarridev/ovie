# Ovie Programming Language - Formal Type System Specification

**Version:** 2.1.0  
**Status:** FROZEN (Stage 2)  
**Last Updated:** 2024-01-28  

## Overview

This document defines the complete formal type system for the Ovie Programming Language. The type system is **FROZEN** as part of Stage 2 development - no type system changes are permitted without formal RFC process.

The Ovie type system is designed to be:
- **Sound**: Well-typed programs cannot exhibit undefined behavior
- **Decidable**: Type checking always terminates with a definitive result
- **Predictable**: Type inference follows clear, documented rules
- **Deterministic**: Identical programs produce identical type checking results

## Type System Foundations

### Core Principles

1. **Static Typing**: All types are determined at compile-time
2. **Type Safety**: No undefined behavior in well-typed programs
3. **Explicit Mutability**: Mutability must be declared explicitly
4. **No Null**: No null pointer exceptions (use Option<T> instead)
5. **Memory Safety**: Ownership and borrowing prevent memory errors

### Type Categories

```ebnf
type = primitive_type
     | compound_type
     | function_type
     | generic_type
     ;

primitive_type = "Number" | "String" | "Boolean" | "Unit" ;

compound_type = struct_type | enum_type | array_type ;

function_type = "fn" "(" parameter_types? ")" "->" return_type ;

generic_type = identifier "<" type_list ">" ;
```

## Primitive Types

### Number Type
- **Representation**: 64-bit floating point (IEEE 754)
- **Range**: ±1.7976931348623157E+308
- **Special Values**: +∞, -∞, NaN (deterministic handling)
- **Operations**: +, -, *, /, %, ==, !=, <, <=, >, >=

**Type Rules:**
```
Γ ⊢ n : Number    (where n is a numeric literal)
Γ ⊢ e₁ : Number   Γ ⊢ e₂ : Number
─────────────────────────────────────
Γ ⊢ e₁ op e₂ : Number    (where op ∈ {+, -, *, /, %})
```

### String Type
- **Representation**: UTF-8 encoded byte sequence
- **Immutable**: Strings cannot be modified after creation
- **Operations**: +, ==, !=, indexing (read-only)

**Type Rules:**
```
Γ ⊢ s : String    (where s is a string literal)
Γ ⊢ e₁ : String   Γ ⊢ e₂ : String
─────────────────────────────────────
Γ ⊢ e₁ + e₂ : String    (string concatenation)
```

### Boolean Type
- **Values**: `true`, `false`
- **Operations**: &&, ||, !, ==, !=

**Type Rules:**
```
Γ ⊢ true : Boolean    Γ ⊢ false : Boolean
Γ ⊢ e₁ : Boolean   Γ ⊢ e₂ : Boolean
─────────────────────────────────────
Γ ⊢ e₁ && e₂ : Boolean
Γ ⊢ e₁ || e₂ : Boolean
```

### Unit Type
- **Representation**: Empty tuple `()`
- **Usage**: Functions with no return value, empty expressions
- **Size**: Zero bytes

## Compound Types

### Struct Types

**Definition Syntax:**
```ovie
struct TypeName {
    field1: Type1,
    field2: Type2,
    // ...
}
```

**Type Rules:**
```
Γ ⊢ struct S { f₁: T₁, ..., fₙ: Tₙ }
─────────────────────────────────────
Γ, S: struct{f₁: T₁, ..., fₙ: Tₙ} ⊢ S : Type

Γ ⊢ e₁ : T₁   ...   Γ ⊢ eₙ : Tₙ
─────────────────────────────────────
Γ ⊢ S { f₁: e₁, ..., fₙ: eₙ } : S

Γ ⊢ e : S   S has field f : T
─────────────────────────────
Γ ⊢ e.f : T
```

**Structural Properties:**
- Fields are ordered as declared
- All fields must be initialized during construction
- Field access is compile-time verified
- No inheritance or subtyping

### Enum Types

**Definition Syntax:**
```ovie
enum TypeName {
    Variant1,
    Variant2(Type),
    Variant3 { field: Type },
}
```

**Type Rules:**
```
Γ ⊢ enum E { V₁, ..., Vₙ }
─────────────────────────────
Γ, E: enum{V₁, ..., Vₙ} ⊢ E : Type

Γ ⊢ E::V : E    (for unit variants)

Γ ⊢ e : T
─────────────────────────────
Γ ⊢ E::V(e) : E    (for tuple variants)
```

**Pattern Matching (Future):**
- Exhaustiveness checking required
- Unreachable pattern detection
- Variable binding in patterns

## Function Types

### Function Signatures

**Syntax:**
```ovie
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // body
}
```

**Type Rules:**
```
Γ, x₁: T₁, ..., xₙ: Tₙ ⊢ body : R
─────────────────────────────────────
Γ ⊢ fn f(x₁: T₁, ..., xₙ: Tₙ) -> R { body } : fn(T₁, ..., Tₙ) -> R

Γ ⊢ f : fn(T₁, ..., Tₙ) -> R   Γ ⊢ e₁ : T₁   ...   Γ ⊢ eₙ : Tₙ
─────────────────────────────────────────────────────────────────
Γ ⊢ f(e₁, ..., eₙ) : R
```

### Function Properties

- **First-class values**: Functions can be stored in variables, passed as arguments
- **Lexical scoping**: Functions capture their lexical environment
- **No overloading**: Each function name has exactly one signature
- **Tail call optimization**: Guaranteed for self-recursive tail calls

## Type Inference

### Hindley-Milner Algorithm (Simplified)

The Ovie type system uses a simplified version of Hindley-Milner type inference:

1. **Constraint Generation**: Generate type constraints from expressions
2. **Constraint Solving**: Unify constraints to find most general types
3. **Type Substitution**: Apply solutions to infer concrete types

**Inference Rules:**

**Variable:**
```
x : T ∈ Γ
─────────
Γ ⊢ x : T
```

**Application:**
```
Γ ⊢ f : T₁ -> T₂   Γ ⊢ e : T₁
─────────────────────────────
Γ ⊢ f(e) : T₂
```

**Let Binding:**
```
Γ ⊢ e₁ : T₁   Γ, x : T₁ ⊢ e₂ : T₂
─────────────────────────────────────
Γ ⊢ let x = e₁ in e₂ : T₂
```

### Type Inference Limitations

- **No higher-rank polymorphism**: All type variables are prenex quantified
- **No type classes**: No ad-hoc polymorphism (yet)
- **Explicit annotations required**: For recursive functions and complex cases
- **Local inference only**: No global type inference across modules

## Mutability System

### Mutability Rules

1. **Immutable by default**: All bindings are immutable unless marked `mut`
2. **Explicit mutability**: Must use `mut` keyword for mutable bindings
3. **Mutability propagation**: Mutable access requires mutable path
4. **No aliasing**: Cannot have mutable and immutable references simultaneously

**Mutability Type Rules:**
```
Γ ⊢ e : T
─────────────────────────
Γ ⊢ mut x = e : mut T

Γ ⊢ x : mut T
─────────────
Γ ⊢ x : T    (coercion from mutable to immutable)

Γ ⊢ e : mut T
─────────────
Γ ⊢ e = new_value : Unit    (assignment)
```

### Borrowing Rules (Simplified)

- **Shared borrowing**: Multiple immutable references allowed
- **Exclusive borrowing**: Only one mutable reference allowed
- **Lifetime tracking**: References cannot outlive their referents
- **No dangling pointers**: Compile-time prevention of use-after-free

## Error Types and Handling

### Result Type (Future)
```ovie
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Option Type (Future)
```ovie
enum Option<T> {
    Some(T),
    None,
}
```

### Error Propagation
- **Explicit error handling**: No exceptions, use Result types
- **Error propagation operator**: `?` for convenient error handling
- **Compile-time verification**: All error cases must be handled

## Type Checking Algorithm

### Type Checking Process

1. **Lexical Analysis**: Tokenize source code
2. **Syntax Analysis**: Build AST with type annotations
3. **Name Resolution**: Resolve all identifiers to declarations
4. **Type Inference**: Infer types for all expressions
5. **Type Checking**: Verify type consistency and safety
6. **Error Reporting**: Generate detailed error messages

### Type Environment

```rust
pub struct TypeEnvironment {
    bindings: HashMap<Symbol, Type>,
    functions: HashMap<Symbol, FunctionSignature>,
    types: HashMap<Symbol, TypeDefinition>,
    scopes: Vec<Scope>,
}
```

### Type Checking Rules Implementation

**Expression Type Checking:**
```rust
fn check_expression(expr: &Expression, env: &TypeEnvironment) -> Result<Type, TypeError> {
    match expr {
        Expression::Literal(lit) => check_literal(lit),
        Expression::Variable(name) => env.lookup_variable(name),
        Expression::BinaryOp { left, op, right } => {
            let left_type = check_expression(left, env)?;
            let right_type = check_expression(right, env)?;
            check_binary_op(op, left_type, right_type)
        }
        Expression::FunctionCall { name, args } => {
            let func_sig = env.lookup_function(name)?;
            let arg_types: Result<Vec<_>, _> = args.iter()
                .map(|arg| check_expression(arg, env))
                .collect();
            check_function_call(func_sig, arg_types?)
        }
        // ... other expression types
    }
}
```

## Type System Extensions (Future)

### Planned Extensions

1. **Generics**: Parametric polymorphism with type parameters
2. **Traits**: Interface-based polymorphism and type classes
3. **Associated Types**: Types associated with traits
4. **Higher-Kinded Types**: Types that take type parameters
5. **Dependent Types**: Types that depend on values (limited)

### Generic Types (Post-Stage 2)
```ovie
struct List<T> {
    head: Option<T>,
    tail: Option<Box<List<T>>>,
}

fn map<T, U>(list: List<T>, f: fn(T) -> U) -> List<U> {
    // implementation
}
```

### Trait System (Post-Stage 2)
```ovie
trait Display {
    fn display(self) -> String;
}

impl Display for Number {
    fn display(self) -> String {
        // convert number to string
    }
}
```

## Type System Guarantees

### Safety Guarantees

1. **Memory Safety**: No buffer overflows, use-after-free, or double-free
2. **Type Safety**: No type confusion or invalid casts
3. **Thread Safety**: Data races prevented by ownership system
4. **Null Safety**: No null pointer dereferences

### Performance Guarantees

1. **Zero-cost abstractions**: High-level constructs compile to efficient code
2. **Predictable performance**: No hidden allocations or garbage collection
3. **Compile-time optimization**: Type information enables aggressive optimization
4. **Stack allocation**: Most values allocated on stack, not heap

### Determinism Guarantees

1. **Deterministic type checking**: Same program always gets same types
2. **Reproducible builds**: Type information doesn't affect output determinism
3. **Platform independence**: Type system behavior identical across platforms
4. **Version stability**: Type checking behavior preserved across compiler versions

## Error Messages and Diagnostics

### Error Message Format

```
error[E0308]: mismatched types
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
```

### Diagnostic Categories

1. **Type Mismatch**: Expected type A, found type B
2. **Undefined Variable**: Variable not found in scope
3. **Undefined Function**: Function not found or wrong arity
4. **Field Access Error**: Field doesn't exist on type
5. **Mutability Error**: Attempting to mutate immutable value
6. **Ownership Error**: Borrowing or lifetime violation

### Suggestion System

- **Automatic fixes**: Simple type coercions and imports
- **Contextual help**: Explain why error occurred
- **Alternative approaches**: Suggest different ways to achieve goal
- **Learning resources**: Link to documentation for complex topics

## Compliance and Testing

### Type System Testing

1. **Positive Tests**: Valid programs that should type check
2. **Negative Tests**: Invalid programs that should be rejected
3. **Inference Tests**: Programs where types should be inferred correctly
4. **Error Message Tests**: Verify quality of error messages
5. **Performance Tests**: Type checking should complete in reasonable time

### Specification Compliance

- All type rules must be implemented exactly as specified
- Error messages must follow the documented format
- Type inference must produce the most general types
- Mutability and ownership rules must be enforced consistently

### Property-Based Testing

```rust
#[test]
fn type_soundness_property() {
    // Property: If a program type checks, it should not crash at runtime
    proptest!(|(program in arbitrary_well_typed_program())| {
        let type_result = type_check(&program);
        if type_result.is_ok() {
            let runtime_result = execute(&program);
            prop_assert!(runtime_result.is_ok());
        }
    });
}
```

## Migration and Compatibility

### Version Compatibility

- Type system changes require major version bump
- Backward compatibility maintained within major versions
- Migration guides provided for breaking changes
- Deprecation warnings for removed features

### Tooling Integration

- IDE support for type information display
- Compiler API for external tools
- JSON output format for machine consumption
- Integration with documentation generators

---

**This type system specification is the authoritative definition of Ovie's type system and must be implemented exactly as specified.**