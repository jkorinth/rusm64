use crate::ast::{visitors::*, *};
use proptest::prelude::*;

mod formatting;
pub(crate) mod strategies;
mod visitor_tests;
mod visitors;

use strategies::*;

proptest! {
    #[test]
    fn test_ref_visitor_finds_all_references(expr in expr_strategy()) {
        let mut visitor = RefVisitor::default();
        expr.visit(&mut visitor);
        let refs = visitor.refs();

        // Property: All references found should be valid identifiers
        for reference in &refs {
            prop_assert!(!reference.is_empty());
            prop_assert!(reference.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.'));
        }

        // Property: References should match what we expect from manual inspection
        let manual_refs = expr.references();
        prop_assert_eq!(refs, manual_refs);
    }

    #[test]
    fn test_expr_references_are_consistent(expr in expr_strategy()) {
        // Property: calling references() multiple times should give same result
        let refs1 = expr.references();
        let refs2 = expr.references();
        prop_assert_eq!(refs1, refs2);
    }

    #[test]
    fn test_number_literal_str_consistency(expr in expr_strategy()) {
        // Property: if number_literal_str returns Some, it should be a valid number format
        if let Some(num_str) = expr.number_literal_str() {
            use Expr::*;
            use LiteralExpr::*;
            use crate::NumberLiteral::*;
            let hex = regex::Regex::new(r"^[0-9a-fA-F]+$").unwrap();
            let bin = regex::Regex::new(r"^[0-1]+$").unwrap();
            let dec = regex::Regex::new(r"^[0-9]+$").unwrap();
            match &expr {
                Literal(NumberLiteral(HexLiteral(_))) => {
                    prop_assert!(hex.is_match(num_str));
                }
                Literal(NumberLiteral(BinLiteral(_))) => {
                    prop_assert!(bin.is_match(num_str));
                }
                Literal(NumberLiteral(DecLiteral(_))) => {
                    prop_assert!(dec.is_match(num_str));
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_char_literal_str_consistency(expr in expr_strategy()) {
        // Property: if char_literal_str returns Some, it should be a valid char format
        if let Some(char_str) = expr.char_literal_str() {
            prop_assert_eq!(char_str.len(), 1);
        }
    }

    #[test]
    fn test_op_as_opaddr_consistency(op in op_strategy()) {
        let (opcode, addr_mode) = op.as_opaddr();

        // Property: opcode should match the original
        prop_assert_eq!(opcode, op.opcode());

        // Property: addressing mode should be consistent
        match &op.operand() {
            Some(operand) => prop_assert_eq!(addr_mode, operand.addressing_mode()),
            None => prop_assert_eq!(addr_mode, AddressingMode::Implied),
        }
    }

    #[test]
    fn test_line_accessors_consistency(line in line_strategy()) {
        // Property: line accessors should be consistent with construction
        match line.instruction() {
            Some(Instruction::Directive(_)) => prop_assert!(line.directive().is_some()),
            Some(Instruction::Op(_)) => prop_assert!(line.op().is_some()),
            None => {
                prop_assert!(line.directive().is_none());
                prop_assert!(line.op().is_none());
            }
        }
    }
}
