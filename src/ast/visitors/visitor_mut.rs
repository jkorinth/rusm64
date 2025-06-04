use crate::ast::*;

pub trait VisitorMut<Context> {
    fn visit_ast_mut(&mut self, _: &mut Ast, _: &mut Context) {}
    fn visit_line_mut(&mut self, _: &mut Line, _: &mut Context) {}
    fn visit_label_mut(&mut self, _: &mut Label, _: &mut Context) {}
    fn visit_instruction_mut(&mut self, _: &mut Instruction, _: &mut Context) {}
    fn visit_directive_mut(&mut self, _: &mut Directive, _: &mut Context) {}
    fn visit_op_mut(&mut self, _: &mut Op, _: &mut Context) {}
    fn visit_opcode_mut(&mut self, _: &mut Opcode, _: &mut Context) {}
    fn visit_operand_mut(&mut self, _: &mut Operand, _: &mut Context) {}
    fn visit_addressing_mode_mut(&mut self, _: &mut AddressingMode, _: &mut Context) {}
    fn visit_comment_mut(&mut self, _: &mut Comment, _: &mut Context) {}
    fn visit_expr_mut(&mut self, _: &mut Expr, _: &mut Context) {}
    fn visit_rhai_expr_mut(&mut self, _: &mut RhaiExpr, _: &mut Context) {}
    fn visit_lower_expr_mut(&mut self, _: &mut LowerExpr, _: &mut Context) {}
    fn visit_upper_expr_mut(&mut self, _: &mut UpperExpr, _: &mut Context) {}
    fn visit_literal_expr_mut(&mut self, _: &mut LiteralExpr, _: &mut Context) {}
    fn visit_ref_expr_mut(&mut self, _: &mut RefExpr, _: &mut Context) {}
    fn visit_number_literal_mut(&mut self, _: &mut NumberLiteral, _: &mut Context) {}
    fn visit_char_literal_mut(&mut self, _: &mut CharLiteral, _: &mut Context) {}
}

pub trait VisitableMut<Context> {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context);
}

impl<Context> VisitableMut<Context> for Ast {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        self.lines_mut().for_each(|l| l.visit_mut(visitor, ctx));
    }
}

impl<Context> VisitableMut<Context> for Expr {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_expr_mut(self, ctx);
        match self {
            Expr::Ref(e) => e.visit_mut(visitor, ctx),
            Expr::Literal(e) => e.visit_mut(visitor, ctx),
            Expr::Upper(e) => e.visit_mut(visitor, ctx),
            Expr::Lower(e) => e.visit_mut(visitor, ctx),
            Expr::Rhai(e) => e.visit_mut(visitor, ctx),
        }
    }
}

impl<Context> VisitableMut<Context> for Label {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_label_mut(self, ctx)
    }
}

impl<Context> VisitableMut<Context> for Line {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_line_mut(self, ctx);
        if let Some(label) = self.label_mut() {
            label.visit_mut(visitor, ctx);
        }
        if let Some(instr) = self.instruction_mut() {
            instr.visit_mut(visitor, ctx);
        }
        if let Some(comment) = self.comment_mut() {
            comment.visit_mut(visitor, ctx);
        }
    }
}

impl<Context> VisitableMut<Context> for Instruction {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_instruction_mut(self, ctx);
        match self {
            Instruction::Op(op) => {
                op.visit_mut(visitor, ctx);
            }
            Instruction::Directive(directive) => {
                directive.visit_mut(visitor, ctx);
            }
        }
    }
}

impl<Context> VisitableMut<Context> for Directive {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_directive_mut(self, ctx);
        match self {
            Self::Org(expr) => {
                expr.visit_mut(visitor, ctx);
            }
            Self::Const(_, expr) => {
                expr.visit_mut(visitor, ctx);
            }
            Self::Byte(expr) => {
                expr.visit_mut(visitor, ctx);
            }
            Self::Word(expr) => {
                expr.visit_mut(visitor, ctx);
            }
            Self::Dword(expr) => {
                expr.visit_mut(visitor, ctx);
            }
            _ => {}
        }
    }
}

impl<Context> VisitableMut<Context> for Op {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_op_mut(self, ctx);
        self.opcode().visit_mut(visitor, ctx);
        if let Some(operand) = self.operand_mut() {
            operand.visit_mut(visitor, ctx);
        }
    }
}

impl<Context> VisitableMut<Context> for Opcode {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_opcode_mut(self, ctx);
    }
}

impl<Context> VisitableMut<Context> for Operand {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_operand_mut(self, ctx);
        self.addressing_mode().visit_mut(visitor, ctx);
        self.expr_mut().visit_mut(visitor, ctx);
    }
}

impl<Context> VisitableMut<Context> for AddressingMode {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_addressing_mode_mut(self, ctx);
    }
}

impl<Context> VisitableMut<Context> for Comment {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_comment_mut(self, ctx);
    }
}

impl<Context> VisitableMut<Context> for RhaiExpr {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_rhai_expr_mut(self, ctx);
    }
}

impl<Context> VisitableMut<Context> for LiteralExpr {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_literal_expr_mut(self, ctx);
        match self {
            LiteralExpr::CharLiteral(e) => e.visit_mut(visitor, ctx),
            LiteralExpr::NumberLiteral(e) => e.visit_mut(visitor, ctx),
        }
    }
}

impl<Context> VisitableMut<Context> for CharLiteral {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_char_literal_mut(self, ctx);
    }
}

impl<Context> VisitableMut<Context> for NumberLiteral {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_number_literal_mut(self, ctx);
    }
}

impl<Context> VisitableMut<Context> for UpperExpr {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_upper_expr_mut(self, ctx);
        self.expr_mut().visit_mut(visitor, ctx);
    }
}

impl<Context> VisitableMut<Context> for LowerExpr {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_lower_expr_mut(self, ctx);
        self.expr_mut().visit_mut(visitor, ctx);
    }
}

impl<Context> VisitableMut<Context> for RefExpr {
    fn visit_mut(&mut self, visitor: &mut dyn VisitorMut<Context>, ctx: &mut Context) {
        visitor.visit_ref_expr_mut(self, ctx);
    }
}
