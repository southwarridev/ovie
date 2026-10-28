# Getting Started with Ovie

Ovie is a systems programming language with natural syntax. This guide gets you writing real programs in under 10 minutes.

## Install

**Windows (PowerShell as Admin):**
```powershell
iwr -useb https://raw.githubusercontent.com/southwarridev/ovie/main/easy-windows-install.ps1 | iex
```

**Linux:**
```bash
curl -sSL https://raw.githubusercontent.com/southwarridev/ovie/main/easy-linux-install.sh | bash
```

**macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/southwarridev/ovie/main/easy-macos-install.sh | bash
```

**From source:**
```bash
git clone https://github.com/couragereward/ovie.git
cd ovie
cargo build --release --bin oviec
```

## Verify

```
oviec --version
oviec --self-check
```

## Your First Program

Create `hello.ov`:

```ovie
oya greeting = "Hello from Ovie!"
seeAm greeting
```

Run it:

```
oviec run hello.ov
```

Output:
```
Hello from Ovie!
```

## The Two Declaration Keywords

The first thing to learn in Ovie:

```ovie
oya name  = "Ovie"    // immutable — never changes
mut count = 0         // mutable — can be reassigned
```

**`oya`** is for values that are fixed. App names, constants, messages, config. Use it by default.

**`mut`** is for values that change. Counters, state, user input results.

```ovie
oya VERSION = "2.3"
oya LIMIT   = 100

mut score = 0
score = score + 10
seeAm "Score: " + number_to_string(score)
```

## A Complete First Program

```ovie
oya WELCOME = "Welcome to Ovie!"
oya PROMPT  = "Enter your name: "

seeAm WELCOME
print(PROMPT)
mut name = read_line()
seeAm "Hello, " + name + "!"
```

Run it:
```
oviec run greet.ov
```

## Next Steps

- [Language Guide](language-guide.md) — complete syntax reference
- [The Ovie Book](book/ovie-in-southern-kaduna/INDEX.md) — learn by building
- [Examples](../examples/) — real programs to explore
- `oviec run examples/hello.ov` — run an example right now

## Quick Reference

```ovie
oya x = 10              // immutable
mut y = 0               // mutable

fn add(a: Number, b: Number) -> Number {
    return a + b
}

if x > 5 {
    seeAm "big"
}

for i in 0..3 {
    seeAm i
}

mut n = 0
while n < 3 {
    n = n + 1
    seeAm n
}
```