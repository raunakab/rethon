mod common;

use common::S::T;
use common::{S, collect};
use lexer::{Res, StringType, Token};

#[rstest::rstest]
// Normal string literal
#[case(
    "\"hello\"",
    Ok(vec![T(Token::String("hello", StringType::Normal))])
)]
// Formatted string literal
#[case(
    "f\"hello\"",
    Ok(vec![T(Token::String("hello", StringType::Formatted))])
)]
// Empty normal string
#[case(
    "\"\"",
    Ok(vec![T(Token::String("", StringType::Normal))])
)]
// Empty formatted string
#[case(
    "f\"\"",
    Ok(vec![T(Token::String("", StringType::Formatted))])
)]
// String with internal spaces
#[case(
    "\"hello world\"",
    Ok(vec![T(Token::String("hello world", StringType::Normal))])
)]
// Formatted string with interpolation syntax (content is opaque at tokenizer level)
#[case(
    "f\"hello ${name}\"",
    Ok(vec![T(Token::String("hello ${name}", StringType::Formatted))])
)]
// String in a constant assignment
#[case(
    "x := \"value\"",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::String("value", StringType::Normal)),
    ])
)]
// Formatted string in a constant assignment
#[case(
    "x := f\"value ${y}\"",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::String("value ${y}", StringType::Formatted)),
    ])
)]
// Two strings in sequence
#[case(
    "\"a\" \"b\"",
    Ok(vec![
        T(Token::String("a", StringType::Normal)),
        T(Token::String("b", StringType::Normal)),
    ])
)]
// String immediately adjacent to a keyword token (no space)
#[case(
    "\"hi\"there",
    Ok(vec![
        T(Token::String("hi", StringType::Normal)),
        T(Token::Identifier("there")),
    ])
)]
// f prefix not followed by string is a plain identifier
#[case(
    "f x",
    Ok(vec![
        T(Token::Identifier("f")),
        T(Token::Identifier("x")),
    ])
)]
fn test_strings(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
