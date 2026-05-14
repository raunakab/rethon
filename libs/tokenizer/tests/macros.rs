mod common;

use common::S::T;
use common::{S, collect};
use scoper::{Res, Token};

#[rstest::rstest]
// Macro with a single identifier argument
#[case(
    "!print x",
    Ok(vec![
        T(Token::MacroIdentifier("print")),
        T(Token::Identifier("x")),
    ])
)]
// Macro with a keyword argument (true)
#[case(
    "!assert true",
    Ok(vec![
        T(Token::MacroIdentifier("assert")),
        T(Token::True),
    ])
)]
// Macro with multiple arguments
#[case(
    "!log a b c",
    Ok(vec![
        T(Token::MacroIdentifier("log")),
        T(Token::Identifier("a")),
        T(Token::Identifier("b")),
        T(Token::Identifier("c")),
    ])
)]
// Macro with numeric argument
#[case(
    "!repeat 3",
    Ok(vec![
        T(Token::MacroIdentifier("repeat")),
        T(Token::Number("3")),
    ])
)]
// Two macros on consecutive lines
#[case(
    "!open x\n!close x",
    Ok(vec![
        T(Token::MacroIdentifier("open")),
        T(Token::Identifier("x")),
        T(Token::MacroIdentifier("close")),
        T(Token::Identifier("x")),
    ])
)]
// Macro with an expression argument
#[case(
    "!check x and y",
    Ok(vec![
        T(Token::MacroIdentifier("check")),
        T(Token::Identifier("x")),
        T(Token::And),
        T(Token::Identifier("y")),
    ])
)]
// Macro inside a function body
#[case(
    "fn f\n    !print x\n    return x",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        common::S::Open,
        T(Token::MacroIdentifier("print")),
        T(Token::Identifier("x")),
        T(Token::Return),
        T(Token::Identifier("x")),
        common::S::Close,
    ])
)]
// Lone ! without identifier is Promotion, not MacroIdentifier
#[case(
    "x!",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::Promotion),
    ])
)]
fn test_macros(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
