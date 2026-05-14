mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{Res, TokenType};

#[rstest::rstest]
// Function header with no body
#[case(
    "fn name",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("name")),
    ])
)]
// Function with a single return
#[case(
    "fn greet\n    return x",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("greet")),
        Open,
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// Function with multiple body statements
#[case(
    "fn add\n    x := 1\n    y := 2\n    return x",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("add")),
        Open,
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("1")),
        T(TokenType::Identifier("y")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("2")),
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// Two consecutive functions at the same indent level
#[case(
    "fn a\n    return 1\nfn b\n    return 2",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("a")),
        Open,
        T(TokenType::Return),
        T(TokenType::Number("1")),
        Close,
        T(TokenType::Function),
        T(TokenType::Identifier("b")),
        Open,
        T(TokenType::Return),
        T(TokenType::Number("2")),
        Close,
    ])
)]
// Blank line inside function body is ignored
#[case(
    "fn f\n\n    return x",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// Multiple blank lines inside body
#[case(
    "fn f\n\n\n    return x",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// Nested scope (if) inside function body
#[case(
    "fn check\n    if x\n        return true",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("check")),
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
// Three levels of nesting inside a function
#[case(
    "fn f\n    if a\n        if b\n            return true",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
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
        Close,
    ])
)]
// Statements before and after a function
#[case(
    "x := 1\nfn f\n    return x\ny := 2",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("1")),
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
        T(TokenType::Identifier("y")),
        T(TokenType::StaticAssignment),
        T(TokenType::Number("2")),
    ])
)]
fn test_functions(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
