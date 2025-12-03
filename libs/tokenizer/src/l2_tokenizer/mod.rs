#[cfg(test)]
mod tests;

use std::{iter::Peekable, ops::Range};

use crate::{
    Error, Res,
    l1_tokenizer::{L1Token, L1TokenType, l1_tokenize},
};

pub(crate) fn l2_tokenize(source: &str) -> impl Iterator<Item = Res<L2Token<'_>>> {
    let iter = l1_tokenize(source).peekable();
    L2Tokenizer { source, iter }
}

#[derive(Debug, Clone)]
struct L2Tokenizer<'a, I>
where
    I: Iterator<Item = L1Token<'a>>,
{
    source: &'a str,
    iter: Peekable<I>,
}

impl<'a, I> L2Tokenizer<'a, I>
where
    I: Iterator<Item = L1Token<'a>>,
{
    fn parse_string(&mut self, string_type: StringType, opening: Range<usize>) -> Res<L2Token<'a>> {
        let closing = loop {
            let Some(L1Token {
                token,
                range,
                token_type,
            }) = self.iter.next()
            else {
                return Err(Error::UnterminatedString(opening.start));
            };

            if matches!(token_type, L1TokenType::Punctuation) && matches!(token, "\"") {
                break range;
            }
        };

        let range = opening.end..closing.start;
        let content = &self.source[range.clone()];

        Ok(L2Token {
            token_type: L2TokenType::String(content, string_type),
            range,
        })
    }
}

impl<'a, I> Iterator for L2Tokenizer<'a, I>
where
    I: Iterator<Item = L1Token<'a>>,
{
    type Item = Res<L2Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        // Helper macro to peek and match on next token
        // Consumes the token if it matches any of the patterns
        // Automatically wraps tuple patterns in Some()
        macro_rules! peek {
            ($(($($inner:tt)*) => $result:expr),+ $(, _ => $default:expr)? $(,)?) => {{
                let next_token = self.iter.peek().map(|t| (t.token, t.range.clone(), t.token_type));
                match next_token {
                    $(
                        Some(($($inner)*)) => {
                            self.iter.next();
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
            let Some(L1Token {
                token,
                range,
                token_type,
            }) = self.iter.next()
            else {
                break None;
            };

            let token_type = match token_type {
                L1TokenType::Whitespace => match token {
                    "\n" => L2TokenType::Newline,
                    "\t" => L2TokenType::Tab,
                    " " => {
                        let mut count = 1usize;
                        loop {
                            peek! {
                                (" ", ..) => count = count.checked_add(1).unwrap(),
                                _ => break L2TokenType::Whitespace(count),
                            }
                        }
                    }
                    token => break Some(Err(Error::UnknownToken(token.to_string()))),
                },
                L1TokenType::Keyword => match token {
                    "fn" => L2TokenType::Function,
                    "scope" => L2TokenType::Scope,
                    "return" => L2TokenType::Return,
                    "yield" => L2TokenType::Yield,
                    "not" => L2TokenType::Not,
                    "and" => L2TokenType::And,
                    "or" => L2TokenType::Or,
                    "for" => L2TokenType::For,
                    "loop" => L2TokenType::Loop,
                    "if" => L2TokenType::If,
                    "else" => L2TokenType::Else,
                    "true" => L2TokenType::True,
                    "false" => L2TokenType::False,
                    "struct" => L2TokenType::Struct,
                    "enum" => L2TokenType::Enum,
                    "panic" => L2TokenType::Panic,
                    "todo" => L2TokenType::Todo,
                    "unimplemented" => L2TokenType::Unimplemented,
                    "mut" => L2TokenType::Mutable,

                    // string-formatting
                    "f" => peek! {
                        ("\"", string_range, _) => break Some(self.parse_string(StringType::Formatted, string_range)),
                        _ => L2TokenType::Identifier(token),
                    },
                    _ => L2TokenType::Identifier(token),
                },
                L1TokenType::Numeric => peek! {
                    (".", ..) => {
                        peek! {
                            (fraction, _, L1TokenType::Numeric) => L2TokenType::Float(token, Some(fraction)),
                            _ => L2TokenType::Float(token, None),
                        }
                    },
                    _ => L2TokenType::Number(token),
                },
                L1TokenType::Punctuation => match token {
                    ";" => L2TokenType::Semicolon,
                    "," => L2TokenType::Comma,
                    "\"" => break Some(self.parse_string(StringType::Normal, range)),
                    "=" => peek! {
                        ("=", ..) => L2TokenType::Equals,
                        _ => L2TokenType::Assignment,
                    },
                    "!" => peek! {
                        (ident, _, L1TokenType::Keyword) => L2TokenType::MacroIdentifier(ident),
                        _ => L2TokenType::Promotion,
                    },
                    "?" => L2TokenType::Coalescence,
                    "@" => L2TokenType::Ampersand,
                    ":" => peek! {
                        ("=", ..) => L2TokenType::ConstantAssignment,
                        _ => L2TokenType::Colon,
                    },
                    "." => peek! {
                        (".", ..) => L2TokenType::DoubleDot,
                        _ => L2TokenType::Dot,
                    },
                    "+" => L2TokenType::Plus,
                    "-" => peek! {
                        ("-", ..) => L2TokenType::DoubleMinus,
                        (">", ..) => L2TokenType::Arrow,
                        _ => L2TokenType::Minus,
                    },
                    "*" => peek! {
                        ("*", ..) => L2TokenType::DoubleAsterisk,
                        _ => L2TokenType::Asterisk,
                    },
                    "/" => L2TokenType::Slash,
                    "|" => peek! {
                        (">", ..) => peek! {
                            (">", ..) => L2TokenType::PipeDoubleForward,
                            _ => L2TokenType::PipeForward,
                        },
                        _ => L2TokenType::Pipe,
                    },
                    ">" => peek! {
                        ("=", ..) => L2TokenType::GreaterOrEqual,
                        (">", ..) => L2TokenType::DoubleGreater,
                        _ => L2TokenType::Greater,
                    },
                    "<" => peek! {
                        ("=", ..) => L2TokenType::LesserOrEqual,
                        ("<", ..) => L2TokenType::DoubleLesser,
                        _ => L2TokenType::Lesser,
                    },
                    "(" => L2TokenType::Brace(Brace::Round, BraceDirection::Open),
                    ")" => L2TokenType::Brace(Brace::Round, BraceDirection::Close),
                    "[" => L2TokenType::Brace(Brace::Square, BraceDirection::Open),
                    "]" => L2TokenType::Brace(Brace::Square, BraceDirection::Close),
                    "{" => L2TokenType::Brace(Brace::Curly, BraceDirection::Open),
                    "}" => L2TokenType::Brace(Brace::Curly, BraceDirection::Close),
                    token => break Some(Err(Error::UnknownToken(token.to_string()))),
                },
                L1TokenType::Unknown => {
                    break Some(Err(Error::UnknownToken(token.to_string())));
                }
            };

            break Some(Ok(L2Token { token_type, range }));
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct L2Token<'a> {
    pub(crate) token_type: L2TokenType<'a>,
    pub(crate) range: Range<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2TokenType<'a> {
    // Control
    Newline,           // \n
    Tab,               // \t
    Whitespace(usize), // ` `; also keeps a track of the number of consecutive whitespace chars
    Semicolon,         // ;
    Comma,             // ,

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringType {
    Normal,
    Formatted,
}
