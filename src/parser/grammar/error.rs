use super::Rule;
use derive_more::FromStrError;
use pest::error::Error as PestError;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Pest error: {0}")]
    Pest(#[from] Box<PestError<Rule>>),

    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),

    #[error("Unknown opcode: {0}")]
    UnknownOpcode(String),

    #[error("Multiple errors: {0:?}")]
    MultipleErrors(Vec<ParseError>),
}

impl From<PestError<Rule>> for ParseError {
    fn from(e: PestError<Rule>) -> Self {
        Self::Pest(Box::new(e))
    }
}

impl From<FromStrError> for ParseError {
    fn from(value: FromStrError) -> Self {
        Self::InvalidSyntax(value.to_string())
    }
}

#[macro_export]
macro_rules! unexpected_rule {
    ($got:expr => $exp:expr) => {
        //panic!("unexpected rule {:?}, expected {}", $got, $exp)
        Err(ParseError::InvalidSyntax(format!(
            "unexpected rule {:?}, expected {}",
            $got, $exp
        )))
    };
}
