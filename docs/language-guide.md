# Ovie Language Guide

A complete reference for the Ovie programming language, v2.3.

## Table of Contents

1. [Variables and Mutability](#variables-and-mutability)
2. [Types](#types)
3. [Keywords](#keywords)
4. [Functions](#functions)
5. [Control Flow](#control-flow)
6. [Data Structures](#data-structures)
7. [Pattern Matching](#pattern-matching)
8. [Error Handling](#error-handling)
9. [Module System](#module-system)
10. [Built-in Functions](#built-in-functions)

---

## Variables and Mutability

Ovie has two declaration keywords:

### `oya` — immutable declaration

Use `oya` when a value never changes. This is the default choice.

```ovie
oya name    = "Ovie"
oya version = 2.3
oya active  = true
oya PI      = 3.14159
```

`oya` values cannot be reassigned. The compiler will stop you if you try.

### `mut` — mutable declaration

Use `mut` when you need to change a value after declaration.

```ovie
mut counter = 0
counter = counter + 1

mut score = 0
score = score + 10
```

### Rule of thumb

> Start with `oya`. If you need to reassign it, change to `mut`.

```ovie
// Good: config stays fixed
oya APP_NAME = "My App"
oya MAX_RETRIES = 3

// Good: loop state changes
mut attempts = 0
while attempts < MAX_RETRIES {
    attempts = attempts + 1
}
```

---

## Types

| Type | Description | Example |
|------|-------------|---------|
| `Number` | Integer or decimal | `42`, `3.14` |
| `String` | Text | `"Hello"` |
| `Boolean` | True or false | `true`, `false` |
| `Array` | Ordered list | `[1, 2, 3]` |

Type annotations are optional — Ovie infers types:

```ovie
oya n = 42           // Number
oya s = "hello"      // String
oya b = true         // Boolean
oya arr = [1, 2, 3]  // Array
```

Explicit annotations:

```ovie
oya n: Number  = 42
oya s: String  = "hello"
mut b: Boolean = false
```

---

## Keywords

Ovie's complete keyword set:

| Keyword | Purpose |
|---------|---------|
| `oya` | Immutable variable declaration |
| `mut` | Mutable variable declaration |
| `fn` | Function definition |
| `return` | Return a value from a function |
| `if` | Conditional branch |
| `else` | Alternative branch |
| `for` | Iterate over a range or array |
| `while` | Conditional loop |
| `break` | Exit a loop |
| `continue` | Skip to next loop iteration |
| `match` | Pattern matching |
| `struct` | Define a data structure |
| `enum` | Define an enumeration |
| `const` | Compile-time constant |
| `seeAm` | Print a value to the terminal |
| `use` | Import from standard library |
| `import` | Import from another file |
| `export` | Export for use by other files |
| `true` | Boolean true |
| `false` | Boolean false |
| `unsafe` | Mark an unsafe block |

---

## Functions

```ovie
fn add(a: Number, b: Number) -> Number {
    return a + b
}

seeAm add(3, 4)   // 7
```

No return type (void):

```ovie
fn greet(name: String) {
    seeAm "Hello, " + name
}
```

Using `oya` inside a function:

```ovie
fn tax(price: Number) -> Number {
    oya RATE = 0.075
    return price * RATE
}
```

Recursive functions:

```ovie
fn factorial(n: Number) -> Number {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

seeAm factorial(5)  // 120
```

---

## Control Flow

### if / else if / else

```ovie
mut x = 42

if x > 100 {
    seeAm "big"
} else if x > 10 {
    seeAm "medium"
} else {
    seeAm "small"
}
```

### while

```ovie
mut i = 0
while i < 5 {
    seeAm i
    i = i + 1
}
```

### for

```ovie
for i in 0..5 {
    seeAm i    // 0 1 2 3 4
}
```

### break / continue

```ovie
mut n = 0
while true {
    n = n + 1
    if n == 3 { continue }
    if n == 6 { break }
    seeAm n
}
```

---

## Data Structures

### struct

```ovie
struct Point {
    x: Number,
    y: Number,
}

mut p = Point { x: 10.0, y: 20.0 }
seeAm p.x   // 10
seeAm p.y   // 20
```

### enum

```ovie
enum Status { Active, Inactive, Pending }

mut s = Status.Active
seeAm s  // Active
```

---

## Pattern Matching

```ovie
enum Color { Red, Green, Blue }
mut c = Color.Green

match c {
    Color.Red   => seeAm "red"
    Color.Green => seeAm "green"
    Color.Blue  => seeAm "blue"
}
```

---

## Error Handling

```ovie
fn divide(a: Number, b: Number) -> String {
    if b == 0 {
        return "Error: division by zero"
    }
    return number_to_string(a / b)
}

seeAm divide(10, 2)   // 5
seeAm divide(10, 0)   // Error: division by zero
```

---

## Module System

```ovie
// Import from stdlib
use std::math::{sqrt}
use std::io::{println}

// Import from another file
import "./utils.ov"

// Export a function
export fn greet(name: String) -> String {
    return "Hello, " + name
}
```

---

## Built-in Functions

### String

| Function | Description |
|----------|-------------|
| `string_length(s)` | Length of string |
| `string_trim(s)` | Strip whitespace |
| `string_split(s, sep)` | Split into array |
| `string_contains(s, sub)` | Check substring |
| `string_starts_with(s, pre)` | Check prefix |
| `string_replace(s, from, to)` | Replace substring |
| `string_to_lowercase(s)` | Lowercase |
| `string_to_uppercase(s)` | Uppercase |
| `string_find(s, sub)` | Find index of sub |
| `string_char_at(s, i)` | Character at index |
| `string_substring(s, start, end)` | Slice |

### Number / Conversion

| Function | Description |
|----------|-------------|
| `number_to_string(n)` | 42 → "42" |
| `string_to_number(s)` | "42" → 42 |

### Array

| Function | Description |
|----------|-------------|
| `array_length(arr)` | Number of items |
| `array_get(arr, i)` | Item at index i |
| `array_push(arr, val)` | Append item |

### I/O

| Function | Description |
|----------|-------------|
| `seeAm value` | Print with newline |
| `print(text)` | Print without newline |
| `read_line()` | Read a line from stdin |
## oviev - Self-Hosted Compiler

See [oviev/README.md](../oviev/README.md).
