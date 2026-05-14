mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use scoper::{LexType, Res};

#[rstest::rstest]
// Bare struct keyword with name
#[case(
    "struct Foo",
    Ok(vec![
        T(LexType::Struct),
        T(LexType::Identifier("Foo")),
    ])
)]
// Bare enum keyword with name
#[case(
    "enum Bar",
    Ok(vec![
        T(LexType::Enum),
        T(LexType::Identifier("Bar")),
    ])
)]
// Struct with body
#[case(
    "struct Foo\n    x := 0",
    Ok(vec![
        T(LexType::Struct),
        T(LexType::Identifier("Foo")),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("0")),
        Close,
    ])
)]
// Enum with body
#[case(
    "enum Color\n    red := 0\n    green := 1",
    Ok(vec![
        T(LexType::Enum),
        T(LexType::Identifier("Color")),
        Open,
        T(LexType::Identifier("red")),
        T(LexType::StaticAssignment),
        T(LexType::Number("0")),
        T(LexType::Identifier("green")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        Close,
    ])
)]
// Multiple type definitions in sequence
#[case(
    "struct Foo\nenum Bar",
    Ok(vec![
        T(LexType::Struct),
        T(LexType::Identifier("Foo")),
        T(LexType::Enum),
        T(LexType::Identifier("Bar")),
    ])
)]
// Type annotation with semicolon separator
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
// Type annotation with a parameterised type name
#[case(
    "xs ; List := items",
    Ok(vec![
        T(LexType::Identifier("xs")),
        T(LexType::Semicolon),
        T(LexType::Identifier("List")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("items")),
    ])
)]
// Struct nested inside a function
#[case(
    "fn f\n    struct Inner\n        x := 0",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::Struct),
        T(LexType::Identifier("Inner")),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("0")),
        Close,
        Close,
    ])
)]
fn test_type_definitions(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
