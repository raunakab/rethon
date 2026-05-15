# Parser

Converts a `TokenTree` stream (from the lexer) into an AST.

## Grammar notation

The full grammar lives in [`GRAMMAR.md`](./GRAMMAR.md). A few conventions used there:

| Notation | Meaning |
|---|---|
| `$x` | a single required `x` |
| `$(x)?` | `x` is optional |
| `$($x)*` | zero or more `x` |
| `$($x)+` | one or more `x` |
| `$($x)[;]*` | zero or more `x` separated by **newlines or semicolons** |
| `$($x)[|]*` | zero or more `x` separated by **newlines or pipes** |

### Delimiter equivalence

Newlines and semicolons are interchangeable as statement separators. The following two forms are always equivalent:

```
x := 1
y := 2
z := 3
```

```
x := 1; y := 2; z := 3
```

This applies anywhere the grammar uses `[;]`. The scoper layer of the lexer handles indentation-based scoping (Python-style), and semicolons can be used to write inline what would otherwise be a multi-line indented block.

Similarly, `[|]` allows enum variants to be written either vertically (one per line) or horizontally separated by `|`.

## Scoping

Scopes are delimited either by indentation or by explicit braces (`{}`, `()`, `[]`). The lexer emits:

- `Scope((Open, None))` / `Scope((Close, None))` — implicit indentation boundary
- `Scope((Open, Some((brace, pos))))` / `Scope((Close, Some((brace, pos))))` — explicit brace

The parser uses indentation level to determine which items belong to a given scope.

## Key constructs

### Definitions

```
$(mut) $pat $(: $type)? = $expr $(else $expr)?
```

The optional `: $type` annotation is a type ascription. The optional `else` branch handles pattern-match failure.

### `if`/`else`

```
if x
    expr
else if y
    expr
else
    expr
```

Or inline: `if x; expr; else; expr`

### `match`

```
match expr
    pat $(if guard)? => expr
    pat => expr
```

### `loop`

Three forms:
- `loop expr` — infinite loop
- `loop (cond) expr` — while-style
- `loop (pat in iter) expr` — for-style

### Functions

```
fn (param: Type, param2: Type) -> ReturnType
    body
```

### Structs and enums

```
struct Point
    x: Int
    y: Int
```

```
enum Shape
    | Circle(Float)
    | Rect{ width: Float, height: Float }
```

## Current state

The parser is a work in progress. Parsing is implemented for:

- `Definition` (pattern, optional type, value, optional pattern-match-fail branch)
- `Pattern::CatchAll` (identifier catch-all)

Not yet implemented: full expression parsing, `if`/`match`/`loop`/`fn`/`struct`/`enum`, operator expressions, function invocation, string/number literals in expressions, macros.
