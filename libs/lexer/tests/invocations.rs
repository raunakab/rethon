mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use lexer::{Res, Token};

#[rstest::rstest]
// No arguments
#[case(
    "func()",
    Ok(vec![
        T(Token::Identifier("func")),
        Open,
        Close,
    ])
)]
// Single identifier argument
#[case(
    "func(x)",
    Ok(vec![
        T(Token::Identifier("func")),
        Open,
        T(Token::Identifier("x")),
        Close,
    ])
)]
// Multiple arguments separated by commas
#[case(
    "func(a, b, c)",
    Ok(vec![
        T(Token::Identifier("func")),
        Open,
        T(Token::Identifier("a")),
        T(Token::Comma),
        T(Token::Identifier("b")),
        T(Token::Comma),
        T(Token::Identifier("c")),
        Close,
    ])
)]
// Numeric argument
#[case(
    "func(42)",
    Ok(vec![
        T(Token::Identifier("func")),
        Open,
        T(Token::Number("42")),
        Close,
    ])
)]
// Float argument
#[case(
    "func(3.14)",
    Ok(vec![
        T(Token::Identifier("func")),
        Open,
        T(Token::Float("3", Some("14"))),
        Close,
    ])
)]
// String argument
#[case(
    "func(\"hello\")",
    Ok(vec![
        T(Token::Identifier("func")),
        Open,
        T(Token::String("hello", lexer::StringType::Normal)),
        Close,
    ])
)]
// Boolean argument
#[case(
    "func(true)",
    Ok(vec![
        T(Token::Identifier("func")),
        Open,
        T(Token::True),
        Close,
    ])
)]
// Result assigned to a variable
#[case(
    "x := func()",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Identifier("func")),
        Open,
        Close,
    ])
)]
// Invocation inside a function body
#[case(
    "fn f\n    func()",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Identifier("func")),
        Open,
        Close,
        Close,
    ])
)]
// Consecutive invocations inside a function body
#[case(
    "fn f\n    foo()\n    bar(x)",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Identifier("foo")),
        Open,
        Close,
        T(Token::Identifier("bar")),
        Open,
        T(Token::Identifier("x")),
        Close,
        Close,
    ])
)]
// Nested invocation as argument
#[case(
    "outer(inner())",
    Ok(vec![
        T(Token::Identifier("outer")),
        Open,
        T(Token::Identifier("inner")),
        Open,
        Close,
        Close,
    ])
)]
// Invocation result used in assignment
#[case(
    "x = rand()",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::Assignment),
        T(Token::Identifier("rand")),
        Open,
        Close,
    ])
)]
fn test_invocations(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
