use crate::ast::{visitors::Visitor, *};

/// A visitor that collects all numeric values from expressions
#[derive(Default, Debug)]
pub struct NumericCollector {
    pub hex_literals: Vec<String>,
    pub bin_literals: Vec<String>,
    pub dec_literals: Vec<String>,
}

impl Visitor for NumericCollector {
    fn visit_number_literal(&mut self, nl: &NumberLiteral) {
        match nl {
            NumberLiteral::HexLiteral(s) => self.hex_literals.push(s.clone()),
            NumberLiteral::BinLiteral(s) => self.bin_literals.push(s.clone()),
            NumberLiteral::DecLiteral(s) => self.dec_literals.push(s.clone()),
        }
    }
}
