use lexer::{Token, tokens};

use crate::{Block, ElseIf, Error, If, Res, block::parse_optional_indented_block, parse_block};

pub(crate) fn parse_if<'a>(tokens: &mut tokens!('a)) -> Res<If<'a>> {
    bind!((Token::If, indent_level) = tokens);
    let condition = parse_block(tokens)?;
    let consequent = parse_optional_indented_block(tokens, indent_level)?;

    let (conditional_antequents, antequent) = parse_conditional_antequents(tokens, indent_level)?;

    // `Option<Option<Block>>` is the right type for the final antequent.
    // Namely, there are three possible cases:
    // 1. `else $expr` (the "normal" case - `Some(Some(..))`)
    // 2. `else` (empty else-clause - `Some(None)`)
    // 3. n/a (nothing - `None`)
    //
    // However, during storage, we don't care of the difference between cases 1 and
    // 2. They're effectively, through the compiler's eyes, the same thing.
    //
    // Therefore, we flatten.
    let antequent = antequent.flatten();

    Ok(If {
        condition,
        consequent,
        conditional_antequents,
        antequent,
    })
}

fn parse_conditional_antequents<'a>(
    tokens: &mut tokens!('a),
    indent_level: usize,
) -> Res<(Vec<ElseIf<'a>>, Option<Option<Block<'a>>>)> {
    // temporary
    enum Either<'a> {
        Left(ElseIf<'a>),
        Right(Option<Block<'a>>),
    }

    fn parse_conditional_antequent_optional<'a>(
        tokens: &mut tokens!('a),
        indent_level: usize,
    ) -> Res<Option<Either<'a>>> {
        bindif!((Token::Else, else_indent_level) = tokens);
        if indent_level != else_indent_level {
            return Err(Error::UnexpectedIndentation);
        }

        Ok(Some(match peek!(tokens) {
            token!(Token::If, _) => {
                step!(tokens);
                let condition = parse_block(tokens)?;
                let consequent = parse_optional_indented_block(tokens, indent_level)?;
                Either::Left(ElseIf {
                    condition,
                    consequent,
                })
            }
            _ => {
                let consequent = parse_optional_indented_block(tokens, indent_level)?;
                Either::Right(consequent)
            }
        }))
    }

    let mut conditional_antequents = vec![];
    let mut antequent = None;

    Ok(loop {
        let either = parse_conditional_antequent_optional(tokens, indent_level)?;
        let Some(either) = either else {
            break (conditional_antequents, antequent);
        };
        match either {
            Either::Left(conditional_antequent) => {
                conditional_antequents.push(conditional_antequent)
            }
            Either::Right(antequent_) => antequent = Some(antequent_),
        }
    })
}
