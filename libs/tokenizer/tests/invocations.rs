mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{LexType, Res};

#[rstest::rstest]
// No arguments
#[case(
    "func()",
    Ok(vec![
        T(LexType::Identifier("func")),
        Open,
        Close,
    ])
)]
// Single identifier argument
#[case(
    "func(x)",
    Ok(vec![
        T(LexType::Identifier("func")),
        Open,
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// Multiple arguments separated by commas
#[case(
    "func(a, b, c)",
    Ok(vec![
        T(LexType::Identifier("func")),
        Open,
        T(LexType::Identifier("a")),
        T(LexType::Comma),
        T(LexType::Identifier("b")),
        T(LexType::Comma),
        T(LexType::Identifier("c")),
        Close,
    ])
)]
// Numeric argument
#[case(
    "func(42)",
    Ok(vec![
        T(LexType::Identifier("func")),
        Open,
        T(LexType::Number("42")),
        Close,
    ])
)]
// Float argument
#[case(
    "func(3.14)",
    Ok(vec![
        T(LexType::Identifier("func")),
        Open,
        T(LexType::Float("3", Some("14"))),
        Close,
    ])
)]
// String argument
#[case(
    "func(\"hello\")",
    Ok(vec![
        T(LexType::Identifier("func")),
        Open,
        T(LexType::String("hello", tokenizer::StringType::Normal)),
        Close,
    ])
)]
// Boolean argument
#[case(
    "func(true)",
    Ok(vec![
        T(LexType::Identifier("func")),
        Open,
        T(LexType::True),
        Close,
    ])
)]
// Result assigned to a variable
#[case(
    "x := func()",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("func")),
        Open,
        Close,
    ])
)]
// Invocation inside a function body
#[case(
    "fn f\n    func()",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::Identifier("func")),
        Open,
        Close,
        Close,
    ])
)]
// Consecutive invocations inside a function body
#[case(
    "fn f\n    foo()\n    bar(x)",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::Identifier("foo")),
        Open,
        Close,
        T(LexType::Identifier("bar")),
        Open,
        T(LexType::Identifier("x")),
        Close,
        Close,
    ])
)]
// Nested invocation as argument
#[case(
    "outer(inner())",
    Ok(vec![
        T(LexType::Identifier("outer")),
        Open,
        T(LexType::Identifier("inner")),
        Open,
        Close,
        Close,
    ])
)]
// Invocation result used in assignment
#[case(
    "x = rand()",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::Assignment),
        T(LexType::Identifier("rand")),
        Open,
        Close,
    ])
)]
fn test_invocations(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
