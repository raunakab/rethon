mod common;

use common::S::T;
use common::{S, collect};
use tokenizer::{LexType, Res};

#[rstest::rstest]
// Plain integer
#[case("42", Ok(vec![T(LexType::Number("42"))]))]
// Zero
#[case("0", Ok(vec![T(LexType::Number("0"))]))]
// Multi-digit integer
#[case("1234", Ok(vec![T(LexType::Number("1234"))]))]
// Float with fractional part
#[case(
    "3.14",
    Ok(vec![T(LexType::Float("3", Some("14")))])
)]
// Float without fractional part (trailing dot)
#[case(
    "12.",
    Ok(vec![T(LexType::Float("12", None))])
)]
// Float with leading zero
#[case(
    "0.5",
    Ok(vec![T(LexType::Float("0", Some("5")))])
)]
// Float zero dot zero
#[case(
    "0.0",
    Ok(vec![T(LexType::Float("0", Some("0")))])
)]
// Number in a constant assignment
#[case(
    "x := 99",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Number("99")),
    ])
)]
// Float in a mutable assignment
#[case(
    "x = 1.5",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::Assignment),
        T(LexType::Float("1", Some("5"))),
    ])
)]
// Number in an arithmetic expression
#[case(
    "x + 42",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::Plus),
        T(LexType::Number("42")),
    ])
)]
// Integer immediately followed by identifier — two distinct tokens
#[case(
    "42abc",
    Ok(vec![
        T(LexType::Number("42")),
        T(LexType::Identifier("abc")),
    ])
)]
fn test_numbers(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
