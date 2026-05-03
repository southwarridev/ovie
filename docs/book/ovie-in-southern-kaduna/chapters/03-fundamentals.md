# Chapter 3: Language Fundamentals

## Variables

Immutable by default:

```ovie
name = "Shedrack"
age = 25
```

Mutable with `mut`:

```ovie
mut counter = 0
counter = counter + 1
```

## Types

Ovie infers types. The core types:

- `Number` — integers and floats
- `String` — text
- `Boolean` — `true` or `false`
- `Array` — ordered collection

```ovie
x = 42
pi = 3.14
greeting = "Hello"
active = true
items = [1, 2, 3]
```

## Functions

```ovie
fn add(a, b) {
    return a + b
}

mut result = add(10, 20)
seeAm result
```

With type annotations:

```ovie
fn divide(a: Number, b: Number) -> Number {
    if b == 0 {
        return 0
    }
    return a / b
}
```

## Control Flow

```ovie
if counter < 10 {
    seeAm "small"
} else {
    seeAm "big"
}
```

## Loops

```ovie
// For loop with range
for i in 0..5 {
    seeAm i
}

// While loop
mut n = 0
while n < 3 {
    n = n + 1
    seeAm n
}
```

## Structs

```ovie
struct Person {
    name: String,
    age: Number,
}

mut p = Person {
    name: "Amina",
    age: 28,
}

seeAm p.name
```

## Enums

```ovie
enum Direction {
    North,
    South,
    East,
    West,
}

mut dir = Direction.North
```

## Error Handling

Use `Result` from the standard library:

```ovie
use std::core::{Result}

fn safe_divide(a: Number, b: Number) -> Result {
    if b == 0 {
        return Result.Err("Division by zero")
    }
    return Result.Ok(a / b)
}

mut r = safe_divide(10, 2)
if r.is_ok() {
    seeAm r.unwrap()
}
```

## Output

```ovie
seeAm "Hello, World!"
seeAm 42
seeAm "Result: " + number_to_string(result)
```

## Comments

```ovie
// Single line comment

/// Doc comment — used for documentation generation
fn my_function() {
    // ...
}
```

## Unsafe Blocks

For low-level operations that bypass safety checks:

```ovie
unsafe {
    // Direct memory operations here
}
```

Use sparingly. Aproko will flag unsafe blocks in its analysis.
