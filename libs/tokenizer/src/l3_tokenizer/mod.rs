use std::ops::Range;

use crate::{Res, l2_tokenizer::L2TokenType};

#[cfg(test)]
mod tests;

pub fn l3_tokenize(source: &str) -> impl Iterator<Item = Res<L3Token<'_>>> {
    [].into_iter()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct L3Token<'a> {
    pub token_type: L2TokenType<'a>,
    pub source_range: Range<usize>,
    pub line: usize,
    pub line_range: Range<usize>,
    pub indentation_level: usize,
}
