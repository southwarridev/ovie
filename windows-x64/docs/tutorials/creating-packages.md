# Creating Packages

This tutorial shows you how to create a reusable Ovie package that others can depend on.

## What is a Package?

A package is a directory with an `ovie.toml` manifest and a `src/mod.ov` entry point. Other projects add it as a dependency and import from it using `use package_name::symbol`.

---

## Step 1: Initialize the Package

```bash
ovie init my-math-utils
cd my-math-utils
```

Edit `ovie.toml` to describe your package:

```toml
[package]
name = "my-math-utils"
version = "0.1.0"
description = "Useful math utilities for Ovie"
authors = ["Your Name <you@example.com>"]
license = "MIT"

[dependencies]
# no external dependencies yet

[build]
entry = "src/mod.ov"
```

---

## Step 2: Write the Public API

Create `src/mod.ov` — the package entry point that re-exports your public API:

```ovie
/// my-math-utils — Useful math utilities
///
/// # Examples
/// ```ovie
/// use my-math-utils::{clamp, lerp, map_range}
/// ```

export use ./clamp::{clamp, clamp_f};
export use ./interpolate::{lerp, lerp_f, map_range};
export use ./stats::{mean, median, variance};
```

---

## Step 3: Implement the Modules

Create `src/clamp.ov`:

```ovie
/// Clamp an integer value between min and max
/// # Parameters
/// - value: The value to clamp
/// - min: Minimum allowed value
/// - max: Maximum allowed value
/// # Returns
/// value clamped to [min, max]
/// # Examples
/// ```ovie
/// mut x = clamp(15, 0, 10)  // 10
/// mut y = clamp(-5, 0, 10)  // 0
/// mut z = clamp(5, 0, 10)   // 5
/// ```
export fn clamp(value: Number, min: Number, max: Number) -> Number {
    if value < min { return min }
    if value > max { return max }
    return value
}

/// Clamp a float value between min and max
/// # Parameters
/// - value: The float value to clamp
/// - min: Minimum allowed value
/// - max: Maximum allowed value
/// # Returns
/// value clamped to [min, max]
/// # Examples
/// ```ovie
/// mut x = clamp_f(1.5, 0.0, 1.0)  // 1.0
/// ```
export fn clamp_f(value: Float, min: Float, max: Float) -> Float {
    if value < min { return min }
    if value > max { return max }
    return value
}
```

Create `src/interpolate.ov`:

```ovie
/// Linear interpolation between two values
/// # Parameters
/// - a: Start value
/// - b: End value
/// - t: Interpolation factor (0.0 = a, 1.0 = b)
/// # Returns
/// Interpolated value
/// # Examples
/// ```ovie
/// mut mid = lerp(0.0, 10.0, 0.5)  // 5.0
/// ```
export fn lerp(a: Float, b: Float, t: Float) -> Float {
    return a + (b - a) * t
}

/// Linear interpolation for integers
/// # Parameters
/// - a: Start value
/// - b: End value
/// - t: Interpolation factor (0.0 to 1.0)
/// # Returns
/// Interpolated integer value
/// # Examples
/// ```ovie
/// mut mid = lerp_f(0, 100, 0.5)  // 50
/// ```
export fn lerp_f(a: Number, b: Number, t: Float) -> Number {
    return a + ((b - a) as Float * t) as Number
}

/// Map a value from one range to another
/// # Parameters
/// - value: Input value
/// - in_min: Input range minimum
/// - in_max: Input range maximum
/// - out_min: Output range minimum
/// - out_max: Output range maximum
/// # Returns
/// Value mapped to output range
/// # Examples
/// ```ovie
/// mut mapped = map_range(5.0, 0.0, 10.0, 0.0, 100.0)  // 50.0
/// ```
export fn map_range(value: Float, in_min: Float, in_max: Float, out_min: Float, out_max: Float) -> Float {
    return out_min + (value - in_min) / (in_max - in_min) * (out_max - out_min)
}
```

---

## Step 4: Validate Documentation

```bash
ovie doc check
```

Fix any missing doc comments before publishing.

---

## Step 5: Generate Documentation

```bash
ovie doc --format html
```

Documentation is generated in `docs/output/`. Review it to make sure everything looks correct.

---

## Step 6: Use Your Package Locally

In another project, add your package as a path dependency:

```bash
ovie add my-math-utils --path ../my-math-utils
```

This adds to `ovie.toml`:

```toml
[dependencies]
my-math-utils = { path = "../my-math-utils" }
```

Then import and use it:

```ovie
use my-math-utils::{clamp, lerp, map_range}

fn main() {
    mut health = clamp(150, 0, 100)
    seeAm("Health: " + health)  // Health: 100

    mut progress = lerp(0.0, 1.0, 0.75)
    seeAm("Progress: " + progress)  // Progress: 0.75

    mut screen_x = map_range(0.5, 0.0, 1.0, 0.0, 1920.0)
    seeAm("Screen X: " + screen_x)  // Screen X: 960.0
}

main()
```

---

## Package Structure Summary

```
my-math-utils/
├── ovie.toml          # package manifest
├── src/
│   ├── mod.ov         # public API entry point (re-exports)
│   ├── clamp.ov       # clamp implementation
│   ├── interpolate.ov # interpolation implementation
│   └── stats.ov       # statistics implementation
└── tests/
    └── test_clamp.ov  # tests
```

## Next Steps

- [Using Third-Party Packages](using-third-party-packages.md) — consume packages like this one
- [Best Practices](best-practices.md) — guidelines for well-structured packages
- [Module System API Reference](../module-system-api.md) — complete API docs
