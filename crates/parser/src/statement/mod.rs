use lexer::tokens;

use crate::{Res, Statement};

pub(crate) fn parse_statement<'a>(_tokens: &mut tokens!('a)) -> Res<Statement<'a>> {
    todo!()
}

#[cfg(test)]
mod tests;
