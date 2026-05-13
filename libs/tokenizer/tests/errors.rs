mod common;

use common::collect;
use tokenizer::Error;

#[rstest::rstest]
// Opening quote with nothing after it
#[case("\"", Error::UnterminatedString(0))]
// Unclosed string starting at byte 0
#[case("\"hello", Error::UnterminatedString(0))]
// Unclosed string after other tokens — byte offset points into the source
#[case("x := \"unterminated", Error::UnterminatedString(5))]
// Unclosed formatted string
#[case("f\"unterminated", Error::UnterminatedString(1))]
// Invalid indentation: 3 spaces (not a multiple of 4)
#[case("fn\n   x", Error::InvalidIndentation { found: 3, position: 3 })]
// Invalid indentation: 2 spaces
#[case("x\n  y", Error::InvalidIndentation { found: 2, position: 2 })]
// Invalid indentation: 5 spaces
#[case("x\n     y", Error::InvalidIndentation { found: 5, position: 2 })]
// Invalid indentation: 6 spaces
#[case("x\n      y", Error::InvalidIndentation { found: 6, position: 2 })]
// Unexpected brace — bare { at top level
#[case("{", Error::UnexpectedBrace)]
// Unexpected brace — bare { inside indented body
#[case("fn\n    {x}", Error::UnexpectedBrace)]
// Unexpected closing brace
#[case("}", Error::UnexpectedBrace)]
// Tab character — invalid whitespace
#[case("\t", Error::InvalidWhitespace("\t".to_string()))]
// Tab inside a line
#[case("a\tb", Error::InvalidWhitespace("\t".to_string()))]
// Unknown token (ASCII control character)
#[case("\x01", Error::UnknownToken("\x01".to_string()))]
fn test_errors(#[case] source: &str, #[case] expected: Error) {
    assert_eq!(collect(source).unwrap_err(), expected);
}
