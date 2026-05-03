# Chapter 4: Module System Deep Dive

## What is a Module?

A module is a single `.ov` file that can export definitions and import from other modules. Modules let you organize code, control what's public, and reuse functionality across projects.

## Basic Module Structure

A module that exports a function:

```ovie
/// Adds two numbers together
///
/// # Parameters
/// - a: First number
/// - b: Second number
///
/// # Returns
/// Sum of a and b
///
/// # Examples
/// ```ovie
/// use math::add;
/// mut result = add(3, 4);
/// seeAm result
/// ```
export fn add(a: Number, b: Number) -> Number {
    return a + b
}
```

Private functions (no `export`) are only accessible within the module:

```ovie
fn internal_helper(x: Number) -> Number {
    return x * 2
}

export fn double(x: Number) -> Number {
    return internal_helper(x)
}
```

## Importing Modules

Import all exports from a module:

```ovie
use std::math;

mut result = math::sqrt(16.0)
seeAm result
```

Import specific symbols:

```ovie
use std::math::{sqrt, abs, floor}

mut r = sqrt(25.0)
mut a = abs(-10)
```

Import with alias:

```ovie
use std::math as m

mut r = m::sqrt(9.0)
```

Relative imports:

```ovie
import "./utils.ov"
import "../shared/helpers.ov"
```

## Standard Library Organization

The standard library uses `std::` namespace:

```ovie
use std::core::{Result, Option, Vec, HashMap}
use std::io::{println, read_line}
use std::fs::{read_file, write_file}
use std::math::{sqrt, pow, abs}
use std::time::{now, duration_ms}
use std::env::{get_var, args}
use std::cli::{parse_args, print_usage}
use std::log::{info, warn, error}
use std::testing::{assert_eq, assert_true}
```

## Third-Party Packages

Add a dependency to `ovie.toml`:

```toml
[dependencies]
oba = { path = "./oba" }
```

Then import:

```ovie
use oba::qubit::{qubit_new_zero}
use oba::gates::{gate_h}
use oba::io::{demand}

mut q = qubit_new_zero()
q = gate_h(q)
mut result = demand(q)
seeAm result
```

## Creating Your Own Package

Initialize a new package:

```bash
ovie init my-library
cd my-library
```

This creates:

```
my-library/
├── ovie.toml
└── src/
    └── main.ov
```

Edit `ovie.toml`:

```toml
[package]
name = "my-library"
version = "1.0.0"
authors = ["Your Name"]
description = "A useful Ovie library"

[dependencies]
```

Create your module at `src/mod.ov`:

```ovie
/// My Library
///
/// Provides useful utilities for Ovie programs.

export fn greet(name: String) -> String {
    return "Hello, " + name + "!"
}
```

## Dependency Management

Install dependencies:

```bash
ovie install
```

Add a new dependency:

```bash
ovie add some-package
```

This updates `ovie.toml` and generates `ovie.lock` for reproducible builds.

## Circular Dependencies

Ovie detects circular dependencies at compile time:

```
Error: Circular dependency detected
  module_a → module_b → module_c → module_a

Suggestion: Extract shared code into a new module that both can import.
```

To fix: move shared code to a third module that both `module_a` and `module_c` import.

## Namespace Management

When two modules export the same name, use explicit qualification:

```ovie
use std::math as std_math
use my_math as my_math

mut r1 = std_math::sqrt(4.0)
mut r2 = my_math::sqrt(4.0)
```

## Module Caching

Ovie caches compiled modules in `.ovie/cache/`. On subsequent builds, unchanged modules are loaded from cache — making incremental compilation fast.

Clear the cache:

```bash
ovie cache clear
```

## Best Practices

- One concern per module — keep modules focused
- Export only what consumers need
- Document all exported functions (required by the compiler)
- Use `std::` modules before writing your own utilities
- Pin dependency versions in `ovie.toml` for reproducible builds
