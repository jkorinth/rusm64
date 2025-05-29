use crate::ast::*;

pub trait Visitor {
    fn visit_ast(&mut self, _: &Ast) {}
    fn visit_line(&mut self, _: &Line) {}
    fn visit_label(&mut self, _: &Label) {}
    fn visit_instruction(&mut self, _: &Instruction) {}
    fn visit_directive(&mut self, _: &Directive) {}
    fn visit_op(&mut self, _: &Op) {}
    fn visit_opcode(&mut self, _: &Opcode) {}
    fn visit_operand(&mut self, _: &Operand) {}
    fn visit_addressing_mode(&mut self, _: &AddressingMode) {}
    fn visit_comment(&mut self, _: &Comment) {}
    fn visit_expr(&mut self, _: &Expr) {}
    fn visit_rhai_expr(&mut self, _: &RhaiExpr) {}
    fn visit_lower_expr(&mut self, _: &LowerExpr) {}
    fn visit_upper_expr(&mut self, _: &UpperExpr) {}
    fn visit_literal_expr(&mut self, _: &LiteralExpr) {}
    fn visit_ref_expr(&mut self, _: &RefExpr) {}
    fn visit_number_literal(&mut self, _: &NumberLiteral) {}
    fn visit_char_literal(&mut self, _: &CharLiteral) {}
}

pub trait Visitable {
    fn visit(&self, visitor: &mut dyn Visitor);
}

impl Visitable for Ast {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_ast(self);
        for line in self.lines() {
            line.visit(visitor);
        }
    }
}

impl Visitable for Label {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_label(self)
    }
}

impl Visitable for Line {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_line(self);
        if let Some(label) = self.label() {
            label.visit(visitor);
        }
        if let Some(instr) = self.instruction() {
            instr.visit(visitor);
        }
        if let Some(comment) = self.comment() {
            comment.visit(visitor);
        }
    }
}

impl Visitable for Instruction {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_instruction(self);
        match self {
            Instruction::Op(op) => {
                op.visit(visitor);
            }
            Instruction::Directive(directive) => {
                directive.visit(visitor);
            }
        }
    }
}

impl Visitable for Directive {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_directive(self);
        match self {
            Self::Org(expr) => {
                expr.visit(visitor);
            }
            Self::Const(_, expr) => {
                expr.visit(visitor);
            }
            _ => {}
        }
    }
}

impl Visitable for Op {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_op(self);
        self.opcode().visit(visitor);
        if let Some(operand) = self.operand() {
            operand.visit(visitor);
        }
    }
}

impl Visitable for Opcode {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_opcode(self);
    }
}

impl Visitable for Operand {
    fn visit(&self, visitor: &mut dyn Visitor) {
        self.addressing_mode().visit(visitor);
        self.expr().visit(visitor);
    }
}

impl Visitable for AddressingMode {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_addressing_mode(self);
    }
}

impl Visitable for Comment {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_comment(self);
    }
}

impl Visitable for Expr {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_expr(self);
        match self {
            Expr::Rhai(e) => e.visit(visitor),
            Expr::Literal(e) => e.visit(visitor),
            Expr::Upper(e) => e.visit(visitor),
            Expr::Lower(e) => e.visit(visitor),
            Expr::Ref(e) => e.visit(visitor),
        }
    }
}

impl Visitable for RhaiExpr {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_rhai_expr(self);
    }
}

impl Visitable for LiteralExpr {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_literal_expr(self);
        match self {
            LiteralExpr::CharLiteral(e) => e.visit(visitor),
            LiteralExpr::NumberLiteral(e) => e.visit(visitor),
        }
    }
}

impl Visitable for NumberLiteral {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_number_literal(self);
    }
}

impl Visitable for CharLiteral {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_char_literal(self);
    }
}

impl Visitable for UpperExpr {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_upper_expr(self);
        self.expr().visit(visitor);
    }
}

impl Visitable for LowerExpr {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_lower_expr(self);
        self.expr().visit(visitor);
    }
}

impl Visitable for RefExpr {
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_ref_expr(self);
    }
}
