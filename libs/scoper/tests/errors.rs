mod common;

use common::collect;
use scoper::Error;

#[rstest::rstest]
// Unterminated string errors (lex-level)
#[case("\"", Error::Lex(lexer::Error::UnterminatedString(0)))]
#[case("\"hello", Error::Lex(lexer::Error::UnterminatedString(0)))]
#[case("x := \"unterminated", Error::Lex(lexer::Error::UnterminatedString(5)))]
#[case("f\"unterminated", Error::Lex(lexer::Error::UnterminatedString(1)))]
// Invalid indentation
#[case("fn\n   x", Error::InvalidIndentation { found: 3, position: 3 })]
#[case("x\n  y", Error::InvalidIndentation { found: 2, position: 2 })]
#[case("x\n     y", Error::InvalidIndentation { found: 5, position: 2 })]
#[case("x\n      y", Error::InvalidIndentation { found: 6, position: 2 })]
// Tab character — invalid whitespace (lex-level)
#[case("\t", Error::Lex(lexer::Error::InvalidWhitespace("\t".to_string())))]
#[case("a\tb", Error::Lex(lexer::Error::InvalidWhitespace("\t".to_string())))]
// Unknown token (lex-level)
#[case("\x01", Error::Lex(lexer::Error::UnknownItem("\x01".to_string())))]
fn test_errors(#[case] source: &str, #[case] expected: Error) {
    assert_eq!(collect(source).unwrap_err(), expected);
}
