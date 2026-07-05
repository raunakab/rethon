# Rethon

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

## Grammar

Rethon uses an indentation-based, colon-delimited syntax. The grammar is written in a BNF-like notation where `$x` denotes a non-terminal, `$(...)` denotes repetition, `[sep]*` / `[sep]+` denote zero-or-more / one-or-more with a separator, and `?` denotes optionality.

```
block ::=
  $($item)[;]*

item ::=
  | $statement
  | $expr

statement ::=
  | $ident $(: $type)? := $block                           -- static (immutable) binding
  | $(mut) $pat $(: $type)? = $block $(else $block)?       -- normal (mutable) binding

expr ::=
  -- primitives
  | $ident
  | $literal
  | ($($expr),+)                    -- tuple (requires at least one comma)
  | [$($expr),*]                    -- list
  | [[$($expr),*]]                  -- set
  | {$($expr:$expr),*}              -- map

  -- control flow
  | $if-else
  | $match
  | $loop

  -- functions
  | $function
  | $function-invocation
  | return $($block)?
  | yield $($block)?
  | throw $($block)?

  -- type definitions
  | $struct
  | $enum
  | $ident { $($ident $(: $expr)?)[,]* }

  -- impl holes
  | panic
  | todo
  | unimplemented

  -- other
  | $expr: $type                    -- type ascription
  | $block
  | $macro

if-else ::=
  if $block:
    $block
  $(else if $block:
    $block)*
  $(else:
    $block)?

match ::=
  match $block:
    $($pat $(if $block)? => $block)[,]*

loop ::=
  | loop: $block                          -- unconditional
  | loop $block: $block                   -- conditional (while)
  | loop $pat in $block: $block           -- iterative (for)

function ::=
  fn $($ident $(: $type)?,)* $(-> $type)?:
    $($block)?

function-invocation ::=
  | $block($($block),*)                           -- positional args
  | $block($($ident=$block),*)                    -- keyword args
  | $block($($block,)+ $($ident=$block),*)        -- positional then keyword

struct ::=
  struct:
    $($ident: $type)[;]*

enum ::=
  enum:
    $($enum-variant)[|]*

enum-variant ::=
  | $ident
  | $ident($($type),*)
  | $ident{$($ident: $type),*}

pat ::=
  | $ident
  | $ident @ $pat                         -- binding pattern
  | $pat | $pat                           -- or pattern
  | ($pat)
  | _                                     -- wildcard
  | $literal
  | ($($pat),+)                           -- tuple pattern
  | [$($pat),*]                           -- list pattern
  | {$($literal:$pat),*}                  -- map pattern
  | $ident($($pat),*)                     -- enum tuple-variant pattern
  | $ident { $($ident$(: $pat)?),* }      -- enum struct-variant pattern

literal ::=
  | true
  | false
  | $number
  | $float
  | $string
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
