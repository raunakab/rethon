mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use scoper::{LexType, Res};

#[rstest::rstest]
// Bare if with no body
#[case(
    "if cond",
    Ok(vec![
        T(LexType::If),
        T(LexType::Identifier("cond")),
    ])
)]
// Simple if with single-statement body
#[case(
    "if cond\n    return true",
    Ok(vec![
        T(LexType::If),
        T(LexType::Identifier("cond")),
        Open,
        T(LexType::Return),
        T(LexType::True),
        Close,
    ])
)]
// If with multi-statement body
#[case(
    "if cond\n    x := 1\n    return x",
    Ok(vec![
        T(LexType::If),
        T(LexType::Identifier("cond")),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// If/else
#[case(
    "if cond\n    return true\nelse\n    return false",
    Ok(vec![
        T(LexType::If),
        T(LexType::Identifier("cond")),
        Open,
        T(LexType::Return),
        T(LexType::True),
        Close,
        T(LexType::Else),
        Open,
        T(LexType::Return),
        T(LexType::False),
        Close,
    ])
)]
// If/else with multi-statement bodies
#[case(
    "if cond\n    x := 1\n    return x\nelse\n    y := 2\n    return y",
    Ok(vec![
        T(LexType::If),
        T(LexType::Identifier("cond")),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
        T(LexType::Else),
        Open,
        T(LexType::Identifier("y")),
        T(LexType::StaticAssignment),
        T(LexType::Number("2")),
        T(LexType::Return),
        T(LexType::Identifier("y")),
        Close,
    ])
)]
// Nested if inside if body
#[case(
    "if a\n    if b\n        return true",
    Ok(vec![
        T(LexType::If),
        T(LexType::Identifier("a")),
        Open,
        T(LexType::If),
        T(LexType::Identifier("b")),
        Open,
        T(LexType::Return),
        T(LexType::True),
        Close,
        Close,
    ])
)]
// Nested if/else chain (else-if pattern)
#[case(
    "if a\n    return 1\nelse\n    if b\n        return 2\n    else\n        return 3",
    Ok(vec![
        T(LexType::If),
        T(LexType::Identifier("a")),
        Open,
        T(LexType::Return),
        T(LexType::Number("1")),
        Close,
        T(LexType::Else),
        Open,
        T(LexType::If),
        T(LexType::Identifier("b")),
        Open,
        T(LexType::Return),
        T(LexType::Number("2")),
        Close,
        T(LexType::Else),
        Open,
        T(LexType::Return),
        T(LexType::Number("3")),
        Close,
        Close,
    ])
)]
// If inside function body
#[case(
    "fn f\n    if x\n        return true\n    return false",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::If),
        T(LexType::Identifier("x")),
        Open,
        T(LexType::Return),
        T(LexType::True),
        Close,
        T(LexType::Return),
        T(LexType::False),
        Close,
    ])
)]
// If with boolean expression condition
#[case(
    "if x and y\n    return true",
    Ok(vec![
        T(LexType::If),
        T(LexType::Identifier("x")),
        T(LexType::And),
        T(LexType::Identifier("y")),
        Open,
        T(LexType::Return),
        T(LexType::True),
        Close,
    ])
)]
fn test_conditionals(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
