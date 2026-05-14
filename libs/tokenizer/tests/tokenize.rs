mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{Res, StringType, TokenType};

#[rstest::rstest]
// Empty source
#[case("", Ok(vec![]))]
// Simple assignment
#[case(
    "x := 42",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("42")),
    ])
)]
// Function with body
#[case(
    "fn add\n    return x",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("add")),
        Open,
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// String literal
#[case(
    "x := \"hello\"",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::String("hello", StringType::Normal)),
    ])
)]
// CRLF treated identically to LF
#[case(
    "fn f\r\n    return x",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
fn test_tokenize(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
