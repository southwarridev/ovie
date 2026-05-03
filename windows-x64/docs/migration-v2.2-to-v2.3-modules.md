# Migration Guide: Ovie v2.2 → v2.3 Module System

## Overview

Ovie 2.3 introduces a first-class module system. Existing standalone `.ov` files
continue to work without changes (backward compatible), but you can opt in to the
new module system to get better code organisation, explicit imports/exports, and
dependency management.

---

## What Changed

| Feature | v2.2 (standalone) | v2.3 (modular) |
|---|---|---|
| Code organisation | Single file | Multiple files with `import`/`export` |
| Std library access | Implicit | Explicit `use std::math;` |
| Third-party packages | Not supported | `ovie.toml` manifest |
| Circular dependency detection | None | Compile-time error |
| Incremental compilation | None | SHA-256 cache |

---

## Backward Compatibility

**All existing `.ov` files compile unchanged.** Files with no `import` or `export`
statements are automatically detected as *standalone* (legacy) files and compiled
with the same semantics as v2.2.

You will see a deprecation warning:

```
Warning: 'examples/hello.ov' uses legacy standalone mode.
Consider adding explicit import/export statements.
```

To suppress warnings, pass `--no-legacy-warnings` to `oviec`.

---

## Step-by-Step Migration

### 1. Single-file programs (no change needed)

```ovie
// hello.ov — works exactly as before
fn main() {
    seeAm("Hello, world!");
}
```

### 2. Programs that use the standard library

**Before (v2.2):**
```ovie
use core::{Vec};

fn main() {
    let v = Vec.new();
}
```

**After (v2.3):**
```ovie
use std::core::{Vec};

fn main() {
    let v = Vec.new();
}
```

The `std::` prefix is now required for standard library modules.

### 3. Multi-file projects

**Before (v2.2):** No official way to split code across files.

**After (v2.3):** Create a `mod.ov` entry point and use `export`/`import`:

```
my_project/
├── ovie.toml
├── src/
│   ├── main.ov
│   ├── utils.ov
│   └── models/
│       └── mod.ov
```

`utils.ov`:
```ovie
/// Add two numbers
/// # Parameters
/// - `a`: First number
/// - `b`: Second number
/// # Returns
/// Sum of a and b
/// # Examples
/// ```
/// assert(add(2, 3) == 5);
/// ```
export fn add(a: Number, b: Number) -> Number {
    return a + b;
}
```

`main.ov`:
```ovie
use ./utils::{add};

fn main() {
    seeAm(add(2, 3));
}
```

### 4. Creating a package with ovie.toml

Run `ovie init` to create a new package:

```
ovie init my_package
```

This creates:
```toml
# ovie.toml
[package]
name = "my_package"
version = "0.1.0"
description = ""

[dependencies]
```

### 5. Using third-party packages

Add to `ovie.toml`:
```toml
[dependencies]
oba = { path = "../oba" }
```

Then in your code:
```ovie
use oba::circuit::{Circuit};
```

---

## Export Statement Syntax

```ovie
// Export a function
export fn my_function() -> Number { ... }

// Export a struct
export struct MyStruct { ... }

// Export a constant
export const MAX: Number = 100;

// Export an enum
export enum Status { Active, Inactive }

// Re-export from another module
export use ./utils::{add, subtract};
```

---

## Import Statement Syntax

```ovie
// Import all exports from a module
use std::math;

// Import specific symbols
use std::math::{sin, cos, PI};

// Import with alias
use std::math as math;

// Relative import
use ./utils::{add};
import "./relative/path.ov";
```

---

## Deprecation Timeline

| Version | Status |
|---|---|
| v2.3 | Standalone mode supported with warnings |
| v2.4 | Warnings become errors for new projects |
| v3.0 | Standalone mode removed |

---

## Automated Migration

Use `ovie migrate` to automatically add import/export statements:

```bash
ovie migrate src/my_file.ov
```

This will:
1. Analyse the file for used symbols
2. Add appropriate `use std::...` imports
3. Add `export` to public functions
4. Create `ovie.toml` if missing

---

## Getting Help

- Run `ovie help modules` for CLI reference
- See `docs/language-guide.md` for the full module system guide
- File issues at the project repository
