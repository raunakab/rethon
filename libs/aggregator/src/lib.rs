pub fn aggregator<'a>(tokenizer: impl Iterator<Item = tokenizer::Token<'a>>) {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    MacroIdentifier(&'a str),
    Identifier(&'a str),
    Number(&'a str),

    // Keywords
    Function, // fn
    Scope,    // scope
    Return,   // return
    Yield,    // yield
    Not,      // not
    And,      // and
    Or,       // or
    For,      // for
    Loop,     // loop
    If,       // if
    Else,     // else
    True,     // true
    False,    // false

    // Operators
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
    PipeDoubleBackward, // |<<

    Greater,        // >
    DoubleGreater,  // >>
    GreaterOrEqual, // >=
    Lesser,         // <
    DoubleLesser,   // <<
    LesserOrEqual,  // <=

    // Keywords
    Brace(Brace, BraceDirection),
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
