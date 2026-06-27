use lexer::tokens;

use crate::{Block, Res};

pub(crate) fn parse_block<'a>(_tokens: &mut tokens!('a)) -> Res<Block<'a>> {
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

// pub(crate) fn parse_expression<'a>(tokens: &mut tokens!('a)) -> Res<Expression<'a>> {
//     todo!()
// }
