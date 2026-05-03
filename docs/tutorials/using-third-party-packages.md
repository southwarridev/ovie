# Using Third-Party Packages

This tutorial shows you how to add and use external packages in your Ovie project, using `oba` (a quantum computing library) as the example.

## Prerequisites

- An existing Ovie project with `ovie.toml`
- Ovie v2.3.0 installed

---

## Step 1: Find the Package

The `oba` package is a quantum computing and hypercomplex math library included in this repository at `./oba`. It provides:

- `oba::hypercomplex` — quaternions, octonions, complex numbers
- `oba::circuit` — quantum circuit construction
- `oba::qubit` — qubit state management
- `oba::gates` — quantum gate operations

---

## Step 2: Add the Dependency

```bash
ovie add oba --path ./oba
```

This updates your `ovie.toml`:

```toml
[package]
name = "my-quantum-app"
version = "0.1.0"

[dependencies]
oba = { path = "./oba" }
```

Then install:

```bash
ovie install
```

This resolves dependencies and generates `ovie.lock`:

```toml
# ovie.lock — auto-generated, commit to version control
version = 1

[[package]]
name = "oba"
version = "0.3.0"
source = "path+./oba"
checksum = "sha256:a1b2c3..."
```

---

## Step 3: Import and Use the Package

### Using Hypercomplex Numbers

```ovie
use oba::hypercomplex::{Quaternion, new_quaternion, quat_multiply, quat_norm}

fn main() {
    // Create two quaternions
    mut q1 = new_quaternion(1.0, 0.0, 0.0, 0.0)  // identity
    mut q2 = new_quaternion(0.0, 1.0, 0.0, 0.0)  // pure i

    // Multiply them
    mut product = quat_multiply(q1, q2)
    seeAm("Product: " + product.w + " + " + product.x + "i + " + product.y + "j + " + product.z + "k")

    // Get the norm
    mut norm = quat_norm(q1)
    seeAm("Norm of identity quaternion: " + norm)  // 1.0
}

main()
```

### Using Quantum Circuits

```ovie
use oba::circuit::{new_circuit, add_gate, measure_all}
use oba::gates::{hadamard, cnot}
use oba::qubit::{new_qubit_register}

fn main() {
    // Create a 2-qubit register
    mut qubits = new_qubit_register(2)

    // Build a Bell state circuit
    mut circuit = new_circuit(2)
    add_gate(&mut circuit, hadamard(0))   // H on qubit 0
    add_gate(&mut circuit, cnot(0, 1))    // CNOT: control=0, target=1

    // Measure
    mut results = measure_all(&circuit, &qubits)
    seeAm("Measurement results: " + results)
}

main()
```

---

## Step 4: Understand the Lock File

The `ovie.lock` file pins exact versions of all dependencies. This ensures:

- **Reproducible builds** — everyone on your team gets the same versions
- **Security** — checksums verify package integrity
- **Stability** — updates only happen when you explicitly run `ovie install` after changing `ovie.toml`

**Always commit `ovie.lock` to version control.**

To update a dependency to a newer version:

```bash
# Edit ovie.toml to change the version constraint, then:
ovie install
```

---

## Step 5: Import Styles

Ovie supports several import styles:

```ovie
// Import specific symbols (recommended)
use oba::hypercomplex::{Quaternion, new_quaternion}

// Import with alias (avoids name collisions)
use oba::hypercomplex::{Quaternion as Quat}

// Import all exports (use sparingly — can cause name collisions)
use oba::hypercomplex

// Relative import (within same package)
use ./utils::{helper_fn}

// Relative import with path
import "./vendor/special.ov"
```

---

## Step 6: Handling Missing Packages

If a package can't be found, you'll get a clear error:

```
Error: Package 'oba' not found
Searched paths:
  - ./oba/mod.ov
  - packages/oba/mod.ov
  - ~/.ovie/packages/oba/mod.ov

Suggestion: Run 'ovie install' to install dependencies, or check that
the path in ovie.toml is correct.
```

Fix by running `ovie install` or correcting the path in `ovie.toml`.

---

## Summary

| Command | What it does |
|---------|-------------|
| `ovie add <pkg> --path <path>` | Add a local path dependency |
| `ovie install` | Install all dependencies and update lock file |
| `use pkg::module::{symbol}` | Import a specific symbol |
| `use pkg::module as alias` | Import with alias |

## Next Steps

- [Best Practices](best-practices.md) — guidelines for dependency management
- [Creating Packages](creating-packages.md) — create your own reusable package
- [Module System API Reference](../module-system-api.md) — complete API docs
