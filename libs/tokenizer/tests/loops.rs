mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{Res, TokenType};

#[rstest::rstest]
// Bare for with no body
#[case(
    "for items",
    Ok(vec![
        T(TokenType::For),
        T(TokenType::Identifier("items")),
    ])
)]
// Bare loop with no body
#[case(
    "loop",
    Ok(vec![T(TokenType::Loop)])
)]
// For loop with single-statement body
#[case(
    "for items\n    x := item",
    Ok(vec![
        T(TokenType::For),
        T(TokenType::Identifier("items")),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("item")),
        Close,
    ])
)]
// Loop with single-statement body
#[case(
    "loop\n    x := 1",
    Ok(vec![
        T(TokenType::Loop),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("1")),
        Close,
    ])
)]
// For loop with multi-statement body
#[case(
    "for items\n    x := item\n    y := x",
    Ok(vec![
        T(TokenType::For),
        T(TokenType::Identifier("items")),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("item")),
        T(TokenType::Identifier("y")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// Nested for inside for
#[case(
    "for xs\n    for ys\n        x := y",
    Ok(vec![
        T(TokenType::For),
        T(TokenType::Identifier("xs")),
        Open,
        T(TokenType::For),
        T(TokenType::Identifier("ys")),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("y")),
        Close,
        Close,
    ])
)]
// Loop with nested if
#[case(
    "loop\n    if done\n        return result",
    Ok(vec![
        T(TokenType::Loop),
        Open,
        T(TokenType::If),
        T(TokenType::Identifier("done")),
        Open,
        T(TokenType::Return),
        T(TokenType::Identifier("result")),
        Close,
        Close,
    ])
)]
// For loop inside function body
#[case(
    "fn f\n    for xs\n        x := item\n    return done",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::For),
        T(TokenType::Identifier("xs")),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Identifier("item")),
        Close,
        T(TokenType::Return),
        T(TokenType::Identifier("done")),
        Close,
    ])
)]
fn test_loops(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
