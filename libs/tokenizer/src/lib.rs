#[macro_export]
macro_rules! tokens {
    (
        $l:lifetime
        $(,)?
    ) => {
        std::iter::Peekable<impl Iterator<Item = $crate::Res<$crate::Token<$l>>>>
    };
    () => {
        std::iter::Peekable<impl Iterator<Item = $crate::Res<$crate::Token<'_>>>>
    };
}

mod s1_whitespace_stripper;
mod s2_scoper;

use std::ops::Range;

use thiserror::Error;

pub use lexer::{Brace, BraceDirection, LexType, StringType};

use crate::{
    s1_whitespace_stripper::{INDENTATION_SIZE, strip},
    s2_scoper::scope as scope_inner,
};

pub type Res<T = ()> = Result<T, Error>;

pub fn scope(source: &str) -> tokens!() {
    scope_inner(strip(lexer::lex(source))).peekable()
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Token(LexType<'a>, Position),
    ScopeStart(Option<(Brace, Position)>),
    ScopeEnd(Option<(Brace, Position)>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub source_range: Range<usize>,
    pub line: usize,
    pub line_range: Range<usize>,
    pub indentation_level: usize,
}
