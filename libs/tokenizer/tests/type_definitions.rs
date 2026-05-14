mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{Res, TokenType};

#[rstest::rstest]
// Bare struct keyword with name
#[case(
    "struct Foo",
    Ok(vec![
        T(TokenType::Struct),
        T(TokenType::Identifier("Foo")),
    ])
)]
// Bare enum keyword with name
#[case(
    "enum Bar",
    Ok(vec![
        T(TokenType::Enum),
        T(TokenType::Identifier("Bar")),
    ])
)]
// Struct with body
#[case(
    "struct Foo\n    x := 0",
    Ok(vec![
        T(TokenType::Struct),
        T(TokenType::Identifier("Foo")),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("0")),
        Close,
    ])
)]
// Enum with body
#[case(
    "enum Color\n    red := 0\n    green := 1",
    Ok(vec![
        T(TokenType::Enum),
        T(TokenType::Identifier("Color")),
        Open,
        T(TokenType::Identifier("red")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("0")),
        T(TokenType::Identifier("green")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("1")),
        Close,
    ])
)]
// Multiple type definitions in sequence
#[case(
    "struct Foo\nenum Bar",
    Ok(vec![
        T(TokenType::Struct),
        T(TokenType::Identifier("Foo")),
        T(TokenType::Enum),
        T(TokenType::Identifier("Bar")),
    ])
)]
// Type annotation with semicolon separator
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
// Type annotation with a parameterised type name
#[case(
    "xs ; List := items",
    Ok(vec![
        T(TokenType::Identifier("xs")),
        T(TokenType::Semicolon),
        T(TokenType::Identifier("List")),
        T(TokenType::StaticAssignment),
        T(TokenType::Identifier("items")),
    ])
)]
// Struct nested inside a function
#[case(
    "fn f\n    struct Inner\n        x := 0",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::Struct),
        T(TokenType::Identifier("Inner")),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("0")),
        Close,
        Close,
    ])
)]
fn test_type_definitions(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
