use lexer::{Token, tokens};

use crate::{
    Error, Match, MatchArm, Res,
    block::{BlockTerminator, parse_block},
    pattern::parse_pattern,
};

pub(crate) fn parse_match<'a>(tokens: &mut tokens!('a)) -> Res<Match<'a>> {
    bind!((Token::Match, indent_level) = tokens);
    let condition = parse_block(tokens, Some(BlockTerminator::Colon))?;
    let match_arms = parse_match_arms(tokens, indent_level)?;

    Ok(Match {
        condition,
        match_arms,
    })
}

fn parse_match_arms<'a>(tokens: &mut tokens!('a), indent_level: usize) -> Res<Vec<MatchArm<'a>>> {
    fn parse_match_arm_optional<'a>(
        tokens: &mut tokens!('a),
        _indent_level: usize,
        _first: bool,
    ) -> Res<Option<MatchArm<'a>>> {
        let _pattern = parse_pattern(tokens)?;
        bind!((Token::FatArrow, _) = tokens);
        let _body = parse_block(tokens, Some(BlockTerminator::Comma))?;

        todo!()
    }

    let mut match_arms = vec![];

    Ok(loop {
        let first = match_arms.is_empty();
        let match_arm = parse_match_arm_optional(tokens, indent_level, first)?;
        let Some(match_arm) = match_arm else {
            break match_arms;
        };
        match_arms.push(match_arm);
    })
}

#[cfg(test)]
mod tests;
