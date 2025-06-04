use crate::ast::*;

pub trait VisitorFn<R> {
    fn visit_ast_fn(&mut self, _: &Ast, r: R) -> R {
        r
    }
    fn visit_line_fn(&mut self, _: &Line, r: R) -> R {
        r
    }
    fn visit_label_fn(&mut self, _: &Label, r: R) -> R {
        r
    }
    fn visit_instruction_fn(&mut self, _: &Instruction, r: R) -> R {
        r
    }
    fn visit_directive_fn(&mut self, _: &Directive, r: R) -> R {
        r
    }
    fn visit_op_fn(&mut self, _: &Op, r: R) -> R {
        r
    }
    fn visit_opcode_fn(&mut self, _: &Opcode, r: R) -> R {
        r
    }
    fn visit_operand_fn(&mut self, _: &Operand, r: R) -> R {
        r
    }
    fn visit_addressing_mode_fn(&mut self, _: &AddressingMode, r: R) -> R {
        r
    }
    fn visit_comment_fn(&mut self, _: &Comment, r: R) -> R {
        r
    }
    fn visit_expr_fn(&mut self, _: &Expr, r: R) -> R {
        r
    }
    fn visit_rhai_expr_fn(&mut self, _: &RhaiExpr, r: R) -> R {
        r
    }
    fn visit_lower_expr_fn(&mut self, _: &LowerExpr, r: R) -> R {
        r
    }
    fn visit_upper_expr_fn(&mut self, _: &UpperExpr, r: R) -> R {
        r
    }
    fn visit_literal_expr_fn(&mut self, _: &LiteralExpr, r: R) -> R {
        r
    }
    fn visit_ref_expr_fn(&mut self, _: &RefExpr, r: R) -> R {
        r
    }
    fn visit_number_literal_fn(&mut self, _: &NumberLiteral, r: R) -> R {
        r
    }
    fn visit_char_literal_fn(&mut self, _: &CharLiteral, r: R) -> R {
        r
    }
}

pub trait VisitableFn<R> {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R;
}

impl<R> VisitableFn<R> for Ast {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        let mut r = visitor.visit_ast_fn(self, r);
        for line in self.lines() {
            r = line.visit_fn(visitor, r);
        }
        r
    }
}

impl<R> VisitableFn<R> for Line {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        let mut r = visitor.visit_line_fn(self, r);
        if let Some(label) = self.label() {
            r = label.visit_fn(visitor, r);
        }
        if let Some(instr) = self.instruction() {
            r = instr.visit_fn(visitor, r);
        }
        if let Some(comment) = self.comment() {
            r = comment.visit_fn(visitor, r);
        }
        r
    }
}

impl<R> VisitableFn<R> for Label {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        visitor.visit_label_fn(self, r)
    }
}

impl<R> VisitableFn<R> for Instruction {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        let mut r = visitor.visit_instruction_fn(self, r);
        match self {
            Instruction::Op(op) => {
                r = op.visit_fn(visitor, r);
            }
            Instruction::Directive(directive) => {
                r = directive.visit_fn(visitor, r);
            }
        }
        r
    }
}

impl<R> VisitableFn<R> for Directive {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        let mut r = visitor.visit_directive_fn(self, r);
        match self {
            Self::Org(expr) => {
                r = expr.visit_fn(visitor, r);
            }
            Self::Const(_, expr) => {
                r = expr.visit_fn(visitor, r);
            }
            Self::Byte(expr) => {
                r = expr.visit_fn(visitor, r);
            }
            Self::Word(expr) => {
                r = expr.visit_fn(visitor, r);
            }
            Self::Dword(expr) => {
                r = expr.visit_fn(visitor, r);
            }
            _ => {}
        }
        r
    }
}

impl<R> VisitableFn<R> for Op {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        let mut r = visitor.visit_op_fn(self, r);
        r = self.opcode().visit_fn(visitor, r);
        if let Some(operand) = self.operand() {
            r = operand.visit_fn(visitor, r);
        }
        r
    }
}

impl<R> VisitableFn<R> for Opcode {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        visitor.visit_opcode_fn(self, r)
    }
}

impl<R> VisitableFn<R> for Operand {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        let r = self.addressing_mode().visit_fn(visitor, r);
        self.expr().visit_fn(visitor, r)
    }
}

impl<R> VisitableFn<R> for AddressingMode {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        visitor.visit_addressing_mode_fn(self, r)
    }
}

impl<R> VisitableFn<R> for Comment {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        visitor.visit_comment_fn(self, r)
    }
}

impl<R> VisitableFn<R> for Expr {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        let r = visitor.visit_expr_fn(self, r);
        match self {
            Expr::Rhai(e) => e.visit_fn(visitor, r),
            Expr::Literal(e) => e.visit_fn(visitor, r),
            Expr::Upper(e) => e.visit_fn(visitor, r),
            Expr::Lower(e) => e.visit_fn(visitor, r),
            Expr::Ref(e) => e.visit_fn(visitor, r),
        }
    }
}

impl<R> VisitableFn<R> for RhaiExpr {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        visitor.visit_rhai_expr_fn(self, r)
    }
}

impl<R> VisitableFn<R> for LiteralExpr {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        let r = visitor.visit_literal_expr_fn(self, r);
        match self {
            LiteralExpr::CharLiteral(e) => e.visit_fn(visitor, r),
            LiteralExpr::NumberLiteral(e) => e.visit_fn(visitor, r),
        }
    }
}

impl<R> VisitableFn<R> for CharLiteral {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        visitor.visit_char_literal_fn(self, r)
    }
}

impl<R> VisitableFn<R> for NumberLiteral {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        visitor.visit_number_literal_fn(self, r)
    }
}

impl<R> VisitableFn<R> for UpperExpr {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        let r = visitor.visit_upper_expr_fn(self, r);
        self.expr().visit_fn(visitor, r)
    }
}

impl<R> VisitableFn<R> for LowerExpr {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        let r = visitor.visit_lower_expr_fn(self, r);
        self.expr().visit_fn(visitor, r)
    }
}

impl<R> VisitableFn<R> for RefExpr {
    fn visit_fn(&self, visitor: &mut dyn VisitorFn<R>, r: R) -> R {
        visitor.visit_ref_expr_fn(self, r)
    }
}
