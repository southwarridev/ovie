# Ovie CLI Reference

The Ovie command-line interface provides all the tools you need to create, build, test, and manage Ovie projects. This comprehensive reference covers all available commands and their options.

## Installation and Setup

### Verifying Installation

```bash
ovie --version
# Output: ovie 0.1.0 (Stage 0 - Rust Implementation)

ovie --help
# Shows all available commands
```

### Global Configuration

```bash
# Set global configuration
ovie config set editor "code"
ovie config set default-backend "wasm"

# View current configuration
ovie config list
```

## Core Commands

### `ovie new` - Create New Project

Creates a new Ovie project with standard structure.

```bash
# Basic project creation
ovie new my-project

# Create with specific template
ovie new my-app --template=web
ovie new my-lib --template=library

# Create in current directory
ovie new . --name=my-project

# Create with custom configuration
ovie new my-project --aproko-config=strict --backend=llvm
```

**Options:**
- `--template=<template>`: Project template (default, web, library, cli)
- `--name=<name>`: Project name (defaults to directory name)
- `--aproko-config=<level>`: Aproko configuration (minimal, normal, strict)
- `--backend=<backend>`: Default compilation backend (wasm, llvm)
- `--no-git`: Don't initialize git repository
- `--no-aproko`: Disable Aproko assistant

**Generated Structure:**
```
my-project/
├── ovie.toml          # Project configuration
├── src/
│   └── main.ov        # Main source file
├── tests/             # Test files
│   └── main.test.ov
├── .ovie/             # Ovie configuration
│   ├── aproko.toml    # Aproko settings
│   └── build.toml     # Build configuration
├── .gitignore         # Git ignore file
└── README.md          # Project documentation
```

### `ovie build` - Compile Project

Compiles your Ovie project to the target format.

```bash
# Basic build
ovie build

# Build with specific backend
ovie build --backend=wasm
ovie build --backend=llvm

# Release build (optimized)
ovie build --release

# Build with Aproko analysis
ovie build --with-aproko

# Build specific target
ovie build --target=wasm32-unknown-unknown

# Verbose build output
ovie build --verbose

# Clean build (remove cache)
ovie build --clean
```

**Options:**
- `--backend=<backend>`: Compilation backend (wasm, llvm)
- `--target=<target>`: Specific compilation target
- `--release`: Enable optimizations
- `--debug`: Include debug information
- `--with-aproko`: Run Aproko analysis during build
- `--aproko-strict`: Fail build on Aproko warnings
- `--clean`: Clean build cache before building
- `--verbose`: Show detailed build output
- `--jobs=<n>`: Number of parallel build jobs

**Output:**
- WASM: `target/wasm/output.wasm`
- LLVM: `target/release/project-name` or `target/debug/project-name`

### `ovie run` - Execute Project

Runs your Ovie project.

```bash
# Run main function
ovie run

# Run with arguments
ovie run -- arg1 arg2 arg3

# Run specific file
ovie run src/example.ov

# Run with specific backend
ovie run --backend=wasm

# Run in debug mode
ovie run --debug

# Run with environment variables
ovie run --env KEY=value --env DEBUG=true
```

**Options:**
- `--backend=<backend>`: Runtime backend
- `--debug`: Run with debug information
- `--env=<key=value>`: Set environment variables
- `--release`: Run optimized build
- `--`: Pass arguments to the program

### `ovie test` - Run Tests

Executes unit tests and property-based tests.

```bash
# Run all tests
ovie test

# Run specific test file
ovie test tests/math.test.ov

# Run tests matching pattern
ovie test --filter="test_addition"

# Run only unit tests
ovie test --unit-only

# Run only property-based tests
ovie test --property-only

# Run tests with coverage
ovie test --coverage

# Run tests in parallel
ovie test --parallel

# Generate test report
ovie test --report=json > test-results.json
```

