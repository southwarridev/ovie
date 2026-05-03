# Ovie v2.3.0 — Complete Module System

**Release Date:** April 23, 2026  
**Version:** 2.3.0  
**Status:** Stable

---

## 🎉 What's New in v2.3

### Complete Module System

The headline feature of v2.3 is a fully self-hosted module system implemented entirely in `.ov` files.

**Import/Export**
```ovie
// Export from a module
export fn add(a: Number, b: Number) -> Number {
    return a + b
}

// Import in another module
use std::math::{sqrt, pow}
use std::core::{Result, Option}
import "./utils.ov"
```

**Package Manager**
```bash
ovie init my-package    # Initialize new package
ovie add some-lib       # Add dependency
ovie install            # Install all dependencies
```

**New `ovie.toml` format:**
```toml
[package]
name = "my-app"
version = "1.0.0"

[dependencies]
oba = { path = "./oba" }
```

### Aproko Knowledge Base

Persistent AI-accessible storage for type information, reasoning rules, and code patterns.

```bash
ovie aproko query --category TypeInformation --symbol add
ovie aproko export --output knowledge.json
```

### Documentation Enforcement

The compiler now requires doc comments on all exported functions:

```ovie
/// Add two numbers
/// # Parameters
/// - a: First number
/// - b: Second number
/// # Returns
/// Sum of a and b
/// # Examples
/// ```ovie
/// mut r = add(2, 3)  // 5
/// ```
export fn add(a: Number, b: Number) -> Number {
    return a + b
}
```

```bash
ovie doc check      # Validate documentation completeness
ovie doc            # Generate HTML/Markdown/JSON docs
```

### New Standard Library Modules

- **`std::module`** — Module loader, resolver, dependency graph, cache, package manager
- **`std::aproko`** — Knowledge base storage and query for AI/LLM integration

### Complete Book

"Ovie — The Complete Book" — 12 chapters from first principles to production deployment. Available at `website/docs/book/index.html`.

---

## 📦 What's Included

Each installation includes:
- Ovie compiler (`oviec`) and CLI (`ovie`) — v2.3.0
- Complete Runtime Environment (ORE)
- Standard library — 11 modules, 160KB+
- Aproko reasoning engine
- Module system (fully self-hosted in `.ov`)
- Documentation and 22+ examples
- The Complete Book (12 chapters)

---

## 🔧 Breaking Changes

None. v2.3 is fully backward compatible with v2.2 programs.

Standalone `.ov` files without import/export statements continue to work exactly as before. The module system is opt-in.

---

## 🚀 Upgrade from v2.2

```bash
# Linux/macOS
curl -sSL https://raw.githubusercontent.com/southwarridev/ovie/main/easy-linux-install.sh | bash

# Windows
iwr -useb https://raw.githubusercontent.com/southwarridev/ovie/main/easy-windows-install.ps1 | iex

# Verify
ovie --version  # Should show 2.3.0
```

No code changes required for existing programs.

---

## 📊 Module System Performance

- Standard library load: < 100ms
- Incremental compilation: 10x faster than full build
- Cache hit per module: < 10ms
- Dependency graph construction: O(n) with number of modules

---

## 🔗 Links

- GitHub: https://github.com/southwarridev/ovie
- GitLab: https://gitlab.com/ovie1/ovie
- Website: https://ovie-lang.org
- Book: https://ovie-lang.org/docs/book/

---

## 📝 Full Changelog

### Added
- Complete module system (`std/module/`) — loader, resolver, dependency graph, cache, package manager, type checker, namespace management, error handling, introspection, compiler integration, Aproko integration
- `std::aproko` — knowledge base with persistent storage, query interface, JSON export
- `std::module` — module system APIs
- Documentation enforcement — compiler requires doc comments on all exports
- `ovie doc` command — generate HTML/Markdown/JSON documentation
- `ovie doc check` command — validate documentation completeness
- `ovie aproko query` command — query knowledge base
- `ovie aproko export` command — export knowledge base to JSON
- `ovie init`, `ovie add`, `ovie install` — package management CLI
- `ovie.lock` — lock file for reproducible builds
- Cross-module type checking
- Namespace management with collision detection
- Circular dependency detection with cycle path reporting
- Module caching with SHA-256 content hashing
- Parallel module loading for independent modules
- Migration guide: v2.2 → v2.3 (`docs/migration-v2.2-to-v2.3-modules.md`)
- "Ovie — The Complete Book" — 12 chapters
- Updated public website with module system documentation

### Changed
- Version bumped to 2.3.0
- Website updated with v2.3 features, book section, module system showcase
- README updated to reflect v2.3

### Fixed
- No breaking changes from v2.2

---

*"Low-level control meets high-level productivity — systems programming made accessible."*
