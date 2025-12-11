#[macro_export]
macro_rules! nodes {
    (
        $l:lifetime
        $(,)?
    ) => {
        std::iter::Peekable<impl Iterator<Item = $crate::Res<$crate::types::Node<$l>>>>
    };
    () => {
        std::iter::Peekable<impl Iterator<Item = $crate::Res<$crate::types::Node<'_>>>>
    };
}

mod l1_tokenizer;
mod l2_tokenizer;
mod l3_tokenizer;
mod l4_tokenizer;
pub mod types;

use thiserror::Error;

use crate::{
    l1_tokenizer::l1_tokenize,
    l2_tokenizer::l2_tokenize,
    l3_tokenizer::{INDENTATION_SIZE, l3_tokenize},
    l4_tokenizer::l4_tokenize,
};

pub type Res<T = ()> = Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("Invalid whitespace being used: {0}")]
    InvalidWhitespace(String),

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

pub fn tokenize(source: &str) -> nodes!() {
    l4_tokenize(l3_tokenize(l2_tokenize(l1_tokenize(source)))).peekable()
}
