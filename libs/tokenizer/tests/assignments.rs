mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use scoper::{Res, StringType, Token};

#[rstest::rstest]
// Constant assignment with integer
#[case(
    "x := 42",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("42")),
    ])
)]
// Constant assignment with float
#[case(
    "x := 3.14",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Float("3", Some("14"))),
    ])
)]
// Constant assignment with string
#[case(
    "x := \"hello\"",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::String("hello", StringType::Normal)),
    ])
)]
// Constant assignment with boolean
#[case(
    "x := true",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::True),
    ])
)]
// Mutable reassignment
#[case(
    "x = 99",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::Assignment),
        T(Token::Number("99")),
    ])
)]
// Mutable declaration with mut keyword
#[case(
    "mut x = 0",
    Ok(vec![
        T(Token::Mutable),
        T(Token::Identifier("x")),
        T(Token::Assignment),
        T(Token::Number("0")),
    ])
)]
// Type-annotated constant assignment
#[case(
    "x ; Int := 42",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::Semicolon),
        T(Token::Identifier("Int")),
        T(Token::StaticAssignment),
        T(Token::Number("42")),
    ])
)]
// Type-annotated mutable assignment
#[case(
    "x ; String = \"hi\"",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::Semicolon),
        T(Token::Identifier("String")),
        T(Token::Assignment),
        T(Token::String("hi", StringType::Normal)),
    ])
)]
// Multiple consecutive assignments
#[case(
    "x := 1\ny := 2\nz := 3",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        T(Token::Identifier("y")),
        T(Token::StaticAssignment),
        T(Token::Number("2")),
        T(Token::Identifier("z")),
        T(Token::StaticAssignment),
        T(Token::Number("3")),
    ])
)]
// Assignment inside a function body
#[case(
    "fn f\n    x := 1\n    return x",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// Assignment with identifier RHS
#[case(
    "y := x",
    Ok(vec![
        T(Token::Identifier("y")),
        T(Token::StaticAssignment),
        T(Token::Identifier("x")),
    ])
)]
fn test_assignments(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
