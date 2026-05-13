mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{Res, TokenType};

#[rstest::rstest]
// Bare scope keyword with no body
#[case(
    "scope",
    Ok(vec![T(TokenType::Scope)])
)]
// Scope with single statement
#[case(
    "scope\n    x := 1",
    Ok(vec![
        T(TokenType::Scope),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("1")),
        Close,
    ])
)]
// Scope with multiple statements
#[case(
    "scope\n    x := 1\n    y := 2\n    z := 3",
    Ok(vec![
        T(TokenType::Scope),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("1")),
        T(TokenType::Identifier("y")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("2")),
        T(TokenType::Identifier("z")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("3")),
        Close,
    ])
)]
// Two consecutive scopes at the same level
#[case(
    "scope\n    x := 1\nscope\n    y := 2",
    Ok(vec![
        T(TokenType::Scope),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("1")),
        Close,
        T(TokenType::Scope),
        Open,
        T(TokenType::Identifier("y")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("2")),
        Close,
    ])
)]
// Nested scopes
#[case(
    "scope\n    scope\n        x := 1",
    Ok(vec![
        T(TokenType::Scope),
        Open,
        T(TokenType::Scope),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("1")),
        Close,
        Close,
    ])
)]
// Scope inside function body
#[case(
    "fn f\n    scope\n        x := 1\n    return x",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::Scope),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("1")),
        Close,
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// Scope with nested if
#[case(
    "scope\n    if x\n        return true",
    Ok(vec![
        T(TokenType::Scope),
        Open,
        T(TokenType::If),
        T(TokenType::Identifier("x")),
        Open,
        T(TokenType::Return),
        T(TokenType::True),
        Close,
        Close,
    ])
)]
fn test_scopes(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
