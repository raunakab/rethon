#[macro_export]
macro_rules! tokens {
    (
        $l:lifetime
        $(,)?
    ) => {
        std::iter::Peekable<impl Iterator<Item = $crate::Res<$crate::ScopeItem<$l>>>>
    };
    () => {
        std::iter::Peekable<impl Iterator<Item = $crate::Res<$crate::ScopeItem<'_>>>>
    };
}

mod s1_whitespace_stripper;
mod s2_scoper;

use std::ops::Range;

use thiserror::Error;

pub use lexer::{Brace, BraceDirection, StringType, Token};

use crate::{
    s1_whitespace_stripper::{INDENTATION_SIZE, whitespace_strip},
    s2_scoper::scope as scope_inner,
};

pub type Res<T = ()> = Result<T, Error>;

pub fn scope(source: &str) -> tokens!() {
    scope_inner(whitespace_strip(lexer::lex(source))).peekable()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeItem<'a> {
    Token(Token<'a>, Position),
    Start(Option<(Brace, Position)>),
    End(Option<(Brace, Position)>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub source_range: Range<usize>,
    pub line: usize,
    pub line_range: Range<usize>,
    pub indentation_level: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error(transparent)]
    Lex(#[from] lexer::Error),

    #[error(
        "Invalid indentation at byte {position}: expected multiple of {}, found {found}",
        INDENTATION_SIZE
    )]
    InvalidIndentation { found: usize, position: usize },
}
