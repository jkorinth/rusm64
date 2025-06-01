use rhai::EvalAltResult;

use super::{AssembleError, opcodes::OPCODE_TBL, state::State};
use crate::ast::{
    visitors::{VisitableMut, VisitorMut},
    *,
};

/// Generic visit function.
pub trait VisitFn<V, T>: Fn(&mut V, &mut T) {}

/// Blanket implementation.
impl<F, V, T> VisitFn<V, T> for F where F: Fn(&mut V, &mut T) {}

/// A Pass is a State transformer that includes some pre-defined
/// mechanics for updating the program counter and line numbers.
/// The internal `VisitorMut` can use and modifythat information
/// while traversing the AST.
#[derive(Default)]
pub struct Pass {
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

#[allow(unused)]
impl Pass {
    pub fn execute(&mut self, mut state: State) -> State {
        state.reset();
        let mut ast = std::mem::take(state.ast_mut());
        ast.visit_mut(self, &mut state);
        *state.ast_mut() = ast;
        state
    }

    #[inline]
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
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

impl VisitorMut<State> for Pass {
    fn visit_ast_mut(&mut self, ast: &mut Ast, state: &mut State) {
        state.set_pc(0);
        state.set_origin(0);
        if let Some(f) = self.f_visit_ast.as_ref() {
            f(state, ast)
        }
    }

    fn visit_line_mut(&mut self, line: &mut Line, state: &mut State) {
        state.next_line();
        if let Some(f) = self.f_visit_line.as_ref() {
            f(state, line)
        }
    }

    fn visit_label_mut(&mut self, label: &mut Label, state: &mut State) {
        if let Some(f) = self.f_visit_label.as_ref() {
            f(state, label)
        }
    }

    fn visit_instruction_mut(&mut self, instr: &mut Instruction, state: &mut State) {
        if let Some(f) = self.f_visit_instruction.as_ref() {
            f(state, instr)
        }
    }

    fn visit_directive_mut(&mut self, directive: &mut Directive, state: &mut State) {
        if let Directive::Org(e) = directive {
            match state.eval(e) {
                Ok(addr) => {
                    state.set_pc(addr.try_into().unwrap());
                    state.set_origin(addr.try_into().unwrap());
                }
                Err(e) => {
                    state.error(e);
                }
            }
        }
        if let Some(f) = self.f_visit_directive.as_ref() {
            f(state, directive)
        }
    }

    fn visit_op_mut(&mut self, op: &mut Op, state: &mut State) {
        let state = state;
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
            f(state, op)
        }
    }

    fn visit_opcode_mut(&mut self, opcode: &mut Opcode, state: &mut State) {
        if let Some(f) = self.f_visit_opcode.as_ref() {
            f(state, opcode)
        }
    }

    fn visit_operand_mut(&mut self, operand: &mut Operand, state: &mut State) {
        if let Some(f) = self.f_visit_operand.as_ref() {
            f(state, operand)
        }
    }

    fn visit_addressing_mode_mut(
        &mut self,
        addressing_mode: &mut AddressingMode,
        state: &mut State,
    ) {
        if let Some(f) = self.f_visit_addressing_mode.as_ref() {
            f(state, addressing_mode)
        }
    }

    fn visit_comment_mut(&mut self, comment: &mut Comment, state: &mut State) {
        if let Some(f) = self.f_visit_comment.as_ref() {
            f(state, comment)
        }
    }

    fn visit_expr_mut(&mut self, expr: &mut Expr, state: &mut State) {
        if let Some(f) = self.f_visit_expr.as_ref() {
            f(state, expr)
        }
    }

    fn visit_literal_expr_mut(&mut self, literal_expr: &mut LiteralExpr, state: &mut State) {
        if let Some(f) = self.f_visit_literal_expr.as_ref() {
            f(state, literal_expr)
        }
    }

    fn visit_ref_expr_mut(&mut self, ref_expr: &mut RefExpr, state: &mut State) {
        if let Some(f) = self.f_visit_ref_expr.as_ref() {
            f(state, ref_expr)
        }
    }

    fn visit_lower_expr_mut(&mut self, lower_expr: &mut LowerExpr, state: &mut State) {
        if let Some(f) = self.f_visit_lower_expr.as_ref() {
            f(state, lower_expr)
        }
    }

    fn visit_upper_expr_mut(&mut self, upper_expr: &mut UpperExpr, state: &mut State) {
        if let Some(f) = self.f_visit_upper_expr.as_ref() {
            f(state, upper_expr)
        }
    }

