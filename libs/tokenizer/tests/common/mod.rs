use scoper::{Res, ScopeItem, Token, scope};

#[derive(Debug, PartialEq)]
pub enum S<'a> {
    T(Token<'a>),
    Open,
    Close,
}

pub fn collect(source: &str) -> Res<Vec<S<'_>>> {
    scope(source)
        .map(|res| {
            res.map(|token| match token {
                ScopeItem::Token(ty, _) => S::T(ty),
                ScopeItem::ScopeStart(_) => S::Open,
                ScopeItem::ScopeEnd(_) => S::Close,
            })
        })
        .collect()
}
