use lexer::{Brace, BraceDirection, Token, lex};

use crate::{
    Error, Res,
    s1_whitespace_stripper::{StrippedToken, StrippedTokenKind, strip},
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct SimpleToken<'a> {
    kind: StrippedTokenKind<'a>,
    line: usize,
    indentation_level: usize,
}

impl<'a> From<StrippedToken<'a>> for SimpleToken<'a> {
    fn from(token: StrippedToken<'a>) -> Self {
        SimpleToken {
            kind: token.kind,
            line: token.position.line,
            indentation_level: token.position.indentation_level,
        }
    }
}

fn n(token_type: Token<'_>, line: usize, indentation_level: usize) -> SimpleToken<'_> {
    SimpleToken {
        kind: StrippedTokenKind::Normal(token_type),
        line,
        indentation_level,
    }
}

fn b(
    brace: Brace,
    dir: BraceDirection,
    line: usize,
    indentation_level: usize,
) -> SimpleToken<'static> {
    SimpleToken {
        kind: StrippedTokenKind::Brace(brace, dir),
        line,
        indentation_level,
    }
}

#[rstest::rstest]
#[case("", Ok(vec![]))]
#[case("fn", Ok(vec![n(Token::Function, 0, 0)]))]
#[case("fn add", Ok(vec![
    n(Token::Function, 0, 0),
    n(Token::Identifier("add"), 0, 0),
]))]
#[case("fn add\nreturn", Ok(vec![
    n(Token::Function, 0, 0),
    n(Token::Identifier("add"), 0, 0),
    n(Token::Return, 1, 0),
]))]
#[case("fn\n    add", Ok(vec![
    n(Token::Function, 0, 0),
    n(Token::Identifier("add"), 1, 1),
]))]
#[case("fn\n    if\n        x", Ok(vec![
    n(Token::Function, 0, 0),
    n(Token::If, 1, 1),
    n(Token::Identifier("x"), 2, 2),
]))]
#[case("fn\n    x\ny", Ok(vec![
    n(Token::Function, 0, 0),
    n(Token::Identifier("x"), 1, 1),
    n(Token::Identifier("y"), 2, 0),
]))]
#[case("a\n    x\n    y", Ok(vec![
    n(Token::Identifier("a"), 0, 0),
    n(Token::Identifier("x"), 1, 1),
    n(Token::Identifier("y"), 2, 1),
]))]
#[case("fn\n    if\n        x\n        y\n    else\n        z", Ok(vec![
    n(Token::Function, 0, 0),
    n(Token::If, 1, 1),
    n(Token::Identifier("x"), 2, 2),
    n(Token::Identifier("y"), 3, 2),
    n(Token::Else, 4, 1),
    n(Token::Identifier("z"), 5, 2),
]))]
#[case("a\n    b\n        c\n            d", Ok(vec![
    n(Token::Identifier("a"), 0, 0),
    n(Token::Identifier("b"), 1, 1),
    n(Token::Identifier("c"), 2, 2),
    n(Token::Identifier("d"), 3, 3),
]))]
#[case("fn\r\n    add", Ok(vec![
    n(Token::Function, 0, 0),
    n(Token::Identifier("add"), 1, 1),
]))]
#[case("fn\n\n\n    add", Ok(vec![
    n(Token::Function, 0, 0),
    n(Token::Identifier("add"), 3, 1),
]))]
#[case("fn   \n    add", Ok(vec![
    n(Token::Function, 0, 0),
    n(Token::Identifier("add"), 1, 1),
]))]
#[case("func()", Ok(vec![
    n(Token::Identifier("func"), 0, 0),
    b(Brace::Round, BraceDirection::Open, 0, 0),
    b(Brace::Round, BraceDirection::Close, 0, 0),
]))]
#[case("{x}", Ok(vec![
    b(Brace::Curly, BraceDirection::Open, 0, 0),
    n(Token::Identifier("x"), 0, 0),
    b(Brace::Curly, BraceDirection::Close, 0, 0),
]))]
#[case("[x]", Ok(vec![
    b(Brace::Square, BraceDirection::Open, 0, 0),
    n(Token::Identifier("x"), 0, 0),
    b(Brace::Square, BraceDirection::Close, 0, 0),
]))]
#[case("fn\n   add", Err(Error::InvalidIndentation { found: 3, position: 3 }))]
#[case("x\n     y", Err(Error::InvalidIndentation { found: 5, position: 2 }))]
#[case("a\n  b", Err(Error::InvalidIndentation { found: 2, position: 2 }))]
#[case("x\n        y", Ok(vec![
    n(Token::Identifier("x"), 0, 0),
    n(Token::Identifier("y"), 1, 2),
]))]
#[case("fn\n    x = y", Ok(vec![
    n(Token::Function, 0, 0),
    n(Token::Identifier("x"), 1, 1),
    n(Token::Assignment, 1, 1),
    n(Token::Identifier("y"), 1, 1),
]))]
fn test_s1_strip(#[case] source: &str, #[case] expected: Res<Vec<SimpleToken<'static>>>) {
    assert_eq!(
        strip(lex(source))
            .map(|token| {
                let token = token?;
                Ok(SimpleToken::from(token))
            })
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
