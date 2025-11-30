#[cfg(test)]
mod tests;

use std::{iter::Peekable, ops::Range};

pub fn aggregator<'a>(
    tokenizer: impl Iterator<Item = tokenizer::Token<'a>>,
) -> impl Iterator<Item = Token<'a>> {
    Aggregator {
        tokenizer: tokenizer.peekable(),
    }
}

struct Aggregator<'a, I>
where
    I: Iterator<Item = tokenizer::Token<'a>>,
{
    tokenizer: Peekable<I>,
}

impl<'a, I> Iterator for Aggregator<'a, I>
where
    I: Iterator<Item = tokenizer::Token<'a>>,
{
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Helper macro to peek and match on next token
        // Consumes the token if it matches any of the patterns
        // Automatically wraps tuple patterns in Some()
        macro_rules! peek {
            ($(($($inner:tt)*) => $result:expr),+ $(, _ => $default:expr)? $(,)?) => {{
                let next_token = self.tokenizer.peek().map(|t| (t.token, t.token_type));
                match next_token {
                    $(
                        Some(($($inner)*)) => {
                            self.tokenizer.next();
                            $result
                        }
                    )+
                    $(
                        _ => $default
                    )?
                }
            }};
        }

        loop {
            let Some(tokenizer::Token {
                token,
                range,
                token_type,
            }) = self.tokenizer.next()
            else {
                break None;
            };

            let token_type = match token_type {
                tokenizer::TokenType::Whitespace => match token {
                    "\n" => TokenType::Newline,
                    "\t" => TokenType::Tab,
                    _ => continue, // Skip other whitespace (spaces, etc.)
                },
                tokenizer::TokenType::Keyword => match token {
                    "fn" => TokenType::Function,
                    "scope" => TokenType::Scope,
                    "return" => TokenType::Return,
                    "yield" => TokenType::Yield,
                    "not" => TokenType::Not,
                    "and" => TokenType::And,
                    "or" => TokenType::Or,
                    "for" => TokenType::For,
                    "loop" => TokenType::Loop,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "true" => TokenType::True,
                    "false" => TokenType::False,
                    _ => TokenType::Identifier(token),
                },
                tokenizer::TokenType::Numeric => peek! {
                    (".", ..) => {
                        peek! {
                            (fraction, tokenizer::TokenType::Numeric) => TokenType::Float(token, Some(fraction)),
                            _ => TokenType::Float(token, None),
                        }
                    },
                    _ => TokenType::Number(token),
                },
                tokenizer::TokenType::Punctuation => match token {
                    "=" => peek! {
                        ("=", ..) => TokenType::Equals,
                        _ => TokenType::Assignment,
                    },
                    "!" => peek! {
                        (ident, tokenizer::TokenType::Keyword) => TokenType::MacroIdentifier(ident),
                        _ => TokenType::Promotion,
                    },
                    "?" => TokenType::Coalescence,
                    "@" => TokenType::Ampersand,
                    ":" => peek! {
                        ("=", ..) => TokenType::ConstantAssignment,
                        _ => TokenType::Colon,
                    },
                    "." => peek! {
                        (".", ..) => TokenType::DoubleDot,
                        _ => TokenType::Dot,
                    },
                    "+" => TokenType::Plus,
                    "-" => peek! {
                        ("-", ..) => TokenType::DoubleMinus,
                        (">", ..) => TokenType::Arrow,
                        _ => TokenType::Minus,
                    },
                    "*" => peek! {
                        ("*", ..) => TokenType::DoubleAsterisk,
                        _ => TokenType::Asterisk,
                    },
                    "/" => TokenType::Slash,
                    "|" => peek! {
                        (">", ..) => peek! {
                            (">", ..) => TokenType::PipeDoubleForward,
                            _ => TokenType::PipeForward,
                        },
                        _ => TokenType::Pipe,
                    },
                    ">" => peek! {
                        ("=", ..) => TokenType::GreaterOrEqual,
                        (">", ..) => TokenType::DoubleGreater,
                        _ => TokenType::Greater,
                    },
                    "<" => peek! {
                        ("=", ..) => TokenType::LesserOrEqual,
                        ("<", ..) => TokenType::DoubleLesser,
                        _ => TokenType::Lesser,
                    },
                    "(" => TokenType::Brace(Brace::Round, BraceDirection::Open),
                    ")" => TokenType::Brace(Brace::Round, BraceDirection::Close),
                    "[" => TokenType::Brace(Brace::Square, BraceDirection::Open),
                    "]" => TokenType::Brace(Brace::Square, BraceDirection::Close),
                    "{" => TokenType::Brace(Brace::Curly, BraceDirection::Open),
                    "}" => TokenType::Brace(Brace::Curly, BraceDirection::Close),
                    _ => continue, // Skip unknown punctuation
                },
                tokenizer::TokenType::Unknown => continue,
            };

            break Some(Token { token_type, range });
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'a> {
    token_type: TokenType<'a>,
    range: Range<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType<'a> {
    // Control
    Newline,
    Tab,

    // Identifiers
    MacroIdentifier(&'a str),
    Identifier(&'a str),
    Number(&'a str),
    Float(&'a str, Option<&'a str>),

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

    // Braces
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
