mod common;

use common::S::T;
use common::{S, collect};
use tokenizer::{LexType, Res};

#[rstest::rstest]
// Macro with a single identifier argument
#[case(
    "!print x",
    Ok(vec![
        T(LexType::MacroIdentifier("print")),
        T(LexType::Identifier("x")),
    ])
)]
// Macro with a keyword argument (true)
#[case(
    "!assert true",
    Ok(vec![
        T(LexType::MacroIdentifier("assert")),
        T(LexType::True),
    ])
)]
// Macro with multiple arguments
#[case(
    "!log a b c",
    Ok(vec![
        T(LexType::MacroIdentifier("log")),
        T(LexType::Identifier("a")),
        T(LexType::Identifier("b")),
        T(LexType::Identifier("c")),
    ])
)]
// Macro with numeric argument
#[case(
    "!repeat 3",
    Ok(vec![
        T(LexType::MacroIdentifier("repeat")),
        T(LexType::Number("3")),
    ])
)]
// Two macros on consecutive lines
#[case(
    "!open x\n!close x",
    Ok(vec![
        T(LexType::MacroIdentifier("open")),
        T(LexType::Identifier("x")),
        T(LexType::MacroIdentifier("close")),
        T(LexType::Identifier("x")),
    ])
)]
// Macro with an expression argument
#[case(
    "!check x and y",
    Ok(vec![
        T(LexType::MacroIdentifier("check")),
        T(LexType::Identifier("x")),
        T(LexType::And),
        T(LexType::Identifier("y")),
    ])
)]
// Macro inside a function body
#[case(
    "fn f\n    !print x\n    return x",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        common::S::Open,
        T(LexType::MacroIdentifier("print")),
        T(LexType::Identifier("x")),
        T(LexType::Return),
        T(LexType::Identifier("x")),
        common::S::Close,
    ])
)]
// Lone ! without identifier is Promotion, not MacroIdentifier
#[case(
    "x!",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::Promotion),
    ])
)]
fn test_macros(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
