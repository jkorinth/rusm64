use crate::ast::{
    visitors::{Visitor, VisitorFn},
    *,
};

#[derive(Default)]
pub struct RefVisitor {
    refs: Vec<String>,
}

impl RefVisitor {
    pub fn refs(self) -> Vec<String> {
        self.refs
    }
}

impl Visitor for RefVisitor {
    fn visit_ref_expr(&mut self, re: &RefExpr) {
        self.refs.push(re.as_str().to_string())
    }
}

#[derive(Default)]
pub struct RefVisitorFn;

impl VisitorFn<Vec<String>> for RefVisitorFn {
    fn visit_ref_expr_fn(&mut self, refex: &RefExpr, r: Vec<String>) -> Vec<String> {
        let mut r = r.clone();
        r.push(refex.as_str().into());
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        tests::strategies::*,
        visitors::{Visitable, VisitableFn},
    };
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn refvisitor_variants_agree(ast in ast_strategy()) {
            let mut r = RefVisitor::default();
            ast.visit(&mut r);
            let mut rf = RefVisitorFn;
            let c = ast.visit_fn(&mut rf, Vec::<String>::new());
            assert_eq!(r.refs, c);
        }
    }
}
