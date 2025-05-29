use derive_more::From;

use super::assembler::{AssembleError, AssembleErrorList};
use super::parser::grammar::ParseError;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Parser error: {0}")]
    Parse(#[from] ParseError),

    #[error("Assembler error: {0}")]
    Assembler(#[from] AssembleError),

    #[error("multiple errors:\n{0}")]
    MultipleErrors(#[from] ErrorList),
}

#[derive(Clone, Debug, Eq, From, PartialEq, thiserror::Error)]
pub struct ErrorList(Vec<Error>);

impl std::fmt::Display for ErrorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("f")
    }
}

impl From<AssembleErrorList> for ErrorList {
    fn from(value: AssembleErrorList) -> Self {
        Self(
            value
                .iter()
                .map(|(_, e)| Error::Assembler(e.clone()))
                .collect::<Vec<_>>(),
        )
    }
}

pub type Result<T> = std::result::Result<T, Error>;
