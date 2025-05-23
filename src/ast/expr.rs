use derive_more::{Display, From};

type Bexpr = Box<Expr>;

#[derive(Debug, Display, Eq, From, Hash, PartialEq)]
pub enum Expr {
    Binary(BinaryExpr),
    L(LExpr),
}

#[derive(Debug, Display, Eq, From, Hash, PartialEq)]
pub enum LExpr {
    LiteralExpr(LiteralExpr),
    RefExpr(RefExpr),
    ParenExpr(ParenExpr),
    LowerExpr(LowerExpr),
    UpperExpr(UpperExpr),
}

#[derive(Debug, Display, Eq, From, Hash, PartialEq)]
pub enum LiteralExpr {
    NumberLiteral(NumberLiteral),
    CharLiteral(CharLiteral),
}

#[derive(Debug, Display, Eq, Hash, PartialEq)]
pub enum NumberLiteral {
    HexLiteral(String),
    BinLiteral(String),
    DecLiteral(String),
}

#[derive(Debug, Display, Eq, From, Hash, PartialEq)]
pub struct CharLiteral(String);

#[derive(Debug, Display, Eq, Hash, PartialEq)]
pub enum RefExpr {
    LabelRef(String),
    SymbolRef(String),
}

#[derive(Debug, Display, Eq, From, Hash, PartialEq)]
pub struct ParenExpr(Bexpr);
#[derive(Debug, Display, Eq, From, Hash, PartialEq)]
pub struct LowerExpr(Bexpr);
#[derive(Debug, Display, Eq, From, Hash, PartialEq)]
pub struct UpperExpr(Bexpr);

#[derive(Debug, Display, Eq, From, Hash, PartialEq)]
#[display("{} {} {}", self.0, self.1, self.2)]
pub struct BinaryExpr(Bexpr, BinOp, Bexpr);

#[derive(Debug, Display, Eq, From, Hash, PartialEq)]
pub struct BinOp(String);

#[derive(Default)]
pub struct BinaryExprBuilder {
    lhs: Option<Expr>,
    rhs: Option<Expr>,
    op: Option<BinOp>,
}

impl BinaryExprBuilder {
    pub fn lhs(mut self, lhs: Expr) -> Self {
        self.lhs = Some(lhs);
        self
    }

    pub fn op(mut self, op: BinOp) -> Self {
        self.op = Some(op);
        self
    }

    pub fn rhs(mut self, rhs: Expr) -> Self {
        self.rhs = Some(rhs);
        self
    }

    pub fn build(self) -> BinaryExpr {
        BinaryExpr::from((
            Box::new(self.lhs.expect("cannot build BinaryExpr without lhs")),
            self.op.expect("cannot build BinaryExpr without op"),
            Box::new(self.rhs.expect("cannot build BinaryExpr without rhs")),
        ))
    }
}
