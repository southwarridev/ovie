# Chapter 3: Language Fundamentals

This chapter covers everything you need to write real Ovie programs. We start with the two most important keywords — `oya` and `mut` — and build up from there.

---

## Variables: oya vs mut

Ovie has two ways to declare a variable.

**`oya` — immutable. Use this by default.**

```ovie
oya name    = "Shedrack"
oya version = 2.3
oya active  = true
```

Once you declare something with `oya`, its value is set. You cannot reassign it. This is not a limitation — it is a feature. When you read code later and see `oya`, you immediately know that value never changes anywhere in the program.

**`mut` — mutable. Use this when the value actually changes.**

```ovie
mut counter = 0
counter = counter + 1
seeAm counter  // 1
```

**Side by side:**

```ovie
oya MAX_SCORE  = 100      // fixed — never changes
oya GAME_NAME  = "Ovie Runner"

mut player_score = 0      // changes as the player plays
mut lives        = 3

player_score = player_score + 10
lives = lives - 1

seeAm player_score  // 10
seeAm lives         // 2
```

When to use which:
- App name, version, limits, error messages → `oya`
- Counters, accumulators, state that changes → `mut`
- When in doubt — start with `oya`. If the compiler complains you tried to reassign it, switch to `mut`.

---

## Types

Ovie infers types. The core types you will use:

| Type | What it holds | Example |
|------|--------------|---------|
| `Number` | integers and decimals | `42`, `3.14` |
| `String` | text | `"Hello"` |
| `Boolean` | true or false | `true`, `false` |
| `Array` | ordered list | `[1, 2, 3]` |

```ovie
oya count    = 42
oya pi       = 3.14
oya greeting = "Hello"
oya active   = true
oya scores   = [10, 20, 30]
```

You can add type annotations if you want to be explicit:

```ovie
oya name: String  = "Ovie"
oya age:  Number  = 25
mut flag: Boolean = false
```

---

## Functions

```ovie
fn add(a: Number, b: Number) -> Number {
    return a + b
}

mut result = add(10, 20)
seeAm result  // 30
```

Functions with no return value:

```ovie
fn greet(name: String) {
    seeAm "Hello, " + name
}

greet("Amina")  // Hello, Amina
```

Using `oya` inside a function for fixed values:

```ovie
fn circle_area(radius: Number) -> Number {
    oya PI = 3.14159
    return PI * radius * radius
}

seeAm circle_area(5.0)  // 78.53975
```

---

## Output and Input

Print a value:
```ovie
seeAm "Hello World"
seeAm 42
seeAm true
```

Print without newline (for prompts):
```ovie
print("Enter your name: ")
```

Read input from the user:
```ovie
print("What is your name? ")
mut name = read_line()
seeAm "Hello, " + name
```

---

## if / else

```ovie
mut score = 75

if score >= 90 {
    seeAm "Excellent"
} else if score >= 60 {
    seeAm "Good"
} else {
    seeAm "Keep going"
}
```

---

## while

```ovie
mut count = 0
while count < 5 {
    seeAm count
    count = count + 1
}
```

---

## for

```ovie
for i in 0..5 {
    seeAm i   // prints 0 1 2 3 4
}
```

---

## break and continue

```ovie
mut n = 0
while true {
    n = n + 1
    if n == 3 { continue }  // skip 3
    if n == 6 { break }     // stop at 6
    seeAm n
}
// prints 1 2 4 5
```

---

## struct

```ovie
struct Person {
    name: String,
    age: Number,
}

mut p = Person { name: "Amina", age: 28 }
seeAm p.name   // Amina
seeAm p.age    // 28
```

---

## enum

```ovie
enum Direction { North, South, East, West }

mut heading = Direction.North
seeAm heading   // North
```

---

## match

```ovie
enum Status { Active, Inactive, Pending }
mut s = Status.Active

match s {
    Status.Active   => seeAm "Running"
    Status.Inactive => seeAm "Stopped"
    Status.Pending  => seeAm "Waiting"
}
```

---

## Putting it Together

Here is a small program that uses everything in this chapter:

```ovie
// Config — all fixed, so oya
oya APP   = "Grade Calculator"
oya LIMIT = 100

fn grade(score: Number) -> String {
    if score >= 90 { return "A" }
    if score >= 80 { return "B" }
    if score >= 70 { return "C" }
    if score >= 60 { return "D" }
    return "F"
}

seeAm APP
seeAm "Enter scores (type 0 to finish):"

mut total = 0
mut count = 0

mut running = true
while running {
    print("> ")
    mut input = read_line()
    mut score = string_to_number(string_trim(input))

    if score == 0 {
        running = false
    } else {
        total = total + score
        count = count + 1
        seeAm "Grade: " + grade(score)
    }
}

if count > 0 {
    mut avg = total / count
    seeAm "Average: " + number_to_string(avg)
    seeAm "Overall: " + grade(avg)
} else {
    seeAm "No scores entered."
}
```

Notice: `APP`, `LIMIT` and `PI` use `oya` because they never change. `total`, `count`, `running` use `mut` because they are updated in the loop.

---

## Built-in String and Number Functions

| Function | What it does |
|----------|-------------|
| `string_length(s)` | length of a string |
| `string_trim(s)` | remove leading/trailing whitespace |
| `string_split(s, sep)` | split string into array by separator |
| `string_to_number(s)` | convert "42" to 42 |
| `number_to_string(n)` | convert 42 to "42" |
| `array_length(arr)` | number of items in array |
| `array_get(arr, i)` | get item at index i |
| `string_contains(s, sub)` | true if s contains sub |
| `string_replace(s, from, to)` | replace text in string |