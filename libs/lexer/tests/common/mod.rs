use lexer::{BraceDirection, Res, Token, TokenTree, lex};

#[derive(Debug, PartialEq)]
pub enum S<'a> {
    T(Token<'a>),
    Open,
    Close,
}

pub fn collect(source: &str) -> Res<Vec<S<'_>>> {
    lex(source)
        .map(|res| {
            res.map(|token| match token {
                TokenTree::Token(ty, _) => S::T(ty),
                TokenTree::Scope((BraceDirection::Open, _)) => S::Open,
                TokenTree::Scope((BraceDirection::Close, _)) => S::Close,
            })
        })
        .collect()
}
