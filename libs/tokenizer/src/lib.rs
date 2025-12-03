mod l1_tokenizer;
mod l2_tokenizer;
mod l3_tokenizer;

use thiserror::Error;

pub type Res<T = ()> = Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("Unknown token: {0}")]
    UnknownToken(String),

    #[error("Unterminated string at byte {0}")]
    UnterminatedString(usize),
}

pub fn tokenize(source: &str) -> impl Iterator<Item = Res<crate::l2_tokenizer::L2Token<'_>>> {
    crate::l2_tokenizer::l2_tokenize(source)
}
