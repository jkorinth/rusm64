use super::{AddressingMode, Expr};
use derive_more::{Display, From};

#[derive(Debug, Display, Eq, From, Hash, PartialEq)]
#[display("{} {}", self.0, self.1)]
pub struct Operand(AddressingMode, Expr);

#[derive(Default)]
pub struct OperandBuilder {
    addrmode: Option<AddressingMode>,
    expr: Option<Expr>,
}

impl OperandBuilder {
    pub fn addressing_mode(mut self, addrmode: AddressingMode) -> Self {
        self.addrmode = Some(addrmode);
        self
    }

    pub fn expr(mut self, expr: Expr) -> Self {
        self.expr = Some(expr);
        self
    }

    pub fn build(self) -> Operand {
        Operand(
            self.addrmode
                .expect("cannot build operand without addressing mode"),
            self.expr.expect("cannot build operand without expr"),
        )
    }
}
