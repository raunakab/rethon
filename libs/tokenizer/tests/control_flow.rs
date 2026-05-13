mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{Res, TokenType};

#[rstest::rstest]
// Bare return with no value
#[case("return", Ok(vec![T(TokenType::Return)]))]
// Return with identifier value
#[case(
    "return x",
    Ok(vec![T(TokenType::Return), T(TokenType::Identifier("x"))])
)]
// Return with numeric value
#[case(
    "return 42",
    Ok(vec![T(TokenType::Return), T(TokenType::Number("42"))])
)]
// Return with boolean value
#[case(
    "return true",
    Ok(vec![T(TokenType::Return), T(TokenType::True)])
)]
// Yield with value
#[case(
    "yield x",
    Ok(vec![T(TokenType::Yield), T(TokenType::Identifier("x"))])
)]
// Throw with value
#[case(
    "throw err",
    Ok(vec![T(TokenType::Throw), T(TokenType::Identifier("err"))])
)]
// Standalone panic
#[case("panic", Ok(vec![T(TokenType::Panic)]))]
// Standalone todo
#[case("todo", Ok(vec![T(TokenType::Todo)]))]
// Standalone unimplemented
#[case("unimplemented", Ok(vec![T(TokenType::Unimplemented)]))]
// Multiple type-hole keywords
#[case(
    "panic\ntodo\nunimplemented",
    Ok(vec![
        T(TokenType::Panic),
        T(TokenType::Todo),
        T(TokenType::Unimplemented),
    ])
)]
// Return inside function body
#[case(
    "fn f\n    return x",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::Return),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// Yield inside function body
#[case(
    "fn f\n    yield x",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::Yield),
        T(TokenType::Identifier("x")),
        Close,
    ])
)]
// Throw inside conditional
#[case(
    "if err\n    throw err",
    Ok(vec![
        T(TokenType::If),
        T(TokenType::Identifier("err")),
        Open,
        T(TokenType::Throw),
        T(TokenType::Identifier("err")),
        Close,
    ])
)]
// Conditional return — early return before final return
#[case(
    "fn f\n    if x\n        return true\n    return false",
    Ok(vec![
        T(TokenType::Function),
        T(TokenType::Identifier("f")),
        Open,
        T(TokenType::If),
        T(TokenType::Identifier("x")),
        Open,
        T(TokenType::Return),
        T(TokenType::True),
        Close,
        T(TokenType::Return),
        T(TokenType::False),
        Close,
    ])
)]
fn test_control_flow(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
