use tokenizer::{Res, Token, TokenType, tokenize};

#[derive(Debug, PartialEq)]
pub enum S<'a> {
    T(TokenType<'a>),
    Open,
    Close,
}

pub fn collect(source: &str) -> Res<Vec<S<'_>>> {
    tokenize(source)
        .map(|res| {
            res.map(|token| match token {
                Token::Token(ty, _) => S::T(ty),
                Token::ScopeStart(_) => S::Open,
                Token::ScopeEnd(_) => S::Close,
            })
        })
        .collect()
}
