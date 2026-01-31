<div align="center">
  <img src="../ovie.png" alt="Ovie Programming Language" width="120" height="120">
  
  # Ovie Language Guide
  
  ### ✅ **SELF-HOSTED PROGRAMMING LANGUAGE**
</div>

This comprehensive guide covers all aspects of the Ovie Programming Language, from basic syntax to advanced features.

**🎉 Status: Complete language reference for the self-hosted Ovie compiler!**

## Table of Contents

1. [Language Philosophy](#language-philosophy)
2. [Core Keywords](#core-keywords)
3. [Basic Syntax](#basic-syntax)
4. [Data Types](#data-types)
5. [Variables and Mutability](#variables-and-mutability)
6. [Functions](#functions)
7. [Control Flow](#control-flow)
8. [Data Structures](#data-structures)
9. [Error Handling](#error-handling)
10. [Memory Safety](#memory-safety)
11. [Advanced Features](#advanced-features)

## Language Philosophy

Ovie is designed around four core principles:

1. **Accessibility**: Code should be readable by both programmers and non-programmers
2. **Safety**: The language should prevent common programming errors
3. **Determinism**: Builds should be reproducible and predictable
4. **✅ Self-Hosting**: The language can compile itself (ACHIEVED!)

### Natural Language Patterns

Ovie uses pidgin English syntax to make code more accessible:

```ovie
// Traditional languages might use:
// printf("Hello");
// console.log("Hello");
// System.out.println("Hello");

// Ovie uses natural language:
seeAm "Hello"
```

## Core Keywords

Ovie has exactly **13 core keywords**:

| Keyword | Purpose | Example |
|---------|---------|---------|
| `fn` | Define functions | `fn greet() { ... }` |
| `mut` | Declare mutable variables | `mut count = 0` |
| `if` | Conditional execution | `if x > 0 { ... }` |
| `else` | Alternative condition | `if x > 0 { ... } else { ... }` |
| `for` | Loop over collections | `for item in list { ... }` |
| `while` | Conditional loops | `while count < 10 { ... }` |
| `struct` | Define data structures | `struct Person { ... }` |
| `enum` | Define enumerations | `enum Color { Red, Blue }` |
| `unsafe` | Mark unsafe operations | `unsafe { ... }` |
| `return` | Return from functions | `return value` |
| `true` | Boolean true value | `mut flag = true` |
| `false` | Boolean false value | `mut flag = false` |
| `seeAm` | Print/output expression | `seeAm "Hello World"` |

## Basic Syntax

### Comments

```ovie
// Single-line comment

/*
   Multi-line comment
   Can span multiple lines
*/
```

### Identifiers

Variable and function names must:
- Start with a letter or underscore
- Contain only letters, numbers, and underscores
- Use snake_case convention

```ovie
// Valid identifiers
mut user_name = "Alice"
mut _private_var = 42
mut count2 = 0

// Invalid identifiers
// mut 2count = 0      // Cannot start with number
// mut user-name = ""  // Hyphens not allowed
```

### Expressions and Statements

```ovie
// Expressions (return values)
5 + 3                    // Arithmetic expression
user_name == "Alice"     // Comparison expression
calculate_total()        // Function call expression

// Statements (perform actions)
mut x = 5               // Variable declaration
seeAm "Hello"           // Print statement
return result           // Return statement
```

## Data Types

### Primitive Types

```ovie
// Integers
mut small_num: u8 = 255        // 8-bit unsigned
mut medium_num: u16 = 65535    // 16-bit unsigned
mut large_num: u32 = 4294967295 // 32-bit unsigned
mut huge_num: u64 = 18446744073709551615 // 64-bit unsigned

mut signed_small: i8 = -128    // 8-bit signed
mut signed_medium: i16 = -32768 // 16-bit signed
mut signed_large: i32 = -2147483648 // 32-bit signed
mut signed_huge: i64 = -9223372036854775808 // 64-bit signed

// Floating point
mut decimal: f32 = 3.14        // 32-bit float
mut precise: f64 = 3.141592653589793 // 64-bit float

// Boolean
mut is_active: bool = true
mut is_complete: bool = false

// Strings
mut greeting: string = "Hello, World!"
mut empty: string = ""
```

### Type Inference

Ovie can often infer types automatically:

```ovie
// Explicit types
mut count: u32 = 0
mut name: string = "Alice"

// Inferred types (preferred)
mut count = 0u32        // Inferred as u32
mut name = "Alice"      // Inferred as string
mut pi = 3.14          // Inferred as f64
mut active = true      // Inferred as bool
```

## Variables and Mutability

### Immutable by Default

Variables are immutable by default. Use `mut` for mutable variables:

```ovie
// Immutable (cannot be changed)
name = "Alice"
age = 25

// Mutable (can be changed)
mut count = 0
mut status = "active"

// This would cause an error:
// name = "Bob"  // Error: cannot assign to immutable variable

// This is allowed:
count = count + 1  // OK: count is mutable
```

### Variable Shadowing

You can declare a new variable with the same name:

```ovie
mut value = "42"        // string
mut value = 42          // now it's a number
seeAm value            // prints: 42
```

## Functions

### Function Definition

```ovie
// Basic function
fn greet() {
    seeAm "Hello!"
}

// Function with parameters
fn greet_person(name: string) {
    seeAm "Hello, " + name + "!"
}

// Function with return value
fn add(x: u32, y: u32) -> u32 {
    return x + y
}

// Function with multiple parameters and return
fn calculate_area(width: f64, height: f64) -> f64 {
    return width * height
}
```

### Function Calls

```ovie
// Call functions
greet()
greet_person("Alice")

mut sum = add(5, 3)
mut area = calculate_area(10.0, 5.0)

seeAm "Sum: " + sum
seeAm "Area: " + area
```

### Early Return

```ovie
fn check_age(age: u32) -> string {
    if age < 18 {
        return "Minor"
    }
    
    if age >= 65 {
        return "Senior"
    }
    
    return "Adult"
}
```

## Control Flow

### Conditional Statements

```ovie
// Basic if statement
if age >= 18 {
    seeAm "You can vote"
}

// If-else
if temperature > 30 {
    seeAm "It's hot!"
} else {
    seeAm "It's not too hot"
}

// Multiple conditions
if score >= 90 {
    seeAm "Grade: A"
} else if score >= 80 {
    seeAm "Grade: B"
} else if score >= 70 {
    seeAm "Grade: C"
} else {
    seeAm "Grade: F"
}
```

### Loops

#### While Loops

```ovie
// Basic while loop
mut count = 0
while count < 5 {
    seeAm "Count: " + count
    count = count + 1
}

// Infinite loop (use with caution)
while true {
    // Do something
    // Make sure to have a break condition!
}
```

#### For Loops

```ovie
// For loop with range (conceptual - actual syntax may vary)
for i in 0..5 {
    seeAm "Number: " + i
}

// For loop with collection
mut names = ["Alice", "Bob", "Charlie"]
for name in names {
    seeAm "Hello, " + name
}
```

## Data Structures

### Structs

Structs group related data together:

```ovie
// Define a struct
struct Person {
    name: string,
    age: u32,
    email: string,
}

// Create struct instances
mut alice = Person {
    name: "Alice",
    age: 30,
    email: "alice@example.com",
}

// Access struct fields
seeAm alice.name
seeAm "Age: " + alice.age

// Modify mutable fields
alice.age = 31
```

### Nested Structs

```ovie
struct Address {
    street: string,
    city: string,
    zip_code: string,
}

struct Person {
    name: string,
    age: u32,
    address: Address,
}

mut person = Person {
    name: "Bob",
    age: 25,
    address: Address {
        street: "123 Main St",
        city: "Anytown",
        zip_code: "12345",
    },
}

seeAm person.address.city
```

### Enums

Enums define a type with a fixed set of possible values:

```ovie
// Simple enum
enum Color {
    Red,
    Green,
    Blue,
}

// Enum with data
enum Status {
    Active,
    Inactive,
    Pending(string),  // Holds a string value
    Error(u32),       // Holds an error code
}

// Using enums
mut current_color = Color.Red
mut user_status = Status.Pending("Waiting for approval")

// Pattern matching (conceptual)
match current_color {
    Color.Red => seeAm "Stop!",
    Color.Green => seeAm "Go!",
    Color.Blue => seeAm "Caution!",
}
```

## Error Handling

Ovie emphasizes safe error handling:

```ovie
// Functions that might fail should return result types
fn divide(x: f64, y: f64) -> Result<f64, string> {
    if y == 0.0 {
        return Error("Division by zero")
    }
    return Ok(x / y)
}

// Handle results safely
mut result = divide(10.0, 2.0)
match result {
    Ok(value) => seeAm "Result: " + value,
    Error(message) => seeAm "Error: " + message,
}
```

## Memory Safety

### Ownership Rules

Ovie enforces memory safety through ownership:

1. Each value has a single owner
2. When the owner goes out of scope, the value is cleaned up
3. Values can be borrowed temporarily

```ovie
// Ownership transfer
mut data = "Hello"
mut other_data = data  // Ownership transferred
// seeAm data          // Error: data no longer valid

// Borrowing (conceptual syntax)
fn print_message(message: &string) {
    seeAm message
}

mut greeting = "Hello, World!"
print_message(&greeting)  // Borrow greeting
seeAm greeting           // Still valid
```

### Unsafe Operations

Some operations require explicit `unsafe` blocks:

```ovie
// Unsafe operations must be explicitly marked
unsafe {
    // Direct memory access
    // Foreign function calls
    // Other potentially dangerous operations
}
```

## Advanced Features

### Generic Functions (Future Feature)

```ovie
// Generic function (planned for future versions)
fn swap<T>(mut a: T, mut b: T) {
    mut temp = a
    a = b
    b = temp
}
```

### Modules and Imports (Future Feature)

```ovie
// Import from other modules
use std.collections.List
use my_module.helper_functions

// Define modules
mod math_utils {
    fn add(x: u32, y: u32) -> u32 {
        return x + y
    }
}
```

## Best Practices

### Code Style

1. **Use descriptive names**:
   ```ovie
   // Good
   mut user_count = 0
   fn calculate_total_price() -> f64 { ... }
   
   // Avoid
   mut c = 0
   fn calc() -> f64 { ... }
   ```

2. **Keep functions small and focused**:
   ```ovie
   // Good: Single responsibility
   fn validate_email(email: string) -> bool { ... }
   fn send_email(to: string, subject: string, body: string) { ... }
   
   // Avoid: Multiple responsibilities
   fn validate_and_send_email(...) { ... }
   ```

3. **Use immutable variables when possible**:
   ```ovie
   // Prefer immutable
   name = "Alice"
   age = 25
   
   // Only use mut when necessary
   mut counter = 0
   ```

### Error Handling

1. **Handle errors explicitly**:
   ```ovie
   mut result = risky_operation()
   match result {
       Ok(value) => process_value(value),
       Error(err) => handle_error(err),
   }
   ```

2. **Provide meaningful error messages**:
   ```ovie
   if input.is_empty() {
       return Error("Input cannot be empty")
   }
   ```

### Performance

1. **Avoid unnecessary allocations**:
   ```ovie
   // Prefer borrowing over copying when possible
   fn process_data(data: &string) { ... }
   ```

2. **Use appropriate data types**:
   ```ovie
   // Use the smallest type that fits your needs
   mut small_counter: u8 = 0    // For values 0-255
   mut large_counter: u64 = 0   // For very large values
   ```

## Integration with Aproko

The Aproko assistant helps you write better Ovie code by:

1. **Auto-correcting typos**: `seeam` → `seeAm`
2. **Suggesting improvements**: Performance and style recommendations
3. **Catching errors early**: Logic and safety issues
4. **Providing explanations**: Clear error messages and suggestions

See the [Aproko Guide](aproko.md) for detailed configuration options.

## Next Steps

- **[CLI Reference](cli.md)**: Learn about Ovie's command-line tools
- **[Testing](testing.md)**: Write tests for your Ovie programs
- **[Project Structure](project-structure.md)**: Organize your Ovie projects
- **[Examples](examples.md)**: See real-world Ovie programs

---

*This guide covers Ovie Stage 0. Features marked as "Future Feature" will be available in later stages of development.*