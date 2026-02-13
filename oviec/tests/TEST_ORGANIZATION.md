# Ovie Test Suite Organization

This document describes the organization and structure of the Ovie compiler test suite.

## Directory Structure

```
oviec/tests/
├── bootstrap/          # Bootstrap verification tests
├── conformance/        # Language specification conformance tests
├── distribution/       # Distribution and release system tests
├── integration/        # Integration tests across components
├── invariants/         # Compiler stage invariant tests
├── performance/        # Performance and benchmark tests
├── property/           # Property-based tests
├── regression/         # Regression detection tests
├── unit/               # Unit tests for individual components
└── *.rs                # Top-level test files
```

## Test Categories

### 1. Bootstrap Tests (`bootstrap/`)
Tests for the bootstrap verification system and self-hosting capabilities.

**Files:**
- `comprehensive_tests.rs` - Complete bootstrap verification tests
- `script_integration_tests.rs` - Bootstrap script integration tests
- `mod.rs` - Module organization

**Purpose:** Verify that the Ovie-in-Ovie compiler produces equivalent output to the Rust compiler.

### 2. Conformance Tests (`conformance/`)
Tests that verify compliance with language specifications.

**Files:**
- `language_spec.rs` - Language specification conformance
- `abi_spec.rs` - ABI specification conformance
- `stdlib_spec.rs` - Standard library specification conformance
- `mod.rs` - Module organization

**Purpose:** Ensure the compiler implements the Ovie language specification correctly.

### 3. Distribution Tests (`distribution/`)
Tests for the distribution and release system.

**Files:**
- `package_builder.rs` - Package creation tests
- `installer.rs` - Installation system tests
- `release_manager.rs` - Release management tests
- `mod.rs` - Module organization

**Purpose:** Verify that distribution packages are created correctly and installation works.

### 4. Integration Tests (`integration/`)
Tests that verify multiple components working together.

**Files:**
- `end_to_end.rs` - Complete compilation pipeline tests
- `invariant_pipeline.rs` - Pipeline invariant integration tests
- `cross_platform_validator.rs` - Cross-platform validation tests
- `mod.rs` - Module organization

**Purpose:** Test the interaction between different compiler stages and components.

### 5. Invariants Tests (`invariants/`)
Tests that verify compiler stage invariants are maintained.

**Files:**
- `ast_invariants.rs` - AST invariant tests
- `hir_invariants.rs` - HIR invariant tests
- `mir_invariants.rs` - MIR invariant tests
- `backend_invariants.rs` - Backend invariant tests
- `pipeline_invariants.rs` - Full pipeline invariant tests
- `mod.rs` - Module organization

**Purpose:** Ensure each compiler stage maintains its documented invariants.

### 6. Performance Tests (`performance/`)
Tests for performance benchmarks and regression detection.

**Files:**
- `benchmarks.rs` - Performance benchmarks
- `regression.rs` - Performance regression detection
- `mod.rs` - Module organization

**Purpose:** Track and prevent performance regressions.

### 7. Property Tests (`property/`)
Property-based tests using proptest/quickcheck.

**Files:**
- `compiler.rs` - Compiler correctness properties
- `stdlib.rs` - Standard library determinism properties
- `bootstrap.rs` - Bootstrap equivalence properties
- `security.rs` - Security properties
- `mod.rs` - Module organization

**Purpose:** Test universal properties that should hold for all inputs.

### 8. Regression Tests (`regression/`)
Tests that prevent known bugs from reappearing.

**Files:**
- `compiler_behavior.rs` - Compiler behavior regression tests
- `performance_regression.rs` - Performance regression tests
- `regression_detector.rs` - Automated regression detection
- `mod.rs` - Module organization

**Purpose:** Catch regressions in compiler behavior and performance.

### 9. Unit Tests (`unit/`)
Tests for individual components in isolation.

**Files:**
- `lexer.rs` - Lexer unit tests
- `parser.rs` - Parser unit tests
- `ast_invariants.rs` - AST invariant unit tests
- `hir_invariants.rs` - HIR invariant unit tests
- `mir_invariants.rs` - MIR invariant unit tests
- `backend_invariants.rs` - Backend invariant unit tests
- `type_checker.rs` - Type checker unit tests
- `codegen.rs` - Code generation unit tests
- `runtime.rs` - Runtime unit tests
- `mod.rs` - Module organization

**Purpose:** Test individual components in isolation.

### 10. Standard Library Tests
Tests for standard library modules (top-level files).

**Files:**
- `stdlib_core_tests.rs` - Core module tests
- `stdlib_math_tests.rs` - Math module tests
- `stdlib_io_tests.rs` - I/O module tests
- `stdlib_fs_tests.rs` - File system module tests
- `stdlib_time_tests.rs` - Time module tests
- `stdlib_env_tests.rs` - Environment module tests
- `stdlib_cli_tests.rs` - CLI module tests
- `stdlib_log_tests.rs` - Logging module tests
- `stdlib_test_tests.rs` - Testing framework tests
- `stdlib_determinism_tests.rs` - Determinism verification
- `stdlib_cross_platform_tests.rs` - Cross-platform compatibility
- `stdlib_performance_tests.rs` - Performance tests
- `stdlib_property_tests.rs` - Property-based tests
- `stdlib_integration_suite.rs` - Integration tests
- `stdlib_api_verification.rs` - API completeness verification
- `stdlib_completeness_verification.rs` - Completeness checks
- `stdlib_documentation_verification.rs` - Documentation checks
- `stdlib_placeholder_detection.rs` - Placeholder detection
- `stdlib_type_checking_verification.rs` - Type checking verification

**Purpose:** Comprehensive testing of all standard library modules.

## Test Utilities

**Files:**
- `utils.rs` - Shared test utilities and helpers
- `runner.rs` - Custom test runner
- `mod.rs` - Test module organization

## Running Tests

### Run All Tests
```bash
cargo test --package oviec
```

### Run Specific Test Category
```bash
cargo test --package oviec --test <category>
```

Examples:
```bash
cargo test --package oviec --test unit
cargo test --package oviec --test integration
cargo test --package oviec --test invariants
```

### Run Tests in a Specific Directory
```bash
cargo test --package oviec <directory_name>
```

Examples:
```bash
cargo test --package oviec bootstrap
cargo test --package oviec distribution
cargo test --package oviec stdlib
```

### Run a Specific Test
```bash
cargo test --package oviec <test_name>
```

## Test Coverage Goals

- **Unit Tests:** 100% coverage of critical paths
- **Integration Tests:** All major component interactions
- **Invariant Tests:** All compiler stage invariants
- **Property Tests:** Key correctness properties
- **Regression Tests:** All known bugs
- **Performance Tests:** Key performance metrics

## Test Philosophy

1. **Simple and Portable:** All tests use standard Rust `#[test]` functions
2. **No External Dependencies:** No complex test frameworks required
3. **Cross-Platform:** Tests work on Windows, Linux, and macOS
4. **Fast Execution:** Tests should complete quickly
5. **Clear Failures:** Test failures should be easy to diagnose
6. **Comprehensive:** Cover all critical paths and edge cases

## Adding New Tests

When adding new tests:

1. Choose the appropriate directory based on test type
2. Follow existing naming conventions
3. Add module exports to `mod.rs` files
4. Document the purpose of the test
5. Ensure tests are cross-platform compatible
6. Keep tests simple and focused

## Test Maintenance

- Review test failures promptly
- Update tests when specifications change
- Remove obsolete tests
- Refactor duplicated test code into utilities
- Keep test documentation up to date
