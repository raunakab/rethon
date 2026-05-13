# L3 Tokenizer

Strips structural whitespace from the `L2Token` stream and attaches source
position metadata to every remaining token, producing `L3Token`s ready for
scope analysis.

## Input / Output

- **Input:** `impl Iterator<Item = Res<L2Token<'a>>>`
- **Output:** `impl Iterator<Item = Res<L3Token<'a>>>`

## What it does

### Position tracking

Each emitted `L3Token` carries a `Position`:

| Field | Meaning |
|---|---|
| `source_range` | Byte range in the original source string |
| `line` | Zero-indexed line number |
| `line_range` | Byte-column range within the current line |
| `indentation_level` | Number of `INDENTATION_SIZE`-space blocks at the start of this line |

### Newlines

A `Newline` token increments the line counter, resets `line_position` and
`indentation_level` to zero, and sets an `after_newline` flag. Newline tokens
are not emitted.

### Indentation

The first token on a new line is checked for leading whitespace:

- `Whitespace(n)` where `n % INDENTATION_SIZE == 0` sets
  `indentation_level = n / INDENTATION_SIZE` and advances `line_position`.
  The whitespace token itself is not emitted.
- Any other count returns `Error::InvalidIndentation`.

`INDENTATION_SIZE` is 4 spaces.

### Mid-line whitespace

Whitespace tokens that appear in the middle of a line (not after a newline) are
silently skipped; `line_position` is still advanced so `line_range` values
remain accurate.

### Braces

`Brace` tokens return `Error::UnexpectedBrace`. Explicit brace-based scoping is
not yet implemented.

## Errors

| Error | Condition |
|---|---|
| `Error::InvalidIndentation { found, position }` | Leading spaces not a multiple of `INDENTATION_SIZE` |
| `Error::UnexpectedBrace` | A brace character `(`, `)`, `[`, `]`, `{`, `}` was encountered |
| Any `L2` error | Propagated unchanged |
