use crate::ast::{visitors::Visitor, *};

/// A visitor that validates expression structure
#[derive(Default, Debug)]
pub struct ValidationVisitor {
    pub errors: Vec<String>,
    pub depth: usize,
    pub max_depth: usize,
}

impl ValidationVisitor {
    pub fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            ..Default::default()
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

impl Visitor for ValidationVisitor {
    fn visit_expr(&mut self, _: &Expr) {
        self.depth += 1;
        if self.depth > self.max_depth {
            self.errors.push(format!(
                "Expression depth {} exceeds maximum {}",
                self.depth, self.max_depth
            ));
        }
    }

    fn visit_ref_expr(&mut self, re: &RefExpr) {
        let ref_str = re.as_str();
        if ref_str.is_empty() {
            self.errors.push("Empty reference found".to_string());
        }
        if !ref_str.chars().next().unwrap_or('0').is_alphabetic() && !ref_str.starts_with('_') {
            self.errors
                .push(format!("Invalid reference identifier: {}", ref_str));
        }
    }

    fn visit_number_literal(&mut self, nl: &NumberLiteral) {
        match nl {
            NumberLiteral::HexLiteral(s) => {
                if !s.starts_with('$') {
                    self.errors
                        .push(format!("Invalid hex literal format: {}", s));
                }
            }
            NumberLiteral::BinLiteral(s) => {
                if !s.starts_with('%') {
                    self.errors
                        .push(format!("Invalid binary literal format: {}", s));
                }
            }
            NumberLiteral::DecLiteral(s) => {
                if s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()) {
                    self.errors
                        .push(format!("Invalid decimal literal format: {}", s));
                }
            }
        }
    }
}
