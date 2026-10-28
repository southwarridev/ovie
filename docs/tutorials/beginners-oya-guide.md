# Beginner Guide: The oya Keyword in Ovie

`oya` is Ovie is immutable declaration keyword. This guide explains what it means, why it matters, and how to use it.

---

## What "immutable" means in plain English

Immutable means: once you set it, it stays set.

Think of `oya` like writing something in permanent marker. `mut` is like writing in pencil — you can erase and rewrite it.

```ovie
oya city = "Lagos"    // written in permanent marker
mut score = 0         // written in pencil
```

If you try to change an `oya` variable:
```ovie
oya city = "Lagos"
city = "Abuja"        // Ovie will stop you with an error
```

That is a feature, not a bug. It tells you "this was not meant to change."

---

## Program 1: Hello with oya

```ovie
oya greeting = "Hello, Ovie!"
oya author   = "Shedrack"

seeAm greeting
seeAm "Written by " + author
```

Run:
```
oviec run hello.ov
```

Output:
```
Hello, Ovie!
Written by Shedrack
```

---

## Program 2: oya for configuration constants

Real programs have config values that never change — version numbers, limits, app names. These belong to `oya`:

```ovie
oya APP_NAME    = "Student Register"
oya MAX_SCORE   = 100
oya PASS_SCORE  = 40
oya SCHOOL_NAME = "Kaduna Polytechnic"

seeAm APP_NAME + " — " + SCHOOL_NAME
seeAm "Maximum score: " + number_to_string(MAX_SCORE)
seeAm "Pass mark: " + number_to_string(PASS_SCORE)
```

---

## Program 3: Mixing oya and mut in a counter

```ovie
oya TARGET = 5        // the goal never changes
mut count  = 0        // the count changes every loop

seeAm "Counting to " + number_to_string(TARGET)

while count < TARGET {
    count = count + 1
    seeAm count
}

seeAm "Done!"
```

Output:
```
Counting to 5
1
2
3
4
5
Done!
```

---

## Program 4: oya inside a function

You can use `oya` inside a function for values that are fixed within that function:

```ovie
fn describe_temperature(temp: Number) -> String {
    oya FREEZING = 0
    oya COLD     = 15
    oya WARM     = 25

    if temp <= FREEZING { return "Freezing" }
    if temp <= COLD     { return "Cold" }
    if temp <= WARM     { return "Warm" }
    return "Hot"
}

seeAm describe_temperature(-5)   // Freezing
seeAm describe_temperature(10)   // Cold
seeAm describe_temperature(22)   // Warm
seeAm describe_temperature(38)   // Hot
```

---

## Program 5: The Calculator — oya for all fixed strings

This is the full calculator from the tutorial. Every fixed string uses `oya`. Only the loop state and user input use `mut`:

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
seeAm ""

mut running = true

while running {
    print(PROMPT)
    mut input   = read_line()
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
            seeAm "= " + calculate(a, op, b)
        }
    }
}
```

---

## When to use oya vs mut

| Use `oya` when... | Use `mut` when... |
|-------------------|-------------------|
| The value never changes | The value will be reassigned |
| It is a constant (PI, MAX, VERSION) | It is a counter or accumulator |
| It is a fixed message or label | It holds user input |
| It is config that should not drift | It is loop state |
| You want the compiler to protect it | You intentionally update it |

**Default rule:** start with `oya`. If you get a compiler error saying you tried to reassign it, change it to `mut`.