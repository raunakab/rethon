#![doc = include_str!("README.md")]

#[cfg(test)]
mod tests;

use std::ops::Range;

use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

use crate::{Error, Res};

pub(crate) fn l1_tokenize(source: &str) -> impl Iterator<Item = Res<L1Token<'_>>> {
    L1Tokenizer {
        source,
        iter: source.grapheme_indices(true),
        iter_state: None,
    }
}

struct L1Tokenizer<'a> {
    source: &'a str,
    iter: GraphemeIndices<'a>,
    iter_state: Option<(usize, L1TokenType)>,
}

impl<'a> L1Tokenizer<'a> {
    fn parse_string(&mut self, opening_start: usize) -> Res<L1Token<'a>> {
        let opening_end = opening_start + 1; // The quote is always 1 byte

        Ok(loop {
            match self.iter.next() {
                Some((index, grapheme)) => {
                    if grapheme != "\"" {
                        continue;
                    }

                    // Found closing quote - clear iter_state for next token
                    self.iter_state = None;
                    let range = opening_end..index;
                    break L1Token {
                        token: &self.source[range.clone()],
                        range,
                        token_type: L1TokenType::String,
                    };
                }
                None => return Err(Error::UnterminatedString(opening_start)),
            }
        })
    }
}

impl<'a> Iterator for L1Tokenizer<'a> {
    type Item = Res<L1Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some((curr_index, curr_grapheme)) = self.iter.next() else {
                match self.iter_state {
                    Some((prev_index, prev_type)) => {
                        self.iter_state = None;
                        break Some(Ok(L1Token {
                            token: &self.source[prev_index..],
                            range: prev_index..self.source.len(),
                            token_type: prev_type,
                        }));
                    }
                    None => break None,
                }
            };

            // Check if we're starting or closing a string
            if curr_grapheme == "\"" {
                match self.iter_state {
                    None => {
                        break Some(self.parse_string(curr_index));
                    }
                    Some((prev_index, L1TokenType::String)) => {
                        // curr_index is the closing quote; string content is between quotes
                        self.iter_state = None;
                        let range = (prev_index + 1)..curr_index;
                        break Some(Ok(L1Token {
                            token: &self.source[range.clone()],
                            range,
                            token_type: L1TokenType::String,
                        }));
                    }
                    Some((prev_index, prev_type)) => {
                        // Defer string start; emit the preceding token first
                        self.iter_state = Some((curr_index, L1TokenType::String));
                        let range = prev_index..curr_index;
                        break Some(Ok(L1Token {
                            token: &self.source[range.clone()],
                            range,
                            token_type: prev_type,
                        }));
                    }
                }
            }

            // If we had a pending String marker (from deferred string parsing), parse it now
            if let Some((quote_index, L1TokenType::String)) = self.iter_state {
                self.iter_state = None;
                break Some(self.parse_string(quote_index));
            }

            let curr_type = curr_grapheme.into();
            match self.iter_state {
                Some((prev_index, prev_type)) => {
                    if matches!(prev_type, L1TokenType::Punctuation)
                        || matches!(prev_type, L1TokenType::Whitespace)
                        || curr_type != prev_type
                    {
                        self.iter_state = Some((curr_index, curr_type));
                        break Some(Ok(L1Token {
                            token: &self.source[prev_index..curr_index],
                            range: prev_index..curr_index,
                            token_type: prev_type,
                        }));
                    }
                }
                None => self.iter_state = Some((curr_index, curr_type)),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct L1Token<'a> {
    pub(crate) token: &'a str,
    pub(crate) range: Range<usize>,
    pub(crate) token_type: L1TokenType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum L1TokenType {
    Whitespace,
    Keyword,
    Numeric,
    Punctuation,
    String,
    Unknown,
}

impl<'a> From<&'a str> for L1TokenType {
    fn from(source: &'a str) -> Self {
        if source == "\r\n" {
            return Self::Whitespace;
        }

        if source.len() != 1 {
            return Self::Keyword;
        }

        // If this line is reached, `source` must be a `char`.
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

        return Self::Unknown;
    }
}
