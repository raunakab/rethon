# Rethon

[![CI](https://github.com/raunakab/rethon/actions/workflows/test-and-lint.yml/badge.svg)](https://github.com/raunakab/rethon/actions/workflows/test-and-lint.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

Python re-imagined.

## Features

- Static, Turing-incomplete type system
- Ultra-aggressive type-inference
- Algebraic datatypes
- Concise, pythonic syntax
- First-class expressions
- Advanced pattern matching
- Precedence on _pure, functional programming_ (with some leeway for mutability)
- Macros
- Arc based memory-management (no garbage collection)

## Examples

Bindings are immutable by default. `:=` is a static (immutable) binding; `=` is mutable.

```
x := 42
mut count = 0
```

Functions are expressions. The body is the last evaluated expression.

```
factorial := fn n: int -> int:
    match n:
        0 => 1,
        n => n * factorial(n - 1),
```

Algebraic datatypes with pattern matching:

```
Shape := enum:
    Circle(float)
    | Rect(float, float)
    | Triangle(float, float, float)

area := fn shape: Shape -> float:
    match shape:
        Circle(r)        => 3.14159 * r * r,
        Rect(w, h)       => w * h,
        Triangle(a, b, c):
            s := (a + b + c) / 2.0
            (s * (s - a) * (s - b) * (s - c)) ** 0.5,
```

`if` and `match` are expressions — they can appear anywhere a value is expected:

```
sign := fn x: int -> str:
    if x > 0: "positive"
    else if x < 0: "negative"
    else: "zero"
```

Loops come in three forms — unconditional, conditional, and iterative:

```
loop:
    do_something()

loop not_done():
    step()

loop item in collection:
    process(item)
```

## Inspiration

- Rust
  - Algebraic datatypes
  - First class expressions
  - Pattern matching
  - Macros
- Python
  - Syntax
  - Generators
- Swift
  - Arc based memory-management
- Go
  - Brevity
  - Async generators
