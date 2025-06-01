use std::hash::Hash;

use derive_more::{Display, From};
use rusm64_macros::EqModAddressing;

use super::EqModAddressing;

type Bexpr = Box<Expr>;

#[derive(Clone, Debug, Display, Eq, EqModAddressing, Hash, PartialEq)]
#[display("{}", _0)]
pub enum Expr {
    Rhai(RhaiExpr),
    Lower(LowerExpr),
    Upper(UpperExpr),
    Literal(LiteralExpr),
    Ref(RefExpr),
}

impl From<RhaiExpr> for Expr {
    fn from(value: RhaiExpr) -> Self {
        Expr::Rhai(value)
    }
}

impl From<LowerExpr> for Expr {
    fn from(value: LowerExpr) -> Self {
        Expr::Lower(value)
    }
}

impl From<UpperExpr> for Expr {
    fn from(value: UpperExpr) -> Self {
        Expr::Upper(value)
    }
}

impl From<LiteralExpr> for Expr {
    fn from(value: LiteralExpr) -> Self {
        Expr::Literal(value)
    }
}

impl From<RefExpr> for Expr {
    fn from(value: RefExpr) -> Self {
        Expr::Ref(value)
    }
}

impl Expr {
    pub fn number_literal_str(&self) -> Option<&str> {
        use self::LiteralExpr::*;
        use self::NumberLiteral::*;
        use Expr::*;
        match self {
            Literal(NumberLiteral(BinLiteral(s))) => Some(s),
            Literal(NumberLiteral(DecLiteral(s))) => Some(s),
            Literal(NumberLiteral(HexLiteral(s))) => Some(s),
            _ => None,
        }
    }

    pub fn numeric_value(&self) -> Option<i64> {
        use self::LiteralExpr::*;
        use self::NumberLiteral::*;
        use Expr::*;
        match self {
            Literal(NumberLiteral(BinLiteral(s))) => Some(i64::from_str_radix(s, 2).unwrap()),
            Literal(NumberLiteral(DecLiteral(s))) => Some(i64::from_str_radix(s, 10).unwrap()),
            Literal(NumberLiteral(HexLiteral(s))) => Some(i64::from_str_radix(s, 16).unwrap()),
            Literal(CharLiteral(self::CharLiteral(s))) => {
                Some((s.chars().nth(0).unwrap() as u8).into())
            }
            _ => None,
        }
    }

    pub fn char_literal_str(&self) -> Option<&str> {
        use self::LiteralExpr::*;
        use Expr::*;
        match self {
            Literal(CharLiteral(self::CharLiteral(s))) => Some(s),
            _ => None,
        }
    }

    pub fn references(&self) -> Vec<String> {
        use super::visitors::Visitable;
        let mut vis = super::visitors::RefVisitor::default();
        self.visit(&mut vis);
        vis.refs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_references_visitor() {
        let expr = Expr::Literal(LiteralExpr::CharLiteral(CharLiteral::from("c".to_string())));
        let expected: Vec<String> = vec![];
        assert_eq!(expr.references(), expected);

        let expr = Expr::Ref(RefExpr::LabelRef("start".to_string()));
        let expected: Vec<String> = vec!["start".to_string()];
        assert_eq!(expr.references(), expected);
    }
}

#[derive(Clone, Debug, Display, Eq, EqModAddressing, Hash, PartialEq)]
#[display("{}", _0)]
pub enum LiteralExpr {
    NumberLiteral(NumberLiteral),
    CharLiteral(CharLiteral),
}

impl From<NumberLiteral> for LiteralExpr {
    fn from(value: NumberLiteral) -> Self {
        LiteralExpr::NumberLiteral(value)
    }
}

impl From<CharLiteral> for LiteralExpr {
    fn from(value: CharLiteral) -> Self {
        LiteralExpr::CharLiteral(value)
    }
}

#[derive(Clone, Debug, Display, Eq, EqModAddressing, Hash, PartialEq)]
pub enum NumberLiteral {
    #[display("${}", _0)]
    HexLiteral(String),
    #[display("%{}", _0)]
    BinLiteral(String),
    #[display("{}", _0)]
    DecLiteral(String),
}

#[derive(Clone, Debug, Display, Eq, EqModAddressing, From, Hash, PartialEq)]
#[display("'{}'", _0)]
pub struct CharLiteral(String);

#[derive(Clone, Debug, Display, Eq, EqModAddressing, Hash, PartialEq)]
pub enum RefExpr {
    #[display("{}", _0)]
    LabelRef(String),
    #[display("{}", _0)]
    SymbolRef(String),
}

impl RefExpr {
    pub fn as_str(&self) -> &str {
        match self {
            RefExpr::LabelRef(reference) => reference,
            RefExpr::SymbolRef(reference) => reference,
        }
    }
}

#[derive(Clone, Debug, Display, Eq, EqModAddressing, From, Hash, PartialEq)]
#[display("<{}", _0)]
pub struct LowerExpr(Bexpr);

impl LowerExpr {
    #[inline]
    pub fn expr(&self) -> &Expr {
        &self.0
    }

    #[inline]
    pub fn expr_mut(&mut self) -> &mut Expr {
        &mut self.0
    }
}

#[derive(Clone, Debug, Display, Eq, EqModAddressing, From, Hash, PartialEq)]
#[display(">{}", _0)]
pub struct UpperExpr(Bexpr);

impl UpperExpr {
    #[inline]
    pub fn expr(&self) -> &Expr {
        &self.0
    }

    #[inline]
    pub fn expr_mut(&mut self) -> &mut Expr {
        &mut self.0
    }
}

#[derive(Clone, Debug, Display, Eq, EqModAddressing, Hash, PartialEq, From)]
#[display("{{{{{}}}}}", _0)]
pub struct RhaiExpr(String);

impl RhaiExpr {
    #[inline]
    pub fn rhai(&self) -> &str {
        &self.0
    }

    #[inline]
    pub fn rhai_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
