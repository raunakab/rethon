#[cfg(test)]
mod tests;

use std::{iter::Peekable, ops::Range};

use crate::{
    Error, Res,
    l1_tokenizer::{L1Token, L1TokenType, l1_tokenize},
    types::{Brace, BraceDirection, StringType, TokenType},
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
            token_type: TokenType::String(content, string_type),
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
                    "\n" => TokenType::Newline,
                    " " => {
                        let mut count = 1usize;
                        loop {
                            peek! {
                                (" ", ..) => count = count.checked_add(1).unwrap(),
                                _ => break TokenType::Whitespace(count),
                            }
                        }
                    }
                    token => break Some(Err(Error::InvalidWhitespace(token.to_string()))),
                },
                L1TokenType::Keyword => match token {
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

                    // string-formatting
                    "f" => peek! {
                        ("\"", string_range, _) => break Some(self.parse_string(StringType::Formatted, string_range)),
                        _ => TokenType::Identifier(token),
                    },
                    _ => TokenType::Identifier(token),
                },
                L1TokenType::Numeric => peek! {
                    (".", ..) => {
                        peek! {
                            (fraction, _, L1TokenType::Numeric) => TokenType::Float(token, Some(fraction)),
                            _ => TokenType::Float(token, None),
                        }
                    },
                    _ => TokenType::Number(token),
                },
                L1TokenType::Punctuation => match token {
                    ";" => TokenType::Semicolon,
                    "," => TokenType::Comma,
                    "\"" => break Some(self.parse_string(StringType::Normal, range)),
                    "=" => peek! {
                        ("=", ..) => TokenType::Equals,
                        _ => TokenType::Assignment,
                    },
                    "!" => peek! {
                        (ident, _, L1TokenType::Keyword) => TokenType::MacroIdentifier(ident),
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
    pub(crate) token_type: TokenType<'a>,
    pub(crate) range: Range<usize>,
}
