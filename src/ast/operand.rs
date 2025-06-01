use super::EqModAddressing;
use super::{AddressingMode, Expr};
use derive_more::From;
use rusm64_macros::EqModAddressing;

#[derive(Clone, Debug, Eq, EqModAddressing, From, Hash, PartialEq)]
pub struct Operand(AddressingMode, Expr);

impl Operand {
    #[inline]
    pub fn addressing_mode(&self) -> AddressingMode {
        self.0
    }

    #[inline]
    pub fn expr(&self) -> &Expr {
        &self.1
    }

    #[inline]
    pub fn expr_mut(&mut self) -> &mut Expr {
        &mut self.1
    }
}

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

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AddressingMode::*;
        let m = match self.addressing_mode() {
            Implied | Accumulator => format!("{}", self.expr()),
            Immediate => format!("#{}", self.expr()),
            Indirect => format!("({})", self.expr()),
            Absolute | ZeroPage | Relative => format!("{}", self.expr()),
            AbsoluteX | ZeroPageX => format!("{}, x", self.expr()),
            AbsoluteY | ZeroPageY => format!("{}, y", self.expr()),
            IndexedIndirect => format!("({}, x)", self.expr()),
            IndirectIndexed => format!("({}), y", self.expr()),
        };
        f.write_str(&m)
    }
}
