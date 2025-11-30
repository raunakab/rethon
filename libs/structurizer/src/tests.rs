use aggregator::{Brace, TokenType};
use rstest::rstest;

use crate::{Block, Error, Node, Structure, TokenSnapshot, structurizer};

type StructResult<'a> = Result<Structure<'a>, Error<'a>>;

#[rstest]
#[case("", Ok(Structure { nodes: vec![] }))]
#[case("a", Ok(Structure { nodes: vec![Node::Token(TokenSnapshot { token_type: TokenType::Identifier("a"), range: 0..1 })] }))]
#[case(
    "{a}",
    Ok(Structure {
        nodes: vec![Node::Block(Block {
            brace: Brace::Curly,
            structure: Structure {
                nodes: vec![Node::Token(TokenSnapshot { token_type: TokenType::Identifier("a"), range: 1..2 })],
            },
        })],
    })
)]
#[case(
    "{a{b}}",
    Ok(Structure {
        nodes: vec![Node::Block(Block {
            brace: Brace::Curly,
            structure: Structure {
                nodes: vec![
                    Node::Token(TokenSnapshot { token_type: TokenType::Identifier("a"), range: 1..2 }),
                    Node::Block(Block {
                        brace: Brace::Curly,
                        structure: Structure {
                            nodes: vec![Node::Token(TokenSnapshot { token_type: TokenType::Identifier("b"), range: 3..4 })],
                        },
                    }),
                ],
            },
        })],
    })
)]
#[case("}", Err(Error::UnexpectedClosingBrace { brace: Brace::Curly, position: 0 }))]
#[case("{", Err(Error::UnterminatedBlock { brace: Brace::Curly }))]
fn test_structurizer(#[case] source: &str, #[case] expected: StructResult<'static>) {
    assert_eq!(structurizer(source), expected);
}
