#[cfg(test)]
mod tests;

use std::{iter::Peekable, ops::Range};

use crate::{
    Brace, BraceDirection, Error, Res, StringType, Token,
    s1_segmenter::{Segment, SegmentKind},
};

pub(crate) fn cluster<'a>(
    iter: impl Iterator<Item = Res<Segment<'a>>>,
) -> impl Iterator<Item = Res<LexItem<'a>>> {
    Clusterer {
        iter: iter.peekable(),
    }
}

#[derive(Debug, Clone)]
struct Clusterer<'a, I>
where
    I: Iterator<Item = Res<Segment<'a>>>,
{
    iter: Peekable<I>,
}

impl<'a, I> Iterator for Clusterer<'a, I>
where
    I: Iterator<Item = Res<Segment<'a>>>,
{
    type Item = Res<LexItem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! peek {
            ($(($($inner:tt)*) => $result:expr),+ $(, _ => $default:expr)? $(,)?) => {{
                let next_segment = self.iter.peek().and_then(|res| {
                    res.as_ref().ok().map(|s| (s.segment, s.range.clone(), s.kind))
                });
                match next_segment {
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

        let segment = match self.iter.next()? {
            Ok(segment) => segment,
            Err(err) => return Some(Err(err)),
        };

        let Segment {
            segment: text,
            range,
            kind,
        } = segment;

        let lex_kind = match kind {
            SegmentKind::Whitespace => match text {
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
                text => return Some(Err(Error::InvalidWhitespace(text.to_string()))),
            },
            SegmentKind::String => LexKind::Normal(Token::String(text, StringType::Normal)),
            SegmentKind::Keyword => LexKind::Normal(match text {
                "fn" => Token::Function,
                "mut" => Token::Mutable,
                "return" => Token::Return,
                "yield" => Token::Yield,
                "throw" => Token::Throw,
                "true" => Token::True,
                "false" => Token::False,
                "not" => Token::Not,
                "and" => Token::And,
                "or" => Token::Or,
                "loop" => Token::Loop,
                "if" => Token::If,
                "else" => Token::Else,
                "match" => Token::Match,
                "struct" => Token::Struct,
                "enum" => Token::Enum,
                "panic" => Token::Panic,
                "todo" => Token::Todo,
                "unimplemented" => Token::Unimplemented,
                "f" => peek! {
                    (string_content, _, SegmentKind::String) => Token::String(string_content, StringType::Formatted),
                    _ => Token::Identifier(text),
                },
                _ => Token::Identifier(text),
            }),
            SegmentKind::Numeric => LexKind::Normal(peek! {
                (".", ..) => {
                    peek! {
                        (fraction, _, SegmentKind::Numeric) => Token::Float(text, Some(fraction)),
                        _ => Token::Float(text, None),
                    }
                },
                _ => Token::Number(text),
            }),
            SegmentKind::Punctuation => match text {
                ";" => LexKind::Normal(Token::Semicolon),
                "," => LexKind::Normal(Token::Comma),
                "=" => LexKind::Normal(peek! {
                    ("=", ..) => Token::Equals,
                    (">", ..) => Token::FatArrow,
                    _ => Token::Assignment,
                }),
                "!" => LexKind::Normal(peek! {
                    (ident, _, SegmentKind::Keyword) => Token::MacroIdentifier(ident),
                    _ => Token::Promotion,
                }),
                "?" => LexKind::Normal(Token::Coalescence),
                "@" => LexKind::Normal(Token::Ampersand),
                ":" => LexKind::Normal(peek! {
                    ("=", ..) => Token::StaticAssignment,
                    _ => Token::Colon,
                }),
                "." => LexKind::Normal(peek! {
                    (".", ..) => Token::DoubleDot,
                    _ => Token::Dot,
                }),
                "+" => LexKind::Normal(Token::Plus),
                "-" => LexKind::Normal(peek! {
                    ("-", ..) => Token::DoubleMinus,
                    (">", ..) => Token::Arrow,
                    _ => Token::Minus,
                }),
                "*" => LexKind::Normal(peek! {
                    ("*", ..) => Token::DoubleAsterisk,
                    _ => Token::Asterisk,
                }),
                "/" => LexKind::Normal(Token::Slash),
                "|" => LexKind::Normal(peek! {
                    (">", ..) => peek! {
                        (">", ..) => Token::PipeDoubleForward,
                        _ => Token::PipeForward,
                    },
                    _ => Token::Pipe,
                }),
                ">" => LexKind::Normal(peek! {
                    ("=", ..) => Token::GreaterOrEqual,
                    (">", ..) => Token::DoubleGreater,
                    _ => Token::Greater,
                }),
                "<" => LexKind::Normal(peek! {
                    ("=", ..) => Token::LesserOrEqual,
                    ("<", ..) => Token::DoubleLesser,
                    _ => Token::Lesser,
                }),
                "(" => LexKind::Brace(Brace::Round, BraceDirection::Open),
                ")" => LexKind::Brace(Brace::Round, BraceDirection::Close),
                "[" => LexKind::Brace(Brace::Square, BraceDirection::Open),
                "]" => LexKind::Brace(Brace::Square, BraceDirection::Close),
                "{" => LexKind::Brace(Brace::Curly, BraceDirection::Open),
                "}" => LexKind::Brace(Brace::Curly, BraceDirection::Close),
                text => return Some(Err(Error::UnknownItem(text.to_string()))),
            },
            SegmentKind::Unknown => {
                return Some(Err(Error::UnknownItem(text.to_string())));
            }
        };

        Some(Ok(LexItem {
            kind: lex_kind,
            range,
        }))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LexItem<'a> {
    pub(crate) kind: LexKind<'a>,
    pub(crate) range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum LexKind<'a> {
    Normal(Token<'a>),
    Whitespace(usize),
    Newline,
    Brace(Brace, BraceDirection),
}
