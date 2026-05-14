mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use scoper::{LexType, Res};

#[rstest::rstest]
// Bare scope keyword with no body
#[case(
    "scope",
    Ok(vec![T(LexType::Scope)])
)]
// Scope with single statement
#[case(
    "scope\n    x := 1",
    Ok(vec![
        T(LexType::Scope),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        Close,
    ])
)]
// Scope with multiple statements
#[case(
    "scope\n    x := 1\n    y := 2\n    z := 3",
    Ok(vec![
        T(LexType::Scope),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        T(LexType::Identifier("y")),
        T(LexType::StaticAssignment),
        T(LexType::Number("2")),
        T(LexType::Identifier("z")),
        T(LexType::StaticAssignment),
        T(LexType::Number("3")),
        Close,
    ])
)]
// Two consecutive scopes at the same level
#[case(
    "scope\n    x := 1\nscope\n    y := 2",
    Ok(vec![
        T(LexType::Scope),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        Close,
        T(LexType::Scope),
        Open,
        T(LexType::Identifier("y")),
        T(LexType::StaticAssignment),
        T(LexType::Number("2")),
        Close,
    ])
)]
// Nested scopes
#[case(
    "scope\n    scope\n        x := 1",
    Ok(vec![
        T(LexType::Scope),
        Open,
        T(LexType::Scope),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        Close,
        Close,
    ])
)]
// Scope inside function body
#[case(
    "fn f\n    scope\n        x := 1\n    return x",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::Scope),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        Close,
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// Scope with nested if
#[case(
    "scope\n    if x\n        return true",
    Ok(vec![
        T(LexType::Scope),
        Open,
        T(LexType::If),
        T(LexType::Identifier("x")),
        Open,
        T(LexType::Return),
        T(LexType::True),
        Close,
        Close,
    ])
)]
fn test_scopes(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
