mod common;

use common::S::T;
use common::{S, collect};
use scoper::{Res, Token};

#[rstest::rstest]
// ── Arithmetic ────────────────────────────────────────────────────────────────
#[case("a + b", Ok(vec![T(Token::Identifier("a")), T(Token::Plus), T(Token::Identifier("b"))]))]
#[case("a - b", Ok(vec![T(Token::Identifier("a")), T(Token::Minus), T(Token::Identifier("b"))]))]
#[case("a * b", Ok(vec![T(Token::Identifier("a")), T(Token::Asterisk), T(Token::Identifier("b"))]))]
#[case("a / b", Ok(vec![T(Token::Identifier("a")), T(Token::Slash), T(Token::Identifier("b"))]))]
#[case("a ** b", Ok(vec![T(Token::Identifier("a")), T(Token::DoubleAsterisk), T(Token::Identifier("b"))]))]
// ── Comparison ────────────────────────────────────────────────────────────────
#[case("a == b", Ok(vec![T(Token::Identifier("a")), T(Token::Equals), T(Token::Identifier("b"))]))]
#[case("a >= b", Ok(vec![T(Token::Identifier("a")), T(Token::GreaterOrEqual), T(Token::Identifier("b"))]))]
#[case("a <= b", Ok(vec![T(Token::Identifier("a")), T(Token::LesserOrEqual), T(Token::Identifier("b"))]))]
#[case("a > b", Ok(vec![T(Token::Identifier("a")), T(Token::Greater), T(Token::Identifier("b"))]))]
#[case("a < b", Ok(vec![T(Token::Identifier("a")), T(Token::Lesser), T(Token::Identifier("b"))]))]
// ── Boolean ───────────────────────────────────────────────────────────────────
#[case("not a", Ok(vec![T(Token::Not), T(Token::Identifier("a"))]))]
#[case("a and b", Ok(vec![T(Token::Identifier("a")), T(Token::And), T(Token::Identifier("b"))]))]
#[case("a or b", Ok(vec![T(Token::Identifier("a")), T(Token::Or), T(Token::Identifier("b"))]))]
// ── Pipe ──────────────────────────────────────────────────────────────────────
#[case("x |> f", Ok(vec![T(Token::Identifier("x")), T(Token::PipeForward), T(Token::Identifier("f"))]))]
#[case("x |>> g", Ok(vec![T(Token::Identifier("x")), T(Token::PipeDoubleForward), T(Token::Identifier("g"))]))]
#[case(
    "x |> f |>> g",
    Ok(vec![
        T(Token::Identifier("x")),
        T(Token::PipeForward),
        T(Token::Identifier("f")),
        T(Token::PipeDoubleForward),
        T(Token::Identifier("g")),
    ])
)]
// ── Bitwise / shift ───────────────────────────────────────────────────────────
#[case("a | b", Ok(vec![T(Token::Identifier("a")), T(Token::Pipe), T(Token::Identifier("b"))]))]
#[case("a >> b", Ok(vec![T(Token::Identifier("a")), T(Token::DoubleGreater), T(Token::Identifier("b"))]))]
#[case("a << b", Ok(vec![T(Token::Identifier("a")), T(Token::DoubleLesser), T(Token::Identifier("b"))]))]
// ── Misc ──────────────────────────────────────────────────────────────────────
#[case("x?", Ok(vec![T(Token::Identifier("x")), T(Token::Coalescence)]))]
#[case("@x", Ok(vec![T(Token::Ampersand), T(Token::Identifier("x"))]))]
#[case("x!", Ok(vec![T(Token::Identifier("x")), T(Token::Promotion)]))]
#[case("a..b", Ok(vec![T(Token::Identifier("a")), T(Token::DoubleDot), T(Token::Identifier("b"))]))]
#[case("a -> b", Ok(vec![T(Token::Identifier("a")), T(Token::Arrow), T(Token::Identifier("b"))]))]
#[case("a -- b", Ok(vec![T(Token::Identifier("a")), T(Token::DoubleMinus), T(Token::Identifier("b"))]))]
// ── Dot disambiguation: single dot vs double dot ──────────────────────────────
#[case("a.b", Ok(vec![T(Token::Identifier("a")), T(Token::Dot), T(Token::Identifier("b"))]))]
#[case("a..b", Ok(vec![T(Token::Identifier("a")), T(Token::DoubleDot), T(Token::Identifier("b"))]))]
// ── Colon disambiguation: colon vs := ────────────────────────────────────────
#[case("x : y", Ok(vec![T(Token::Identifier("x")), T(Token::Colon), T(Token::Identifier("y"))]))]
// ── Chained expression ────────────────────────────────────────────────────────
#[case(
    "a + b * c - d / e",
    Ok(vec![
        T(Token::Identifier("a")),
        T(Token::Plus),
        T(Token::Identifier("b")),
        T(Token::Asterisk),
        T(Token::Identifier("c")),
        T(Token::Minus),
        T(Token::Identifier("d")),
        T(Token::Slash),
        T(Token::Identifier("e")),
    ])
)]
fn test_operators(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
