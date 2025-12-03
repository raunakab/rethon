use aggregator::{Brace, TokenType};
use rstest::rstest;

use crate::{Error, Node, Structure, structurizer};

// Simplified node type without ranges for easier testing
#[derive(Clone, Debug, PartialEq, Eq)]
enum SimpleNode<'a> {
    Token(TokenType<'a>),
    Block(SimpleBlock<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SimpleBlock<'a> {
    brace: Brace,
    structure: Vec<SimpleNode<'a>>,
}

type SimpleStructure<'a> = Vec<SimpleNode<'a>>;

// Convert a full Structure to a SimpleStructure (strips ranges)
fn to_simple_structure(structure: Structure<'_>) -> SimpleStructure<'_> {
    structure
        .into_iter()
        .map(|node| match node {
            Node::Token(token) => SimpleNode::Token(token.token_type),
            Node::Block(block) => SimpleNode::Block(SimpleBlock {
                brace: block.brace,
                structure: to_simple_structure(block.structure),
            }),
        })
        .collect()
}

// Basic successful parsing tests
#[rstest]
#[case("", Ok(vec![]))]
#[case(
    "a",
    Ok(vec![SimpleNode::Token(TokenType::Identifier("a"))])
)]
#[case(
    "fn",
    Ok(vec![SimpleNode::Token(TokenType::Function)])
)]
#[case(
    "42",
    Ok(vec![SimpleNode::Token(TokenType::Number("42"))])
)]
#[case(
    "true",
    Ok(vec![SimpleNode::Token(TokenType::True)])
)]
#[case(
    "+",
    Ok(vec![SimpleNode::Token(TokenType::Plus)])
)]
fn test_basic_tokens(
    #[case] source: &str,
    #[case] expected: Result<SimpleStructure<'static>, Error>,
) {
    assert_eq!(structurizer(source).map(to_simple_structure), expected);
}

// Block structure tests
#[rstest]
#[case(
    "{}",
    Ok(vec![SimpleNode::Block(SimpleBlock {
        brace: Brace::Curly,
        structure: vec![],
    })])
)]
#[case(
    "{a}",
    Ok(vec![SimpleNode::Block(SimpleBlock {
        brace: Brace::Curly,
        structure: vec![SimpleNode::Token(TokenType::Identifier("a"))],
    })])
)]
#[case(
    "[]",
    Ok(vec![SimpleNode::Block(SimpleBlock {
        brace: Brace::Square,
        structure: vec![],
    })])
)]
#[case(
    "()",
    Ok(vec![SimpleNode::Block(SimpleBlock {
        brace: Brace::Round,
        structure: vec![],
    })])
)]
#[case(
    "a{b}c",
    Ok(vec![
        SimpleNode::Token(TokenType::Identifier("a")),
        SimpleNode::Block(SimpleBlock {
            brace: Brace::Curly,
            structure: vec![SimpleNode::Token(TokenType::Identifier("b"))],
        }),
        SimpleNode::Token(TokenType::Identifier("c")),
    ])
)]
#[case(
    "{a}{b}",
    Ok(vec![
        SimpleNode::Block(SimpleBlock {
            brace: Brace::Curly,
            structure: vec![SimpleNode::Token(TokenType::Identifier("a"))],
        }),
        SimpleNode::Block(SimpleBlock {
            brace: Brace::Curly,
            structure: vec![SimpleNode::Token(TokenType::Identifier("b"))],
        }),
    ])
)]
fn test_block_structures(
    #[case] source: &str,
    #[case] expected: Result<SimpleStructure<'static>, Error>,
) {
    assert_eq!(structurizer(source).map(to_simple_structure), expected);
}

