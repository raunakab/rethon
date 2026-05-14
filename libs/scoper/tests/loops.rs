mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use scoper::{Res, Token};

#[rstest::rstest]
// Bare for with no body
#[case(
    "for items",
    Ok(vec![
        T(Token::For),
        T(Token::Identifier("items")),
    ])
)]
// Bare loop with no body
#[case(
    "loop",
    Ok(vec![T(Token::Loop)])
)]
// For loop with single-statement body
#[case(
    "for items\n    x := item",
    Ok(vec![
        T(Token::For),
        T(Token::Identifier("items")),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("item")),
        Close,
    ])
)]
// Loop with single-statement body
#[case(
    "loop\n    x := 1",
    Ok(vec![
        T(Token::Loop),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        Close,
    ])
)]
// For loop with multi-statement body
#[case(
    "for items\n    x := item\n    y := x",
    Ok(vec![
        T(Token::For),
        T(Token::Identifier("items")),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("item")),
        T(Token::Identifier("y")),
        T(Token::StaticAssignment),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// Nested for inside for
#[case(
    "for xs\n    for ys\n        x := y",
    Ok(vec![
        T(Token::For),
        T(Token::Identifier("xs")),
        Open,
        T(Token::For),
        T(Token::Identifier("ys")),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("y")),
        Close,
        Close,
    ])
)]
// Loop with nested if
#[case(
    "loop\n    if done\n        return result",
    Ok(vec![
        T(Token::Loop),
        Open,
        T(Token::If),
        T(Token::Identifier("done")),
        Open,
        T(Token::Return),
        T(Token::Identifier("result")),
        Close,
        Close,
    ])
)]
// For loop inside function body
#[case(
    "fn f\n    for xs\n        x := item\n    return done",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::For),
        T(Token::Identifier("xs")),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("item")),
        Close,
        T(Token::Return),
        T(Token::Identifier("done")),
        Close,
    ])
)]
fn test_loops(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
