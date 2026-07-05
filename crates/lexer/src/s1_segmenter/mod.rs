#[cfg(test)]
mod tests;

use std::ops::Range;

use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

use crate::{Error, Res};

pub(crate) fn segment(source: &str) -> impl Iterator<Item = Res<Segment<'_>>> {
    Segmenter {
        source,
        iter: source.grapheme_indices(true),
        iter_state: None,
    }
}

struct Segmenter<'a> {
    source: &'a str,
    iter: GraphemeIndices<'a>,
    iter_state: Option<(usize, SegmentKind)>,
}

impl<'a> Segmenter<'a> {
    fn parse_string(&mut self, opening_start: usize) -> Res<Segment<'a>> {
        let opening_end = opening_start + 1;

        Ok(loop {
            match self.iter.next() {
                Some((index, grapheme)) => {
                    if grapheme != "\"" {
                        continue;
                    }

                    self.iter_state = None;
                    let range = opening_end..index;
                    break Segment {
                        segment: &self.source[range.clone()],
                        range,
                        kind: SegmentKind::String,
                    };
                }
                None => return Err(Error::UnterminatedString(opening_start)),
            }
        })
    }
}

impl<'a> Iterator for Segmenter<'a> {
    type Item = Res<Segment<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some((curr_index, curr_grapheme)) = self.iter.next() else {
                match self.iter_state {
                    Some((prev_index, prev_kind)) => {
                        self.iter_state = None;
                        break Some(Ok(Segment {
                            segment: &self.source[prev_index..],
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
                    Some((prev_index, SegmentKind::String)) => {
                        self.iter_state = None;
                        let range = (prev_index + 1)..curr_index;
                        break Some(Ok(Segment {
                            segment: &self.source[range.clone()],
                            range,
                            kind: SegmentKind::String,
                        }));
                    }
                    Some((prev_index, prev_kind)) => {
                        self.iter_state = Some((curr_index, SegmentKind::String));
                        let range = prev_index..curr_index;
                        break Some(Ok(Segment {
                            segment: &self.source[range.clone()],
                            range,
                            kind: prev_kind,
                        }));
                    }
                }
            }

            if let Some((quote_index, SegmentKind::String)) = self.iter_state {
                self.iter_state = None;
                break Some(self.parse_string(quote_index));
            }

            let curr_kind = curr_grapheme.into();
            match self.iter_state {
                Some((prev_index, prev_kind)) => {
                    if matches!(prev_kind, SegmentKind::Punctuation)
                        || matches!(prev_kind, SegmentKind::Whitespace)
                        || curr_kind != prev_kind
                    {
                        self.iter_state = Some((curr_index, curr_kind));
                        break Some(Ok(Segment {
                            segment: &self.source[prev_index..curr_index],
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
pub(crate) struct Segment<'a> {
    pub(crate) segment: &'a str,
    pub(crate) range: Range<usize>,
    pub(crate) kind: SegmentKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SegmentKind {
    Whitespace,
    Keyword,
    Numeric,
    Punctuation,
    String,
    Unknown,
}

impl<'a> From<&'a str> for SegmentKind {
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
