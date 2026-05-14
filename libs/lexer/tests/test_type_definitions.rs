mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use lexer::{Res, Token};

#[rstest::rstest]
// Bare struct keyword with name
#[case(
    "struct Foo",
    Ok(vec![
        T(Token::Struct),
        T(Token::Identifier("Foo")),
    ])
)]
// Bare enum keyword with name
#[case(
    "enum Bar",
    Ok(vec![
        T(Token::Enum),
        T(Token::Identifier("Bar")),
    ])
)]
// Struct with body
#[case(
    "struct Foo\n    x := 0",
    Ok(vec![
        T(Token::Struct),
        T(Token::Identifier("Foo")),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("0")),
        Close,
    ])
)]
// Enum with body
#[case(
    "enum Color\n    red := 0\n    green := 1",
    Ok(vec![
        T(Token::Enum),
        T(Token::Identifier("Color")),
        Open,
        T(Token::Identifier("red")),
        T(Token::StaticAssignment),
        T(Token::Number("0")),
        T(Token::Identifier("green")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        Close,
    ])
)]
// Multiple type definitions in sequence
#[case(
    "struct Foo\nenum Bar",
    Ok(vec![
        T(Token::Struct),
        T(Token::Identifier("Foo")),
        T(Token::Enum),
        T(Token::Identifier("Bar")),
    ])
)]
// Type annotation with semicolon separator
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
// Type annotation with a parameterised type name
#[case(
    "xs ; List := items",
    Ok(vec![
        T(Token::Identifier("xs")),
        T(Token::Semicolon),
        T(Token::Identifier("List")),
        T(Token::StaticAssignment),
        T(Token::Identifier("items")),
    ])
)]
// Struct nested inside a function
#[case(
    "fn f\n    struct Inner\n        x := 0",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Struct),
        T(Token::Identifier("Inner")),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("0")),
        Close,
        Close,
    ])
)]
fn test_type_definitions(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
