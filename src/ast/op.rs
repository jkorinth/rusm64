use super::AddressingMode;
use super::Opcode;
use super::Operand;
use crate::EqModAddressing;
use derive_more::{Display, From};
use rusm64_macros::EqModAddressing;

#[derive(Clone, Debug, Display, Eq, EqModAddressing, From, Hash)]
#[display("{} {}", _0, _1.as_ref().map(|o| format!("{}", o)).unwrap_or("".to_string()))]
pub struct Op(pub Opcode, pub Option<Operand>);

impl Op {
    #[inline]
    pub fn opcode(&self) -> Opcode {
        self.0
    }

    #[inline]
    pub fn opcode_mut(&mut self) -> &mut Opcode {
        &mut self.0
    }

    #[inline]
    pub fn operand(&self) -> &Option<Operand> {
        &self.1
    }

    #[inline]
    pub fn operand_mut(&mut self) -> &mut Option<Operand> {
        &mut self.1
    }

    pub fn as_opaddr(&self) -> (Opcode, AddressingMode) {
        (
            self.0,
            self.1
                .clone()
                .map(|o| o.addressing_mode())
                .unwrap_or_else(|| AddressingMode::Implied),
        )
    }
}

impl PartialEq for Op {
    fn eq(&self, other: &Self) -> bool {
        self.opcode() == other.opcode()
            && self.operand().is_some() == other.operand().is_some()
            && self
                .operand()
                .as_ref()
                .map(|o| {
                    o.addressing_mode()
                        .eq_mod_addressing(&other.operand().as_ref().unwrap().addressing_mode())
                })
                .unwrap_or(true)
    }
}

#[derive(Default)]
pub struct OpBuilder {
    opcode: Option<Opcode>,
    operand: Option<Operand>,
}

impl OpBuilder {
    pub fn opcode(mut self, opcode: Opcode) -> Self {
        self.opcode = Some(opcode);
        self
    }

    pub fn operand(mut self, operand: Operand) -> Self {
        self.operand = Some(operand);
        self
    }

    pub fn build(self) -> Op {
        Op(
            self.opcode.expect("cannot build Op without Opcode"),
            self.operand,
        )
    }
}
