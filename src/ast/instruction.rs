use super::EqModAddressing;
use derive_more::Display;
use rusm64_macros::EqModAddressing;

#[derive(Clone, Debug, Display, Eq, EqModAddressing, Hash, PartialEq)]
pub enum Instruction {
    Directive(super::Directive),
    Op(super::Op),
}

impl From<super::Op> for Instruction {
    fn from(value: super::Op) -> Self {
        Self::Op(value)
    }
}

impl From<super::Directive> for Instruction {
    fn from(value: super::Directive) -> Self {
        Self::Directive(value)
    }
}
