mod common;

use common::S::T;
use common::{S, collect};
use tokenizer::{LexType, Res};

#[rstest::rstest]
// ── Arithmetic ────────────────────────────────────────────────────────────────
#[case("a + b", Ok(vec![T(LexType::Identifier("a")), T(LexType::Plus), T(LexType::Identifier("b"))]))]
#[case("a - b", Ok(vec![T(LexType::Identifier("a")), T(LexType::Minus), T(LexType::Identifier("b"))]))]
#[case("a * b", Ok(vec![T(LexType::Identifier("a")), T(LexType::Asterisk), T(LexType::Identifier("b"))]))]
#[case("a / b", Ok(vec![T(LexType::Identifier("a")), T(LexType::Slash), T(LexType::Identifier("b"))]))]
#[case("a ** b", Ok(vec![T(LexType::Identifier("a")), T(LexType::DoubleAsterisk), T(LexType::Identifier("b"))]))]
// ── Comparison ────────────────────────────────────────────────────────────────
#[case("a == b", Ok(vec![T(LexType::Identifier("a")), T(LexType::Equals), T(LexType::Identifier("b"))]))]
#[case("a >= b", Ok(vec![T(LexType::Identifier("a")), T(LexType::GreaterOrEqual), T(LexType::Identifier("b"))]))]
#[case("a <= b", Ok(vec![T(LexType::Identifier("a")), T(LexType::LesserOrEqual), T(LexType::Identifier("b"))]))]
#[case("a > b", Ok(vec![T(LexType::Identifier("a")), T(LexType::Greater), T(LexType::Identifier("b"))]))]
#[case("a < b", Ok(vec![T(LexType::Identifier("a")), T(LexType::Lesser), T(LexType::Identifier("b"))]))]
// ── Boolean ───────────────────────────────────────────────────────────────────
#[case("not a", Ok(vec![T(LexType::Not), T(LexType::Identifier("a"))]))]
#[case("a and b", Ok(vec![T(LexType::Identifier("a")), T(LexType::And), T(LexType::Identifier("b"))]))]
#[case("a or b", Ok(vec![T(LexType::Identifier("a")), T(LexType::Or), T(LexType::Identifier("b"))]))]
// ── Pipe ──────────────────────────────────────────────────────────────────────
#[case("x |> f", Ok(vec![T(LexType::Identifier("x")), T(LexType::PipeForward), T(LexType::Identifier("f"))]))]
#[case("x |>> g", Ok(vec![T(LexType::Identifier("x")), T(LexType::PipeDoubleForward), T(LexType::Identifier("g"))]))]
#[case(
    "x |> f |>> g",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::PipeForward),
        T(LexType::Identifier("f")),
        T(LexType::PipeDoubleForward),
        T(LexType::Identifier("g")),
    ])
)]
// ── Bitwise / shift ───────────────────────────────────────────────────────────
#[case("a | b", Ok(vec![T(LexType::Identifier("a")), T(LexType::Pipe), T(LexType::Identifier("b"))]))]
#[case("a >> b", Ok(vec![T(LexType::Identifier("a")), T(LexType::DoubleGreater), T(LexType::Identifier("b"))]))]
#[case("a << b", Ok(vec![T(LexType::Identifier("a")), T(LexType::DoubleLesser), T(LexType::Identifier("b"))]))]
// ── Misc ──────────────────────────────────────────────────────────────────────
#[case("x?", Ok(vec![T(LexType::Identifier("x")), T(LexType::Coalescence)]))]
#[case("@x", Ok(vec![T(LexType::Ampersand), T(LexType::Identifier("x"))]))]
#[case("x!", Ok(vec![T(LexType::Identifier("x")), T(LexType::Promotion)]))]
#[case("a..b", Ok(vec![T(LexType::Identifier("a")), T(LexType::DoubleDot), T(LexType::Identifier("b"))]))]
#[case("a -> b", Ok(vec![T(LexType::Identifier("a")), T(LexType::Arrow), T(LexType::Identifier("b"))]))]
#[case("a -- b", Ok(vec![T(LexType::Identifier("a")), T(LexType::DoubleMinus), T(LexType::Identifier("b"))]))]
// ── Dot disambiguation: single dot vs double dot ──────────────────────────────
#[case("a.b", Ok(vec![T(LexType::Identifier("a")), T(LexType::Dot), T(LexType::Identifier("b"))]))]
#[case("a..b", Ok(vec![T(LexType::Identifier("a")), T(LexType::DoubleDot), T(LexType::Identifier("b"))]))]
// ── Colon disambiguation: colon vs := ────────────────────────────────────────
#[case("x : y", Ok(vec![T(LexType::Identifier("x")), T(LexType::Colon), T(LexType::Identifier("y"))]))]
// ── Chained expression ────────────────────────────────────────────────────────
#[case(
    "a + b * c - d / e",
    Ok(vec![
        T(LexType::Identifier("a")),
        T(LexType::Plus),
        T(LexType::Identifier("b")),
        T(LexType::Asterisk),
        T(LexType::Identifier("c")),
        T(LexType::Minus),
        T(LexType::Identifier("d")),
        T(LexType::Slash),
        T(LexType::Identifier("e")),
    ])
)]
fn test_operators(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
