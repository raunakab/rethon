use crate::{
    Error, Res,
    l1_tokenizer::l1_tokenize,
    l2_tokenizer::l2_tokenize,
    l3_tokenizer::{L3Token, l3_tokenize},
    types::TokenType,
};

// Simplified token type for easier testing (strips ranges)
#[derive(Clone, Debug, PartialEq, Eq)]
struct SimpleToken<'a> {
    token_type: TokenType<'a>,
    line: usize,
    indentation_level: usize,
}

impl<'a> From<L3Token<'a>> for SimpleToken<'a> {
    fn from(token: L3Token<'a>) -> Self {
        SimpleToken {
            token_type: token.token_type,
            line: token.position.line,
            indentation_level: token.position.indentation_level,
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
        SimpleToken {
            token_type: TokenType::Function,
            line: 0,
            indentation_level: 0,
        },
    ])
)]
// Multiple tokens on single line
#[case(
    "fn add",
    Ok(vec![
        SimpleToken {
            token_type: TokenType::Function,
            line: 0,
            indentation_level: 0,
        },
        SimpleToken {
            token_type: TokenType::Identifier("add"),
            line: 0,
            indentation_level: 0,
        },
    ])
)]
// Multiple lines without indentation
#[case(
    "fn add\nreturn",
    Ok(vec![
        SimpleToken {
            token_type: TokenType::Function,
            line: 0,
            indentation_level: 0,
        },
        SimpleToken {
            token_type: TokenType::Identifier("add"),
            line: 0,
            indentation_level: 0,
        },
        SimpleToken {
            token_type: TokenType::Return,
            line: 1,
            indentation_level: 0,
        },
    ])
)]
// Single level indentation
#[case(
    "fn\n    add",
    Ok(vec![
        SimpleToken {
            token_type: TokenType::Function,
            line: 0,
            indentation_level: 0,
        },
        SimpleToken {
            token_type: TokenType::Identifier("add"),
            line: 1,
            indentation_level: 1,
        },
    ])
)]
// Nested indentation (2 levels)
#[case(
    "fn\n    if\n        x",
    Ok(vec![
        SimpleToken {
            token_type: TokenType::Function,
            line: 0,
            indentation_level: 0,
        },
        SimpleToken {
            token_type: TokenType::If,
            line: 1,
            indentation_level: 1,
        },
        SimpleToken {
            token_type: TokenType::Identifier("x"),
            line: 2,
            indentation_level: 2,
        },
    ])
)]
// Indentation reset after newline
#[case(
    "fn\n    x\ny",
    Ok(vec![
        SimpleToken {
            token_type: TokenType::Function,
            line: 0,
            indentation_level: 0,
        },
        SimpleToken {
            token_type: TokenType::Identifier("x"),
            line: 1,
            indentation_level: 1,
        },
        SimpleToken {
            token_type: TokenType::Identifier("y"),
            line: 2,
            indentation_level: 0,
        },
    ])
)]
// Multiple indented sections at same level
#[case(
    "a\n    x\n    y",
    Ok(vec![
        SimpleToken {
            token_type: TokenType::Identifier("a"),
            line: 0,
            indentation_level: 0,
        },
        SimpleToken {
            token_type: TokenType::Identifier("x"),
            line: 1,
            indentation_level: 1,
        },
        SimpleToken {
            token_type: TokenType::Identifier("y"),
            line: 2,
            indentation_level: 1,
        },
    ])
)]
// Complex nested structure
#[case(
    "fn\n    if\n        x\n        y\n    else\n        z",
    Ok(vec![
        SimpleToken {
            token_type: TokenType::Function,
            line: 0,
            indentation_level: 0,
        },
        SimpleToken {
            token_type: TokenType::If,
            line: 1,
            indentation_level: 1,
        },
        SimpleToken {
            token_type: TokenType::Identifier("x"),
            line: 2,
            indentation_level: 2,
        },
        SimpleToken {
            token_type: TokenType::Identifier("y"),
            line: 3,
            indentation_level: 2,
        },
        SimpleToken {
            token_type: TokenType::Else,
            line: 4,
            indentation_level: 1,
        },
        SimpleToken {
            token_type: TokenType::Identifier("z"),
            line: 5,
            indentation_level: 2,
        },
    ])
)]
// Three levels of nesting
#[case(
    "a\n    b\n        c\n            d",
    Ok(vec![
        SimpleToken {
            token_type: TokenType::Identifier("a"),
            line: 0,
            indentation_level: 0,
        },
        SimpleToken {
            token_type: TokenType::Identifier("b"),
            line: 1,
            indentation_level: 1,
        },
        SimpleToken {
            token_type: TokenType::Identifier("c"),
            line: 2,
            indentation_level: 2,
        },
        SimpleToken {
            token_type: TokenType::Identifier("d"),
            line: 3,
            indentation_level: 3,
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
        SimpleToken {
            token_type: TokenType::Identifier("x"),
            line: 0,
            indentation_level: 0,
        },
        SimpleToken {
            token_type: TokenType::Identifier("y"),
            line: 1,
            indentation_level: 2,
        },
    ])
)]
// Assignment with indentation
#[case(
    "fn\n    x = y",
    Ok(vec![
        SimpleToken {
            token_type: TokenType::Function,
            line: 0,
            indentation_level: 0,
        },
        SimpleToken {
            token_type: TokenType::Identifier("x"),
            line: 1,
            indentation_level: 1,
        },
        SimpleToken {
            token_type: TokenType::Assignment,
            line: 1,
            indentation_level: 1,
        },
        SimpleToken {
            token_type: TokenType::Identifier("y"),
            line: 1,
            indentation_level: 1,
        },
    ])
)]
fn test_l3_tokenization(#[case] source: &str, #[case] expected: Res<Vec<SimpleToken<'static>>>) {
    assert_eq!(
        l3_tokenize(l2_tokenize(l1_tokenize(source)))
            .map(|token| {
                let token = token?;
                Ok(SimpleToken::from(token))
            })
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
