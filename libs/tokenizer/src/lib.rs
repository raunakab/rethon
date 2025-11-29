#[cfg(test)]
mod tests;

use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

pub fn tokenize(source: &str) -> impl Iterator<Item = (&str, GraphemeState)> {
    TokenIter::from(source)
}

struct TokenIter<'a> {
    source: &'a str,
    graphemes: GraphemeIndices<'a>,
    grapheme_state: Option<(usize, GraphemeState)>,
}

impl<'a> From<&'a str> for TokenIter<'a> {
    fn from(source: &'a str) -> Self {
        return Self {
            source,
            graphemes: source.grapheme_indices(true),
            grapheme_state: None,
        };
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = (&'a str, GraphemeState);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.graphemes.next() {
                Some((curr_index, curr_grapheme)) => {
                    let curr_state = curr_grapheme.into();
                    match self.grapheme_state {
                        Some((prev_index, prev_state)) => {
                            if matches!(prev_state, GraphemeState::Punctuation)
                                || curr_state != prev_state
                            {
                                self.grapheme_state = Some((curr_index, curr_state));
                                break Some((&self.source[prev_index..curr_index], prev_state));
                            }
                        }
                        None => self.grapheme_state = Some((curr_index, curr_state)),
                    }
                }
                None => match self.grapheme_state {
                    Some((index, state)) => {
                        self.grapheme_state = None;
                        break Some((&self.source[index..], state));
                    }
                    None => break None,
                },
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphemeState {
    Whitespace,
    Keyword,
    Numeric,
    Punctuation,
    Control,
    Unknown,
}

impl<'a> From<&'a str> for GraphemeState {
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
        } else if character.is_ascii_control() {
            return Self::Control;
        }

        return Self::Unknown;
    }
}
