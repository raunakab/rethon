use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'a> {
    Token(Token<'a>),
    Scope(Scope<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope<'a> {
    pub brace: Option<Brace>,
    pub nodes: Vec<Node<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub source_range: Range<usize>,
    pub line: usize,
    pub line_range: Range<usize>,
    pub indentation_level: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType<'a> {
    // Control
    Semicolon, // ;
    Comma,     // ,

    // Identifiers
    MacroIdentifier(&'a str),
    Identifier(&'a str),
    String(&'a str, StringType),
    Number(&'a str),
    Float(&'a str, Option<&'a str>),

    // Keywords
    Function,      // fn
    Scope,         // scope
    Return,        // return
    Yield,         // yield
    Not,           // not
    And,           // and
    Or,            // or
    For,           // for
    Loop,          // loop
    If,            // if
    Else,          // else
    True,          // true
    False,         // false
    Struct,        // struct
    Enum,          // enum
    Panic,         // panic
    Todo,          // todo
    Unimplemented, // unimplemented
    Mutable,       // mut

    // Operators
    ConstantAssignment, // :=
    Assignment,         // =
    Equals,             // ==
    Promotion,          // !
    Coalescence,        // ?
    Ampersand,          // @
    Colon,              // :
    Dot,                // .
    DoubleDot,          // ..
    Plus,               // +
    Minus,              // -
    DoubleMinus,        // --
    Arrow,              // ->
    Asterisk,           // *
    DoubleAsterisk,     // **
    Slash,              // /
    Pipe,               // |
    PipeForward,        // |>
    PipeDoubleForward,  // |>>
    Greater,            // >
    DoubleGreater,      // >>
    GreaterOrEqual,     // >=
    Lesser,             // <
    DoubleLesser,       // <<
    LesserOrEqual,      // <=
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brace {
    Whitespace,
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
