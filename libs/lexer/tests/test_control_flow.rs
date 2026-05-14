mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use lexer::{Res, Token};

#[rstest::rstest]
// Bare return with no value
#[case("return", Ok(vec![T(Token::Return)]))]
// Return with identifier value
#[case(
    "return x",
    Ok(vec![T(Token::Return), T(Token::Identifier("x"))])
)]
// Return with numeric value
#[case(
    "return 42",
    Ok(vec![T(Token::Return), T(Token::Number("42"))])
)]
// Return with boolean value
#[case(
    "return true",
    Ok(vec![T(Token::Return), T(Token::True)])
)]
// Yield with value
#[case(
    "yield x",
    Ok(vec![T(Token::Yield), T(Token::Identifier("x"))])
)]
// Throw with value
#[case(
    "throw err",
    Ok(vec![T(Token::Throw), T(Token::Identifier("err"))])
)]
// Standalone panic
#[case("panic", Ok(vec![T(Token::Panic)]))]
// Standalone todo
#[case("todo", Ok(vec![T(Token::Todo)]))]
// Standalone unimplemented
#[case("unimplemented", Ok(vec![T(Token::Unimplemented)]))]
// Multiple type-hole keywords
#[case(
    "panic\ntodo\nunimplemented",
    Ok(vec![
        T(Token::Panic),
        T(Token::Todo),
        T(Token::Unimplemented),
    ])
)]
// Return inside function body
#[case(
    "fn f\n    return x",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Return),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// Yield inside function body
#[case(
    "fn f\n    yield x",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Yield),
        T(Token::Identifier("x")),
        Close,
    ])
)]
// Throw inside conditional
#[case(
    "if err\n    throw err",
    Ok(vec![
        T(Token::If),
        T(Token::Identifier("err")),
        Open,
        T(Token::Throw),
        T(Token::Identifier("err")),
        Close,
    ])
)]
// Conditional return — early return before final return
#[case(
    "fn f\n    if x\n        return true\n    return false",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::If),
        T(Token::Identifier("x")),
        Open,
        T(Token::Return),
        T(Token::True),
        Close,
        T(Token::Return),
        T(Token::False),
        Close,
    ])
)]
fn test_control_flow(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
