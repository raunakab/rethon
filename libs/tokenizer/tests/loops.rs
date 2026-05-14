mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use scoper::{LexType, Res};

#[rstest::rstest]
// Bare for with no body
#[case(
    "for items",
    Ok(vec![
        T(LexType::For),
        T(LexType::Identifier("items")),
    ])
)]
// Bare loop with no body
#[case(
    "loop",
    Ok(vec![T(LexType::Loop)])
)]
// For loop with single-statement body
#[case(
    "for items\n    x := item",
    Ok(vec![
        T(LexType::For),
        T(LexType::Identifier("items")),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("item")),
        Close,
    ])
)]
// Loop with single-statement body
#[case(
    "loop\n    x := 1",
    Ok(vec![
        T(LexType::Loop),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        Close,
    ])
)]
// For loop with multi-statement body
#[case(
    "for items\n    x := item\n    y := x",
    Ok(vec![
        T(LexType::For),
        T(LexType::Identifier("items")),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("item")),
        T(LexType::Identifier("y")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// Nested for inside for
#[case(
    "for xs\n    for ys\n        x := y",
    Ok(vec![
        T(LexType::For),
        T(LexType::Identifier("xs")),
        Open,
        T(LexType::For),
        T(LexType::Identifier("ys")),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("y")),
        Close,
        Close,
    ])
)]
// Loop with nested if
#[case(
    "loop\n    if done\n        return result",
    Ok(vec![
        T(LexType::Loop),
        Open,
        T(LexType::If),
        T(LexType::Identifier("done")),
        Open,
        T(LexType::Return),
        T(LexType::Identifier("result")),
        Close,
        Close,
    ])
)]
// For loop inside function body
#[case(
    "fn f\n    for xs\n        x := item\n    return done",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::For),
        T(LexType::Identifier("xs")),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("item")),
        Close,
        T(LexType::Return),
        T(LexType::Identifier("done")),
        Close,
    ])
)]
fn test_loops(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
