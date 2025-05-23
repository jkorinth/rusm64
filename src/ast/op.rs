use super::Opcode;
use super::Operand;
use derive_more::{Display, From};

#[derive(Debug, Display, Eq, From, Hash, PartialEq)]
#[display("{} {:?}", self.0, self.1)]
pub struct Op(Opcode, Option<Operand>);

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
