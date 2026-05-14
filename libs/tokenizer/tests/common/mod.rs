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
                ScopeItem::Start(_) => S::Open,
                ScopeItem::End(_) => S::Close,
            })
        })
        .collect()
}
