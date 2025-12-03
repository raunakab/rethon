mod l1_tokenizer;
mod l2_tokenizer;
mod l3_tokenizer;

use thiserror::Error;

use crate::l3_tokenizer::{INDENTATION_SIZE, L3Token, l3_tokenize};

pub type Res<T = ()> = Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("Unknown token: {0}")]
    UnknownToken(String),

    #[error("Unterminated string at byte {0}")]
    UnterminatedString(usize),

    #[error(
        "Invalid indentation at byte {position}: expected multiple of {}, found {found}",
        INDENTATION_SIZE
    )]
    InvalidIndentation { found: usize, position: usize },
}

pub fn tokenize(source: &str) -> impl Iterator<Item = Res<L3Token<'_>>> {
    l3_tokenize(source)
}
