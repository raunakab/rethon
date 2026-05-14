mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use lexer::{Res, Token};

#[rstest::rstest]
// Bare if with no body
#[case(
    "if cond",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("cond")),
    ])
)]
// Simple if with single-statement body
#[case(
    "if cond\n    return true",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("cond")),
        Open,
        T(Token::Return),
        T(Token::True),
        Close,
    ])
)]
// If with multi-statement body
#[case(
    "if cond\n    x := 1\n    return x",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("cond")),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// If/else
#[case(
    "if cond\n    return true\nelse\n    return false",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("cond")),
        Open,
        T(Token::Return),
        T(Token::True),
        Close,
        T(Token::Else),
        Open,
        T(Token::Return),
        T(Token::False),
        Close,
    ])
)]
// If/else with multi-statement bodies
#[case(
    "if cond\n    x := 1\n    return x\nelse\n    y := 2\n    return y",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("cond")),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
        T(Token::Else),
        Open,
        T(Token::Identifier("y")),
        T(Token::StaticAssignment),
        T(Token::Number("2")),
        T(Token::Return),
        T(Token::Identifier("y")),
        Close,
    ])
)]
// Nested if inside if body
#[case(
    "if a\n    if b\n        return true",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("a")),
        Open,
        T(Token::If),
        T(Token::Identifier("b")),
        Open,
        T(Token::Return),
        T(Token::True),
        Close,
        Close,
    ])
)]
// Nested if/else chain (else-if pattern)
#[case(
    "if a\n    return 1\nelse\n    if b\n        return 2\n    else\n        return 3",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("a")),
        Open,
        T(Token::Return),
        T(Token::Number("1")),
        Close,
        T(Token::Else),
        Open,
        T(Token::If),
        T(Token::Identifier("b")),
        Open,
        T(Token::Return),
        T(Token::Number("2")),
        Close,
        T(Token::Else),
        Open,
        T(Token::Return),
        T(Token::Number("3")),
        Close,
        Close,
    ])
)]
// If inside function body
#[case(
    "fn f\n    if x\n        return true\n    return false",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::If),
        T(Token::Identifier("x")),
        Open,
        T(Token::Return),
        T(Token::True),
        Close,
        T(Token::Return),
        T(Token::False),
        Close,
    ])
)]
// If with boolean expression condition
#[case(
    "if x and y\n    return true",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("x")),
        T(Token::And),
        T(Token::Identifier("y")),
        Open,
        T(Token::Return),
        T(Token::True),
        Close,
    ])
)]
// `do` separates condition from indented body on the same header line
#[case(
    "if cond do\n    return true",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("cond")),
        T(Token::Do),
        Open,
        T(Token::Return),
        T(Token::True),
        Close,
    ])
)]
// `do` with compound condition
#[case(
    "if x > 0 do\n    return x",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("x")),
        T(Token::Greater),
        T(Token::Number("0")),
        T(Token::Do),
        Open,
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// `do` followed by else branch
#[case(
    "if cond do\n    return true\nelse\n    return false",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("cond")),
        T(Token::Do),
        Open,
        T(Token::Return),
        T(Token::True),
        Close,
        T(Token::Else),
        Open,
        T(Token::Return),
        T(Token::False),
        Close,
    ])
)]
// `do` is not treated as an identifier even when adjacent to other keywords
#[case(
    "if a do\n    if b do\n        return true",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("a")),
        T(Token::Do),
        Open,
        T(Token::If),
        T(Token::Identifier("b")),
        T(Token::Do),
        Open,
        T(Token::Return),
        T(Token::True),
        Close,
        Close,
    ])
)]
fn test_conditionals(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
