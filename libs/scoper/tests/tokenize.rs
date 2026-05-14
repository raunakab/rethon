mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use scoper::{Res, StringType, Token};

#[rstest::rstest]
// Empty source
#[case("", Ok(vec![]))]
// Simple assignment
#[case(
    "x := 42",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("42")),
    ])
)]
// Function with body
#[case(
    "fn add\n    return x",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("add")),
        Open,
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// String literal
#[case(
    "x := \"hello\"",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::String("hello", StringType::Normal)),
    ])
)]
// CRLF treated identically to LF
#[case(
    "fn f\r\n    return x",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
    ])
)]
fn test_tokenize(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