    fn visit_rhai_expr_mut(&mut self, rhai_expr: &mut RhaiExpr, state: &mut State) {
        if let Some(f) = self.f_visit_rhai_expr.as_ref() {
            f(state, rhai_expr)
        }
    }

    fn visit_number_literal_mut(&mut self, number_literal: &mut NumberLiteral, state: &mut State) {
        if let Some(f) = self.f_visit_number_literal.as_ref() {
            f(state, number_literal)
        }
    }

    fn visit_char_literal_mut(&mut self, char_literal: &mut CharLiteral, state: &mut State) {
        if let Some(f) = self.f_visit_char_literal.as_ref() {
            f(state, char_literal)
        }
    }
}

fn make_hex_literal(val: i64) -> Expr {
    Expr::Literal(LiteralExpr::NumberLiteral(NumberLiteral::HexLiteral(
        format!("{val:04x}"),
    )))
}

// Factory methods
impl Pass {
    pub fn validate_labels_pass() -> Self {
        Pass::default().with_visit_label(Box::new(|state, label| {
            let pc = state.pc();
            if let Some(_old) = state.symbols_mut().insert(label.name().into(), pc.into()) {
                state.error(AssembleError::DuplicateLabel(label.name().to_string()));
            }
        }))
    }

    pub fn resolve_labels_pass() -> Self {
        Pass::default().with_visit_label(Box::new(|state, label| {
            let pc = state.pc();
            state.symbols_mut().insert(label.name().into(), pc.into());
        }))
    }

    pub fn resolve_constants_pass() -> Self {
        Pass::default().with_visit_directive(Box::new(|state, dir| {
            if let Directive::Const(name, expr) = dir {
                if state.symbol(name).is_none() {
                    match state.eval(expr) {
                        Ok(val) => {
                            state.symbols_mut().insert(name.clone(), val);
                        }
                        Err(err) => {
                            state.error(err.into());
                        }
                    }
                }
            }
        }))
    }

    pub fn resolve_references_pass() -> Self {
        Pass::default().with_visit_expr(Box::new(|state, refex| match refex {
            Expr::Ref(RefExpr::LabelRef(label)) => {
                if let Some(val) = state.symbol(label) {
                    *refex = make_hex_literal(val);
                }
            }
            Expr::Ref(RefExpr::SymbolRef(symbol)) => {
                if let Some(val) = state.symbol(symbol) {
                    *refex = make_hex_literal(val);
                }
            }
            _ => {}
        }))
    }

    pub fn determine_addressing_pass() -> Self {
        Pass::default().with_visit_operand(Box::new(|_, operand| {
            if let Some(val) = operand.expr().numeric_value() {
                use AddressingMode::*;
                let is_zero_page = val >= u8::MIN.into() && val <= u8::MAX.into();
                match operand.addressing_mode() {
                    Absolute => {
                        if is_zero_page {
                            *operand = OperandBuilder::from(&*operand)
                                .addressing_mode(ZeroPage)
                                .build();
                        }
                    }
                    AbsoluteX => {
                        if is_zero_page {
                            *operand = OperandBuilder::from(&*operand)
                                .addressing_mode(ZeroPageX)
                                .build();
                        }
                    }
                    AbsoluteY => {
                        if is_zero_page {
                            *operand = OperandBuilder::from(&*operand)
                                .addressing_mode(ZeroPageY)
                                .build();
                        }
                    }
                    ZeroPage => {
                        if !is_zero_page {
                            *operand = OperandBuilder::from(&*operand)
                                .addressing_mode(Absolute)
                                .build();
                        }
                    }
                    ZeroPageX => {
                        if !is_zero_page {
                            *operand = OperandBuilder::from(&*operand)
                                .addressing_mode(AbsoluteX)
                                .build();
                        }
                    }
                    ZeroPageY => {
                        if !is_zero_page {
                            *operand = OperandBuilder::from(&*operand)
                                .addressing_mode(AbsoluteX)
                                .build();
                        }
                    }
                    _ => {}
                }
            }
        }))
    }

    pub fn resolve_rhai_pass() -> Self {
        Pass::default().with_visit_expr(Box::new(|state, expr| match expr {
            Expr::Rhai(re) => {
                let engine = rhai::Engine::new_raw();
                let mut scope: rhai::Scope<'_> = state.clone().into();
                let res = engine.eval_expression_with_scope::<i64>(&mut scope, re.rhai());
                if let Ok(result) = res {
                    *expr = make_hex_literal(result);
                }
            }
            _ => {}
        }))
    }

    pub fn generate_code_pass() -> Self {
        Pass::default().with_visit_op(Box::new(|state, op| {
            let (opcode, addrmode) = op.as_opaddr();
        }))
    }
}
