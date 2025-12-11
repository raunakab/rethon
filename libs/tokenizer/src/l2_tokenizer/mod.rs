#[cfg(test)]
mod tests;

use std::{iter::Peekable, ops::Range};

use crate::{
    Error, Res,
    l1_tokenizer::{L1Token, L1TokenType},
    types::{Brace, BraceDirection, StringType, TokenType},
};

pub(crate) fn l2_tokenize<'a>(
    iter: impl Iterator<Item = Res<L1Token<'a>>>,
) -> impl Iterator<Item = Res<L2Token<'a>>> {
    L2Tokenizer {
        iter: iter.peekable(),
    }
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
                L1TokenType::String => {
                    L2TokenType::Normal(TokenType::String(token, StringType::Normal))
                }
                L1TokenType::Keyword => L2TokenType::Normal(match token {
                    // function keywords
                    "fn" => TokenType::Function,

                    // definitional keywords
                    "mut" => TokenType::Mutable,
                    "scope" => TokenType::Scope,

                    // control keywords
                    "return" => TokenType::Return,
                    "yield" => TokenType::Yield,
                    "throw" => TokenType::Throw,
                    "otherwise" => TokenType::Otherwise,

                    // boolean keywords
                    "true" => TokenType::True,
                    "false" => TokenType::False,
                    "not" => TokenType::Not,
                    "and" => TokenType::And,
                    "or" => TokenType::Or,

                    // conditional keywords
                    "for" => TokenType::For,
                    "loop" => TokenType::Loop,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,

                    // type-algebra keywords
                    "struct" => TokenType::Struct,
                    "enum" => TokenType::Enum,

                    // type-hole keywords
                    "panic" => TokenType::Panic,
                    "todo" => TokenType::Todo,
                    "unimplemented" => TokenType::Unimplemented,

                    // Check for formatted string prefix
                    "f" => peek! {
                        (string_content, _, L1TokenType::String) => TokenType::String(string_content, StringType::Formatted),
                        _ => TokenType::Identifier(token),
                    },
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
