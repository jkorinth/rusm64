use super::{AssembleError, opcodes::OPCODE_TBL, state::State};
use crate::ast::{
    visitors::{Visitable, Visitor},
    *,
};

/// Generic visit function.
pub trait VisitFn<V, T>: Fn(&mut V, &T) {}

/// Blanket implementation.
impl<F, V, T> VisitFn<V, T> for F where F: Fn(&mut V, &T) {}

#[derive(Default)]
pub struct Pass {
    state: State,
    f_visit_ast: Option<Box<dyn VisitFn<State, Ast>>>,
    f_visit_line: Option<Box<dyn VisitFn<State, Line>>>,
    f_visit_label: Option<Box<dyn VisitFn<State, Label>>>,
    f_visit_instruction: Option<Box<dyn VisitFn<State, Instruction>>>,
    f_visit_directive: Option<Box<dyn VisitFn<State, Directive>>>,
    f_visit_op: Option<Box<dyn VisitFn<State, Op>>>,
    f_visit_opcode: Option<Box<dyn VisitFn<State, Opcode>>>,
    f_visit_operand: Option<Box<dyn VisitFn<State, Operand>>>,
    f_visit_addressing_mode: Option<Box<dyn VisitFn<State, AddressingMode>>>,
    f_visit_comment: Option<Box<dyn VisitFn<State, Comment>>>,

    f_visit_expr: Option<Box<dyn VisitFn<State, Expr>>>,

    f_visit_literal_expr: Option<Box<dyn VisitFn<State, LiteralExpr>>>,
    f_visit_ref_expr: Option<Box<dyn VisitFn<State, RefExpr>>>,
    f_visit_rhai_expr: Option<Box<dyn VisitFn<State, RhaiExpr>>>,
    f_visit_lower_expr: Option<Box<dyn VisitFn<State, LowerExpr>>>,
    f_visit_upper_expr: Option<Box<dyn VisitFn<State, UpperExpr>>>,

    f_visit_number_literal: Option<Box<dyn VisitFn<State, NumberLiteral>>>,
    f_visit_char_literal: Option<Box<dyn VisitFn<State, CharLiteral>>>,
}

impl Pass {
    pub fn new(state: State) -> Self {
        Self {
            state,
            ..Default::default()
        }
    }

    pub fn execute(mut self) -> State {
        let ast = self.state.ast().clone();
        ast.visit(&mut self);
        self.state
    }

    pub fn with_visit_ast(self, f: Box<dyn VisitFn<State, Ast>>) -> Self {
        Self {
            f_visit_ast: Some(f),
            ..self
        }
    }
    pub fn with_visit_line(self, f: Box<dyn VisitFn<State, Line>>) -> Self {
        Self {
            f_visit_line: Some(f),
            ..self
        }
    }
    pub fn with_visit_label(self, f: Box<dyn VisitFn<State, Label>>) -> Self {
        Self {
            f_visit_label: Some(f),
            ..self
        }
    }
    pub fn with_visit_instruction(self, f: Box<dyn VisitFn<State, Instruction>>) -> Self {
        Self {
            f_visit_instruction: Some(f),
            ..self
        }
    }
    pub fn with_visit_directive(self, f: Box<dyn VisitFn<State, Directive>>) -> Self {
        Self {
            f_visit_directive: Some(f),
            ..self
        }
    }
    pub fn with_visit_op(self, f: Box<dyn VisitFn<State, Op>>) -> Self {
        Self {
            f_visit_op: Some(f),
            ..self
        }
    }
    pub fn with_visit_opcode(self, f: Box<dyn VisitFn<State, Opcode>>) -> Self {
        Self {
            f_visit_opcode: Some(f),
            ..self
        }
    }
    pub fn with_visit_operand(self, f: Box<dyn VisitFn<State, Operand>>) -> Self {
        Self {
            f_visit_operand: Some(f),
            ..self
        }
    }
    pub fn with_visit_addressing_mode(self, f: Box<dyn VisitFn<State, AddressingMode>>) -> Self {
        Self {
            f_visit_addressing_mode: Some(f),
            ..self
        }
    }
    pub fn with_visit_comment(self, f: Box<dyn VisitFn<State, Comment>>) -> Self {
        Self {
            f_visit_comment: Some(f),
            ..self
        }
    }
    pub fn with_visit_expr(self, f: Box<dyn VisitFn<State, Expr>>) -> Self {
        Self {
            f_visit_expr: Some(f),
            ..self
        }
    }
    pub fn with_visit_rhai_expr(self, f: Box<dyn VisitFn<State, RhaiExpr>>) -> Self {
        Self {
            f_visit_rhai_expr: Some(f),
            ..self
        }
    }
    pub fn with_visit_literal_expr(self, f: Box<dyn VisitFn<State, LiteralExpr>>) -> Self {
        Self {
            f_visit_literal_expr: Some(f),
            ..self
        }
    }
    pub fn with_visit_ref_expr(self, f: Box<dyn VisitFn<State, RefExpr>>) -> Self {
        Self {
            f_visit_ref_expr: Some(f),
            ..self
        }
    }
    pub fn with_visit_lower_expr(self, f: Box<dyn VisitFn<State, LowerExpr>>) -> Self {
        Self {
            f_visit_lower_expr: Some(f),
            ..self
        }
    }
    pub fn with_visit_upper_expr(self, f: Box<dyn VisitFn<State, UpperExpr>>) -> Self {
        Self {
            f_visit_upper_expr: Some(f),
            ..self
        }
    }

