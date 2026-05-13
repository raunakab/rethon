mod common;

use common::S::T;
use common::{S, collect};
use tokenizer::{Res, TokenType};

#[rstest::rstest]
// Macro with a single identifier argument
#[case(
    "!print x",
    Ok(vec![
        T(TokenType::MacroIdentifier("print")),
        T(TokenType::Identifier("x")),
    ])
)]
// Macro with a keyword argument (true)
#[case(
    "!assert true",
    Ok(vec![
        T(TokenType::MacroIdentifier("assert")),
        T(TokenType::True),
    ])
)]
// Macro with multiple arguments
#[case(
    "!log a b c",
    Ok(vec![
        T(TokenType::MacroIdentifier("log")),
        T(TokenType::Identifier("a")),
        T(TokenType::Identifier("b")),
        T(TokenType::Identifier("c")),
    ])
)]
// Macro with numeric argument
#[case(
    "!repeat 3",
    Ok(vec![
        T(TokenType::MacroIdentifier("repeat")),
        T(TokenType::Number("3")),
    ])
)]
// Two macros on consecutive lines
#[case(
    "!open x\n!close x",
    Ok(vec![
        T(TokenType::MacroIdentifier("open")),
        T(TokenType::Identifier("x")),
        T(TokenType::MacroIdentifier("close")),
        T(TokenType::Identifier("x")),
    ])
)]
// Macro with an expression argument
#[case(
    "!check x and y",
    Ok(vec![
        T(TokenType::MacroIdentifier("check")),
        T(TokenType::Identifier("x")),
        T(TokenType::And),
        T(TokenType::Identifier("y")),
    ])
)]
// Macro inside a function body
#[case(
    "fn f\n    !print x\n    return x",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        common::S::Open,
        T(TokenType::MacroIdentifier("print")),
        T(TokenType::Identifier("x")),
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        common::S::Close,
    ])
)]
// Lone ! without identifier is Promotion, not MacroIdentifier
#[case(
    "x!",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::Promotion),
    ])
)]
fn test_macros(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
