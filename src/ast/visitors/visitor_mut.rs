use crate::ast::*;

pub trait VisitorMut {
    fn visit_ast_mut(&mut self, _: &mut Ast) {}
    fn visit_line_mut(&mut self, _: &mut Line) {}
    fn visit_label_mut(&mut self, _: &mut Label) {}
    fn visit_instruction_mut(&mut self, _: &mut Instruction) {}
    fn visit_directive_mut(&mut self, _: &mut Directive) {}
    fn visit_op_mut(&mut self, _: &mut Op) {}
    fn visit_opcode_mut(&mut self, _: &mut Opcode) {}
    fn visit_operand_mut(&mut self, _: &mut Operand) {}
    fn visit_addressing_mode_mut(&mut self, _: &mut AddressingMode) {}
    fn visit_comment_mut(&mut self, _: &mut Comment) {}
    fn visit_expr_mut(&mut self, _: &mut Expr) {}
    fn visit_rhai_expr_mut(&mut self, _: &mut RhaiExpr) {}
    fn visit_lower_expr_mut(&mut self, _: &mut LowerExpr) {}
    fn visit_upper_expr_mut(&mut self, _: &mut UpperExpr) {}
    fn visit_literal_expr_mut(&mut self, _: &mut LiteralExpr) {}
    fn visit_ref_expr_mut(&mut self, _: &mut RefExpr) {}
    fn visit_number_literal_mut(&mut self, _: &mut NumberLiteral) {}
    fn visit_char_literal_mut(&mut self, _: &mut CharLiteral) {}
}

pub trait VisitableMut {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut);
}

impl VisitableMut for Expr {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_expr_mut(self);
        match self {
            Expr::Ref(e) => e.visit_mut(visitor),
            Expr::Literal(e) => e.visit_mut(visitor),
            Expr::Upper(e) => e.visit_mut(visitor),
            Expr::Lower(e) => e.visit_mut(visitor),
            Expr::Rhai(e) => e.visit_mut(visitor),
        }
    }
}

impl VisitableMut for Label {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_label_mut(self)
    }
}

impl VisitableMut for Line {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_line_mut(self);
        if let Some(label) = self.label_mut() {
            label.visit_mut(visitor);
        }
        if let Some(instr) = self.instruction_mut() {
            instr.visit_mut(visitor);
        }
        if let Some(comment) = self.comment_mut() {
            comment.visit_mut(visitor);
        }
    }
}

impl VisitableMut for Instruction {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_instruction_mut(self);
        match self {
            Instruction::Op(op) => {
                op.visit_mut(visitor);
            }
            Instruction::Directive(directive) => {
                directive.visit_mut(visitor);
            }
        }
    }
}

impl VisitableMut for Directive {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_directive_mut(self);
        match self {
            Self::Org(expr) => {
                expr.visit_mut(visitor);
            }
            Self::Const(_, expr) => {
                expr.visit_mut(visitor);
            }
            _ => {}
        }
    }
}

impl VisitableMut for Op {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_op_mut(self);
        self.opcode().visit_mut(visitor);
        if let Some(operand) = self.operand_mut() {
            operand.visit_mut(visitor);
        }
    }
}

impl VisitableMut for Opcode {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_opcode_mut(self);
    }
}

impl VisitableMut for Operand {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        self.addressing_mode().visit_mut(visitor);
        self.expr_mut().visit_mut(visitor);
    }
}

impl VisitableMut for AddressingMode {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_addressing_mode_mut(self);
    }
}

impl VisitableMut for Comment {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_comment_mut(self);
    }
}

impl VisitableMut for RhaiExpr {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_rhai_expr_mut(self);
    }
}

impl VisitableMut for LiteralExpr {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_literal_expr_mut(self);
        match self {
            LiteralExpr::CharLiteral(e) => e.visit_mut(visitor),
            LiteralExpr::NumberLiteral(e) => e.visit_mut(visitor),
        }
    }
}

impl VisitableMut for CharLiteral {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_char_literal_mut(self);
    }
}

impl VisitableMut for NumberLiteral {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_number_literal_mut(self);
    }
}

impl VisitableMut for UpperExpr {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_upper_expr_mut(self);
        self.expr_mut().visit_mut(visitor);
    }
}

impl VisitableMut for LowerExpr {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_lower_expr_mut(self);
        self.expr_mut().visit_mut(visitor);
    }
}

impl VisitableMut for RefExpr {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_ref_expr_mut(self);
    }
}
