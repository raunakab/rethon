#[cfg(test)]
mod tests;

use std::ops::Range;

use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

pub(crate) fn l1_tokenize(source: &str) -> impl Iterator<Item = L1Token<'_>> {
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

impl<'a> Iterator for L1Tokenizer<'a> {
    type Item = L1Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some((curr_index, curr_grapheme)) => {
                    let curr_type = curr_grapheme.into();
                    match self.iter_state {
                        Some((prev_index, prev_type)) => {
                            if matches!(prev_type, L1TokenType::Punctuation)
                                || matches!(prev_type, L1TokenType::Whitespace)
                                || curr_type != prev_type
                            {
                                self.iter_state = Some((curr_index, curr_type));
                                break Some(L1Token {
                                    token: &self.source[prev_index..curr_index],
                                    range: prev_index..curr_index,
                                    token_type: prev_type,
                                });
                            }
                        }
                        None => self.iter_state = Some((curr_index, curr_type)),
                    }
                }
                None => match self.iter_state {
                    Some((prev_index, prev_type)) => {
                        self.iter_state = None;
                        break Some(L1Token {
                            token: &self.source[prev_index..],
                            range: prev_index..self.source.len(),
                            token_type: prev_type,
                        });
                    }
                    None => break None,
                },
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
    Unknown,
}

impl<'a> From<&'a str> for L1TokenType {
    fn from(source: &'a str) -> Self {
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