**Options:**
- `--filter=<pattern>`: Run tests matching pattern
- `--unit-only`: Run only unit tests
- `--property-only`: Run only property-based tests
- `--coverage`: Generate coverage report
- `--parallel`: Run tests in parallel
- `--report=<format>`: Generate report (json, xml, html)
- `--verbose`: Show detailed test output
- `--iterations=<n>`: Number of property test iterations (default: 100)

### `ovie fmt` - Format Code

Formats Ovie source code according to style guidelines.

```bash
# Format all files in project
ovie fmt

# Format specific files
ovie fmt src/main.ov src/lib.ov

# Check formatting without changing files
ovie fmt --check

# Format with custom configuration
ovie fmt --config=.ovie/fmt.toml

# Format and show diff
ovie fmt --diff
```

**Options:**
- `--check`: Check if files are formatted (exit code 1 if not)
- `--diff`: Show formatting changes without applying
- `--config=<file>`: Use custom formatting configuration
- `--backup`: Create backup files before formatting

### `ovie update` - Update Dependencies

Updates project dependencies while maintaining deterministic builds.

```bash
# Update all dependencies
ovie update

# Update specific dependency
ovie update my-dependency

# Update to specific version
ovie update my-dependency@1.2.3

# Dry run (show what would be updated)
ovie update --dry-run

# Update and regenerate lock file
ovie update --lock
```

**Options:**
- `--dry-run`: Show updates without applying
- `--lock`: Regenerate lock file
- `--offline`: Use only cached dependencies
- `--verify`: Verify cryptographic hashes

### `ovie vendor` - Manage Dependencies

Manages local dependency storage for offline builds.

```bash
# Vendor all dependencies
ovie vendor

# Vendor specific dependency
ovie vendor my-dependency

# Update vendored dependencies
ovie vendor --update

# Verify vendored dependencies
ovie vendor --verify

# Clean vendor directory
ovie vendor --clean
```

**Options:**
- `--update`: Update vendored dependencies
- `--verify`: Verify cryptographic hashes
- `--clean`: Remove unused vendored dependencies
- `--offline`: Work with existing vendor cache only

## Analysis and Quality Commands

### `ovie aproko` - Run Aproko Analysis

Runs the Aproko assistant for code analysis and suggestions.

```bash
# Analyze entire project
ovie aproko

# Analyze specific files
ovie aproko src/main.ov

# Interactive analysis
ovie aproko --interactive

# Generate analysis report
ovie aproko --report=json > analysis.json

# Analyze specific categories
ovie aproko --categories=security,performance

# Auto-fix issues
ovie aproko --fix

# CI mode (machine-readable output)
ovie aproko --ci
```

**Options:**
- `--interactive`: Interactive analysis mode
- `--categories=<list>`: Analyze specific categories
- `--report=<format>`: Generate report (json, xml, html)
- `--fix`: Auto-fix correctable issues
- `--ci`: CI-friendly output format
- `--config=<file>`: Use custom Aproko configuration

### `ovie check` - Quick Health Check

Performs a quick health check of your project.

```bash
# Basic health check
ovie check

# Check with Aproko analysis
ovie check --with-aproko

# Check dependencies
ovie check --deps

# Check formatting
ovie check --fmt
```

**Options:**
- `--with-aproko`: Include Aproko analysis
- `--deps`: Check dependency health
- `--fmt`: Check code formatting
- `--security`: Security-focused checks

## Project Management Commands

### `ovie init` - Initialize Existing Directory

Initializes an existing directory as an Ovie project.

```bash
# Initialize current directory
ovie init

# Initialize with specific configuration
ovie init --template=library --aproko-config=strict
```

### `ovie clean` - Clean Build Artifacts

Removes build artifacts and caches.

```bash
# Clean build artifacts
ovie clean

# Clean everything including vendor cache
ovie clean --all

# Clean specific targets
ovie clean --target=wasm
```

