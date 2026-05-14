use lexer::lex;

use crate::{Brace, Error, Res, ScopeItem, Token, s1_whitespace_stripper::strip};

use super::scope;

// Simplified node type for easier testing (strips ranges and source positions)
#[derive(Clone, Debug, PartialEq, Eq)]
enum SimpleNode<'a> {
    Token(Token<'a>, usize),
    ScopeStart(Option<Brace>),
    ScopeEnd,
}

fn simplify_node(node: ScopeItem<'_>) -> SimpleNode<'_> {
    match node {
        ScopeItem::Token(token_type, position) => {
            SimpleNode::Token(token_type, position.indentation_level)
        }
        ScopeItem::ScopeStart(brace_opt) => {
            SimpleNode::ScopeStart(brace_opt.map(|(brace, _)| brace))
        }
        ScopeItem::ScopeEnd(_) => SimpleNode::ScopeEnd,
    }
}

#[rstest::rstest]
// Empty input
#[case("", Ok(vec![]))]
// Single token, no indentation
#[case(
    "fn",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
    ])
)]
// Multiple tokens on single line, no indentation
#[case(
    "fn add",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
        SimpleNode::Token(Token::Identifier("add"), 0),
    ])
)]
// Multiple lines, no indentation
#[case(
    "fn\nreturn",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
        SimpleNode::Token(Token::Return, 0),
    ])
)]
// Single level indentation
#[case(
    "fn\n    add",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("add"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
// Single level indentation with multiple tokens
#[case(
    "fn\n    x = y",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("x"), 1),
        SimpleNode::Token(Token::Assignment, 1),
        SimpleNode::Token(Token::Identifier("y"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
// Nested indentation (2 levels)
#[case(
    "fn\n    if\n        x",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::If, 1),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("x"), 2),
        SimpleNode::ScopeEnd,
        SimpleNode::ScopeEnd,
    ])
)]
// Indentation reset after newline
#[case(
    "fn\n    x\ny",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("x"), 1),
        SimpleNode::ScopeEnd,
        SimpleNode::Token(Token::Identifier("y"), 0),
    ])
)]
// Multiple indented sections at same level
#[case(
    "a\n    x\n    y",
    Ok(vec![
        SimpleNode::Token(Token::Identifier("a"), 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("x"), 1),
        SimpleNode::Token(Token::Identifier("y"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
// Complex nested structure
#[case(
    "fn\n    if\n        x\n        y\n    else\n        z",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::If, 1),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("x"), 2),
        SimpleNode::Token(Token::Identifier("y"), 2),
        SimpleNode::ScopeEnd,
        SimpleNode::Token(Token::Else, 1),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("z"), 2),
        SimpleNode::ScopeEnd,
        SimpleNode::ScopeEnd,
    ])
)]
// Three levels of nesting
#[case(
    "a\n    b\n        c\n            d",
    Ok(vec![
        SimpleNode::Token(Token::Identifier("a"), 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("b"), 1),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("c"), 2),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("d"), 3),
        SimpleNode::ScopeEnd,
        SimpleNode::ScopeEnd,
        SimpleNode::ScopeEnd,
    ])
)]
// Indentation then return to base level
#[case(
    "a\n    b\nc",
    Ok(vec![
        SimpleNode::Token(Token::Identifier("a"), 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("b"), 1),
        SimpleNode::ScopeEnd,
        SimpleNode::Token(Token::Identifier("c"), 0),
    ])
)]
// Multiple scopes at base level
#[case(
    "a\n    b\nc\n    d",
    Ok(vec![
        SimpleNode::Token(Token::Identifier("a"), 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("b"), 1),
        SimpleNode::ScopeEnd,
        SimpleNode::Token(Token::Identifier("c"), 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("d"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
// Realistic function example
#[case(
    "fn add\n    return x",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
        SimpleNode::Token(Token::Identifier("add"), 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Return, 1),
        SimpleNode::Token(Token::Identifier("x"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
#[case(
    "fn add\n\n    return x",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
        SimpleNode::Token(Token::Identifier("add"), 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Return, 1),
        SimpleNode::Token(Token::Identifier("x"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
// File beginning with indented content — ScopeStart emitted before the first token
#[case(
    "    fn",
    Ok(vec![
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Function, 1),
        SimpleNode::ScopeEnd,
    ])
)]
// CRLF line endings work identically to LF through the full pipeline
#[case(
    "fn\r\n    add",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("add"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
// Multiple consecutive blank lines — scoping unaffected
#[case(
    "fn\n\n\n    add",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("add"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
// Multi-level indentation drop (level 2 → 0 in one step)
#[case(
    "a\n    b\n        c\na",
    Ok(vec![
        SimpleNode::Token(Token::Identifier("a"), 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("b"), 1),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Identifier("c"), 2),
        SimpleNode::ScopeEnd,
        SimpleNode::ScopeEnd,
        SimpleNode::Token(Token::Identifier("a"), 0),
    ])
)]
// Error propagation from s1 (invalid indentation reaches s2 unchanged)
#[case(
    "a\n   b",
    Err(Error::InvalidIndentation { found: 3, position: 2 })
)]
// Jump two indentation levels at once
#[case(
    "fn add\n        return x",
    Ok(vec![
        SimpleNode::Token(Token::Function, 0),
        SimpleNode::Token(Token::Identifier("add"), 0),
        SimpleNode::ScopeStart(None),
        SimpleNode::ScopeStart(None),
        SimpleNode::Token(Token::Return, 2),
        SimpleNode::Token(Token::Identifier("x"), 2),
        SimpleNode::ScopeEnd,
        SimpleNode::ScopeEnd,
    ])
)]
fn test_s2_scope(#[case] source: &str, #[case] expected: Res<Vec<SimpleNode<'static>>>) {
    assert_eq!(
        scope(strip(lex(source)))
            .map(|res| res.map(simplify_node))
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
