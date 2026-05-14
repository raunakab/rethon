#[cfg(test)]
mod tests;

use std::iter::Peekable;

use crate::{
    Brace, BraceDirection, Error, LexItem, LexKind, LexType, Res, StringType,
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
            SegmentKind::String => LexKind::Normal(LexType::String(text, StringType::Normal)),
            SegmentKind::Keyword => LexKind::Normal(match text {
                "fn" => LexType::Function,
                "mut" => LexType::Mutable,
                "scope" => LexType::Scope,
                "return" => LexType::Return,
                "yield" => LexType::Yield,
                "throw" => LexType::Throw,
                "otherwise" => LexType::Otherwise,
                "true" => LexType::True,
                "false" => LexType::False,
                "not" => LexType::Not,
                "and" => LexType::And,
                "or" => LexType::Or,
                "for" => LexType::For,
                "loop" => LexType::Loop,
                "if" => LexType::If,
                "else" => LexType::Else,
                "struct" => LexType::Struct,
                "enum" => LexType::Enum,
                "panic" => LexType::Panic,
                "todo" => LexType::Todo,
                "unimplemented" => LexType::Unimplemented,
                "f" => peek! {
                    (string_content, _, SegmentKind::String) => LexType::String(string_content, StringType::Formatted),
                    _ => LexType::Identifier(text),
                },
                _ => LexType::Identifier(text),
            }),
            SegmentKind::Numeric => LexKind::Normal(peek! {
                (".", ..) => {
                    peek! {
                        (fraction, _, SegmentKind::Numeric) => LexType::Float(text, Some(fraction)),
                        _ => LexType::Float(text, None),
                    }
                },
                _ => LexType::Number(text),
            }),
            SegmentKind::Punctuation => match text {
                ";" => LexKind::Normal(LexType::Semicolon),
                "," => LexKind::Normal(LexType::Comma),
                "=" => LexKind::Normal(peek! {
                    ("=", ..) => LexType::Equals,
                    _ => LexType::Assignment,
                }),
                "!" => LexKind::Normal(peek! {
                    (ident, _, SegmentKind::Keyword) => LexType::MacroIdentifier(ident),
                    _ => LexType::Promotion,
                }),
                "?" => LexKind::Normal(LexType::Coalescence),
                "@" => LexKind::Normal(LexType::Ampersand),
                ":" => LexKind::Normal(peek! {
                    ("=", ..) => LexType::StaticAssignment,
                    _ => LexType::Colon,
                }),
                "." => LexKind::Normal(peek! {
                    (".", ..) => LexType::DoubleDot,
                    _ => LexType::Dot,
                }),
                "+" => LexKind::Normal(LexType::Plus),
                "-" => LexKind::Normal(peek! {
                    ("-", ..) => LexType::DoubleMinus,
                    (">", ..) => LexType::Arrow,
                    _ => LexType::Minus,
                }),
                "*" => LexKind::Normal(peek! {
                    ("*", ..) => LexType::DoubleAsterisk,
                    _ => LexType::Asterisk,
                }),
                "/" => LexKind::Normal(LexType::Slash),
                "|" => LexKind::Normal(peek! {
                    (">", ..) => peek! {
                        (">", ..) => LexType::PipeDoubleForward,
                        _ => LexType::PipeForward,
                    },
                    _ => LexType::Pipe,
                }),
                ">" => LexKind::Normal(peek! {
                    ("=", ..) => LexType::GreaterOrEqual,
                    (">", ..) => LexType::DoubleGreater,
                    _ => LexType::Greater,
                }),
                "<" => LexKind::Normal(peek! {
                    ("=", ..) => LexType::LesserOrEqual,
                    ("<", ..) => LexType::DoubleLesser,
                    _ => LexType::Lesser,
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