**Options:**
- `--all`: Clean everything including caches
- `--target=<target>`: Clean specific target
- `--vendor`: Clean vendor directory

### `ovie info` - Project Information

Shows detailed information about your project.

```bash
# Show project info
ovie info

# Show dependency tree
ovie info --deps

# Show build configuration
ovie info --build

# Show Aproko configuration
ovie info --aproko
```

## Configuration Commands

### `ovie config` - Manage Configuration

Manages global and project-specific configuration.

```bash
# List all configuration
ovie config list

# Get specific value
ovie config get editor

# Set configuration value
ovie config set editor "vim"

# Reset to defaults
ovie config reset

# Show configuration file location
ovie config path
```

**Common Configuration Keys:**
- `editor`: Default editor for `ovie edit`
- `backend`: Default compilation backend
- `aproko.enabled`: Enable/disable Aproko globally
- `build.parallel`: Enable parallel builds
- `test.iterations`: Default property test iterations

## Development Commands

### `ovie doc` - Generate Documentation

Generates documentation for your project.

```bash
# Generate API documentation
ovie doc

# Generate and open in browser
ovie doc --open

# Generate specific format
ovie doc --format=html

# Include private items
ovie doc --private
```

### `ovie bench` - Run Benchmarks

Runs performance benchmarks.

```bash
# Run all benchmarks
ovie bench

# Run specific benchmark
ovie bench --filter="sort_benchmark"

# Generate benchmark report
ovie bench --report=json
```

## Advanced Commands

### `ovie self` - Self-Hosting Commands

Commands related to Ovie's self-hosting journey.

```bash
# Show self-hosting status
ovie self status

# Run bootstrap verification
ovie self verify

# Generate Stage 1 components
ovie self bootstrap --stage=1
```

### `ovie release` - Release Management

Manages project releases with cryptographic signing.

```bash
# Create release
ovie release create v1.0.0

# Sign release
ovie release sign v1.0.0

# Verify release
ovie release verify v1.0.0
```

## Global Options

These options work with most commands:

- `--help, -h`: Show help information
- `--version, -V`: Show version information
- `--verbose, -v`: Verbose output
- `--quiet, -q`: Suppress output
- `--color=<when>`: Colorize output (auto, always, never)
- `--config=<file>`: Use specific configuration file

## Environment Variables

Ovie respects these environment variables:

- `OVIE_HOME`: Ovie installation directory
- `OVIE_CACHE`: Cache directory location
- `OVIE_CONFIG`: Configuration file location
- `OVIE_BACKEND`: Default compilation backend
- `OVIE_EDITOR`: Default editor
- `OVIE_LOG`: Log level (error, warn, info, debug, trace)

## Exit Codes

Ovie uses standard exit codes:

- `0`: Success
- `1`: General error
- `2`: Misuse of shell command
- `101`: Compilation error
- `102`: Test failure
- `103`: Aproko analysis failure

## Examples

### Complete Development Workflow

```bash
# Create new project
ovie new my-app --template=web

# Navigate to project
cd my-app

# Build and run
ovie build
ovie run

# Run tests
ovie test

# Format code
ovie fmt

# Run analysis
ovie aproko

# Create release
ovie build --release
```

### CI/CD Integration

```bash
# CI build script
ovie build --release --with-aproko --aproko-strict
ovie test --coverage --report=xml
ovie aproko --ci --report=json
```

### Development Setup

```bash
# Set up development environment
ovie config set editor "code"
ovie config set aproko.verbosity "detailed"
ovie config set build.parallel true
```

## Getting Help

- `ovie help`: General help
- `ovie help <command>`: Command-specific help
- `ovie <command> --help`: Detailed command help
- **Documentation**: [docs.ovie-lang.org](https://docs.ovie-lang.org)
- **Community**: [Discord](https://discord.gg/ovie-lang)

---

*This CLI reference covers Ovie Stage 0. Additional commands and options will be available in future stages.*