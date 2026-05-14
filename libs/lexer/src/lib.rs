mod s1_segmenter;
mod s2_clusterer;

use std::ops::Range;

use derive_more::Display;
use thiserror::Error;

pub type Res<T = ()> = Result<T, Error>;

pub fn lex(source: &str) -> impl Iterator<Item = Res<LexItem<'_>>> {
    s2_clusterer::cluster(s1_segmenter::segment(source))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexItem<'a> {
    pub kind: LexKind<'a>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexKind<'a> {
    Normal(Token<'a>),
    Whitespace(usize),
    Newline,
    Brace(Brace, BraceDirection),
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
}
