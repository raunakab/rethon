#[cfg(test)]
mod tests;

use std::iter::Peekable;

use crate::{
    Res,
    types::{Brace, Node, Token},
};

pub(crate) fn l4_tokenize<'a>(
    iter: impl Iterator<Item = Res<Token<'a>>>,
) -> impl Iterator<Item = Res<Node<'a>>> {
    L4Tokenizer {
        iter: iter.peekable(),
        indent_stack: vec![0],
        pending_scope_ends: 0,
    }
}

struct L4Tokenizer<'a, I>
where
    I: Iterator<Item = Res<Token<'a>>>,
{
    iter: Peekable<I>,
    indent_stack: Vec<usize>,
    pending_scope_ends: usize,
}

impl<'a, I> Iterator for L4Tokenizer<'a, I>
where
    I: Iterator<Item = Res<Token<'a>>>,
{
    type Item = Res<Node<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! build_scope_end {
            (
                $cond:expr
                $(,)?
            ) => {
                while $cond {
                    self.indent_stack.pop();
                    self.pending_scope_ends += 1;
                }
            };
        }

        macro_rules! emit_scope_end {
            () => {
                if self.pending_scope_ends > 0 {
                    self.pending_scope_ends -= 1;
                    return Some(Ok(Node::ScopeEnd));
                }
            };
        }

        // First, emit any pending scope ends
        emit_scope_end!();

        // Peek at the next token to check indentation
        let next_token = match self.iter.peek() {
            Some(Ok(token)) => token,
            Some(Err(_)) => return Some(Err(self.iter.next().unwrap().unwrap_err())),
            None => {
                // End of stream - close all open scopes
                build_scope_end!(self.indent_stack.len() > 1);
                emit_scope_end!();
                return None;
            }
        };

        let &current_indent = self.indent_stack.last().unwrap();
        let next_indent = next_token.indentation_level;

        // A new indentation begins
        if next_indent > current_indent {
            // New scope opening
            self.indent_stack.push(next_indent);
            return Some(Ok(Node::ScopeStart {
                brace: Some(Brace::Whitespace),
            }));
        }

        // The previous indentation closes
        if next_indent < current_indent {
            build_scope_end!(
                self.indent_stack.len() > 1 && *self.indent_stack.last().unwrap() > next_indent
            );
            emit_scope_end!();
        }

        Some(Ok(Node::Token(self.iter.next().unwrap().unwrap())))

        // Consume and return the token
        // match self.iter.next() {
        //     Some(Ok(token)) => Some(Ok(Node::Token(token))),
        //     Some(Err(err)) => Some(Err(err)),
        //     None => {
        //         // Close remaining scopes
        //         build_scope_end!(self.indent_stack.len() > 1);
        //         emit_scope_end!();
        //         None
        //     }
        // }
    }
}
