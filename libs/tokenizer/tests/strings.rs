mod common;

use common::S::T;
use common::{S, collect};
use tokenizer::{Res, StringType, TokenType};

#[rstest::rstest]
// Normal string literal
#[case(
    "\"hello\"",
    Ok(vec![T(TokenType::String("hello", StringType::Normal))])
)]
// Formatted string literal
#[case(
    "f\"hello\"",
    Ok(vec![T(TokenType::String("hello", StringType::Formatted))])
)]
// Empty normal string
#[case(
    "\"\"",
    Ok(vec![T(TokenType::String("", StringType::Normal))])
)]
// Empty formatted string
#[case(
    "f\"\"",
    Ok(vec![T(TokenType::String("", StringType::Formatted))])
)]
// String with internal spaces
#[case(
    "\"hello world\"",
    Ok(vec![T(TokenType::String("hello world", StringType::Normal))])
)]
// Formatted string with interpolation syntax (content is opaque at tokenizer level)
#[case(
    "f\"hello ${name}\"",
    Ok(vec![T(TokenType::String("hello ${name}", StringType::Formatted))])
)]
// String in a constant assignment
#[case(
    "x := \"value\"",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::String("value", StringType::Normal)),
    ])
)]
// Formatted string in a constant assignment
#[case(
    "x := f\"value ${y}\"",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::String("value ${y}", StringType::Formatted)),
    ])
)]
// Two strings in sequence
#[case(
    "\"a\" \"b\"",
    Ok(vec![
        T(TokenType::String("a", StringType::Normal)),
        T(TokenType::String("b", StringType::Normal)),
    ])
)]
// String immediately adjacent to a keyword token (no space)
#[case(
    "\"hi\"there",
    Ok(vec![
        T(TokenType::String("hi", StringType::Normal)),
        T(TokenType::Identifier("there")),
    ])
)]
// f prefix not followed by string is a plain identifier
#[case(
    "f x",
    Ok(vec![
        T(TokenType::Identifier("f")),
        T(TokenType::Identifier("x")),
    ])
)]
fn test_strings(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
