# Chapter 5: Standard Library Reference

## std::core

Core types for safe, expressive programming.

```ovie
use std::core::{Result, Option, Vec, HashMap}
```

### Result

```ovie
fn divide(a: Number, b: Number) -> Result {
    if b == 0 {
        return Result.Err("Division by zero")
    }
    return Result.Ok(a / b)
}

mut r = divide(10, 2)
if r.is_ok() {
    seeAm r.unwrap()
} else {
    seeAm r.unwrap_err()
}
```

### Option

```ovie
fn find(items: Array, target: String) -> Option {
    mut i = 0
    while i < array_length(items) {
        if array_get(items, i) == target {
            return Option.Some(i)
        }
        i = i + 1
    }
    return Option.None()
}

mut idx = find(["a", "b", "c"], "b")
if idx.is_some() {
    seeAm "Found at index: " + number_to_string(idx.unwrap())
}
```

### Vec

```ovie
mut v = Vec.new()
v = Vec.push(v, 1)
v = Vec.push(v, 2)
v = Vec.push(v, 3)
seeAm Vec.length(v)
```

### HashMap

```ovie
mut map = HashMap.new()
map = HashMap.insert(map, "name", "Ovie")
map = HashMap.insert(map, "version", "2.3")
seeAm HashMap.get(map, "name")
```

## std::io

Input and output operations.

```ovie
use std::io::{println, print, read_line}

println("Enter your name:")
mut name = read_line()
println("Hello, " + name + "!")
```

## std::fs

File system operations.

```ovie
use std::fs::{read_file, write_file, file_exists, make_dir}

if file_exists("data.txt") {
    mut content = read_file("data.txt")
    seeAm content
} else {
    write_file("data.txt", "Hello from Ovie!")
}

make_dir("output")
```

## std::math

Mathematical functions.

```ovie
use std::math::{sqrt, pow, abs, floor, ceil, min, max}

seeAm sqrt(16.0)      // 4.0
seeAm pow(2.0, 10.0)  // 1024.0
seeAm abs(-42)        // 42
seeAm floor(3.7)      // 3.0
seeAm ceil(3.2)       // 4.0
seeAm min(5, 3)       // 3
seeAm max(5, 3)       // 5
```

## std::time

Time and duration utilities.

```ovie
use std::time::{now, duration_ms, sleep_ms}

mut start = now()
// ... do work ...
mut elapsed = duration_ms(start, now())
seeAm "Elapsed: " + number_to_string(elapsed) + "ms"
```

## std::env

Environment variables and program arguments.

```ovie
use std::env::{get_var, args, hostname}

mut home = get_var("HOME")
mut program_args = args()
mut host = hostname()

seeAm "Running on: " + host
seeAm "HOME: " + home
```

## std::cli

Command-line interface utilities.

```ovie
use std::cli::{parse_args, print_usage, flag_value}

mut cli = parse_args(args())
mut output = flag_value(cli, "--output", "result.txt")
seeAm "Output file: " + output
```

## std::log

Structured logging.

```ovie
use std::log::{info, warn, error, debug}

info("Server started on port 8080")
warn("Config file not found, using defaults")
error("Failed to connect to database")
debug("Processing item: " + item_id)
```

Log levels: `debug` < `info` < `warn` < `error`. Configure minimum level in `.ovie/aproko.toml`.

## std::testing

Testing framework.

```ovie
use std::testing::{assert_eq, assert_true, assert_false, assert_not_null}

fn test_add() {
    assert_eq(add(2, 3), 5, "2 + 3 should equal 5")
    assert_true(add(0, 0) == 0, "0 + 0 should be 0")
}

test_add()
```

Run tests:

```bash
ovie test
```

## std::module

Module system utilities (new in v2.3).

```ovie
use std::module::{load_module, resolve, kb_query}

// Load a module dynamically
mut mod = load_module("./plugins/my_plugin.ov")

// Query the knowledge base
mut types = kb_query({ category: "TypeInformation", symbol_name: "add" })
```

## std::aproko

Aproko knowledge base access.

```ovie
use std::aproko::{store_entry, get_type_info, kb_entry_new}

// Store analysis result
mut entry = kb_entry_new("my_fn", "TypeInformation", "{\"returns\":\"Number\"}")
entry.symbol_name = "my_fn"
store_entry(entry)

// Query type info
mut info = get_type_info("my_fn")
```
