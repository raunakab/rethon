#![doc = include_str!("README.md")]

#[cfg(test)]
mod tests;

use std::iter::Peekable;

use crate::{Res, Token, l3_tokenizer::L3Token};

pub(crate) fn l4_tokenize<'a>(
    iter: impl Iterator<Item = Res<L3Token<'a>>>,
) -> impl Iterator<Item = Res<Token<'a>>> {
    L4Tokenizer {
        iter: iter.peekable(),
        indent_stack: vec![0],
        pending_scope_ends: 0,
        pending_scope_starts: 0,
    }
}

struct L4Tokenizer<'a, I>
where
    I: Iterator<Item = Res<L3Token<'a>>>,
{
    iter: Peekable<I>,
    indent_stack: Vec<usize>,
    pending_scope_ends: usize,
    pending_scope_starts: usize,
}

impl<'a, I> Iterator for L4Tokenizer<'a, I>
where
    I: Iterator<Item = Res<L3Token<'a>>>,
{
    type Item = Res<Token<'a>>;

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
                    return Some(Ok(Token::ScopeEnd(None)));
                }
            };
        }

        macro_rules! emit_scope_start {
            () => {
                if self.pending_scope_starts > 0 {
                    self.pending_scope_starts -= 1;
                    return Some(Ok(Token::ScopeStart(None)));
                }
            };
        }

        // First, emit any pending scope ends
        emit_scope_end!();

        // Then, emit any pending scope starts
        emit_scope_start!();

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
        let next_indent = next_token.position.indentation_level;

        // A new indentation begins
        if next_indent > current_indent {
            // Calculate how many levels we're jumping
            let indent_jump = next_indent - current_indent;

            // Push all intermediate indentation levels onto the stack
            for level in 1..=indent_jump {
                self.indent_stack.push(current_indent + level);
            }

            // Queue up scope starts for each level (will be emitted one per iteration)
            self.pending_scope_starts = indent_jump;
            emit_scope_start!();
        }

        // The previous indentation closes
        if next_indent < current_indent {
            build_scope_end!(
                self.indent_stack.len() > 1 && *self.indent_stack.last().unwrap() > next_indent
            );
            emit_scope_end!();
        }

        let l3_token = self.iter.next().unwrap().unwrap();
        Some(Ok(Token::Token(l3_token.token_type, l3_token.position)))
    }
}
