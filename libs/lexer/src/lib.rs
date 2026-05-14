mod s1_segmenter;
mod s2_clusterer;
mod s3_whitespace_stripper;
mod s4_scoper;

use std::ops::Range;

use derive_more::Display;
use thiserror::Error;

#[macro_export]
macro_rules! tokens {
    ($l:lifetime $(,)?) => {
        std::iter::Peekable<impl Iterator<Item = $crate::Res<$crate::TokenTree<$l>>>>
    };
    () => {
        std::iter::Peekable<impl Iterator<Item = $crate::Res<$crate::TokenTree<'_>>>>
    };
}

pub type Res<T = ()> = Result<T, Error>;

pub fn lex(source: &str) -> tokens!() {
    s4_scoper::scope(s3_whitespace_stripper::whitespace_strip(
        s2_clusterer::cluster(s1_segmenter::segment(source)),
    ))
    .peekable()
}

#[cfg(test)]
pub(crate) fn lex_items(source: &str) -> impl Iterator<Item = Res<LexItem<'_>>> {
    s2_clusterer::cluster(s1_segmenter::segment(source))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LexItem<'a> {
    pub(crate) kind: LexKind<'a>,
    pub(crate) range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum LexKind<'a> {
    Normal(Token<'a>),
    Whitespace(usize),
    Newline,
    Brace(Brace, BraceDirection),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenTree<'a> {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
pub enum Token<'a> {
    // Control
    #[display(";")]
    Semicolon,
    #[display(",")]
    Comma,

    // Identifiers
    #[display("!{_0}")]
    MacroIdentifier(&'a str),
    #[display("{_0}")]
    Identifier(&'a str),
    #[display(r#""{_0}""#)]
    String(&'a str, StringType),
    #[display("{_0}")]
    Number(&'a str),
    #[display("{_0}.{}", _1.unwrap_or(""))]
    Float(&'a str, Option<&'a str>),

    // Keywords
    #[display("fn")]
    Function,
    #[display("mut")]
    Mutable,
    #[display("scope")]
    Scope,
    #[display("return")]
    Return,
    #[display("yield")]
    Yield,
    #[display("throw")]
    Throw,
    #[display("otherwise")]
    Otherwise,
    #[display("true")]
    True,
    #[display("false")]
    False,
    #[display("not")]
    Not,
    #[display("and")]
    And,
    #[display("or")]
    Or,
    #[display("for")]
    For,
    #[display("loop")]
    Loop,
    #[display("if")]
    If,
    #[display("else")]
    Else,
    #[display("struct")]
    Struct,
    #[display("enum")]
    Enum,
    #[display("panic")]
    Panic,
    #[display("todo")]
    Todo,
    #[display("unimplemented")]
    Unimplemented,

    // Operators
    #[display(":=")]
    StaticAssignment,
    #[display("=")]
    Assignment,
    #[display("==")]
    Equals,
    #[display("!")]
    Promotion,
    #[display("?")]
    Coalescence,
    #[display("@")]
    Ampersand,
    #[display(":")]
    Colon,
    #[display(".")]
    Dot,
    #[display("..")]
    DoubleDot,
    #[display("+")]
    Plus,
    #[display("-")]
    Minus,
    #[display("--")]
    DoubleMinus,
    #[display("->")]
    Arrow,
    #[display("*")]
    Asterisk,
    #[display("**")]
    DoubleAsterisk,
    #[display("/")]
    Slash,
    #[display("|")]
    Pipe,
    #[display("|>")]
    PipeForward,
    #[display("|>>")]
    PipeDoubleForward,
    #[display(">")]
    Greater,
    #[display(">>")]
    DoubleGreater,
    #[display(">=")]
    GreaterOrEqual,
    #[display("<")]
    Lesser,
    #[display("<<")]
    DoubleLesser,
    #[display("<=")]
    LesserOrEqual,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brace {
    Round,
    Square,
    Curly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BraceDirection {
    Open,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringType {
    Normal,
    Formatted,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("Invalid whitespace being used: {0}")]
    InvalidWhitespace(String),
    #[error("Unknown item: {0}")]
    UnknownItem(String),
    #[error("Unterminated string at byte {0}")]
    UnterminatedString(usize),
    #[error("Invalid indentation: found {found} spaces at byte {position}")]
    InvalidIndentation { found: usize, position: usize },
}