// Nested block tests
#[rstest]
#[case(
    "{{}}",
    Ok(vec![SimpleNode::Block(SimpleBlock {
        brace: Brace::Curly,
        structure: vec![SimpleNode::Block(SimpleBlock {
            brace: Brace::Curly,
            structure: vec![],
        })],
    })])
)]
#[case(
    "{a{b}}",
    Ok(vec![SimpleNode::Block(SimpleBlock {
        brace: Brace::Curly,
        structure: vec![
            SimpleNode::Token(TokenType::Identifier("a")),
            SimpleNode::Block(SimpleBlock {
                brace: Brace::Curly,
                structure: vec![SimpleNode::Token(TokenType::Identifier("b"))],
            }),
        ],
    })])
)]
#[case(
    "{[()]}",
    Ok(vec![SimpleNode::Block(SimpleBlock {
        brace: Brace::Curly,
        structure: vec![SimpleNode::Block(SimpleBlock {
            brace: Brace::Square,
            structure: vec![SimpleNode::Block(SimpleBlock {
                brace: Brace::Round,
                structure: vec![],
            })],
        })],
    })])
)]
#[case(
    "{{a}b{c}}",
    Ok(vec![SimpleNode::Block(SimpleBlock {
        brace: Brace::Curly,
        structure: vec![
            SimpleNode::Block(SimpleBlock {
                brace: Brace::Curly,
                structure: vec![SimpleNode::Token(TokenType::Identifier("a"))],
            }),
            SimpleNode::Token(TokenType::Identifier("b")),
            SimpleNode::Block(SimpleBlock {
                brace: Brace::Curly,
                structure: vec![SimpleNode::Token(TokenType::Identifier("c"))],
            }),
        ],
    })])
)]
fn test_nested_blocks(
    #[case] source: &str,
    #[case] expected: Result<SimpleStructure<'static>, Error>,
) {
    assert_eq!(structurizer(source).map(to_simple_structure), expected);
}

// Error cases: Unexpected closing braces
#[rstest]
#[case("}", Err(Error::UnexpectedClosingBrace { brace: Brace::Curly, position: 0 }))]
#[case("]", Err(Error::UnexpectedClosingBrace { brace: Brace::Square, position: 0 }))]
#[case(")", Err(Error::UnexpectedClosingBrace { brace: Brace::Round, position: 0 }))]
#[case("a}", Err(Error::UnexpectedClosingBrace { brace: Brace::Curly, position: 1 }))]
#[case("a}b", Err(Error::UnexpectedClosingBrace { brace: Brace::Curly, position: 1 }))]
#[case("{}}", Err(Error::UnexpectedClosingBrace { brace: Brace::Curly, position: 2 }))]
fn test_unexpected_closing_brace(
    #[case] source: &str,
    #[case] expected: Result<SimpleStructure<'static>, Error>,
) {
    assert_eq!(structurizer(source).map(to_simple_structure), expected);
}

// Error cases: Mismatched braces
#[rstest]
#[case("{]", Err(Error::MismatchedBrace { expected: Brace::Curly, found: Brace::Square, position: 1 }))]
#[case("{)", Err(Error::MismatchedBrace { expected: Brace::Curly, found: Brace::Round, position: 1 }))]
#[case("[}", Err(Error::MismatchedBrace { expected: Brace::Square, found: Brace::Curly, position: 1 }))]
#[case("[)", Err(Error::MismatchedBrace { expected: Brace::Square, found: Brace::Round, position: 1 }))]
#[case("(}", Err(Error::MismatchedBrace { expected: Brace::Round, found: Brace::Curly, position: 1 }))]
#[case("(]", Err(Error::MismatchedBrace { expected: Brace::Round, found: Brace::Square, position: 1 }))]
#[case("{a]", Err(Error::MismatchedBrace { expected: Brace::Curly, found: Brace::Square, position: 2 }))]
#[case("{a{b]}", Err(Error::MismatchedBrace { expected: Brace::Curly, found: Brace::Square, position: 4 }))]
#[case("({a)]", Err(Error::MismatchedBrace { expected: Brace::Curly, found: Brace::Round, position: 3 }))]
fn test_mismatched_braces(
    #[case] source: &str,
    #[case] expected: Result<SimpleStructure<'static>, Error>,
) {
    assert_eq!(structurizer(source).map(to_simple_structure), expected);
}

