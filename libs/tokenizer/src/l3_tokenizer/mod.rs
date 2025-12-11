#[cfg(test)]
mod tests;

use std::iter::Peekable;

use crate::{
    Error, Res,
    l2_tokenizer::{L2Token, L2TokenType},
    types::Token,
};

pub(crate) const INDENTATION_SIZE: usize = 4;

pub(crate) fn l3_tokenize<'a>(
    iter: impl Iterator<Item = Res<L2Token<'a>>>,
) -> impl Iterator<Item = Res<Token<'a>>> {
    L3Tokenizer {
        iter: iter.peekable(),
        line: 0,
        line_position: 0,
        indentation_level: 0,
        after_newline: true,
    }
}

#[derive(Debug, Clone)]
struct L3Tokenizer<'a, I>
where
    I: Iterator<Item = Res<L2Token<'a>>>,
{
    iter: Peekable<I>,
    line: usize,
    line_position: usize,
    indentation_level: usize,
    after_newline: bool,
}

impl<'a, I> Iterator for L3Tokenizer<'a, I>
where
    I: Iterator<Item = Res<L2Token<'a>>>,
{
    type Item = Res<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let l2_token = match self.iter.next()? {
            Ok(l2_token) => l2_token,
            Err(error) => return Some(Err(error)),
        };

        // Handle newline: increment line counter and reset state
        if matches!(l2_token.token_type, L2TokenType::Newline) {
            self.line += 1;
            self.line_position = 0;
            self.indentation_level = 0;
            self.after_newline = true;

            // Skip newline tokens - don't emit them
            return self.next();
        }

        // Handle whitespace after newline for indentation
        if self.after_newline {
            self.after_newline = false;

            if let L2TokenType::Whitespace(count) = l2_token.token_type {
                if count % INDENTATION_SIZE != 0 {
                    return Some(Err(Error::InvalidIndentation {
                        found: count,
                        position: l2_token.range.start,
                    }));
                }

                self.indentation_level = count / INDENTATION_SIZE;
                self.line_position += count;

                // Skip indentation whitespace - don't emit it
                return self.next();
            }
        }

        // Regular token processing
        let token_length = l2_token.range.end - l2_token.range.start;

        // Extract the actual TokenType from L2TokenType
        let token_type = match l2_token.token_type {
            L2TokenType::Normal(tt) => tt,
            L2TokenType::Whitespace(_) | L2TokenType::Newline | L2TokenType::Brace(_, _) => {
                // These should have been handled above, but if we get here, skip them
                self.line_position += token_length;
                return self.next();
            }
        };

        let result = Token {
            token_type,
            source_range: l2_token.range,
            line: self.line,
            line_range: self.line_position..self.line_position + token_length,
            indentation_level: self.indentation_level,
        };
        self.line_position += token_length;
        Some(Ok(result))
    }
}
