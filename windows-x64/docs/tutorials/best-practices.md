# Module System Best Practices

Guidelines for writing well-structured, maintainable Ovie modules and packages.

---

## Module Organization

### One Responsibility Per Module

Each module should do one thing well. If a module is growing large, split it.

```
// Good: focused modules
src/
  parser.ov      // only parsing
  lexer.ov       // only lexing
  ast.ov         // only AST types

// Avoid: one giant module
src/
  everything.ov  // parser + lexer + AST + codegen
```

### Use `mod.ov` as the Public API Surface

The `mod.ov` file should only re-export — it should not contain implementation. This keeps the public API clear and separate from internals.

```ovie
// Good: mod.ov only re-exports
export use ./parser::{parse, ParseResult};
export use ./lexer::{tokenize, Token};

// Avoid: implementation in mod.ov
export fn parse(source: String) -> ParseResult {
    // 200 lines of implementation...
}
```

### Keep Internal Modules Private

Only export what consumers need. Internal helpers should not be exported.

```ovie
// Good: only export the public API
export fn process(data: Array) -> Result { ... }

// Internal helper — not exported
fn validate_item(item: String) -> bool { ... }
```

---

## Naming Conventions

### Module Paths

Use `snake_case` for module names and file names:

```
std/module/dependency_graph.ov   // good
std/module/DependencyGraph.ov    // avoid
std/module/dependency-graph.ov   // avoid
```

### Exported Symbols

- Functions: `snake_case` — `load_module`, `compute_hash`
- Types/Structs: `PascalCase` — `ModuleCache`, `DependencyGraph`
- Constants: `SCREAMING_SNAKE_CASE` — `MAX_CACHE_SIZE`, `DEFAULT_TIMEOUT`
- Enums: `PascalCase` variants — `ModuleError::NotFound`

### Package Names

Use `kebab-case` for package names in `ovie.toml`:

```toml
[package]
name = "my-math-utils"   # good
name = "myMathUtils"     # avoid
name = "my_math_utils"   # avoid
```

---

## Documentation Standards

### Every Exported Symbol Needs a Doc Comment

The compiler enforces this. Missing doc comments cause compilation errors.

```ovie
// Required structure for exported functions:
/// Brief one-line description
/// # Parameters
/// - param_name: Description
/// # Returns
/// Description of return value
/// # Examples
/// ```ovie
/// mut result = my_function(42)
/// ```
export fn my_function(x: Number) -> Number {
    return x * 2
}
```

### Write Runnable Examples

Examples in doc comments are validated by `ovie doc check`. Make sure they actually work.

```ovie
// Good: runnable example
/// # Examples
/// ```ovie
/// mut x = clamp(15, 0, 10)
/// // x == 10
/// ```

// Avoid: pseudocode that won't compile
/// # Examples
/// ```
/// clamp(value, min, max) -> clamped_value
/// ```
```

---

## Dependency Management

### Pin Versions in `ovie.lock`

Always commit `ovie.lock`. This ensures reproducible builds across machines and CI.

### Use Version Ranges Carefully

```toml
# Good: accept compatible minor versions
some-lib = { version = ">=1.2.0, <2.0.0" }

# Risky: accept any version (breaking changes possible)
some-lib = { version = ">=1.0.0" }

# Safe for internal tools: exact version
internal-tool = { version = "1.0.0" }
```

### Avoid Circular Dependencies

Circular dependencies (`A` imports `B` imports `A`) are detected and rejected at compile time. If you hit one:

1. Extract shared types into a third module `C` that both `A` and `B` import
2. Use dependency injection — pass the dependency as a parameter instead of importing it
3. Merge the two modules if they're truly inseparable

### Keep Dependency Trees Shallow

Deep dependency chains slow down compilation and make debugging harder. Prefer flat structures.

---

## Performance Tips

### Use Lazy Loading

Don't load modules you don't need. The module system loads lazily by default — modules are only parsed when their symbols are first accessed.

```ovie
// Good: only import what you use
use std::math::{sqrt}

// Avoid: importing everything when you only need one function
use std::math
```

### Avoid Importing in Loops

Import statements are resolved at module load time, not at runtime. But avoid patterns that cause repeated module lookups:

```ovie
// Good: import once at top of file
use std::math::{sqrt}

fn process_many(values: Array) -> Array {
    mut results = []
    mut i = 0
    while i < array_length(values) {
        results = array_push(results, sqrt(array_get(values, i)))
        i = i + 1
    }
    return results
}
```

### Keep Modules Focused for Better Caching

The cache invalidates a module when its source changes. Smaller, focused modules mean fewer cache invalidations when you make changes.

---

## Testing

### Co-locate Tests with Source

```
src/
  math.ov
  test_math.ov    // tests for math.ov
```

### Test the Public API, Not Internals

Write tests against exported functions. Internal implementation can change without breaking tests.

```ovie
use ./math::{add, subtract}

fn test_add() {
    assert(add(2, 3) == 5, "2 + 3 should be 5")
    assert(add(-1, 1) == 0, "negative + positive")
    assert(add(0, 0) == 0, "zero identity")
    seeAm("PASS: test_add")
}
```

### Run Tests Before Publishing

```bash
ovie test
ovie doc check
```

Both must pass before sharing your package.

---

## Summary Checklist

Before publishing or sharing a package:

- [ ] All exported functions have complete doc comments
- [ ] `ovie doc check` passes with no errors
- [ ] `ovie test` passes
- [ ] `ovie.lock` is committed
- [ ] Module names use `snake_case`
- [ ] `mod.ov` only re-exports, no implementation
- [ ] No circular dependencies
- [ ] Version constraints are appropriate
