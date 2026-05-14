mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{Res, StringType, TokenType};

#[rstest::rstest]
// Constant assignment with integer
#[case(
    "x := 42",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("42")),
    ])
)]
// Constant assignment with float
#[case(
    "x := 3.14",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::Float("3", Some("14"))),
    ])
)]
// Constant assignment with string
#[case(
    "x := \"hello\"",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::String("hello", StringType::Normal)),
    ])
)]
// Constant assignment with boolean
#[case(
    "x := true",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::True),
    ])
)]
// Mutable reassignment
#[case(
    "x = 99",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::Assignment),
        T(TokenType::Number("99")),
    ])
)]
// Mutable declaration with mut keyword
#[case(
    "mut x = 0",
    Ok(vec![
        T(TokenType::Mutable),
        T(TokenType::Identifier("x")),
        T(TokenType::Assignment),
        T(TokenType::Number("0")),
    ])
)]
// Type-annotated constant assignment
#[case(
    "x ; Int := 42",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::Semicolon),
        T(TokenType::Identifier("Int")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("42")),
    ])
)]
// Type-annotated mutable assignment
#[case(
    "x ; String = \"hi\"",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::Semicolon),
        T(TokenType::Identifier("String")),
        T(TokenType::Assignment),
        T(TokenType::String("hi", StringType::Normal)),
    ])
)]
// Multiple consecutive assignments
#[case(
    "x := 1\ny := 2\nz := 3",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("1")),
        T(TokenType::Identifier("y")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("2")),
        T(TokenType::Identifier("z")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("3")),
    ])
)]
// Assignment inside a function body
#[case(
    "fn f\n    x := 1\n    return x",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("1")),
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// Assignment with identifier RHS
#[case(
    "y := x",
    Ok(vec![
        T(TokenType::Identifier("y")),
        T(TokenType::StaticAssignment),
        T(TokenType::Identifier("x")),
    ])
)]
fn test_assignments(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
