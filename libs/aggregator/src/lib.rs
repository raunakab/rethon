#[cfg(test)]
mod tests;

use std::{iter::Peekable, ops::Range};

use thiserror::Error;

type Res<T = ()> = Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("Unknown token: {0}")]
    UnknownToken(String),
    #[error("Unterminated string at byte {0}")]
    UnterminatedString(usize),
}

pub fn aggregator(source: &str) -> impl Iterator<Item = Res<Token<'_>>> {
    Aggregator {
        source,
        tokenizer: tokenizer::tokenize(source).peekable(),
    }
}

struct Aggregator<'a, I>
where
    I: Iterator<Item = tokenizer::Token<'a>>,
{
    source: &'a str,
    tokenizer: Peekable<I>,
}

impl<'a, I> Aggregator<'a, I>
where
    I: Iterator<Item = tokenizer::Token<'a>>,
{
    fn parse_string(&mut self, string_type: StringType, opening: Range<usize>) -> Res<Token<'a>> {
        let closing = loop {
            let Some(tokenizer::Token {
                token,
                range,
                token_type,
            }) = self.tokenizer.next()
            else {
                return Err(Error::UnterminatedString(opening.start));
            };

            if matches!(token_type, tokenizer::TokenType::Punctuation) && matches!(token, "\"") {
                break range;
            }
        };

        let range = opening.end..closing.start;
        let content = &self.source[range.clone()];

        Ok(Token {
            token_type: TokenType::String(content, string_type),
            range,
        })
    }
}

impl<'a, I> Iterator for Aggregator<'a, I>
where
    I: Iterator<Item = tokenizer::Token<'a>>,
{
    type Item = Res<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        // Helper macro to peek and match on next token
        // Consumes the token if it matches any of the patterns
        // Automatically wraps tuple patterns in Some()
        macro_rules! peek {
            ($(($($inner:tt)*) => $result:expr),+ $(, _ => $default:expr)? $(,)?) => {{
                let next_token = self.tokenizer.peek().map(|t| (t.token, t.range.clone(), t.token_type));
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
                    " " => {
                        let mut count = 1usize;
                        loop {
                            peek! {
                                (" ", ..) => count = count.checked_add(1).unwrap(),
                                _ => break TokenType::Whitespace(count),
                            }
                        }
                    }
                    token => break Some(Err(Error::UnknownToken(token.to_string()))),
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
                    "struct" => TokenType::Struct,
                    "enum" => TokenType::Enum,
                    "panic" => TokenType::Panic,
                    "todo" => TokenType::Todo,
                    "unimplemented" => TokenType::Unimplemented,

                    // string-formatting
                    "f" => peek! {
                        ("\"", string_range, _) => break Some(self.parse_string(StringType::Formatted, string_range)),
                        _ => TokenType::Identifier(token),
                    },
                    _ => TokenType::Identifier(token),
                },
                tokenizer::TokenType::Numeric => peek! {
                    (".", ..) => {
                        peek! {
                            (fraction, _, tokenizer::TokenType::Numeric) => TokenType::Float(token, Some(fraction)),
                            _ => TokenType::Float(token, None),
                        }
                    },
                    _ => TokenType::Number(token),
                },
                tokenizer::TokenType::Punctuation => match token {
                    ";" => TokenType::Semicolon,
                    "," => TokenType::Comma,
                    "\"" => break Some(self.parse_string(StringType::Normal, range)),
                    "=" => peek! {
                        ("=", ..) => TokenType::Equals,
                        _ => TokenType::Assignment,
                    },
                    "!" => peek! {
                        (ident, _, tokenizer::TokenType::Keyword) => TokenType::MacroIdentifier(ident),
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
                    token => break Some(Err(Error::UnknownToken(token.to_string()))),
                },
                tokenizer::TokenType::Unknown => {
                    break Some(Err(Error::UnknownToken(token.to_string())));
                }
            };

            break Some(Ok(Token { token_type, range }));
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub range: Range<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType<'a> {
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