    pub fn with_visit_number_literal(self, f: Box<dyn VisitFn<State, NumberLiteral>>) -> Self {
        Self {
            f_visit_number_literal: Some(f),
            ..self
        }
    }
    pub fn with_visit_char_literal(self, f: Box<dyn VisitFn<State, CharLiteral>>) -> Self {
        Self {
            f_visit_char_literal: Some(f),
            ..self
        }
    }
}

impl Visitor for Pass {
    fn visit_ast(&mut self, ast: &Ast) {
        let state = &mut self.state;
        state.set_pc(0);
        state.set_origin(0);
        if let Some(f) = self.f_visit_ast.as_ref() {
            f(state, ast)
        }
    }

    fn visit_line(&mut self, line: &Line) {
        let state = &mut self.state;
        println!("#{}: {:?}", state.line(), line);
        state.next_line();
        if let Some(f) = self.f_visit_line.as_ref() {
            f(state, line)
        }
    }

    fn visit_label(&mut self, label: &Label) {
        if let Some(f) = self.f_visit_label.as_ref() {
            f(&mut self.state, label)
        }
    }

    fn visit_instruction(&mut self, instr: &Instruction) {
        if let Some(f) = self.f_visit_instruction.as_ref() {
            f(&mut self.state, instr)
        }
    }

    fn visit_directive(&mut self, directive: &Directive) {
        if let Directive::Org(e) = directive {
            match self.state.eval(e) {
                Ok(addr) => {
                    self.state.set_pc(addr);
                    self.state.set_origin(addr);
                }
                Err(e) => {
                    self.state.error(e);
                }
            }
        }
        if let Some(f) = self.f_visit_directive.as_ref() {
            f(&mut self.state, directive)
        }
    }

    fn visit_op(&mut self, op: &Op) {
        let state = &mut self.state;
        let am = op
            .operand()
            .as_ref()
            .map(|o| o.addressing_mode())
            .unwrap_or(AddressingMode::Implied);

        if let Some(oce) = OPCODE_TBL.get(&(op.opcode(), am)) {
            state.inc_pc(oce.size as u16);
        } else {
            state.error(AssembleError::InvalidInstruction(op.clone()));
        }

        if let Some(f) = self.f_visit_op.as_ref() {
            f(&mut self.state, op)
        }
    }

    fn visit_opcode(&mut self, opcode: &Opcode) {
        if let Some(f) = self.f_visit_opcode.as_ref() {
            f(&mut self.state, opcode)
        }
    }

    fn visit_operand(&mut self, operand: &Operand) {
        if let Some(f) = self.f_visit_operand.as_ref() {
            f(&mut self.state, operand)
        }
    }

    fn visit_addressing_mode(&mut self, addressing_mode: &AddressingMode) {
        if let Some(f) = self.f_visit_addressing_mode.as_ref() {
            f(&mut self.state, addressing_mode)
        }
    }

    fn visit_comment(&mut self, comment: &Comment) {
        if let Some(f) = self.f_visit_comment.as_ref() {
            f(&mut self.state, comment)
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        if let Some(f) = self.f_visit_expr.as_ref() {
            f(&mut self.state, expr)
        }
    }

    fn visit_literal_expr(&mut self, literal_expr: &LiteralExpr) {
        if let Some(f) = self.f_visit_literal_expr.as_ref() {
            f(&mut self.state, literal_expr)
        }
    }

    fn visit_ref_expr(&mut self, ref_expr: &RefExpr) {
        if let Some(f) = self.f_visit_ref_expr.as_ref() {
            f(&mut self.state, ref_expr)
        }
    }

    fn visit_lower_expr(&mut self, lower_expr: &LowerExpr) {
        if let Some(f) = self.f_visit_lower_expr.as_ref() {
            f(&mut self.state, lower_expr)
        }
    }

    fn visit_upper_expr(&mut self, upper_expr: &UpperExpr) {
        if let Some(f) = self.f_visit_upper_expr.as_ref() {
            f(&mut self.state, upper_expr)
        }
    }

    fn visit_rhai_expr(&mut self, rhai_expr: &RhaiExpr) {
        if let Some(f) = self.f_visit_rhai_expr.as_ref() {
            f(&mut self.state, rhai_expr)
        }
    }

    fn visit_number_literal(&mut self, number_literal: &NumberLiteral) {
        if let Some(f) = self.f_visit_number_literal.as_ref() {
            f(&mut self.state, number_literal)
        }
    }

    fn visit_char_literal(&mut self, char_literal: &CharLiteral) {
        if let Some(f) = self.f_visit_char_literal.as_ref() {
            f(&mut self.state, char_literal)
        }
    }
}

// Factory methods
impl Pass {
    pub fn resolve_labels_pass(state: State) -> Self {
        Pass::new(state).with_visit_label(Box::new(|state, label| {
            let pc = state.pc();
            if let Some(_old) = state.labels_mut().insert(label.into(), pc) {
                state.error(AssembleError::DuplicateLabel(label.into()));
            }
        }))
    }

    pub fn resolve_constants_pass(state: State) -> Self {
        Pass::new(state).with_visit_directive(Box::new(|state, dir| {
            if let Directive::Const(name, expr) = dir {
                if let Ok(val) = state.eval(expr) {
                    state.constants_mut().insert(name.clone(), val);
                }
            }
        }))
    }

    pub fn expand_directives_pass(state: State) -> Self {
        Pass::new(state).with_visit_directive(Box::new(|_state, _dir| {}))
    }
}
