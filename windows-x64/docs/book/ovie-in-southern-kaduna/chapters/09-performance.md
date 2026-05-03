# Chapter 9: Performance Optimization

## Measure First

Never optimize without measuring. Guessing where the bottleneck is wastes time.

```ovie
use std::time::{now, duration_ms}

fn benchmark(label: String, iterations: Number, fn_to_run: Function) {
    mut start = now()
    mut i = 0
    while i < iterations {
        fn_to_run()
        i = i + 1
    }
    mut elapsed = duration_ms(start, now())
    seeAm label + ": " + number_to_string(elapsed) + "ms for " + number_to_string(iterations) + " iterations"
}
```

## Memory Optimization

Avoid unnecessary allocations in hot paths:

```ovie
// Slow: creates new string on every iteration
fn build_slow(n: Number) -> String {
    mut result = ""
    mut i = 0
    while i < n {
        result = result + number_to_string(i) + ","
        i = i + 1
    }
    return result
}

// Better: collect into array, join once
fn build_fast(n: Number) -> String {
    mut parts = []
    mut i = 0
    while i < n {
        parts = array_push(parts, number_to_string(i))
        i = i + 1
    }
    return array_join(parts, ",")
}
```

## Algorithmic Optimization

Choose the right algorithm. A O(n²) algorithm on 10,000 items is 100x slower than O(n log n).

```ovie
// O(n²) — slow for large inputs
fn contains_slow(items: Array, target: String) -> Boolean {
    mut i = 0
    while i < array_length(items) {
        if array_get(items, i) == target {
            return true
        }
        i = i + 1
    }
    return false
}

// O(1) average — use a HashMap for repeated lookups
fn build_lookup(items: Array) -> HashMap {
    mut map = HashMap.new()
    mut i = 0
    while i < array_length(items) {
        map = HashMap.insert(map, array_get(items, i), true)
        i = i + 1
    }
    return map
}
```

## Compiler Optimizations

Use the WASM backend for production — it applies optimizations:

```bash
oviec build --backend wasm src/main.ov
```

The compiler pipeline (AST → HIR → MIR → Backend) applies:
- Dead code elimination
- Constant folding
- Inlining of small functions

## Module Loading Performance

The module cache makes incremental builds fast. Targets:
- Standard library load: < 100ms
- Incremental compilation: 10x faster than full build
- Cache hit per module: < 10ms

If builds are slow, check:

```bash
ovie cache clear   # Force full rebuild to measure baseline
ovie build         # Measure with cache
```

## Benchmarking

Write benchmarks alongside tests:

```ovie
use std::time::{now, duration_ms}
use std::testing::{assert_true}

fn bench_sort() {
    mut data = [5, 3, 8, 1, 9, 2, 7, 4, 6, 0]
    mut start = now()
    mut iterations = 10000
    mut i = 0
    while i < iterations {
        sort(data)
        i = i + 1
    }
    mut elapsed = duration_ms(start, now())
    mut per_op = elapsed / iterations
    seeAm "sort: " + number_to_string(per_op) + "ms per operation"
    assert_true(per_op < 1, "sort should complete in < 1ms")
}

bench_sort()
```

## Common Pitfalls

- String concatenation in loops — use array + join
- Repeated file reads — cache the result
- Unnecessary module reloads — trust the cache
- Unneeded unsafe blocks — they bypass optimizations
- Large dependency trees — keep `ovie.toml` lean
