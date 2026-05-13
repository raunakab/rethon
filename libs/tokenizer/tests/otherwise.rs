mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{Res, TokenType};

#[rstest::rstest]
// Bare otherwise keyword with no body
#[case(
    "otherwise",
    Ok(vec![T(TokenType::Otherwise)])
)]
// Basic otherwise with single-statement body
#[case(
    "x := foo\notherwise\n    x := default",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("foo")),
        T(TokenType::Otherwise),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("default")),
        Close,
    ])
)]
// Otherwise with multi-statement body
#[case(
    "x := foo\notherwise\n    x := fallback\n    y := 0",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("foo")),
        T(TokenType::Otherwise),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("fallback")),
        T(TokenType::Identifier("y")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("0")),
        Close,
    ])
)]
// Otherwise with return in body
#[case(
    "x := compute\notherwise\n    return default",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("compute")),
        T(TokenType::Otherwise),
        Open,
        T(TokenType::Return),
        T(TokenType::Identifier("default")),
        Close,
    ])
)]
// Otherwise inside function body
#[case(
    "fn f\n    x := foo\n    otherwise\n        x := default\n    return x",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("foo")),
        T(TokenType::Otherwise),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("default")),
        Close,
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// Chained — statement, otherwise, then more statements
#[case(
    "x := risky\notherwise\n    x := safe\ny := done",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("risky")),
        T(TokenType::Otherwise),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("safe")),
        Close,
        T(TokenType::Identifier("y")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("done")),
    ])
)]
fn test_otherwise(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
