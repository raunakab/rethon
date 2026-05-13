# L4 Tokenizer

Converts indentation changes in the `L3Token` stream into explicit
`ScopeStart` / `ScopeEnd` markers, producing the final `Token` stream
consumed by the parser.

## Input / Output

- **Input:** `impl Iterator<Item = Res<L3Token<'a>>>`
- **Output:** `impl Iterator<Item = Res<Token<'a>>>`

## Algorithm

An `indent_stack` (initially `[0]`) tracks the currently open indentation
levels. Before emitting each `L3Token`, the tokenizer compares the token's
`indentation_level` against the top of the stack.

### Indent increase

When `next_indent > current_indent`:

1. All intermediate levels are pushed onto `indent_stack` (supporting jumps of
   more than one level).
2. One `ScopeStart(None)` is queued per new level.
3. The queued `ScopeStart`s are emitted one per iteration before the triggering
   token is consumed.

### Indent decrease

When `next_indent < current_indent`:

1. Stack entries are popped while the top is greater than `next_indent`.
2. One `ScopeEnd(None)` is emitted per popped entry, again one per iteration.

### Same level

No scope tokens are emitted; the `L3Token` is forwarded as `Token::Token`
directly.

### End of stream

When the underlying iterator is exhausted, remaining stack entries above level 0
are popped and a `ScopeEnd(None)` is emitted for each.

## Output token variants

| Variant | When emitted |
|---|---|
| `Token::Token(TokenType, Position)` | Normal token passthrough |
| `Token::ScopeStart(None)` | Indentation increased by one level |
| `Token::ScopeEnd(None)` | Indentation decreased by one level, or end of stream |

> `ScopeStart` carries an `Option<(Brace, Position)>` for future explicit
> brace-based scoping; it is always `None` here.
