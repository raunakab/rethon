use std::ops::Range;

use derive_more::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'a> {
    Token(TokenType<'a>, Position),
    ScopeStart(Option<(Brace, Position)>),
    ScopeEnd(Option<Position>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    // pub token_type: TokenType<'a>,
    pub source_range: Range<usize>,
    pub line: usize,
    pub line_range: Range<usize>,
    pub indentation_level: usize,
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
    // function keywords
    #[display("fn")]
    Function,

    // definitional keywords
    #[display("mut")]
    Mutable,
    #[display("scope")]
    Scope,

    // control keywords
    #[display("return")]
    Return,
    #[display("yield")]
    Yield,
    #[display("throw")]
    Throw,
    #[display("otherwise")]
    Otherwise,

    // boolean keywords
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

    // conditional keywords
    #[display("for")]
    For,
    #[display("loop")]
    Loop,
    #[display("if")]
    If,
    #[display("else")]
    Else,

    // type-algebra keywords
    #[display("struct")]
    Struct,
    #[display("enum")]
    Enum,

    // type-hole keywords
    #[display("panic")]
    Panic,
    #[display("todo")]
    Todo,
    #[display("unimplemented")]
    Unimplemented,

    // Operators
    #[display(":=")]
    ConstantAssignment,
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
