use crate::ast::{AddressingMode, Expr, Op, Opcode};
use std::num::ParseIntError;

use super::SourceLocation;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum AssembleError {
    #[error("not implemented yet")]
    Problem,

    #[error("invalid numeric literal found: {0}")]
    InvalidLiteral(ParseIntError),

    #[error("cannot evaluate expression: {0}")]
    CannotEvaluateExpr(Expr),

    #[error("cannot evaluate literal: {0}")]
    CannotEvaluateLiteral(String),

    #[error("duplicate label found: {0}")]
    DuplicateLabel(String),

    #[error("invalid instruction found: {0}")]
    InvalidInstruction(Op),

    #[error("unresolved symbol found: {0}")]
    UnresolvedSymbol(String),

    #[error("invalid addressing mode {1} for opcode {0}")]
    InvalidAddressingMode(Opcode, AddressingMode),

    #[error("cannot use operand value {1} with addressing_mode {0}")]
    OperandTooLarge(AddressingMode, i64),

    #[error("rhai script error: {0}")]
    RhaiError(String),

    #[error("multiple errors:\n{0}")]
    MultipleErrors(#[from] AssembleErrorList),
}

impl From<Vec<(SourceLocation, AssembleError)>> for AssembleError {
    fn from(value: Vec<(SourceLocation, AssembleError)>) -> Self {
        Self::MultipleErrors(value.into())
    }
}

#[derive(Clone, Debug, derive_more::Deref, Eq, PartialEq, thiserror::Error)]
pub struct AssembleErrorList(Vec<(SourceLocation, AssembleError)>);

impl From<Vec<(SourceLocation, AssembleError)>> for AssembleErrorList {
    fn from(value: Vec<(SourceLocation, AssembleError)>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for AssembleErrorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("f")
    }
}

impl From<ParseIntError> for AssembleError {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidLiteral(value)
    }
}
