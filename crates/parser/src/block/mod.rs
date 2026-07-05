use lexer::tokens;

use crate::{Block, Res};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BlockTerminator {
    Colon,
    Comma,
    FatArrow,
}

pub(crate) fn parse_block<'a>(
    _tokens: &mut tokens!('a),
    _terminator: Option<BlockTerminator>,
) -> Res<Block<'a>> {
    todo!()
}

pub(crate) fn parse_indented_block<'a>(
    _tokens: &mut tokens!('a),
    _original_indent_level: usize,
) -> Res<Block<'a>> {
    todo!()
}

pub(crate) fn parse_optional_indented_block<'a>(
    _tokens: &mut tokens!('a),
    _original_indent_level: usize,
) -> Res<Option<Block<'a>>> {
    todo!()
}

#[cfg(test)]
mod tests;
