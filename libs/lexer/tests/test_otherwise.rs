mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use lexer::{Res, Token};

#[rstest::rstest]
// Bare otherwise keyword with no body
#[case(
    "otherwise",
    Ok(vec![T(Token::Otherwise)])
)]
// Basic otherwise with single-statement body
#[case(
    "x := foo\notherwise\n    x := default",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("foo")),
        T(Token::Otherwise),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("default")),
        Close,
    ])
)]
// Otherwise with multi-statement body
#[case(
    "x := foo\notherwise\n    x := fallback\n    y := 0",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("foo")),
        T(Token::Otherwise),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("fallback")),
        T(Token::Identifier("y")),
        T(Token::StaticAssignment),
        T(Token::Number("0")),
        Close,
    ])
)]
// Otherwise with return in body
#[case(
    "x := compute\notherwise\n    return default",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("compute")),
        T(Token::Otherwise),
        Open,
        T(Token::Return),
        T(Token::Identifier("default")),
        Close,
    ])
)]
// Otherwise inside function body
#[case(
    "fn f\n    x := foo\n    otherwise\n        x := default\n    return x",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("foo")),
        T(Token::Otherwise),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("default")),
        Close,
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// Chained — statement, otherwise, then more statements
#[case(
    "x := risky\notherwise\n    x := safe\ny := done",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("risky")),
        T(Token::Otherwise),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("safe")),
        Close,
        T(Token::Identifier("y")),
        T(Token::StaticAssignment),
        T(Token::Identifier("done")),
    ])
)]
fn test_otherwise(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
