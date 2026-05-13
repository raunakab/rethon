# L1 Tokenizer

Splits a raw source string into a flat stream of `L1Token`s by scanning Unicode
grapheme clusters and classifying each one.

## Input / Output

- **Input:** `&str`
- **Output:** `impl Iterator<Item = Res<L1Token<'_>>>`

## Classification

Every grapheme cluster maps to one of:

| `L1TokenType` | Rule |
|---|---|
| `Whitespace` | Single ASCII whitespace character (`' '`, `'\t'`, `'\n'`, …), or the CRLF grapheme cluster (`'\r\n'`) |
| `Keyword` | ASCII alphabetic character, or any multi-byte grapheme cluster |
| `Numeric` | ASCII digit |
| `Punctuation` | ASCII punctuation character |
| `String` | Content between a matched pair of `"` quotes (quotes stripped) |
| `Unknown` | Anything that doesn't fit the above |

## Grouping

Consecutive graphemes of the same type are merged into a single token, with two
exceptions:

- **`Punctuation`** — always emitted one character at a time, since multi-char
  operators are resolved at L2.
- **`Whitespace`** — always emitted one character at a time, since different
  whitespace characters carry different meaning (space vs. newline vs. tab).

## String handling

On encountering an opening `"` the tokenizer enters string-scan mode and
consumes graphemes until a closing `"` is found, emitting the content (without
the surrounding quotes) as a `String` token. If the source ends before the
closing quote, `Error::UnterminatedString` is returned.

## Errors

| Error | Condition |
|---|---|
| `Error::UnterminatedString(byte_offset)` | `"` opened but never closed |
