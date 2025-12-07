use crate::{
    Res,
    l4_tokenizer::l4_tokenize,
    types::{Node, TokenType},
};

// Simplified node type for easier testing (strips ranges and source positions)
#[derive(Clone, Debug, PartialEq, Eq)]
enum SimpleNode<'a> {
    Token(TokenType<'a>, usize), // (token_type, indentation_level)
    Scope(Vec<SimpleNode<'a>>),
}

fn simplify_node(node: Node<'_>) -> SimpleNode<'_> {
    match node {
        Node::Token(token) => SimpleNode::Token(token.token_type, token.indentation_level),
        Node::Scope(scope) => {
            SimpleNode::Scope(scope.nodes.into_iter().map(simplify_node).collect())
        }
    }
}

#[rstest::rstest]
// Empty input
#[case("", Ok(SimpleNode::Scope(vec![])))]
// Single token, no indentation
#[case(
    "fn",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Function, 0),
    ]))
)]
// Multiple tokens on single line, no indentation
#[case(
    "fn add",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::Token(TokenType::Identifier("add"), 0),
    ]))
)]
// Multiple lines, no indentation
#[case(
    "fn\nreturn",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::Token(TokenType::Return, 0),
    ]))
)]
// Single level indentation
#[case(
    "fn\n    add",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::Scope(vec![
            SimpleNode::Token(TokenType::Identifier("add"), 1),
        ]),
    ]))
)]
// Single level indentation with multiple tokens
#[case(
    "fn\n    x = y",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::Scope(vec![
            SimpleNode::Token(TokenType::Identifier("x"), 1),
            SimpleNode::Token(TokenType::Assignment, 1),
            SimpleNode::Token(TokenType::Identifier("y"), 1),
        ]),
    ]))
)]
// Nested indentation (2 levels)
#[case(
    "fn\n    if\n        x",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::Scope(vec![
            SimpleNode::Token(TokenType::If, 1),
            SimpleNode::Scope(vec![
                SimpleNode::Token(TokenType::Identifier("x"), 2),
            ]),
        ]),
    ]))
)]
// Indentation reset after newline
#[case(
    "fn\n    x\ny",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::Scope(vec![
            SimpleNode::Token(TokenType::Identifier("x"), 1),
        ]),
        SimpleNode::Token(TokenType::Identifier("y"), 0),
    ]))
)]
// Multiple indented sections at same level
#[case(
    "a\n    x\n    y",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Identifier("a"), 0),
        SimpleNode::Scope(vec![
            SimpleNode::Token(TokenType::Identifier("x"), 1),
            SimpleNode::Token(TokenType::Identifier("y"), 1),
        ]),
    ]))
)]
// Complex nested structure
#[case(
    "fn\n    if\n        x\n        y\n    else\n        z",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::Scope(vec![
            SimpleNode::Token(TokenType::If, 1),
            SimpleNode::Scope(vec![
                SimpleNode::Token(TokenType::Identifier("x"), 2),
                SimpleNode::Token(TokenType::Identifier("y"), 2),
            ]),
            SimpleNode::Token(TokenType::Else, 1),
            SimpleNode::Scope(vec![
                SimpleNode::Token(TokenType::Identifier("z"), 2),
            ]),
        ]),
    ]))
)]
// Three levels of nesting
#[case(
    "a\n    b\n        c\n            d",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Identifier("a"), 0),
        SimpleNode::Scope(vec![
            SimpleNode::Token(TokenType::Identifier("b"), 1),
            SimpleNode::Scope(vec![
                SimpleNode::Token(TokenType::Identifier("c"), 2),
                SimpleNode::Scope(vec![
                    SimpleNode::Token(TokenType::Identifier("d"), 3),
                ]),
            ]),
        ]),
    ]))
)]
// Indentation then return to base level
#[case(
    "a\n    b\nc",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Identifier("a"), 0),
        SimpleNode::Scope(vec![
            SimpleNode::Token(TokenType::Identifier("b"), 1),
        ]),
        SimpleNode::Token(TokenType::Identifier("c"), 0),
    ]))
)]
// Multiple scopes at base level
#[case(
    "a\n    b\nc\n    d",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Identifier("a"), 0),
        SimpleNode::Scope(vec![
            SimpleNode::Token(TokenType::Identifier("b"), 1),
        ]),
        SimpleNode::Token(TokenType::Identifier("c"), 0),
        SimpleNode::Scope(vec![
            SimpleNode::Token(TokenType::Identifier("d"), 1),
        ]),
    ]))
)]
// Realistic function example
#[case(
    "fn add\n    return x",
    Ok(SimpleNode::Scope(vec![
        SimpleNode::Token(TokenType::Function, 0),
        SimpleNode::Token(TokenType::Identifier("add"), 0),
        SimpleNode::Scope(vec![
            SimpleNode::Token(TokenType::Return, 1),
            SimpleNode::Token(TokenType::Identifier("x"), 1),
        ]),
    ]))
)]
fn test_l4_tokenization(#[case] source: &str, #[case] expected: Res<SimpleNode<'static>>) {
    let result = l4_tokenize(source).map(simplify_node);
    assert_eq!(result, expected);
}
