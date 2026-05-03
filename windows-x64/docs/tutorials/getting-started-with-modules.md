# Getting Started with Modules

This tutorial walks you through creating your first modular Ovie project from scratch.

## Prerequisites

- Ovie v2.3.0 installed (`ovie --version`)
- Basic familiarity with Ovie syntax (see [Language Guide](../language-guide.md))

---

## Step 1: Create a New Project

```bash
ovie init greeter
cd greeter
```

This creates:
```
greeter/
├── src/
│   └── main.ov
└── ovie.toml
```

---

## Step 2: Create a Module

Create `src/greet.ov` — a module that exports a greeting function:

```ovie
/// Greet a person by name
/// # Parameters
/// - name: The person's name
/// # Returns
/// A greeting string
/// # Examples
/// ```ovie
/// mut msg = make_greeting("Shedrack")
/// // "Hello, Shedrack! Welcome to Ovie."
/// ```
export fn make_greeting(name: String) -> String {
    return "Hello, " + name + "! Welcome to Ovie."
}

/// Greet multiple people
/// # Parameters
/// - names: Array of names
/// # Returns
/// Array of greeting strings
/// # Examples
/// ```ovie
/// mut msgs = greet_all(["Alice", "Bob"])
/// ```
export fn greet_all(names: Array) -> Array {
    mut greetings = []
    mut i = 0
    while i < array_length(names) {
        greetings = array_push(greetings, make_greeting(array_get(names, i)))
        i = i + 1
    }
    return greetings
}
```

Key points:
- `export fn` makes a function available to other modules
- Doc comments (`///`) are required on all exported functions
- Non-exported functions are private to the module

---

## Step 3: Import and Use the Module

Edit `src/main.ov`:

```ovie
use ./greet::{make_greeting, greet_all}

fn main() {
    // Use a single greeting
    mut msg = make_greeting("Shedrack")
    seeAm(msg)

    // Greet multiple people
    mut team = ["Alice", "Bob", "Charlie"]
    mut all_greetings = greet_all(team)

    mut i = 0
    while i < array_length(all_greetings) {
        seeAm(array_get(all_greetings, i))
        i = i + 1
    }
}

main()
```

Import syntax:
- `use ./greet::{fn1, fn2}` — import specific symbols from a relative path
- `use std::math::{sqrt}` — import from the standard library
- `use ./greet` — import all exports (use sparingly)

---

## Step 4: Run the Project

```bash
ovie run src/main.ov
```

Output:
```
Hello, Shedrack! Welcome to Ovie.
Hello, Alice! Welcome to Ovie.
Hello, Bob! Welcome to Ovie.
Hello, Charlie! Welcome to Ovie.
```

---

## Step 5: Check Documentation

Verify your doc comments are complete:

```bash
ovie doc check
```

Generate HTML documentation:

```bash
ovie doc
```

Open `docs/output/index.html` in a browser to see the generated docs.

---

## What You Learned

- How to create a module with `export fn`
- How to import specific symbols with `use ./path::{symbol}`
- How to write required doc comments on exported functions
- How to run a modular project

## Next Steps

- [Creating Packages](creating-packages.md) — package your module for reuse
- [Using Third-Party Packages](using-third-party-packages.md) — add external dependencies
- [Module System API Reference](../module-system-api.md) — complete API docs
