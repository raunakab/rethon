mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use lexer::{Res, Token};

#[rstest::rstest]
// Bare match with no body
#[case(
    "match x",
    Ok(vec![
        T(Token::Match),
        T(Token::Identifier("x")),
    ])
)]
// "match" prefix in a longer identifier is not a keyword
#[case(
    "matchbox",
    Ok(vec![T(Token::Identifier("matchbox"))])
)]
// Match with a single indented body
#[case(
    "match x\n    foo",
    Ok(vec![
        T(Token::Match),
        T(Token::Identifier("x")),
        Open,
        T(Token::Identifier("foo")),
        Close,
    ])
)]
// Match with multiple arms
#[case(
    "match x\n    foo\n    bar",
    Ok(vec![
        T(Token::Match),
        T(Token::Identifier("x")),
        Open,
        T(Token::Identifier("foo")),
        T(Token::Identifier("bar")),
        Close,
    ])
)]
// Match nested inside a function
#[case(
    "fn f\n    match x\n        foo",
    Ok(vec![
        T(Token::Function),
        T(Token::Identifier("f")),
        Open,
        T(Token::Match),
        T(Token::Identifier("x")),
        Open,
        T(Token::Identifier("foo")),
        Close,
        Close,
    ])
)]
fn test_match(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
