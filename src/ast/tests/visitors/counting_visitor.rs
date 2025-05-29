use crate::ast::{visitors::Visitor, *};

/// A comprehensive visitor that counts all types of nodes visited.
#[derive(Default, Debug)]
pub struct CountingVisitor {
    pub expr_count: usize,
    pub rhai_count: usize,
    pub literal_count: usize,
    pub ref_count: usize,
    pub lower_count: usize,
    pub upper_count: usize,
    pub number_literal_count: usize,
    pub char_literal_count: usize,
    pub visited_refs: Vec<String>,
}

impl Visitor for CountingVisitor {
    fn visit_expr(&mut self, _: &Expr) {
        self.expr_count += 1;
    }

    fn visit_literal_expr(&mut self, _: &LiteralExpr) {
        self.literal_count += 1;
    }

    fn visit_ref_expr(&mut self, re: &RefExpr) {
        self.ref_count += 1;
        self.visited_refs.push(re.as_str().to_string());
    }

    fn visit_lower_expr(&mut self, _: &LowerExpr) {
        self.lower_count += 1;
    }

    fn visit_upper_expr(&mut self, _: &UpperExpr) {
        self.upper_count += 1;
    }

    fn visit_rhai_expr(&mut self, _: &RhaiExpr) {
        self.rhai_count += 1;
    }

    fn visit_number_literal(&mut self, _: &NumberLiteral) {
        self.number_literal_count += 1;
    }

    fn visit_char_literal(&mut self, _: &CharLiteral) {
        self.char_literal_count += 1;
    }
}
