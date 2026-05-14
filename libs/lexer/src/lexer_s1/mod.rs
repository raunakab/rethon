#[cfg(test)]
mod tests;

use std::ops::Range;

use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

use crate::{Error, Res};

pub(crate) fn tokenize(source: &str) -> impl Iterator<Item = Res<Token<'_>>> {
    Tokenizer {
        source,
        iter: source.grapheme_indices(true),
        iter_state: None,
    }
}

struct Tokenizer<'a> {
    source: &'a str,
    iter: GraphemeIndices<'a>,
    iter_state: Option<(usize, TokenKind)>,
}

impl<'a> Tokenizer<'a> {
    fn parse_string(&mut self, opening_start: usize) -> Res<Token<'a>> {
        let opening_end = opening_start + 1;

        Ok(loop {
            match self.iter.next() {
                Some((index, grapheme)) => {
                    if grapheme != "\"" {
                        continue;
                    }

                    self.iter_state = None;
                    let range = opening_end..index;
                    break Token {
                        token: &self.source[range.clone()],
                        range,
                        kind: TokenKind::String,
                    };
                }
                None => return Err(Error::UnterminatedString(opening_start)),
            }
        })
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Res<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some((curr_index, curr_grapheme)) = self.iter.next() else {
                match self.iter_state {
                    Some((prev_index, prev_kind)) => {
                        self.iter_state = None;
                        break Some(Ok(Token {
                            token: &self.source[prev_index..],
                            range: prev_index..self.source.len(),
                            kind: prev_kind,
                        }));
                    }
                    None => break None,
                }
            };

            if curr_grapheme == "\"" {
                match self.iter_state {
                    None => {
                        break Some(self.parse_string(curr_index));
                    }
                    Some((prev_index, TokenKind::String)) => {
                        self.iter_state = None;
                        let range = (prev_index + 1)..curr_index;
                        break Some(Ok(Token {
                            token: &self.source[range.clone()],
                            range,
                            kind: TokenKind::String,
                        }));
                    }
                    Some((prev_index, prev_kind)) => {
                        self.iter_state = Some((curr_index, TokenKind::String));
                        let range = prev_index..curr_index;
                        break Some(Ok(Token {
                            token: &self.source[range.clone()],
                            range,
                            kind: prev_kind,
                        }));
                    }
                }
            }

            if let Some((quote_index, TokenKind::String)) = self.iter_state {
                self.iter_state = None;
                break Some(self.parse_string(quote_index));
            }

            let curr_kind = curr_grapheme.into();
            match self.iter_state {
                Some((prev_index, prev_kind)) => {
                    if matches!(prev_kind, TokenKind::Punctuation)
                        || matches!(prev_kind, TokenKind::Whitespace)
                        || curr_kind != prev_kind
                    {
                        self.iter_state = Some((curr_index, curr_kind));
                        break Some(Ok(Token {
                            token: &self.source[prev_index..curr_index],
                            range: prev_index..curr_index,
                            kind: prev_kind,
                        }));
                    }
                }
                None => self.iter_state = Some((curr_index, curr_kind)),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Token<'a> {
    pub(crate) token: &'a str,
    pub(crate) range: Range<usize>,
    pub(crate) kind: TokenKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Whitespace,
    Keyword,
    Numeric,
    Punctuation,
    String,
    Unknown,
}

impl<'a> From<&'a str> for TokenKind {
    fn from(source: &'a str) -> Self {
        if source == "\r\n" {
            return Self::Whitespace;
        }

        if source.len() != 1 {
            return Self::Keyword;
        }

        let character = source.chars().next().unwrap();

        if character.is_ascii_whitespace() {
            return Self::Whitespace;
        } else if character.is_ascii_alphabetic() {
            return Self::Keyword;
        } else if character.is_ascii_digit() {
            return Self::Numeric;
        } else if character.is_ascii_punctuation() {
            return Self::Punctuation;
        }

        Self::Unknown
    }
}
