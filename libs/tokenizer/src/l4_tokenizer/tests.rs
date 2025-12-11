use crate::{
    Res,
    l4_tokenizer::l4_tokenize,
    types::{Brace, Node, TokenType},
};

// Simplified node type for easier testing (strips ranges and source positions)
#[derive(Clone, Debug, PartialEq, Eq)]
enum SimpleNode<'a> {
    Token(TokenType<'a>, usize), // (token_type, indentation_level)
    ScopeStart { brace: Option<Brace> },
    ScopeEnd,
}

fn simplify_node(node: Node<'_>) -> SimpleNode<'_> {
    match node {
        Node::Token(token) => SimpleNode::Token(token.token_type, token.indentation_level),
        Node::ScopeStart { brace } => SimpleNode::ScopeStart { brace },
        Node::ScopeEnd => SimpleNode::ScopeEnd,
    }
}

#[rstest::rstest]
// Empty input
#[case("", Ok(vec![]))]
// Single token, no indentation
#[case(
    "fn",
    Ok(vec![
        SimpleNode::Token(TokenType::Function, 0),
    ])
)]
// Multiple tokens on single line, no indentation
#[case(
    "fn add",
    Ok(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::Token(TokenType::Identifier("add"), 0),
    ])
)]
// Multiple lines, no indentation
#[case(
    "fn\nreturn",
    Ok(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::Token(TokenType::Return, 0),
    ])
)]
// Single level indentation
#[case(
    "fn\n    add",
    Ok(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("add"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
// Single level indentation with multiple tokens
#[case(
    "fn\n    x = y",
    Ok(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("x"), 1),
        SimpleNode::Token(TokenType::Assignment, 1),
        SimpleNode::Token(TokenType::Identifier("y"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
// Nested indentation (2 levels)
#[case(
    "fn\n    if\n        x",
    Ok(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::If, 1),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("x"), 2),
        SimpleNode::ScopeEnd,
        SimpleNode::ScopeEnd,
    ])
)]
// Indentation reset after newline
#[case(
    "fn\n    x\ny",
    Ok(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("x"), 1),
        SimpleNode::ScopeEnd,
        SimpleNode::Token(TokenType::Identifier("y"), 0),
    ])
)]
// Multiple indented sections at same level
#[case(
    "a\n    x\n    y",
    Ok(vec![
        SimpleNode::Token(TokenType::Identifier("a"), 0),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("x"), 1),
        SimpleNode::Token(TokenType::Identifier("y"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
// Complex nested structure
#[case(
    "fn\n    if\n        x\n        y\n    else\n        z",
    Ok(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::If, 1),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("x"), 2),
        SimpleNode::Token(TokenType::Identifier("y"), 2),
        SimpleNode::ScopeEnd,
        SimpleNode::Token(TokenType::Else, 1),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("z"), 2),
        SimpleNode::ScopeEnd,
        SimpleNode::ScopeEnd,
    ])
)]
// Three levels of nesting
#[case(
    "a\n    b\n        c\n            d",
    Ok(vec![
        SimpleNode::Token(TokenType::Identifier("a"), 0),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("b"), 1),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("c"), 2),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("d"), 3),
        SimpleNode::ScopeEnd,
        SimpleNode::ScopeEnd,
        SimpleNode::ScopeEnd,
    ])
)]
// Indentation then return to base level
#[case(
    "a\n    b\nc",
    Ok(vec![
        SimpleNode::Token(TokenType::Identifier("a"), 0),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("b"), 1),
        SimpleNode::ScopeEnd,
        SimpleNode::Token(TokenType::Identifier("c"), 0),
    ])
)]
// Multiple scopes at base level
#[case(
    "a\n    b\nc\n    d",
    Ok(vec![
        SimpleNode::Token(TokenType::Identifier("a"), 0),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("b"), 1),
        SimpleNode::ScopeEnd,
        SimpleNode::Token(TokenType::Identifier("c"), 0),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Identifier("d"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
// Realistic function example
#[case(
    "fn add\n    return x",
    Ok(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::Token(TokenType::Identifier("add"), 0),
        SimpleNode::ScopeStart { brace: Some(Brace::Whitespace) },
        SimpleNode::Token(TokenType::Return, 1),
        SimpleNode::Token(TokenType::Identifier("x"), 1),
        SimpleNode::ScopeEnd,
    ])
)]
fn test_l4_tokenization(#[case] source: &str, #[case] expected: Res<Vec<SimpleNode<'static>>>) {
    let result: Res<Vec<_>> = l4_tokenize(source)
        .map(|res| res.map(simplify_node))
        .collect();
    assert_eq!(result, expected);
}
