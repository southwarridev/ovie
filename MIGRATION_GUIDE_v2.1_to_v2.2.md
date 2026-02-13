# Migration Guide: Ovie v2.1 to v2.2

This guide helps you migrate your Ovie projects from v2.1 to v2.2.

---

## Overview

Ovie v2.2 introduces **complete language consolidation** with enforced correctness. While most code will work without changes, there are important infrastructure and tooling updates you need to know about.

**Migration Difficulty**: Easy to Moderate  
**Estimated Time**: 30 minutes to 2 hours depending on project size

---

## Quick Migration Checklist

- [ ] Install complete v2.2 distribution package (not just binary)
- [ ] Verify ORE structure with `oviec --self-check`
- [ ] Update error handling to use structured errors
- [ ] Update scripts to check exit codes properly
- [ ] Run tests to verify compatibility
- [ ] Update documentation references

---

## 1. Installation Changes

### What Changed

**v2.1**: Single binary installation  
**v2.2**: Complete Runtime Environment (ORE) required

### Migration Steps

#### Step 1: Uninstall v2.1

```bash
# Remove old binary
rm /usr/local/bin/oviec  # Linux/macOS
# or
del C:\Program Files\Ovie\oviec.exe  # Windows
```

#### Step 2: Install v2.2 Complete Package

```bash
# Linux x64
wget https://github.com/southwarridev/ovie/releases/download/v2.2.0/ovie-v2.2-linux-x64.tar.gz
tar -xzf ovie-v2.2-linux-x64.tar.gz
cd ovie-v2.2-linux-x64
./install.sh

# macOS (ARM64)
curl -LO https://github.com/southwarridev/ovie/releases/download/v2.2.0/ovie-v2.2-macos-arm64.tar.gz
tar -xzf ovie-v2.2-macos-arm64.tar.gz
cd ovie-v2.2-macos-arm64
./install.sh

# Windows x64
# Download ovie-v2.2-windows-x64.zip
# Extract and run install.bat
```

#### Step 3: Verify Installation

```bash
oviec --version
# Expected: oviec 2.2.0 (Complete Language System)

oviec --env
# Expected: Shows OVIE_HOME and ORE structure

oviec --self-check
# Expected: All components validated successfully
```

### Troubleshooting

**Problem**: `oviec --env` shows "ORE not found"  
**Solution**: Reinstall using complete distribution package, not standalone binary

**Problem**: `oviec` command not found  
**Solution**: Add OVIE_HOME/bin to your PATH

---

## 2. Runtime Environment (ORE)

### What Changed

**v2.1**: Compiler could run from any location  
**v2.2**: Compiler requires canonical ORE structure

### Required Directory Structure

```
OVIE_HOME/
├── bin/
│   ├── oviec       # Compiler
│   └── ovie        # CLI tool
├── std/            # Standard library
│   ├── core/
│   ├── math/
│   ├── io/
│   └── ...
├── aproko/         # Reasoning engine
│   └── rules/
├── targets/        # Backend configurations
│   ├── native/
│   └── wasm/
├── config/         # Runtime configuration
│   └── ovie.toml
└── logs/           # Debug logs
```

### Migration Steps

#### Step 1: Check Current Environment

```bash
oviec --env
```

#### Step 2: Fix Missing Components

If any components are missing, reinstall using the complete distribution package.

#### Step 3: Set OVIE_HOME (Optional)

```bash
# Add to ~/.bashrc or ~/.zshrc
export OVIE_HOME=/path/to/ovie

# Or let Ovie auto-detect from executable location
```

### Environment Discovery Order

1. `OVIE_HOME` environment variable
2. `.ovie/` in current directory
3. Executable directory
4. System-wide locations

---

## 3. Compiler Invariants

### What Changed

**v2.1**: Invalid compiler states were warnings  
**v2.2**: Invalid compiler states panic with exit code 2

### Impact on Your Code

**Good News**: This change catches compiler bugs, not user code errors. Your Ovie code doesn't need changes.

### What to Watch For

If you see exit code 2, it means:
- Compiler bug detected (not your fault!)
- Please report to: https://github.com/southwarridev/ovie/issues

### Example Error

```
PANIC: Invariant violation in HIR stage
Error: Unresolved symbol 'foo' at line 42
This is a compiler bug. Please report it.
Exit code: 2
```

---

## 4. CLI Command Changes

### What Changed

**v2.1**: Some commands had inconsistent behavior  
**v2.2**: All commands have documented exit codes and deterministic output

### New Commands

