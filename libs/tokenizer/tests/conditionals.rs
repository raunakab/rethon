mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{Res, TokenType};

#[rstest::rstest]
// Bare if with no body
#[case(
    "if cond",
    Ok(vec![
        T(TokenType::If),
        T(TokenType::Identifier("cond")),
    ])
)]
// Simple if with single-statement body
#[case(
    "if cond\n    return true",
    Ok(vec![
        T(TokenType::If),
        T(TokenType::Identifier("cond")),
        Open,
        T(TokenType::Return),
        T(TokenType::True),
        Close,
    ])
)]
// If with multi-statement body
#[case(
    "if cond\n    x := 1\n    return x",
    Ok(vec![
        T(TokenType::If),
        T(TokenType::Identifier("cond")),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("1")),
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// If/else
#[case(
    "if cond\n    return true\nelse\n    return false",
    Ok(vec![
        T(TokenType::If),
        T(TokenType::Identifier("cond")),
        Open,
        T(TokenType::Return),
        T(TokenType::True),
        Close,
        T(TokenType::Else),
        Open,
        T(TokenType::Return),
        T(TokenType::False),
        Close,
    ])
)]
// If/else with multi-statement bodies
#[case(
    "if cond\n    x := 1\n    return x\nelse\n    y := 2\n    return y",
    Ok(vec![
        T(TokenType::If),
        T(TokenType::Identifier("cond")),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("1")),
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
        T(TokenType::Else),
        Open,
        T(TokenType::Identifier("y")),
        T(TokenType::ConstantAssignment),
        T(TokenType::Number("2")),
        T(TokenType::Return),
        T(TokenType::Identifier("y")),
        Close,
    ])
)]
// Nested if inside if body
#[case(
    "if a\n    if b\n        return true",
    Ok(vec![
        T(TokenType::If),
        T(TokenType::Identifier("a")),
        Open,
        T(TokenType::If),
        T(TokenType::Identifier("b")),
        Open,
        T(TokenType::Return),
        T(TokenType::True),
        Close,
        Close,
    ])
)]
// Nested if/else chain (else-if pattern)
#[case(
    "if a\n    return 1\nelse\n    if b\n        return 2\n    else\n        return 3",
    Ok(vec![
        T(TokenType::If),
        T(TokenType::Identifier("a")),
        Open,
        T(TokenType::Return),
        T(TokenType::Number("1")),
        Close,
        T(TokenType::Else),
        Open,
        T(TokenType::If),
        T(TokenType::Identifier("b")),
        Open,
        T(TokenType::Return),
        T(TokenType::Number("2")),
        Close,
        T(TokenType::Else),
        Open,
        T(TokenType::Return),
        T(TokenType::Number("3")),
        Close,
        Close,
    ])
)]
// If inside function body
#[case(
    "fn f\n    if x\n        return true\n    return false",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::If),
        T(TokenType::Identifier("x")),
        Open,
        T(TokenType::Return),
        T(TokenType::True),
        Close,
        T(TokenType::Return),
        T(TokenType::False),
        Close,
    ])
)]
// If with boolean expression condition
#[case(
    "if x and y\n    return true",
    Ok(vec![
        T(TokenType::If),
        T(TokenType::Identifier("x")),
        T(TokenType::And),
        T(TokenType::Identifier("y")),
        Open,
        T(TokenType::Return),
        T(TokenType::True),
        Close,
    ])
)]
fn test_conditionals(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
