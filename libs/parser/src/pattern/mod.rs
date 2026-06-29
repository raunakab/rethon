use lexer::{Token, tokens};

use crate::{Error, Literal, Pattern, Res};

pub(crate) fn parse_pattern<'a>(_tokens: &mut tokens!('a)) -> Res<Pattern<'a>> {
    todo!()
}

pub(crate) fn parse_ident<'a>(tokens: &mut tokens!('a)) -> Res<&'a str> {
    bind!((Token::Identifier(ident), _) = tokens);
    Ok(ident)
}

pub(crate) fn parse_literal_optional<'a>(tokens: &mut tokens!('a)) -> Res<Option<Literal<'a>>> {
    let token = peek!(tokens);
    Ok(match token {
        token!(Token::True, _) => {
            step!(tokens);
            Some(Literal::True)
        }
        token!(Token::False, _) => {
            step!(tokens);
            Some(Literal::False)
        }
        token!(Token::Number(number), _) => {
            let &number = number;
            step!(tokens);
            Some(Literal::Number(number))
        }
        token!(Token::Float(number, antissa), _) => {
            let &number = number;
            let &antissa = antissa;
            step!(tokens);
            Some(Literal::Float(number, antissa))
        }
        token!(Token::String(string, string_type), _) => {
            let &string = string;
            let &string_type = string_type;
            step!(tokens);
            Some(Literal::String(string, string_type))
        }
        _ => None,
    })
}

pub(crate) fn parse_literal<'a>(tokens: &mut tokens!('a)) -> Res<Literal<'a>> {
    let Some(literal) = parse_literal_optional(tokens)? else {
        bind!((token, _) = tokens);
        return Err(Error::UnexpectedToken(token.to_string()));
    };
    Ok(literal)
}

#[cfg(test)]
mod tests;