```bash
oviec --env          # Show runtime environment status
oviec --self-check   # Validate installation completeness
oviec explain error E_TYPE_004  # Explain specific error
oviec explain type my_var       # Show type inference
```

### Exit Code Changes

All commands now use standard exit codes:

| Exit Code | Meaning |
|-----------|---------|
| 0 | Success |
| 1 | User error (syntax, type error, etc.) |
| 2 | Compiler bug (invariant violation) |
| 3 | Environment error (missing ORE, etc.) |

### Migration Steps

#### Update Scripts

**Before (v2.1)**:
```bash
oviec build main.ov
if [ $? -ne 0 ]; then
    echo "Build failed"
fi
```

**After (v2.2)**:
```bash
oviec build main.ov
EXIT_CODE=$?
if [ $EXIT_CODE -eq 1 ]; then
    echo "User error in code"
elif [ $EXIT_CODE -eq 2 ]; then
    echo "Compiler bug - please report"
elif [ $EXIT_CODE -eq 3 ]; then
    echo "Environment error - run oviec --self-check"
fi
```

---

## 5. Error Handling

### What Changed

**v2.1**: Errors were simple strings  
**v2.2**: Errors are structured with codes, severity, explanations

### Error Structure

```rust
pub struct StructuredError {
    pub code: ErrorCode,           // E_TYPE_004
    pub severity: Severity,        // Error, Warning, Info, Hint
    pub location: SourceLocation,  // file:line:column
    pub message: String,           // Human-readable message
    pub explanation: String,       // Detailed explanation
    pub suggested_fix: Option<String>, // How to fix it
}
```

### Migration Steps

#### If You Parse Errors

**Before (v2.1)**:
```rust
// Simple string parsing
if error_message.contains("type mismatch") {
    // Handle type error
}
```

**After (v2.2)**:
```rust
// Structured error handling
match error.code {
    ErrorCode::E_TYPE_001 => {
        // Handle type mismatch
        println!("Type error at {}:{}", error.location.line, error.location.column);
        println!("Suggestion: {}", error.suggested_fix.unwrap_or_default());
    }
    _ => {}
}
```

#### If You Display Errors

**Before (v2.1)**:
```bash
# Simple text output
Error: type mismatch
```

**After (v2.2)**:
```bash
# Rich structured output
E_TYPE_001: Type mismatch at main.ov:42:10
Expected: i32
Found: String

Explanation: The function expects an integer but received a string.

Suggested fix: Convert the string to an integer using parse():
    let num = my_string.parse::<i32>()?;
```

---

## 6. Standard Library Changes

### What Changed

**v2.1**: Some stdlib functions were placeholders  
**v2.2**: All stdlib functions are fully implemented

### New Modules

All 9 core modules are now complete:
- std::core (Result, Option, Vec, HashMap)
- std::math (deterministic math operations)
- std::io (offline-first I/O)
- std::fs (file system operations)
- std::time (time operations)
- std::env (environment variables)
- std::cli (command-line interface)
- std::test (testing framework)
- std::log (structured logging)

### Migration Steps

#### Remove Workarounds

If you implemented workarounds for missing stdlib functions, you can now use the official implementations:

**Before (v2.1)**:
```ovie
// Custom Result implementation
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}
```

**After (v2.2)**:
```ovie
// Use official Result from std::core
use std::core::Result

fn my_function() -> Result<i32, String> {
    return Result::Ok(42)
}
```

#### Update Imports

**Before (v2.1)**:
```ovie
// May have used custom implementations
use my_lib::Vec
```

**After (v2.2)**:
```ovie
// Use official stdlib
use std::core::Vec
```

---

## 7. Aproko Reasoning Engine

### What Changed

**v2.1**: Aproko was a basic analyzer  
**v2.2**: Aproko is a formal reasoning layer

### New Features

```bash
# Explain specific errors
oviec explain error E_TYPE_004

# Show type inference reasoning
oviec explain type my_var

# Understand compiler decisions
oviec explain decision optimization_pass
```

### Migration Steps

#### Enable Aproko Explanations

Add to your `.ovie/aproko.toml`:

```toml
[reasoning]
enabled = true
verbosity = "detailed"  # or "summary"

[explanations]
show_type_inference = true
show_optimization_decisions = true
show_error_context = true
```

#### Use in Development

```bash
# Get help with errors
oviec build main.ov
# If error occurs:
oviec explain error E_TYPE_004

# Understand type inference
oviec explain type my_variable
```

---

## 8. Bootstrap Verification

### What Changed

