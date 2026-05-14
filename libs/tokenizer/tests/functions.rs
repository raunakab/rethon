mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{LexType, Res};

#[rstest::rstest]
// Function header with no body
#[case(
    "fn name",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("name")),
    ])
)]
// Function with a single return
#[case(
    "fn greet\n    return x",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("greet")),
        Open,
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// Function with multiple body statements
#[case(
    "fn add\n    x := 1\n    y := 2\n    return x",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("add")),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        T(LexType::Identifier("y")),
        T(LexType::StaticAssignment),
        T(LexType::Number("2")),
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// Two consecutive functions at the same indent level
#[case(
    "fn a\n    return 1\nfn b\n    return 2",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("a")),
        Open,
        T(LexType::Return),
        T(LexType::Number("1")),
        Close,
        T(LexType::Function),
        T(LexType::Identifier("b")),
        Open,
        T(LexType::Return),
        T(LexType::Number("2")),
        Close,
    ])
)]
// Blank line inside function body is ignored
#[case(
    "fn f\n\n    return x",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// Multiple blank lines inside body
#[case(
    "fn f\n\n\n    return x",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// Nested scope (if) inside function body
#[case(
    "fn check\n    if x\n        return true",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("check")),
        Open,
        T(LexType::If),
        T(LexType::Identifier("x")),
        Open,
        T(LexType::Return),
        T(LexType::True),
        Close,
        Close,
    ])
)]
// Three levels of nesting inside a function
#[case(
    "fn f\n    if a\n        if b\n            return true",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
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
        Close,
    ])
)]
// Statements before and after a function
#[case(
    "x := 1\nfn f\n    return x\ny := 2",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("1")),
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
        T(LexType::Identifier("y")),
        T(LexType::StaticAssignment),
        T(LexType::Number("2")),
    ])
)]
fn test_functions(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
