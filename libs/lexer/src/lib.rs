mod lexer_s1;
mod lexer_s2;

use std::ops::Range;

use derive_more::Display;
use thiserror::Error;

pub type Res<T = ()> = Result<T, Error>;

pub fn lex(source: &str) -> impl Iterator<Item = Res<LexToken<'_>>> {
    lexer_s2::tokenize(lexer_s1::tokenize(source))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexToken<'a> {
    pub kind: LexKind<'a>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexKind<'a> {
    Normal(TokenType<'a>),
    Whitespace(usize),
    Newline,
    Brace(Brace, BraceDirection),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("Invalid whitespace being used: {0}")]
    InvalidWhitespace(String),
    #[error("Unknown token: {0}")]
    UnknownToken(String),
    #[error("Unterminated string at byte {0}")]
    UnterminatedString(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
pub enum TokenType<'a> {
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