**v2.1**: Self-hosting was claimed but not verified  
**v2.2**: Self-hosting is proven with hash verification

### New Scripts

```bash
# Run bootstrap verification
./scripts/bootstrap_verify.sh      # Linux/macOS
./scripts/bootstrap_verify.ps1     # Windows
```

### What It Does

1. Rust compiler → oviec₀
2. oviec₀ → oviec₁
3. oviec₁ → oviec₂
4. Compare hashes: oviec₁ == oviec₂

### Migration Steps

No action needed - this is for Ovie maintainers and CI systems.

---

## 9. Testing Changes

### What Changed

**v2.1**: Basic test framework  
**v2.2**: Comprehensive testing with property-based tests

### New Test Features

```ovie
use std::test::{assert, assert_eq, property_test}

// Unit tests
#[test]
fn test_addition() {
    assert_eq(2 + 2, 4)
}

// Property-based tests
#[property_test]
fn test_addition_commutative(a: i32, b: i32) {
    assert_eq(a + b, b + a)
}
```

### Migration Steps

#### Update Test Files

**Before (v2.1)**:
```ovie
// Basic assertions
fn test_my_function() {
    mut result = my_function(42)
    if result != 84 {
        panic("Test failed")
    }
}
```

**After (v2.2)**:
```ovie
use std::test::assert_eq

#[test]
fn test_my_function() {
    mut result = my_function(42)
    assert_eq(result, 84)
}
```

---

## 10. Performance Considerations

### What Changed

**v2.2 adds invariant checking at every compiler stage**

### Performance Impact

- Compilation: ~5-10% slower (due to validation)
- Runtime: No impact (validation is compile-time only)
- Memory: ~10MB additional for validation data

### Optimization Tips

```bash
# For production builds, invariant checking is optimized
ovie build --release

# For development, full validation is enabled
ovie build --debug
```

---

## 11. Common Migration Issues

### Issue 1: "ORE not found" Error

**Symptom**: Compiler refuses to run  
**Cause**: Missing Runtime Environment structure  
**Solution**: Install complete v2.2 distribution package

### Issue 2: Exit Code 2 (Invariant Violation)

**Symptom**: Compilation fails with exit code 2  
**Cause**: Compiler bug detected  
**Solution**: Report to https://github.com/southwarridev/ovie/issues

### Issue 3: Structured Error Parsing Fails

**Symptom**: Error parsing scripts break  
**Cause**: Error format changed from strings to structured  
**Solution**: Update parsing to handle structured format (see Section 5)

### Issue 4: Missing Stdlib Functions

**Symptom**: Functions that worked in v2.1 are missing  
**Cause**: Placeholder functions removed, official implementations added  
**Solution**: Update imports to use std::* modules

---

## 12. Rollback Plan

If you need to rollback to v2.1:

### Step 1: Uninstall v2.2

```bash
# Remove v2.2 installation
rm -rf $OVIE_HOME  # Linux/macOS
# or
rmdir /s %OVIE_HOME%  # Windows
```

### Step 2: Reinstall v2.1

```bash
# Download v2.1 binary
wget https://github.com/southwarridev/ovie/releases/download/v2.1.0/oviec

# Install
chmod +x oviec
sudo mv oviec /usr/local/bin/
```

### Step 3: Verify

```bash
oviec --version
# Expected: oviec 2.1.0
```

---

## 13. Getting Help

### Resources

- **Documentation**: https://docs.ovie-lang.org
- **Migration FAQ**: https://docs.ovie-lang.org/migration/v2.2
- **Discord**: https://discord.gg/ovie-lang
- **GitHub Issues**: https://github.com/southwarridev/ovie/issues

### Reporting Issues

If you encounter migration issues:

1. Check this guide first
2. Search existing issues
3. Create new issue with:
   - v2.1 behavior
   - v2.2 behavior
   - Expected behavior
   - Minimal reproduction

---

## 14. Summary

### Key Changes

1. ✅ Install complete ORE package (not just binary)
2. ✅ Verify with `oviec --self-check`
3. ✅ Update error handling for structured errors
4. ✅ Update scripts for new exit codes
5. ✅ Use official stdlib implementations
6. ✅ Enable Aproko explanations

### Benefits

- **Enforced correctness** - no silent corruption
- **Complete stdlib** - no more placeholders
- **Self-diagnosing** - Aproko explains everything
- **Proven self-hosting** - verified with hashes
- **Stable tooling** - guaranteed CLI behavior

---

**Ovie v2.2 - Complete Language Consolidation**  
*Making migration smooth, one step at a time.*
