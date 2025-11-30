#[cfg(test)]
mod tests;

use std::{iter::Peekable, ops::Range};

use aggregator::{
    Brace, BraceDirection, Error as AggregatorError, Token, TokenType,
    aggregator as aggregate_tokens,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error<'a> {
    /// Error from the underlying aggregator/tokenizer
    Aggregator(AggregatorError<'a>),
    /// Mismatched closing brace (expected, found, position)
    MismatchedBrace {
        expected: Brace,
        found: Brace,
        position: usize,
    },
    /// Unexpected closing brace with no corresponding opening (found, position)
    UnexpectedClosingBrace { brace: Brace, position: usize },
    /// Unterminated block missing closing brace (expected)
    UnterminatedBlock { brace: Brace },
}

impl<'a> From<AggregatorError<'a>> for Error<'a> {
    fn from(err: AggregatorError<'a>) -> Self {
        Error::Aggregator(err)
    }
}

pub fn structurizer<'a>(source: &'a str) -> Result<Structure<'a>, Error<'a>> {
    let mut tokens = aggregate_tokens(source).peekable();
    build_structure(&mut tokens, None)
}

fn build_structure<'a, I>(
    tokens: &mut Peekable<I>,
    expected_closing: Option<Brace>,
) -> Result<Structure<'a>, Error<'a>>
where
    I: Iterator<Item = Result<Token<'a>, AggregatorError<'a>>>,
{
    let mut nodes = Vec::new();

    while let Some(token_result) = tokens.next() {
        let token = token_result?;
        let token_type = token.token_type;
        let range = token.range;

        match token_type {
            TokenType::Brace(brace, BraceDirection::Open) => {
                let structure = build_structure(tokens, Some(brace))?;
                nodes.push(Node::Block(Block { brace, structure }));
            }
            TokenType::Brace(brace, BraceDirection::Close) => match expected_closing {
                Some(expected) if expected == brace => return Ok(Structure { nodes }),
                Some(expected) => {
                    return Err(Error::MismatchedBrace {
                        expected,
                        found: brace,
                        position: range.start,
                    });
                }
                None => {
                    return Err(Error::UnexpectedClosingBrace {
                        brace,
                        position: range.start,
                    });
                }
            },
            _ => nodes.push(Node::Token(TokenSnapshot::from_token(token_type, range))),
        }
    }

    if let Some(brace) = expected_closing {
        Err(Error::UnterminatedBlock { brace })
    } else {
        Ok(Structure { nodes })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Structure<'a> {
    pub nodes: Vec<Node<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node<'a> {
    Token(TokenSnapshot<'a>),
    Block(Block<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block<'a> {
    pub brace: Brace,
    pub structure: Structure<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenSnapshot<'a> {
    pub token_type: TokenType<'a>,
    pub range: Range<usize>,
}

impl<'a> TokenSnapshot<'a> {
    fn from_token(token_type: TokenType<'a>, range: Range<usize>) -> Self {
        Self { token_type, range }
    }
}
