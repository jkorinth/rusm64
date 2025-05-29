use super::AddressingMode;
use super::Opcode;
use super::Operand;
use derive_more::{Display, From};

#[derive(Clone, Debug, Display, Eq, From, Hash, PartialEq)]
#[display("{} {}", _0, _1.as_ref().map(|o| format!("{}", o)).unwrap_or("".to_string()))]
pub struct Op(Opcode, Option<Operand>);

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
