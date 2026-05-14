#[cfg(test)]
mod tests;

use std::iter::Peekable;

use lexer::{Brace, BraceDirection, LexItem, LexKind, LexType};

use crate::{Error, Position, Res};

pub(crate) const INDENTATION_SIZE: usize = 4;

pub(crate) fn l3_tokenize<'a>(
    iter: impl Iterator<Item = lexer::Res<LexItem<'a>>>,
) -> impl Iterator<Item = Res<L3Token<'a>>> {
    L3Tokenizer {
        iter: iter.peekable(),
        line: 0,
        line_position: 0,
        indentation_level: 0,
        after_newline: true,
    }
}

#[derive(Debug, Clone)]
struct L3Tokenizer<'a, I>
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
pub(crate) enum L3TokenType<'a> {
    Normal(LexType<'a>),
    Brace(Brace, BraceDirection),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct L3Token<'a> {
    pub(crate) token_type: L3TokenType<'a>,
    pub(crate) position: Position,
}

impl<'a, I> Iterator for L3Tokenizer<'a, I>
where
    I: Iterator<Item = lexer::Res<LexItem<'a>>>,
{
    type Item = Res<L3Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let lex_token = match self.iter.next()? {
            Ok(token) => token,
            Err(error) => return Some(Err(error.into())),
        };

        if matches!(lex_token.kind, LexKind::Newline) {
            self.line += 1;
            self.line_position = 0;
            self.indentation_level = 0;
            self.after_newline = true;
            return self.next();
        }

        if self.after_newline {
            self.after_newline = false;

            if let LexKind::Whitespace(count) = lex_token.kind {
                if count % INDENTATION_SIZE != 0 {
                    return Some(Err(Error::InvalidIndentation {
                        found: count,
                        position: lex_token.range.start,
                    }));
                }

                self.indentation_level = count / INDENTATION_SIZE;
                self.line_position += count;
                return self.next();
            }
        }

        let token_length = lex_token.range.end - lex_token.range.start;

        let token_type = match lex_token.kind {
            LexKind::Normal(tt) => L3TokenType::Normal(tt),
            LexKind::Brace(brace, dir) => L3TokenType::Brace(brace, dir),
            LexKind::Whitespace(_) | LexKind::Newline => {
                self.line_position += token_length;
                return self.next();
            }
        };

        let result = L3Token {
            token_type,
            position: Position {
                source_range: lex_token.range,
                line: self.line,
                line_range: self.line_position..self.line_position + token_length,
                indentation_level: self.indentation_level,
            },
        };
        self.line_position += token_length;
        Some(Ok(result))
    }
}
