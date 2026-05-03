# Chapter 8: Testing and Quality Assurance

## Unit Testing

Write test functions using `std::testing`:

```ovie
use std::testing::{assert_eq, assert_true, assert_false}
use std::core::{Result}

fn add(a: Number, b: Number) -> Number {
    return a + b
}

fn test_add_positive() {
    assert_eq(add(2, 3), 5, "2 + 3 = 5")
    assert_eq(add(0, 0), 0, "0 + 0 = 0")
    assert_eq(add(100, 200), 300, "100 + 200 = 300")
}

fn test_add_negative() {
    assert_eq(add(-1, 1), 0, "-1 + 1 = 0")
    assert_eq(add(-5, -3), -8, "-5 + -3 = -8")
}

test_add_positive()
test_add_negative()
seeAm "All tests passed"
```

Run tests:

```bash
ovie test
```

## Test Organization

Keep tests close to the code they test. For a module `src/math.ov`, create `tests/math_test.ov`:

```ovie
import "../src/math.ov"
use std::testing::{assert_eq}

fn test_sqrt() {
    assert_eq(sqrt(4.0), 2.0, "sqrt(4) = 2")
    assert_eq(sqrt(9.0), 3.0, "sqrt(9) = 3")
    assert_eq(sqrt(0.0), 0.0, "sqrt(0) = 0")
}

test_sqrt()
```

## Property-Based Testing

Property tests verify that a function satisfies a universal property across many inputs. Ovie's compiler itself uses property-based tests extensively.

The idea: instead of testing specific examples, define a property that must always hold:

```ovie
use std::testing::{assert_true}

// Property: add is commutative
fn prop_add_commutative(a: Number, b: Number) -> Boolean {
    return add(a, b) == add(b, a)
}

// Test with many values
mut i = -50
while i <= 50 {
    mut j = -50
    while j <= 50 {
        assert_true(prop_add_commutative(i, j), "add must be commutative")
        j = j + 1
    }
    i = i + 1
}
seeAm "Commutativity property holds"
```

## Integration Testing

Test how modules work together:

```ovie
use std::fs::{write_file, read_file, file_exists}
use std::testing::{assert_eq, assert_true}

fn test_file_roundtrip() {
    mut path = "/tmp/test_ovie.txt"
    mut content = "Hello from Ovie!"

    write_file(path, content)
    assert_true(file_exists(path), "File should exist after write")

    mut read_back = read_file(path)
    assert_eq(read_back, content, "Read content should match written content")
}

test_file_roundtrip()
```

## Test Coverage

Aim for coverage of:
- Happy path (normal inputs)
- Edge cases (empty, zero, max values)
- Error cases (invalid inputs, missing files)
- Boundary conditions (off-by-one, overflow)

```ovie
fn test_divide_edge_cases() {
    // Normal case
    assert_eq(divide(10, 2).unwrap(), 5, "10/2 = 5")

    // Division by zero
    assert_true(divide(10, 0).is_err(), "10/0 should error")

    // Zero numerator
    assert_eq(divide(0, 5).unwrap(), 0, "0/5 = 0")
}
```

## Continuous Integration

The project uses GitHub Actions and GitLab CI. Your CI pipeline should:

1. Build the project
2. Run all tests
3. Run Aproko analysis
4. Check documentation completeness
5. Validate book examples

Example CI step:

```bash
ovie test
oviec analyze src/main.ov
ovie doc check
ovie book test
```

## Quality Metrics

Before releasing:
- All tests pass
- No `Error` or `Critical` Aproko findings
- All exported functions documented
- Book examples compile and run
- Build is reproducible (`oviec --self-check`)
