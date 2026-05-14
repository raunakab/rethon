mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{LexType, Res, StringType};

#[rstest::rstest]
// Empty source
#[case("", Ok(vec![]))]
// Simple assignment
#[case(
    "x := 42",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("42")),
    ])
)]
// Function with body
#[case(
    "fn add\n    return x",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("add")),
        Open,
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// String literal
#[case(
    "x := \"hello\"",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::String("hello", StringType::Normal)),
    ])
)]
// CRLF treated identically to LF
#[case(
    "fn f\r\n    return x",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
fn test_tokenize(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
