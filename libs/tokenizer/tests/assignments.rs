mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{LexType, Res, StringType};

#[rstest::rstest]
// Constant assignment with integer
#[case(
    "x := 42",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("42")),
    ])
)]
// Constant assignment with float
#[case(
    "x := 3.14",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Float("3", Some("14"))),
    ])
)]
// Constant assignment with string
#[case(
    "x := \"hello\"",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::String("hello", StringType::Normal)),
    ])
)]
// Constant assignment with boolean
#[case(
    "x := true",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::True),
    ])
)]
// Mutable reassignment
#[case(
    "x = 99",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::Assignment),
        T(LexType::Number("99")),
    ])
)]
// Mutable declaration with mut keyword
#[case(
    "mut x = 0",
    Ok(vec![
        T(LexType::Mutable),
        T(LexType::Identifier("x")),
        T(LexType::Assignment),
        T(LexType::Number("0")),
    ])
)]
// Type-annotated constant assignment
#[case(
    "x ; Int := 42",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::Semicolon),
        T(LexType::Identifier("Int")),
        T(LexType::StaticAssignment),
        T(LexType::Number("42")),
    ])
)]
// Type-annotated mutable assignment
#[case(
    "x ; String = \"hi\"",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::Semicolon),
        T(LexType::Identifier("String")),
        T(LexType::Assignment),
        T(LexType::String("hi", StringType::Normal)),
    ])
)]
// Multiple consecutive assignments
#[case(
    "x := 1\ny := 2\nz := 3",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        T(LexType::Identifier("y")),
        T(LexType::StaticAssignment),
        T(LexType::Number("2")),
        T(LexType::Identifier("z")),
        T(LexType::StaticAssignment),
        T(LexType::Number("3")),
    ])
)]
// Assignment inside a function body
#[case(
    "fn f\n    x := 1\n    return x",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// Assignment with identifier RHS
#[case(
    "y := x",
    Ok(vec![
        T(LexType::Identifier("y")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("x")),
    ])
)]
fn test_assignments(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
