# tokenizer

Converts a Rethon source string into a stream of typed tokens.

## Pipeline

Tokenization happens in four passes, each building on the previous:

```
source &str
   │
   ▼  L1 — grapheme clustering
   │       groups raw Unicode graphemes into runs of the same character class
   │       (Keyword, Numeric, Punctuation, Whitespace, String, Unknown)
   │
   ▼  L2 — operator / keyword resolution
   │       maps grapheme runs to concrete TokenType variants
   │       resolves multi-character operators (`:=`, `|>>`, `**`, …)
   │       parses string literals and floating-point numbers
   │       strips intra-line whitespace, emits Newline tokens
   │
   ▼  L3 — position metadata + indentation validation
   │       attaches line number, column range, and indentation level
   │       validates that every indented line is a multiple of 4 spaces
   │       rejects bare braces (`{`, `}`)
   │
   ▼  L4 — indent-based scope insertion
           converts indentation level changes into ScopeStart / ScopeEnd tokens
           multiple consecutive levels may be opened at once (jump indentation)
           closes all open scopes at end-of-input
```

## Public API

```rust
pub fn tokenize(source: &str) -> impl Iterator<Item = Res<Token<'_>>>;
```

Returns a peekable iterator that lazily produces `Token` values. All lifetime
annotations borrow from `source`, so no allocation is needed for identifiers or
string contents.

### Token variants

```rust
pub enum Token<'a> {
    Token(TokenType<'a>, Position),  // a concrete token with position metadata
    ScopeStart(Option<(Brace, Position)>),  // indentation increased (or explicit brace opened)
    ScopeEnd(Option<Position>),             // indentation decreased (or explicit brace closed)
}
```

`TokenType` covers all Rethon tokens: identifiers, keywords, operators, literals,
and macro invocations. See its definition in `lib.rs` for the full list.

`Position` carries the byte range in the source, the line number, the byte range
within that line, and the indentation level.

### Error variants

```rust
pub enum Error {
    InvalidWhitespace(String),             // tab or bare \r
    UnknownToken(String),                  // unrecognised grapheme
    UnterminatedString(usize),             // opening quote with no closing quote (byte offset)
    InvalidIndentation { found, position },// indentation not a multiple of 4
    UnexpectedBrace,                       // bare `{` or `}` in source
}
```

### Helper macro

`tokens!()` expands to the concrete iterator type, useful in function signatures:

```rust
fn parse<'a>(tokens: &mut tokens!('a)) { … }
```

## Usage

```rust
use tokenizer::{tokenize, Token, TokenType};

for result in tokenize(source) {
    match result? {
        Token::Token(ty, pos) => println!("{ty} at line {}", pos.line),
        Token::ScopeStart(_) => println!("{{"),
        Token::ScopeEnd(_)   => println!("}}"),
    }
}
```

## Indentation rules

- One indentation level = 4 spaces.
- Tabs are rejected (`InvalidWhitespace`).
- Blank lines are ignored for scoping purposes.
- CRLF (`\r\n`) is treated identically to LF (`\n`).
