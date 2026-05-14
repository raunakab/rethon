mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use lexer::{Res, Token};

#[rstest::rstest]
// Bare scope keyword with no body
#[case(
    "scope",
    Ok(vec![T(Token::Scope)])
)]
// Scope with single statement
#[case(
    "scope\n    x := 1",
    Ok(vec![
        T(Token::Scope),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        Close,
    ])
)]
// Scope with multiple statements
#[case(
    "scope\n    x := 1\n    y := 2\n    z := 3",
    Ok(vec![
        T(Token::Scope),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        T(Token::Identifier("y")),
        T(Token::StaticAssignment),
        T(Token::Number("2")),
        T(Token::Identifier("z")),
        T(Token::StaticAssignment),
        T(Token::Number("3")),
        Close,
    ])
)]
// Two consecutive scopes at the same level
#[case(
    "scope\n    x := 1\nscope\n    y := 2",
    Ok(vec![
        T(Token::Scope),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        Close,
        T(Token::Scope),
        Open,
        T(Token::Identifier("y")),
        T(Token::StaticAssignment),
        T(Token::Number("2")),
        Close,
    ])
)]
// Nested scopes
#[case(
    "scope\n    scope\n        x := 1",
    Ok(vec![
        T(Token::Scope),
        Open,
        T(Token::Scope),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        Close,
        Close,
    ])
)]
// Scope inside function body
#[case(
    "fn f\n    scope\n        x := 1\n    return x",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Scope),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        Close,
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// Scope with nested if
#[case(
    "scope\n    if x\n        return true",
    Ok(vec![
        T(Token::Scope),
        Open,
        T(Token::If),
        T(Token::Identifier("x")),
        Open,
        T(Token::Return),
        T(Token::True),
        Close,
        Close,
    ])
)]
fn test_scopes(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
