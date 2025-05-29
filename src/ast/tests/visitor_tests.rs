use super::visitors::*;
use crate::ast::{visitors::*, *};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_counting_visitor_counts(expr in super::expr_strategy()) {
        let mut visitor = CountingVisitor::default();
        expr.visit(&mut visitor);

        // Property: ref count should match the number of visited refs
        prop_assert_eq!(visitor.ref_count, visitor.visited_refs.len());

        // Property: literal count should equal sum of number and char literals
        prop_assert_eq!(visitor.literal_count, visitor.number_literal_count + visitor.char_literal_count);
    }

    #[test]
    fn test_numeric_collector(expr in super::expr_strategy()) {
        let mut collector = NumericCollector::default();
        expr.visit(&mut collector);

        // Property: all hex literals should start with $
        for hex in &collector.hex_literals {
            prop_assert!(hex.starts_with('$'));
        }

        // Property: all binary literals should start with %
        for bin in &collector.bin_literals {
            prop_assert!(bin.starts_with('%'));
        }

        // Property: all decimal literals should be numeric
        for dec in &collector.dec_literals {
            prop_assert!(dec.chars().all(|c| c.is_ascii_digit()));
        }
    }

    #[test]
    fn test_validation_visitor(expr in super::expr_strategy()) {
        let mut validator = ValidationVisitor::new(50); // Allow deep nesting
        expr.visit(&mut validator);

        // Property: well-formed expressions should validate
        if validator.is_valid() {
            prop_assert!(validator.errors.is_empty());
        }

        // Property: depth should be reasonable for generated expressions
        prop_assert!(validator.depth <= 50);
    }

    #[test]
    fn test_visitor_consistency_across_multiple_visits(expr in super::expr_strategy()) {
        // Property: visiting the same expression multiple times should give consistent results
        let mut visitor1 = RefVisitor::default();
        let mut visitor2 = RefVisitor::default();

        expr.visit(&mut visitor1);
        expr.visit(&mut visitor2);

        prop_assert_eq!(visitor1.refs(), visitor2.refs());
    }

    #[test]
    fn test_visitor_order_independence(expr in super::expr_strategy()) {
        // Property: the order of references should be deterministic
        let refs1 = {
            let mut visitor = RefVisitor::default();
            expr.visit(&mut visitor);
            visitor.refs()
        };

        let refs2 = {
            let mut visitor = RefVisitor::default();
            expr.visit(&mut visitor);
            visitor.refs()
        };

        prop_assert_eq!(refs1, refs2);
    }
}

// Regression tests using specific known cases
#[test]
fn test_known_edge_cases() {
    // Test empty reference handling
    let expr = Expr::Literal(LiteralExpr::CharLiteral(CharLiteral::from(
        "'a'".to_string(),
    )));

    let mut visitor = RefVisitor::default();
    expr.visit(&mut visitor);
    assert!(visitor.refs().is_empty());

    // Test single reference
    let expr = Expr::Ref(RefExpr::LabelRef("test".to_string()));
    let mut visitor = RefVisitor::default();
    expr.visit(&mut visitor);
    assert_eq!(visitor.refs(), vec!["test".to_string()]);
}
