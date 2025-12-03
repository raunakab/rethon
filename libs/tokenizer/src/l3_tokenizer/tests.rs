use std::ops::Range;

use crate::{
    Error, Res,
    l2_tokenizer::L2TokenType,
    l3_tokenizer::{L3Token, l3_tokenize},
};

// Simplified token type for easier testing (strips ranges and source positions)
#[derive(Clone, Debug, PartialEq, Eq)]
struct SimpleL3Token<'a> {
    token_type: L2TokenType<'a>,
    line: usize,
    line_range: Range<usize>,
    indentation_level: usize,
}

impl<'a> From<L3Token<'a>> for SimpleL3Token<'a> {
    fn from(token: L3Token<'a>) -> Self {
        SimpleL3Token {
            token_type: token.token_type,
            line: token.line,
            line_range: token.line_range,
            indentation_level: token.indentation_level,
        }
    }
}

#[rstest::rstest]
// Empty input
#[case("", Ok(vec![]))]
// Single token on single line
#[case(
    "fn",
    Ok(vec![
        SimpleL3Token {
            token_type: L2TokenType::Function,
            line: 0,
            line_range: 0..2,
            indentation_level: 0,
        },
    ])
)]
// Multiple tokens on single line
#[case(
    "fn add",
    Ok(vec![
        SimpleL3Token {
            token_type: L2TokenType::Function,
            line: 0,
            line_range: 0..2,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Whitespace(1),
            line: 0,
            line_range: 2..3,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Identifier("add"),
            line: 0,
            line_range: 3..6,
            indentation_level: 0,
        },
    ])
)]
// Multiple lines without indentation
#[case(
    "fn add\nreturn",
    Ok(vec![
        SimpleL3Token {
            token_type: L2TokenType::Function,
            line: 0,
            line_range: 0..2,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Whitespace(1),
            line: 0,
            line_range: 2..3,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Identifier("add"),
            line: 0,
            line_range: 3..6,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Newline,
            line: 0,
            line_range: 6..7,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Return,
            line: 1,
            line_range: 0..6,
            indentation_level: 0,
        },
    ])
)]
// Single level indentation
#[case(
    "fn\n    add",
    Ok(vec![
        SimpleL3Token {
            token_type: L2TokenType::Function,
            line: 0,
            line_range: 0..2,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Newline,
            line: 0,
            line_range: 2..3,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Whitespace(4),
            line: 1,
            line_range: 0..4,
            indentation_level: 1,
        },
        SimpleL3Token {
            token_type: L2TokenType::Identifier("add"),
            line: 1,
            line_range: 4..7,
            indentation_level: 1,
        },
    ])
)]
// Nested indentation (2 levels)
#[case(
    "fn\n    if\n        x",
    Ok(vec![
        SimpleL3Token {
            token_type: L2TokenType::Function,
            line: 0,
            line_range: 0..2,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Newline,
            line: 0,
            line_range: 2..3,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Whitespace(4),
            line: 1,
            line_range: 0..4,
            indentation_level: 1,
        },
        SimpleL3Token {
            token_type: L2TokenType::If,
            line: 1,
            line_range: 4..6,
            indentation_level: 1,
        },
        SimpleL3Token {
            token_type: L2TokenType::Newline,
            line: 1,
            line_range: 6..7,
            indentation_level: 1,
        },
        SimpleL3Token {
            token_type: L2TokenType::Whitespace(8),
            line: 2,
            line_range: 0..8,
            indentation_level: 2,
        },
        SimpleL3Token {
            token_type: L2TokenType::Identifier("x"),
            line: 2,
            line_range: 8..9,
            indentation_level: 2,
        },
    ])
)]
// Line range resets on new line
#[case(
    "abc def\nghi",
    Ok(vec![
        SimpleL3Token {
            token_type: L2TokenType::Identifier("abc"),
            line: 0,
            line_range: 0..3,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Whitespace(1),
            line: 0,
            line_range: 3..4,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Identifier("def"),
            line: 0,
            line_range: 4..7,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Newline,
            line: 0,
            line_range: 7..8,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Identifier("ghi"),
            line: 1,
            line_range: 0..3,
            indentation_level: 0,
        },
    ])
)]
// Indentation resets after newline
#[case(
    "fn\n    x\ny",
    Ok(vec![
        SimpleL3Token {
            token_type: L2TokenType::Function,
            line: 0,
            line_range: 0..2,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Newline,
            line: 0,
            line_range: 2..3,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Whitespace(4),
            line: 1,
            line_range: 0..4,
            indentation_level: 1,
        },
        SimpleL3Token {
            token_type: L2TokenType::Identifier("x"),
            line: 1,
            line_range: 4..5,
            indentation_level: 1,
        },
        SimpleL3Token {
            token_type: L2TokenType::Newline,
            line: 1,
            line_range: 5..6,
            indentation_level: 1,
        },
        SimpleL3Token {
            token_type: L2TokenType::Identifier("y"),
            line: 2,
            line_range: 0..1,
            indentation_level: 0,
        },
    ])
)]
// Invalid indentation: 3 spaces (not a multiple of 4)
#[case("fn\n   add", Err(Error::InvalidIndentation { found: 3, position: 3 }))]
// Invalid indentation: 5 spaces
#[case("x\n     y", Err(Error::InvalidIndentation { found: 5, position: 2 }))]
// Invalid indentation: 2 spaces
#[case("a\n  b", Err(Error::InvalidIndentation { found: 2, position: 2 }))]
// Valid 8-space indentation (2 levels)
#[case(
    "x\n        y",
    Ok(vec![
        SimpleL3Token {
            token_type: L2TokenType::Identifier("x"),
            line: 0,
            line_range: 0..1,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Newline,
            line: 0,
            line_range: 1..2,
            indentation_level: 0,
        },
        SimpleL3Token {
            token_type: L2TokenType::Whitespace(8),
            line: 1,
            line_range: 0..8,
            indentation_level: 2,
        },
        SimpleL3Token {
            token_type: L2TokenType::Identifier("y"),
            line: 1,
            line_range: 8..9,
            indentation_level: 2,
        },
    ])
)]
fn test_l3_tokenization(#[case] source: &str, #[case] expected: Res<Vec<SimpleL3Token<'static>>>) {
    assert_eq!(
        l3_tokenize(source)
            .map(|token| {
                let token = token?;
                Ok(SimpleL3Token::from(token))
            })
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
