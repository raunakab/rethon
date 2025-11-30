#[cfg(test)]
mod tests;

use std::iter::Peekable;

use aggregator::{Brace, BraceDirection, Token, TokenType, aggregator as aggregate_tokens};
use thiserror::Error;

type Res<T = ()> = Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error(transparent)]
    Aggregator(#[from] aggregator::Error),
    #[error("Mismatched closing brace: expected {expected:?}, found {found:?} at byte {position}")]
    MismatchedBrace {
        expected: Brace,
        found: Brace,
        position: usize,
    },
    #[error("Unexpected closing brace {brace:?} at byte {position}")]
    UnexpectedClosingBrace { brace: Brace, position: usize },
    #[error("Unterminated block for {brace:?}")]
    UnterminatedBlock { brace: Brace },
}

pub fn structurizer<'a>(source: &'a str) -> Res<Structure<'a>> {
    let mut tokens = aggregate_tokens(source).peekable();
    build_structure(&mut tokens, None)
}

fn build_structure<'a, I>(
    tokens: &mut Peekable<I>,
    expected_closing: Option<Brace>,
) -> Res<Structure<'a>>
where
    I: Iterator<Item = Result<Token<'a>, aggregator::Error>>,
{
    let mut nodes = Vec::new();

    while let Some(token) = tokens.next() {
        let token = token?;
        match token.token_type {
            TokenType::Brace(brace, BraceDirection::Open) => {
                let structure = build_structure(tokens, Some(brace))?;
                nodes.push(Node::Block(Block { brace, structure }));
            }
            TokenType::Brace(brace, BraceDirection::Close) => {
                return match expected_closing {
                    Some(expected) if expected == brace => Ok(nodes),
                    Some(expected) => Err(Error::MismatchedBrace {
                        expected,
                        found: brace,
                        position: token.range.start,
                    }),
                    None => Err(Error::UnexpectedClosingBrace {
                        brace,
                        position: token.range.start,
                    }),
                };
            }
            _ => nodes.push(Node::Token(token)),
        }
    }

    if let Some(brace) = expected_closing {
        return Err(Error::UnterminatedBlock { brace });
    }

    Ok(nodes)
}

pub type Structure<'a> = Vec<Node<'a>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node<'a> {
    Token(Token<'a>),
    Block(Block<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block<'a> {
    pub brace: Brace,
    pub structure: Structure<'a>,
}
