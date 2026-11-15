# Aproko: Read Your Errors Like a Pro in Ovie

Aproko is an Urhobo word — it means someone who notices everything and tells it straight.
In Ovie, Aproko runs before every program and explains every error in plain language.

## How Aproko Works

Every time you run oviec run file.ov, two things happen:

1. Aproko pre-flight — scans source for bad patterns BEFORE execution
2. Aproko explanation — if a runtime error occurs, shows the source line and explains it

## Error 1: Wrong keyword

If you type 'see', 'var', 'let', 'def' — Aproko catches it immediately:

    see name = "Nigeria"
    seeAm name

Aproko output (BEFORE the program runs):

    == Aproko =============================================
      [error] line 1: 'see' is not an Ovie keyword
        hint: oya see = value  or  mut see = value
    =====================================================

Aproko explanation (AFTER the runtime error):

    == Aproko Explanation ================================
      Line 1: see name = "Nigeria"
      Error: Undefined variable: see
      What happened: 'see' is not an Ovie keyword.
      Use oya (immutable) or mut (mutable). Try: oya name = value
    =====================================================

Fix:

    oya name = "Nigeria"   // immutable - use by default
    seeAm name

## Error 2: Variable not declared

    seeAm total_price

Aproko says:

    What happened: Variable 'total_price' is not declared.
    Use: oya total_price = value  or  mut total_price = value

Fix:

    oya total_price = 5000
    seeAm total_price

## Error 3: Function not defined

    oya x = calculate(10, 5)
    seeAm x

Aproko says:

    What happened: Function 'calculate' is not defined.
    Check spelling or define it first.

Fix:

    fn calculate(a: Number, b: Number) -> Number {
        return a + b
    }
    oya x = calculate(10, 5)
    seeAm x  // 15

## Error 4: Division by zero

    oya result = 100 / 0

Aproko pre-flight catches this BEFORE the program runs:

    [error] line 1: Division by literal zero - will crash at runtime
      hint: Guard it: if b != 0 { result = a / b }

## Error 5: read_line not trimmed

    mut input = read_line()
    if input == "quit" { seeAm "Bye!" }

Aproko:

    [warning] line 1: read_line() not trimmed - whitespace breaks comparisons
      hint: string_trim(read_line())

Fix:

    mut input = string_trim(read_line())
    if input == "quit" { seeAm "Bye!" }

## Clean code passes Aproko

    oya greeting = "Hello from Ovie!"
    seeAm greeting

Output:

    == Aproko =============================================
      OK: no issues found in hello.ov
    =====================================================
    Hello from Ovie!

## Keywords in Ovie

| If you wrote | What Aproko says | Fix |
|-------------|------------------|-----|
| see name = x | 'see' is not an Ovie keyword | oya name = x |
| var name = x | 'var' is not an Ovie keyword | oya name = x |
| let name = x | 'let' is not an Ovie keyword | oya name = x |
| def fn() {} | 'def' is not an Ovie keyword | fn name() {} |
| print(x) | OK — but use seeAm for output | seeAm x |