// Error cases: Unterminated blocks
#[rstest]
#[case("{", Err(Error::UnterminatedBlock { brace: Brace::Curly }))]
#[case("[", Err(Error::UnterminatedBlock { brace: Brace::Square }))]
#[case("(", Err(Error::UnterminatedBlock { brace: Brace::Round }))]
#[case("{a", Err(Error::UnterminatedBlock { brace: Brace::Curly }))]
#[case("{a{b}", Err(Error::UnterminatedBlock { brace: Brace::Curly }))]
#[case("{{", Err(Error::UnterminatedBlock { brace: Brace::Curly }))]
#[case("{{}a", Err(Error::UnterminatedBlock { brace: Brace::Curly }))]
#[case("(((", Err(Error::UnterminatedBlock { brace: Brace::Round }))]
#[case("{[", Err(Error::UnterminatedBlock { brace: Brace::Square }))]
fn test_unterminated_blocks(
    #[case] source: &str,
    #[case] expected: Result<SimpleStructure<'static>, Error>,
) {
    assert_eq!(structurizer(source).map(to_simple_structure), expected);
}

// Complex real-world examples
#[rstest]
#[case(
    "fn add(a, b) { return a + b }",
    Ok(vec![
        SimpleNode::Token(TokenType::Function),
        SimpleNode::Token(TokenType::Whitespace(1)),
        SimpleNode::Token(TokenType::Identifier("add")),
        SimpleNode::Block(SimpleBlock {
            brace: Brace::Round,
            structure: vec![
                SimpleNode::Token(TokenType::Identifier("a")),
                SimpleNode::Token(TokenType::Comma),
                SimpleNode::Token(TokenType::Whitespace(1)),
                SimpleNode::Token(TokenType::Identifier("b")),
            ],
        }),
        SimpleNode::Token(TokenType::Whitespace(1)),
        SimpleNode::Block(SimpleBlock {
            brace: Brace::Curly,
            structure: vec![
                SimpleNode::Token(TokenType::Whitespace(1)),
                SimpleNode::Token(TokenType::Return),
                SimpleNode::Token(TokenType::Whitespace(1)),
                SimpleNode::Token(TokenType::Identifier("a")),
                SimpleNode::Token(TokenType::Whitespace(1)),
                SimpleNode::Token(TokenType::Plus),
                SimpleNode::Token(TokenType::Whitespace(1)),
                SimpleNode::Token(TokenType::Identifier("b")),
                SimpleNode::Token(TokenType::Whitespace(1)),
            ],
        }),
    ])
)]
#[case(
    "arr[0]",
    Ok(vec![
        SimpleNode::Token(TokenType::Identifier("arr")),
        SimpleNode::Block(SimpleBlock {
            brace: Brace::Square,
            structure: vec![SimpleNode::Token(TokenType::Number("0"))],
        }),
    ])
)]
#[case(
    "if (x > 0) { y }",
    Ok(vec![
        SimpleNode::Token(TokenType::If),
        SimpleNode::Token(TokenType::Whitespace(1)),
        SimpleNode::Block(SimpleBlock {
            brace: Brace::Round,
            structure: vec![
                SimpleNode::Token(TokenType::Identifier("x")),
                SimpleNode::Token(TokenType::Whitespace(1)),
                SimpleNode::Token(TokenType::Greater),
                SimpleNode::Token(TokenType::Whitespace(1)),
                SimpleNode::Token(TokenType::Number("0")),
            ],
        }),
        SimpleNode::Token(TokenType::Whitespace(1)),
        SimpleNode::Block(SimpleBlock {
            brace: Brace::Curly,
            structure: vec![
                SimpleNode::Token(TokenType::Whitespace(1)),
                SimpleNode::Token(TokenType::Identifier("y")),
                SimpleNode::Token(TokenType::Whitespace(1)),
            ],
        }),
    ])
)]
fn test_complex_structures(
    #[case] source: &str,
    #[case] expected: Result<SimpleStructure<'static>, Error>,
) {
    assert_eq!(structurizer(source).map(to_simple_structure), expected);
}
