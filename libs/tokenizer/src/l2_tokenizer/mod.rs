#[cfg(test)]
mod tests;

use std::{iter::Peekable, ops::Range};

use crate::{
    Error, Res,
    l1_tokenizer::{L1Token, L1TokenType, l1_tokenize},
    types::{Brace, BraceDirection, TokenType},
};

pub(crate) fn l2_tokenize(source: &str) -> impl Iterator<Item = Res<L2Token<'_>>> {
    let iter = l1_tokenize(source).peekable();
    L2Tokenizer { iter }
}

#[derive(Debug, Clone)]
struct L2Tokenizer<'a, I>
where
    I: Iterator<Item = Res<L1Token<'a>>>,
{
    iter: Peekable<I>,
}

impl<'a, I> Iterator for L2Tokenizer<'a, I>
where
    I: Iterator<Item = Res<L1Token<'a>>>,
{
    type Item = Res<L2Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        // Helper macro to peek and match on next token
        // Consumes the token if it matches any of the patterns
        // Automatically wraps tuple patterns in Some()
        macro_rules! peek {
            ($(($($inner:tt)*) => $result:expr),+ $(, _ => $default:expr)? $(,)?) => {{
                let next_token = self.iter.peek().and_then(|res| {
                    res.as_ref().ok().map(|t| (t.token, t.range.clone(), t.token_type))
                });
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
            let l1_token = match self.iter.next()? {
                Ok(token) => token,
                Err(err) => return Some(Err(err)),
            };

            let L1Token {
                token,
                range,
                token_type,
            } = l1_token;

            let token_type = match token_type {
                L1TokenType::Whitespace => match token {
                    "\n" => L2TokenType::Newline,
                    " " => {
                        let mut count = 1usize;
                        loop {
                            peek! {
                                (" ", ..) => count = count.checked_add(1).unwrap(),
                                _ => break L2TokenType::Whitespace(count),
                            }
                        }
                    }
                    token => break Some(Err(Error::InvalidWhitespace(token.to_string()))),
                },
                L1TokenType::String(string_type) => {
                    L2TokenType::Normal(TokenType::String(token, string_type))
                }
                L1TokenType::Keyword => L2TokenType::Normal(match token {
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
                    "mut" => TokenType::Mutable,
                    _ => TokenType::Identifier(token),
                }),
                L1TokenType::Numeric => L2TokenType::Normal(peek! {
                    (".", ..) => {
                        peek! {
                            (fraction, _, L1TokenType::Numeric) => TokenType::Float(token, Some(fraction)),
                            _ => TokenType::Float(token, None),
                        }
                    },
                    _ => TokenType::Number(token),
                }),
                L1TokenType::Punctuation => match token {
                    ";" => L2TokenType::Normal(TokenType::Semicolon),
                    "," => L2TokenType::Normal(TokenType::Comma),
                    "=" => L2TokenType::Normal(peek! {
                        ("=", ..) => TokenType::Equals,
                        _ => TokenType::Assignment,
                    }),
                    "!" => L2TokenType::Normal(peek! {
                        (ident, _, L1TokenType::Keyword) => TokenType::MacroIdentifier(ident),
                        _ => TokenType::Promotion,
                    }),
                    "?" => L2TokenType::Normal(TokenType::Coalescence),
                    "@" => L2TokenType::Normal(TokenType::Ampersand),
                    ":" => L2TokenType::Normal(peek! {
                        ("=", ..) => TokenType::ConstantAssignment,
                        _ => TokenType::Colon,
                    }),
                    "." => L2TokenType::Normal(peek! {
                        (".", ..) => TokenType::DoubleDot,
                        _ => TokenType::Dot,
                    }),
                    "+" => L2TokenType::Normal(TokenType::Plus),
                    "-" => L2TokenType::Normal(peek! {
                        ("-", ..) => TokenType::DoubleMinus,
                        (">", ..) => TokenType::Arrow,
                        _ => TokenType::Minus,
                    }),
                    "*" => L2TokenType::Normal(peek! {
                        ("*", ..) => TokenType::DoubleAsterisk,
                        _ => TokenType::Asterisk,
                    }),
                    "/" => L2TokenType::Normal(TokenType::Slash),
                    "|" => L2TokenType::Normal(peek! {
                        (">", ..) => peek! {
                            (">", ..) => TokenType::PipeDoubleForward,
                            _ => TokenType::PipeForward,
                        },
                        _ => TokenType::Pipe,
                    }),
                    ">" => L2TokenType::Normal(peek! {
                        ("=", ..) => TokenType::GreaterOrEqual,
                        (">", ..) => TokenType::DoubleGreater,
                        _ => TokenType::Greater,
                    }),
                    "<" => L2TokenType::Normal(peek! {
                        ("=", ..) => TokenType::LesserOrEqual,
                        ("<", ..) => TokenType::DoubleLesser,
                        _ => TokenType::Lesser,
                    }),
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum L2TokenType<'a> {
    Normal(TokenType<'a>),
    Whitespace(usize),
    Newline,
    Brace(Brace, BraceDirection),
}
