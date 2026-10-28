---
title: "Build a Calculator in Ovie — A Hands-On Intro to the oya Keyword"
published: false
description: "Learn Ovie's natural immutable keyword oya by building an interactive terminal calculator from scratch."
tags: ovie, beginners, systems-programming, tutorial
cover_image: https://ovie.nashedy.io/assets/ovie.png
---

# Build a Calculator in Ovie — Learning the oya Keyword

Ovie is a systems programming language that borrows natural language patterns. One of the keywords that sets it apart is `oya` — Ovie's word for immutable variable declaration.

This tutorial builds an interactive calculator step by step, introducing `oya` as we go.

## Setup

You need Ovie installed. On Windows:

```powershell
# Run as Administrator
iwr -useb https://raw.githubusercontent.com/southwarridev/ovie/main/easy-windows-install.ps1 | iex
```

Or build from source:
```bash
git clone https://github.com/couragereward/ovie.git
cd ovie
cargo build --release --bin oviec
```

Verify:
```
oviec --version
```

## Step 1: Print something

Create `calculator.ov`:

```ovie
seeAm "OVIE CALCULATOR"
```

Run it:
```
oviec run calculator.ov
```

`seeAm` is Ovie's print — from Nigerian Pidgin English for "see for yourself."

## Step 2: Add oya constants

Now let us add the fixed strings our calculator needs. These never change — so they belong to `oya`:

```ovie
oya APP_NAME  = "OVIE CALCULATOR v1.0"
oya BORDER    = "==============================="
oya QUIT_MSG  = "Type q to quit"
oya BYE_MSG   = "Bye!"
oya PROMPT    = "> "

seeAm BORDER
seeAm "   " + APP_NAME
seeAm BORDER
seeAm QUIT_MSG
```

`oya` says: this value is fixed. If you try to reassign it later, Ovie stops you. That is the point — it makes intent clear.

## Step 3: Add mut for things that change

`mut` is for values that will be reassigned:

```ovie
mut running = true
```

The calculator loop will flip `running` to false when the user types q.

## Step 4: The calculate function

```ovie
fn calculate(a: Number, op: String, b: Number) -> String {
    oya DIV_ZERO = "Error: Division by zero"

    if op == "+" { return number_to_string(a + b) }
    if op == "-" { return number_to_string(a - b) }
    if op == "*" { return number_to_string(a * b) }
    if op == "/" {
        if b == 0 { return DIV_ZERO }
        return number_to_string(a / b)
    }
    if op == "%" {
        if b == 0 { return DIV_ZERO }
        return number_to_string(a % b)
    }
    return "Error: Unknown operator"
}
```

Notice `DIV_ZERO` inside the function — it is `oya` because it is a fixed error message.

## Step 5: The input loop

```ovie
while running {
    print(PROMPT)
    mut input = read_line()
    mut trimmed = string_trim(input)

    if trimmed == "q" {
        seeAm BYE_MSG
        running = false
    } else if trimmed == "" {
        // skip empty lines
    } else {
        mut parts = string_split(trimmed, " ")
        if array_length(parts) != 3 {
            seeAm "Usage: <number> <op> <number>   e.g.  10 + 5"
        } else {
            mut a_str  = array_get(parts, 0)
            mut op     = array_get(parts, 1)
            mut b_str  = array_get(parts, 2)
            mut a      = string_to_number(a_str)
            mut b      = string_to_number(b_str)
            mut result = calculate(a, op, b)
            seeAm "= " + result
        }
    }
}
```

## Full Program

```ovie
oya APP_NAME   = "OVIE CALCULATOR v1.0"
oya BORDER     = "==============================="
oya QUIT_CMD   = "q"
oya USAGE_MSG  = "Usage: <number> <op> <number>   e.g.  10 + 5"
oya BYE_MSG    = "Bye!"
oya PROMPT     = "> "
oya DIV_ZERO   = "Error: Division by zero"
oya UNKNOWN_OP = "Error: Unknown operator"

fn calculate(a: Number, op: String, b: Number) -> String {
    if op == "+" { return number_to_string(a + b) }
    if op == "-" { return number_to_string(a - b) }
    if op == "*" { return number_to_string(a * b) }
    if op == "/" {
        if b == 0 { return DIV_ZERO }
        return number_to_string(a / b)
    }
    if op == "%" {
        if b == 0 { return DIV_ZERO }
        return number_to_string(a % b)
    }
    return UNKNOWN_OP
}

seeAm BORDER
seeAm "   " + APP_NAME
seeAm BORDER
seeAm "Enter: <number> <op> <number>"
seeAm "Operators:  + - * / %"
seeAm "Type q to quit"
seeAm BORDER
seeAm ""

mut running = true

while running {
    print(PROMPT)
    mut input = read_line()
    mut trimmed = string_trim(input)

    if trimmed == QUIT_CMD {
        seeAm BYE_MSG
        running = false
    } else if trimmed == "" {
        // skip
    } else {
        mut parts = string_split(trimmed, " ")
        if array_length(parts) != 3 {
            seeAm USAGE_MSG
        } else {
            mut a      = string_to_number(array_get(parts, 0))
            mut op     = array_get(parts, 1)
            mut b      = string_to_number(array_get(parts, 2))
            mut result = calculate(a, op, b)
            seeAm "= " + result
        }
    }
}
```

## Try it

```
> 10 + 5
= 15
> 9 * 9
= 81
> 17 % 5
= 2
> 10 / 0
= Error: Division by zero
> q
Bye!
```

## What we learned

- `oya` declares immutable values — use it for anything that does not change
- `mut` declares mutable values — use it for loop state and things that update
- `seeAm` prints to the terminal
- `print()` prints without a newline (for prompts)
- `read_line()` reads input from the user
- `string_split`, `string_trim`, `string_to_number`, `number_to_string` are built-in string utilities

## What to try next

- Add a history feature — store previous calculations in an array
- Add support for expressions like `(10 + 5) * 2`
- Build a unit converter using the same pattern
- Add `oya` constants for operator symbols and reuse them in the help text