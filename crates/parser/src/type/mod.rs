use lexer::tokens;

use crate::{Expression, Res};

pub(crate) fn parse_type_declaration_optional<'a>(
    _tokens: &mut tokens!('a),
) -> Res<Option<Expression<'a>>> {
    todo!()
}

#[cfg(test)]
mod tests;
