#[cfg(test)]
mod tests;

use std::iter::Peekable;

use crate::{
    Brace, BraceDirection, Error, LexKind, LexToken, Res, StringType, TokenType,
    lexer_s1::{Token, TokenKind},
};

pub(crate) fn tokenize<'a>(
    iter: impl Iterator<Item = Res<Token<'a>>>,
) -> impl Iterator<Item = Res<LexToken<'a>>> {
    Tokenizer {
        iter: iter.peekable(),
    }
}

#[derive(Debug, Clone)]
struct Tokenizer<'a, I>
where
    I: Iterator<Item = Res<Token<'a>>>,
{
    iter: Peekable<I>,
}

impl<'a, I> Iterator for Tokenizer<'a, I>
where
    I: Iterator<Item = Res<Token<'a>>>,
{
    type Item = Res<LexToken<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! peek {
            ($(($($inner:tt)*) => $result:expr),+ $(, _ => $default:expr)? $(,)?) => {{
                let next_token = self.iter.peek().and_then(|res| {
                    res.as_ref().ok().map(|t| (t.token, t.range.clone(), t.kind))
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

        let s1_token = match self.iter.next()? {
            Ok(token) => token,
            Err(err) => return Some(Err(err)),
        };

        let Token { token, range, kind } = s1_token;

        let lex_kind = match kind {
            TokenKind::Whitespace => match token {
                "\n" | "\r\n" => LexKind::Newline,
                " " => {
                    let mut count = 1usize;
                    loop {
                        peek! {
                            (" ", ..) => count = count.checked_add(1).unwrap(),
                            _ => break LexKind::Whitespace(count),
                        }
                    }
                }
                token => return Some(Err(Error::InvalidWhitespace(token.to_string()))),
            },
            TokenKind::String => LexKind::Normal(TokenType::String(token, StringType::Normal)),
            TokenKind::Keyword => LexKind::Normal(match token {
                "fn" => TokenType::Function,
                "mut" => TokenType::Mutable,
                "scope" => TokenType::Scope,
                "return" => TokenType::Return,
                "yield" => TokenType::Yield,
                "throw" => TokenType::Throw,
                "otherwise" => TokenType::Otherwise,
                "true" => TokenType::True,
                "false" => TokenType::False,
                "not" => TokenType::Not,
                "and" => TokenType::And,
                "or" => TokenType::Or,
                "for" => TokenType::For,
                "loop" => TokenType::Loop,
                "if" => TokenType::If,
                "else" => TokenType::Else,
                "struct" => TokenType::Struct,
                "enum" => TokenType::Enum,
                "panic" => TokenType::Panic,
                "todo" => TokenType::Todo,
                "unimplemented" => TokenType::Unimplemented,
                "f" => peek! {
                    (string_content, _, TokenKind::String) => TokenType::String(string_content, StringType::Formatted),
                    _ => TokenType::Identifier(token),
                },
                _ => TokenType::Identifier(token),
            }),
            TokenKind::Numeric => LexKind::Normal(peek! {
                (".", ..) => {
                    peek! {
                        (fraction, _, TokenKind::Numeric) => TokenType::Float(token, Some(fraction)),
                        _ => TokenType::Float(token, None),
                    }
                },
                _ => TokenType::Number(token),
            }),
            TokenKind::Punctuation => match token {
                ";" => LexKind::Normal(TokenType::Semicolon),
                "," => LexKind::Normal(TokenType::Comma),
                "=" => LexKind::Normal(peek! {
                    ("=", ..) => TokenType::Equals,
                    _ => TokenType::Assignment,
                }),
                "!" => LexKind::Normal(peek! {
                    (ident, _, TokenKind::Keyword) => TokenType::MacroIdentifier(ident),
                    _ => TokenType::Promotion,
                }),
                "?" => LexKind::Normal(TokenType::Coalescence),
                "@" => LexKind::Normal(TokenType::Ampersand),
                ":" => LexKind::Normal(peek! {
                    ("=", ..) => TokenType::StaticAssignment,
                    _ => TokenType::Colon,
                }),
                "." => LexKind::Normal(peek! {
                    (".", ..) => TokenType::DoubleDot,
                    _ => TokenType::Dot,
                }),
                "+" => LexKind::Normal(TokenType::Plus),
                "-" => LexKind::Normal(peek! {
                    ("-", ..) => TokenType::DoubleMinus,
                    (">", ..) => TokenType::Arrow,
                    _ => TokenType::Minus,
                }),
                "*" => LexKind::Normal(peek! {
                    ("*", ..) => TokenType::DoubleAsterisk,
                    _ => TokenType::Asterisk,
                }),
                "/" => LexKind::Normal(TokenType::Slash),
                "|" => LexKind::Normal(peek! {
                    (">", ..) => peek! {
                        (">", ..) => TokenType::PipeDoubleForward,
                        _ => TokenType::PipeForward,
                    },
                    _ => TokenType::Pipe,
                }),
                ">" => LexKind::Normal(peek! {
                    ("=", ..) => TokenType::GreaterOrEqual,
                    (">", ..) => TokenType::DoubleGreater,
                    _ => TokenType::Greater,
                }),
                "<" => LexKind::Normal(peek! {
                    ("=", ..) => TokenType::LesserOrEqual,
                    ("<", ..) => TokenType::DoubleLesser,
                    _ => TokenType::Lesser,
                }),
                "(" => LexKind::Brace(Brace::Round, BraceDirection::Open),
                ")" => LexKind::Brace(Brace::Round, BraceDirection::Close),
                "[" => LexKind::Brace(Brace::Square, BraceDirection::Open),
                "]" => LexKind::Brace(Brace::Square, BraceDirection::Close),
                "{" => LexKind::Brace(Brace::Curly, BraceDirection::Open),
                "}" => LexKind::Brace(Brace::Curly, BraceDirection::Close),
                token => return Some(Err(Error::UnknownToken(token.to_string()))),
            },
            TokenKind::Unknown => {
                return Some(Err(Error::UnknownToken(token.to_string())));
            }
        };

        Some(Ok(LexToken {
            kind: lex_kind,
            range,
        }))
    }
}
