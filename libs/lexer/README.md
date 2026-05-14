# lexer

Turns Rethon source text into a flat stream of tokens.

## Pipeline

Lexing runs in two internal stages:

1. **`lexer_s1`** — grapheme clustering. Walks the source string and groups characters into raw chunks: contiguous alphabetic runs, numeric runs, individual punctuation characters, whitespace, and quoted strings.

2. **`lexer_s2`** — token resolution. Consumes the s1 stream and resolves multi-character operators (`:=`, `->`, `|>>`, …), keywords (`fn`, `if`, `return`, …), formatted strings (`f"…"`), and floats (`1.5`).

## Public API

```rust
pub fn lex(source: &str) -> impl Iterator<Item = Res<LexToken<'_>>>
```

Each `LexToken` carries:

- `kind: LexKind<'_>` — what the token is
- `range: Range<usize>` — byte range in the source string

`LexKind` variants:

| Variant | Meaning |
|---|---|
| `Normal(TokenType)` | A keyword, identifier, operator, number, or string |
| `Whitespace(usize)` | One or more consecutive spaces (count) |
| `Newline` | `\n` or `\r\n` |
| `Brace(Brace, BraceDirection)` | `(`, `)`, `[`, `]`, `{`, `}` |

Whitespace and newlines are preserved in the output so downstream stages can use them for indentation tracking.

## Errors

| Error | Cause |
|---|---|
| `InvalidWhitespace(String)` | Tab or bare `\r` used as whitespace |
| `UnknownToken(String)` | Unrecognised character (e.g. `'`, `` ` ``, control characters) |
| `UnterminatedString(usize)` | String literal with no closing `"` (byte offset of the opening quote) |
