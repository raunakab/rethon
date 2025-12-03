#[cfg(test)]
mod tests;

use std::iter::Peekable;

use crate::{
    Res,
    l3_tokenizer::l3_tokenize,
    types::{Brace, Node, Scope, Token},
};

pub(crate) fn l4_tokenize(source: &str) -> Res<Node<'_>> {
    let mut iter = l3_tokenize(source).peekable();
    let nodes = parse_scope(&mut iter, 0)?;

    Ok(Node::Scope(Scope { brace: None, nodes }))
}

fn parse_scope<'a, I>(iter: &mut Peekable<I>, current_indent: usize) -> Res<Vec<Node<'a>>>
where
    I: Iterator<Item = Res<Token<'a>>>,
{
    let mut nodes = Vec::new();

    while let Some(token_result) = iter.peek() {
        let token = match token_result {
            Ok(token) => token,
            Err(_) => return Err(iter.next().unwrap().unwrap_err()),
        };

        // If we encounter a token with lower indentation, return (end of current scope)
        if token.indentation_level < current_indent {
            break;
        }

        // If we encounter a token with higher indentation, this shouldn't happen
        // (we should only see current_indent or higher within parse_scope)
        if token.indentation_level > current_indent {
            break;
        }

        // Consume the token
        let token = iter.next().unwrap()?;

        nodes.push(Node::Token(token));

        // Check if the next token has higher indentation (child scope)
        let next_indent = iter
            .peek()
            .and_then(|res| res.as_ref().ok().map(|token| token.indentation_level));

        if let Some(next_indent) = next_indent {
            if next_indent > current_indent {
                let child_nodes = parse_scope(iter, next_indent)?;
                nodes.push(Node::Scope(Scope {
                    brace: Some(Brace::Whitespace),
                    nodes: child_nodes,
                }));
            }
        }
    }

    Ok(nodes)
}
