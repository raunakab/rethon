mod common;

use common::{
    S,
    S::{Close, Open, T},
    collect,
};
use lexer::{Res, Token};

#[rstest::rstest]
// Function header with no body
#[case(
    "fn name",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("name")),
    ])
)]
// Function with a single return
#[case(
    "fn greet\n    return x",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("greet")),
        Open,
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// Function with multiple body statements
#[case(
    "fn add\n    x := 1\n    y := 2\n    return x",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("add")),
        Open,
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        T(Token::Identifier("y")),
        T(Token::StaticAssignment),
        T(Token::Number("2")),
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// Two consecutive functions at the same indent level
#[case(
    "fn a\n    return 1\nfn b\n    return 2",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("a")),
        Open,
        T(Token::Return),
        T(Token::Number("1")),
        Close,
        T(Token::Function),
        T(Token::Identifier("b")),
        Open,
        T(Token::Return),
        T(Token::Number("2")),
        Close,
    ])
)]
// Blank line inside function body is ignored
#[case(
    "fn f\n\n    return x",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// Multiple blank lines inside body
#[case(
    "fn f\n\n\n    return x",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// Nested scope (if) inside function body
#[case(
    "fn check\n    if x\n        return true",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("check")),
        Open,
        T(Token::If),
        T(Token::Identifier("x")),
        Open,
        T(Token::Return),
        T(Token::True),
        Close,
        Close,
    ])
)]
// Three levels of nesting inside a function
#[case(
    "fn f\n    if a\n        if b\n            return true",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
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
        Close,
    ])
)]
// Statements before and after a function
#[case(
    "x := 1\nfn f\n    return x\ny := 2",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::StaticAssignment),
        T(Token::Number("1")),
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
        T(Token::Identifier("y")),
        T(Token::StaticAssignment),
        T(Token::Number("2")),
    ])
)]
fn test_functions(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
