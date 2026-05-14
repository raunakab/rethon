mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{Res, TokenType};

#[rstest::rstest]
// No arguments
#[case(
    "func()",
    Ok(vec![
        T(TokenType::Identifier("func")),
        Open,
        Close,
    ])
)]
// Single identifier argument
#[case(
    "func(x)",
    Ok(vec![
        T(TokenType::Identifier("func")),
        Open,
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// Multiple arguments separated by commas
#[case(
    "func(a, b, c)",
    Ok(vec![
        T(TokenType::Identifier("func")),
        Open,
        T(TokenType::Identifier("a")),
        T(TokenType::Comma),
        T(TokenType::Identifier("b")),
        T(TokenType::Comma),
        T(TokenType::Identifier("c")),
        Close,
    ])
)]
// Numeric argument
#[case(
    "func(42)",
    Ok(vec![
        T(TokenType::Identifier("func")),
        Open,
        T(TokenType::Number("42")),
        Close,
    ])
)]
// Float argument
#[case(
    "func(3.14)",
    Ok(vec![
        T(TokenType::Identifier("func")),
        Open,
        T(TokenType::Float("3", Some("14"))),
        Close,
    ])
)]
// String argument
#[case(
    "func(\"hello\")",
    Ok(vec![
        T(TokenType::Identifier("func")),
        Open,
        T(TokenType::String("hello", tokenizer::StringType::Normal)),
        Close,
    ])
)]
// Boolean argument
#[case(
    "func(true)",
    Ok(vec![
        T(TokenType::Identifier("func")),
        Open,
        T(TokenType::True),
        Close,
    ])
)]
// Result assigned to a variable
#[case(
    "x := func()",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::StaticAssignment),
        T(TokenType::Identifier("func")),
        Open,
        Close,
    ])
)]
// Invocation inside a function body
#[case(
    "fn f\n    func()",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::Identifier("func")),
        Open,
        Close,
        Close,
    ])
)]
// Consecutive invocations inside a function body
#[case(
    "fn f\n    foo()\n    bar(x)",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::Identifier("foo")),
        Open,
        Close,
        T(TokenType::Identifier("bar")),
        Open,
        T(TokenType::Identifier("x")),
        Close,
        Close,
    ])
)]
// Nested invocation as argument
#[case(
    "outer(inner())",
    Ok(vec![
        T(TokenType::Identifier("outer")),
        Open,
        T(TokenType::Identifier("inner")),
        Open,
        Close,
        Close,
    ])
)]
// Invocation result used in assignment
#[case(
    "x = rand()",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::Assignment),
        T(TokenType::Identifier("rand")),
        Open,
        Close,
    ])
)]
fn test_invocations(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
