# L2 Tokenizer

Consumes the `L1Token` stream and resolves each token into a typed `L2Token`,
handling multi-character operators, keyword recognition, number/float
disambiguation, and whitespace classification.

## Input / Output

- **Input:** `impl Iterator<Item = Res<L1Token<'a>>>`
- **Output:** `impl Iterator<Item = Res<L2Token<'a>>>`

## Token types produced

| `L2TokenType` variant | Description |
|---|---|
| `Normal(TokenType)` | Any fully-resolved language token |
| `Whitespace(usize)` | Run of spaces; the `usize` is the count |
| `Newline` | A `\n` character |
| `Brace(Brace, BraceDirection)` | One of `()`, `[]`, `{}` (open or close) |

## Resolution rules

### Whitespace
- `' '` — counted with lookahead; consecutive spaces become `Whitespace(n)`.
- `'\n'` or `'\r\n'` — emitted as `Newline`.
- Any other whitespace (e.g. `'\t'`, standalone `'\r'`) — `Error::InvalidWhitespace`.

### Strings
- A bare `String` L1 token becomes `TokenType::String(content, Normal)`.
- A `Keyword("f")` immediately followed by a `String` L1 token becomes
  `TokenType::String(content, Formatted)`.

### Keywords vs. identifiers
Each `Keyword` L1 token is matched against the full list of reserved words
(`fn`, `if`, `else`, `return`, …). Unrecognised words become
`TokenType::Identifier`.

### Numbers vs. floats
A `Numeric` token peeks ahead for a `.` and then optional further digits:
- `12` → `Number("12")`
- `12.34` → `Float("12", Some("34"))`
- `12.` → `Float("12", None)`

### Multi-character operators
A `peek!` macro checks the next token before committing, enabling:

| Input | Result |
|---|---|
| `==` | `Equals` |
| `:=` | `ConstantAssignment` |
| `->` | `Arrow` |
| `--` | `DoubleMinus` |
| `**` | `DoubleAsterisk` |
| `..` | `DoubleDot` |
| `\|>` | `PipeForward` |
| `\|>>` | `PipeDoubleForward` |
| `>=` / `>>` | `GreaterOrEqual` / `DoubleGreater` |
| `<=` / `<<` | `LesserOrEqual` / `DoubleLesser` |
| `!ident` | `MacroIdentifier(ident)` |

Single-character forms are emitted when the lookahead doesn't match.

## Errors

| Error | Condition |
|---|---|
| `Error::InvalidWhitespace(s)` | Non-space, non-newline whitespace encountered |
| `Error::UnterminatedString(byte_offset)` | Propagated from L1 |
| `Error::UnknownToken(s)` | `Unknown` L1 token, or unrecognised punctuation |
