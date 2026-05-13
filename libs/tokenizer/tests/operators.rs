mod common;

use common::S::T;
use common::{S, collect};
use tokenizer::{Res, TokenType};

#[rstest::rstest]
// ── Arithmetic ────────────────────────────────────────────────────────────────
#[case("a + b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::Plus), T(TokenType::Identifier("b"))]))]
#[case("a - b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::Minus), T(TokenType::Identifier("b"))]))]
#[case("a * b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::Asterisk), T(TokenType::Identifier("b"))]))]
#[case("a / b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::Slash), T(TokenType::Identifier("b"))]))]
#[case("a ** b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::DoubleAsterisk), T(TokenType::Identifier("b"))]))]
// ── Comparison ────────────────────────────────────────────────────────────────
#[case("a == b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::Equals), T(TokenType::Identifier("b"))]))]
#[case("a >= b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::GreaterOrEqual), T(TokenType::Identifier("b"))]))]
#[case("a <= b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::LesserOrEqual), T(TokenType::Identifier("b"))]))]
#[case("a > b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::Greater), T(TokenType::Identifier("b"))]))]
#[case("a < b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::Lesser), T(TokenType::Identifier("b"))]))]
// ── Boolean ───────────────────────────────────────────────────────────────────
#[case("not a", Ok(vec![T(TokenType::Not), T(TokenType::Identifier("a"))]))]
#[case("a and b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::And), T(TokenType::Identifier("b"))]))]
#[case("a or b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::Or), T(TokenType::Identifier("b"))]))]
// ── Pipe ──────────────────────────────────────────────────────────────────────
#[case("x |> f", Ok(vec![T(TokenType::Identifier("x")), T(TokenType::PipeForward), T(TokenType::Identifier("f"))]))]
#[case("x |>> g", Ok(vec![T(TokenType::Identifier("x")), T(TokenType::PipeDoubleForward), T(TokenType::Identifier("g"))]))]
#[case(
    "x |> f |>> g",
    Ok(vec![
        T(TokenType::Identifier("x")),
        T(TokenType::PipeForward),
        T(TokenType::Identifier("f")),
        T(TokenType::PipeDoubleForward),
        T(TokenType::Identifier("g")),
    ])
)]
// ── Bitwise / shift ───────────────────────────────────────────────────────────
#[case("a | b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::Pipe), T(TokenType::Identifier("b"))]))]
#[case("a >> b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::DoubleGreater), T(TokenType::Identifier("b"))]))]
#[case("a << b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::DoubleLesser), T(TokenType::Identifier("b"))]))]
// ── Misc ──────────────────────────────────────────────────────────────────────
#[case("x?", Ok(vec![T(TokenType::Identifier("x")), T(TokenType::Coalescence)]))]
#[case("@x", Ok(vec![T(TokenType::Ampersand), T(TokenType::Identifier("x"))]))]
#[case("x!", Ok(vec![T(TokenType::Identifier("x")), T(TokenType::Promotion)]))]
#[case("a..b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::DoubleDot), T(TokenType::Identifier("b"))]))]
#[case("a -> b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::Arrow), T(TokenType::Identifier("b"))]))]
#[case("a -- b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::DoubleMinus), T(TokenType::Identifier("b"))]))]
// ── Dot disambiguation: single dot vs double dot ──────────────────────────────
#[case("a.b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::Dot), T(TokenType::Identifier("b"))]))]
#[case("a..b", Ok(vec![T(TokenType::Identifier("a")), T(TokenType::DoubleDot), T(TokenType::Identifier("b"))]))]
// ── Colon disambiguation: colon vs := ────────────────────────────────────────
#[case("x : y", Ok(vec![T(TokenType::Identifier("x")), T(TokenType::Colon), T(TokenType::Identifier("y"))]))]
// ── Chained expression ────────────────────────────────────────────────────────
#[case(
    "a + b * c - d / e",
    Ok(vec![
        T(TokenType::Identifier("a")),
        T(TokenType::Plus),
        T(TokenType::Identifier("b")),
        T(TokenType::Asterisk),
        T(TokenType::Identifier("c")),
        T(TokenType::Minus),
        T(TokenType::Identifier("d")),
        T(TokenType::Slash),
        T(TokenType::Identifier("e")),
    ])
)]
fn test_operators(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
