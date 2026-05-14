mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{LexType, Res};

#[rstest::rstest]
// Bare return with no value
#[case("return", Ok(vec![T(LexType::Return)]))]
// Return with identifier value
#[case(
    "return x",
    Ok(vec![T(LexType::Return), T(LexType::Identifier("x"))])
)]
// Return with numeric value
#[case(
    "return 42",
    Ok(vec![T(LexType::Return), T(LexType::Number("42"))])
)]
// Return with boolean value
#[case(
    "return true",
    Ok(vec![T(LexType::Return), T(LexType::True)])
)]
// Yield with value
#[case(
    "yield x",
    Ok(vec![T(LexType::Yield), T(LexType::Identifier("x"))])
)]
// Throw with value
#[case(
    "throw err",
    Ok(vec![T(LexType::Throw), T(LexType::Identifier("err"))])
)]
// Standalone panic
#[case("panic", Ok(vec![T(LexType::Panic)]))]
// Standalone todo
#[case("todo", Ok(vec![T(LexType::Todo)]))]
// Standalone unimplemented
#[case("unimplemented", Ok(vec![T(LexType::Unimplemented)]))]
// Multiple type-hole keywords
#[case(
    "panic\ntodo\nunimplemented",
    Ok(vec![
        T(LexType::Panic),
        T(LexType::Todo),
        T(LexType::Unimplemented),
    ])
)]
// Return inside function body
#[case(
    "fn f\n    return x",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// Yield inside function body
#[case(
    "fn f\n    yield x",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::Yield),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// Throw inside conditional
#[case(
    "if err\n    throw err",
    Ok(vec![
        T(LexType::If),
        T(LexType::Identifier("err")),
        Open,
        T(LexType::Throw),
        T(LexType::Identifier("err")),
        Close,
    ])
)]
// Conditional return — early return before final return
#[case(
    "fn f\n    if x\n        return true\n    return false",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::If),
        T(LexType::Identifier("x")),
        Open,
        T(LexType::Return),
        T(LexType::True),
        Close,
        T(LexType::Return),
        T(LexType::False),
        Close,
    ])
)]
fn test_control_flow(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
