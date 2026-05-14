#[cfg(test)]
mod tests;

use std::iter::Peekable;

use lexer::{Brace, BraceDirection, LexItem, LexKind, Token};

use crate::{Error, Position, Res};

pub(crate) const INDENTATION_SIZE: usize = 4;

pub(crate) fn whitespace_strip<'a>(
    iter: impl Iterator<Item = lexer::Res<LexItem<'a>>>,
) -> impl Iterator<Item = Res<StrippedToken<'a>>> {
    WhitespaceStripper {
        iter: iter.peekable(),
        line: 0,
        line_position: 0,
        indentation_level: 0,
        after_newline: true,
    }
}

#[derive(Debug, Clone)]
struct WhitespaceStripper<'a, I>
where
    I: Iterator<Item = lexer::Res<LexItem<'a>>>,
{
    iter: Peekable<I>,
    line: usize,
    line_position: usize,
    indentation_level: usize,
    after_newline: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StrippedTokenKind<'a> {
    Normal(Token<'a>),
    Brace(Brace, BraceDirection),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StrippedToken<'a> {
    pub(crate) kind: StrippedTokenKind<'a>,
    pub(crate) position: Position,
}

impl<'a, I> Iterator for WhitespaceStripper<'a, I>
where
    I: Iterator<Item = lexer::Res<LexItem<'a>>>,
{
    type Item = Res<StrippedToken<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let lex_item = match self.iter.next()? {
            Ok(item) => item,
            Err(error) => return Some(Err(error.into())),
        };

        if matches!(lex_item.kind, LexKind::Newline) {
            self.line += 1;
            self.line_position = 0;
            self.indentation_level = 0;
            self.after_newline = true;
            return self.next();
        }

        if self.after_newline {
            self.after_newline = false;

            if let LexKind::Whitespace(count) = lex_item.kind {
                if count % INDENTATION_SIZE != 0 {
                    return Some(Err(Error::InvalidIndentation {
                        found: count,
                        position: lex_item.range.start,
                    }));
                }

                self.indentation_level = count / INDENTATION_SIZE;
                self.line_position += count;
                return self.next();
            }
        }

        let token_length = lex_item.range.end - lex_item.range.start;

        let kind = match lex_item.kind {
            LexKind::Normal(tt) => StrippedTokenKind::Normal(tt),
            LexKind::Brace(brace, dir) => StrippedTokenKind::Brace(brace, dir),
            LexKind::Whitespace(_) | LexKind::Newline => {
                self.line_position += token_length;
                return self.next();
            }
        };

        let result = StrippedToken {
            kind,
            position: Position {
                source_range: lex_item.range,
                line: self.line,
                line_range: self.line_position..self.line_position + token_length,
                indentation_level: self.indentation_level,
            },
        };
        self.line_position += token_length;
        Some(Ok(result))
    }
}
