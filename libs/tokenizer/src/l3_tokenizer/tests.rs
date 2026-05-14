use lexer::{Brace, BraceDirection, TokenType, lex};

use crate::{
    Error, Res,
    l3_tokenizer::{L3Token, L3TokenType, l3_tokenize},
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct SimpleToken<'a> {
    token_type: L3TokenType<'a>,
    line: usize,
    indentation_level: usize,
}

impl<'a> From<L3Token<'a>> for SimpleToken<'a> {
    fn from(token: L3Token<'a>) -> Self {
        SimpleToken {
            token_type: token.token_type,
            line: token.position.line,
            indentation_level: token.position.indentation_level,
        }
    }
}

fn n(token_type: TokenType<'_>, line: usize, indentation_level: usize) -> SimpleToken<'_> {
    SimpleToken {
        token_type: L3TokenType::Normal(token_type),
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
        token_type: L3TokenType::Brace(brace, dir),
        line,
        indentation_level,
    }
}

#[rstest::rstest]
#[case("", Ok(vec![]))]
#[case("fn", Ok(vec![n(TokenType::Function, 0, 0)]))]
#[case("fn add", Ok(vec![
    n(TokenType::Function, 0, 0),
    n(TokenType::Identifier("add"), 0, 0),
]))]
#[case("fn add\nreturn", Ok(vec![
    n(TokenType::Function, 0, 0),
    n(TokenType::Identifier("add"), 0, 0),
    n(TokenType::Return, 1, 0),
]))]
#[case("fn\n    add", Ok(vec![
    n(TokenType::Function, 0, 0),
    n(TokenType::Identifier("add"), 1, 1),
]))]
#[case("fn\n    if\n        x", Ok(vec![
    n(TokenType::Function, 0, 0),
    n(TokenType::If, 1, 1),
    n(TokenType::Identifier("x"), 2, 2),
]))]
#[case("fn\n    x\ny", Ok(vec![
    n(TokenType::Function, 0, 0),
    n(TokenType::Identifier("x"), 1, 1),
    n(TokenType::Identifier("y"), 2, 0),
]))]
#[case("a\n    x\n    y", Ok(vec![
    n(TokenType::Identifier("a"), 0, 0),
    n(TokenType::Identifier("x"), 1, 1),
    n(TokenType::Identifier("y"), 2, 1),
]))]
#[case("fn\n    if\n        x\n        y\n    else\n        z", Ok(vec![
    n(TokenType::Function, 0, 0),
    n(TokenType::If, 1, 1),
    n(TokenType::Identifier("x"), 2, 2),
    n(TokenType::Identifier("y"), 3, 2),
    n(TokenType::Else, 4, 1),
    n(TokenType::Identifier("z"), 5, 2),
]))]
#[case("a\n    b\n        c\n            d", Ok(vec![
    n(TokenType::Identifier("a"), 0, 0),
    n(TokenType::Identifier("b"), 1, 1),
    n(TokenType::Identifier("c"), 2, 2),
    n(TokenType::Identifier("d"), 3, 3),
]))]
#[case("fn\r\n    add", Ok(vec![
    n(TokenType::Function, 0, 0),
    n(TokenType::Identifier("add"), 1, 1),
]))]
#[case("fn\n\n\n    add", Ok(vec![
    n(TokenType::Function, 0, 0),
    n(TokenType::Identifier("add"), 3, 1),
]))]
#[case("fn   \n    add", Ok(vec![
    n(TokenType::Function, 0, 0),
    n(TokenType::Identifier("add"), 1, 1),
]))]
#[case("func()", Ok(vec![
    n(TokenType::Identifier("func"), 0, 0),
    b(Brace::Round, BraceDirection::Open, 0, 0),
    b(Brace::Round, BraceDirection::Close, 0, 0),
]))]
#[case("{x}", Ok(vec![
    b(Brace::Curly, BraceDirection::Open, 0, 0),
    n(TokenType::Identifier("x"), 0, 0),
    b(Brace::Curly, BraceDirection::Close, 0, 0),
]))]
#[case("[x]", Ok(vec![
    b(Brace::Square, BraceDirection::Open, 0, 0),
    n(TokenType::Identifier("x"), 0, 0),
    b(Brace::Square, BraceDirection::Close, 0, 0),
]))]
#[case("fn\n   add", Err(Error::InvalidIndentation { found: 3, position: 3 }))]
#[case("x\n     y", Err(Error::InvalidIndentation { found: 5, position: 2 }))]
#[case("a\n  b", Err(Error::InvalidIndentation { found: 2, position: 2 }))]
#[case("x\n        y", Ok(vec![
    n(TokenType::Identifier("x"), 0, 0),
    n(TokenType::Identifier("y"), 1, 2),
]))]
#[case("fn\n    x = y", Ok(vec![
    n(TokenType::Function, 0, 0),
    n(TokenType::Identifier("x"), 1, 1),
    n(TokenType::Assignment, 1, 1),
    n(TokenType::Identifier("y"), 1, 1),
]))]
fn test_l3_tokenization(#[case] source: &str, #[case] expected: Res<Vec<SimpleToken<'static>>>) {
    assert_eq!(
        l3_tokenize(lex(source))
            .map(|token| {
                let token = token?;
                Ok(SimpleToken::from(token))
            })
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
