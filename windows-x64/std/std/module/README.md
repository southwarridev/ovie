# Ovie Module System

This directory contains the implementation of Ovie's module system, written entirely in Ovie (.ov files) as part of the language's self-hosting journey.

## Overview

The module system enables:
- **Code Organization**: Structure code into reusable modules
- **Namespace Management**: Avoid naming conflicts with module namespaces
- **Dependency Resolution**: Automatically resolve and load dependencies
- **Standard Library**: Access std:: modules (std::math, std::io, etc.)
- **Third-Party Packages**: Use community packages (e.g., oba::hypercomplex)
- **Incremental Compilation**: Fast rebuilds with intelligent caching

## Directory Structure

```
std/module/
├── mod.ov              # Module entry point, re-exports public API
├── types.ov            # Core data structures (Module, Import, ExportTable, etc.)
├── loader.ov           # Module loading and management
├── resolver.ov         # Module path resolution
├── fs_integration.ov   # File system operations
├── dependency_graph.ov # Dependency tracking (Phase 2)
├── cache.ov            # Module caching (Phase 3)
├── exports.ov          # Export/import system (Phase 1-2)
├── package_manager.ov  # Package management (Phase 5)
└── README.md           # This file
```

## Core Components

### 1. Types (`types.ov`)

Defines fundamental data structures:
- `Module`: Represents a loaded module with metadata and exports
- `Import`: Tracks import statements and dependencies
- `ExportTable`: Registry of exported symbols (functions, types, constants, enums)
- `FunctionSignature`: Function signature for type checking
- `TypeDefinition`: Type information for structs, enums, and aliases
- `ModuleError`: Error types for module operations

### 2. Loader (`loader.ov`)

Core module loading functionality:
- `load_module(path)`: Load a module from file path
- `load_dependencies(module)`: Recursively load all dependencies
- `is_loaded(path)`: Check if module is already loaded
- `get_module(path)`: Retrieve a loaded module
- `register_export(module, symbol)`: Register an exported symbol
- `validate_imports(module)`: Validate all imports can be resolved

### 3. Resolver (`resolver.ov`)

Module path resolution:
- `resolve(module_path, config)`: Resolve module path to file location
- `path_to_file(module_path)`: Convert module path to file path (std::math → std/math/mod.ov)
- `search_paths(module_name, dirs)`: Search for module in multiple directories
- `normalize_path(path, base)`: Convert relative path to absolute
- `is_valid_module(path)`: Check if path is a valid .ov module

### 4. File System Integration (`fs_integration.ov`)

File system operations for module system:
- `read_ov_file(path)`: Read .ov file contents
- `file_exists(path)`: Check file existence
- `is_directory(path)`: Check if path is directory
- `list_ov_files(dir)`: List all .ov files in directory
- `ensure_directory(dir)`: Create directory if needed

## Usage Examples

### Loading a Module

```ovie
use std::module::{load_module};

// Load a module
let module = load_module("std/math/mod.ov")?;

// Access exports
for (name, func) in module.exports.functions {
    println!("Function: {}", name);
}
```

### Resolving Module Paths

```ovie
use std::module::{resolve, default_config};

// Resolve standard library module
let config = default_config();
let resolved = resolve("std::math", config)?;
println!("Resolved to: {}", resolved.absolute_path);

// Resolve relative module
let resolved = resolve("./utils.ov", config)?;

// Resolve third-party package
let resolved = resolve("oba::hypercomplex", config)?;
```

### Module Path Formats

The module system supports multiple path formats:

1. **Standard Library**: `std::math`, `std::io::File`
   - Resolves to: `std/math/mod.ov`, `std/io/File.ov`

2. **Relative Paths**: `./utils.ov`, `../common/types.ov`
   - Resolves relative to current file

3. **Direct Files**: `path/to/module.ov`
   - Direct file reference

4. **Third-Party Packages**: `oba::hypercomplex`, `package::module`
   - Searches in package directories

## Implementation Status

### Phase 1: Core Module Loading (Current)
- ✅ Directory structure created
- ✅ Core data structures defined
- ✅ Module loader skeleton implemented
- ✅ Module resolver skeleton implemented
- ✅ File system integration created
- ⏳ Export/import parsing (in progress)
- ⏳ Property-based tests (pending)

### Phase 2: Dependency Management (Upcoming)
- ⏳ Dependency graph implementation
- ⏳ Circular dependency detection
- ⏳ Topological sort for compilation order

### Phase 3: Caching and Performance (Upcoming)
- ⏳ Module cache implementation
- ⏳ Hash-based cache invalidation
- ⏳ Incremental compilation support

### Phase 4-9: See tasks.md for full roadmap

## Design Principles

1. **100% Ovie Implementation**: No Rust code, fully self-hosted
2. **Performance First**: Lazy loading, caching, parallel compilation
3. **Clear Error Messages**: Helpful errors with suggestions
4. **Backward Compatible**: Existing standalone .ov files continue to work
5. **AI-Friendly**: Aproko knowledge base for LLM integration

## Testing

The module system uses property-based testing to ensure correctness:

```ovie
// Property: Export-Import Round Trip
// For any valid symbol exported from a module,
// importing should preserve all information
fn test_export_import_round_trip() {
    let symbol = generate_random_symbol();
    let module = create_module_with_export(symbol);
    let imported = import_symbol_from_module(module, symbol.name);
    
    assert(imported.name == symbol.name);
    assert(imported.type_signature == symbol.type_signature);
}
```

See `tests/property/module_*.ov` for full test suite.

## Contributing

When adding functionality to the module system:

1. Update relevant .ov files in std/module/
2. Add comprehensive documentation with examples
3. Write property-based tests for new features
4. Update this README with new functionality
5. Ensure backward compatibility

## Related Documentation

- [Requirements Document](/.kiro/specs/ovie-2-3-module-system/requirements.md)
- [Design Document](/.kiro/specs/ovie-2-3-module-system/design.md)
- [Implementation Tasks](/.kiro/specs/ovie-2-3-module-system/tasks.md)
- [Ovie Language Guide](/docs/language-guide.md)

## License

Part of the Ovie programming language project.
