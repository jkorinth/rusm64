use super::{Addr, AssembleError, LineNum, SourceLocation};
use crate::ast::{Ast, Expr};
use derive_more::Display;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Display, Eq, PartialEq)]
#[display("{}", format!("{:?}", self))]
pub struct State {
    file: String,
    line: LineNum,
    pc: Addr,
    origin: Addr,
    bin: Vec<u8>,
    ast: Ast,
    labels: HashMap<String, Addr>,
    constants: HashMap<String, Addr>,
    errors: Vec<(SourceLocation, AssembleError)>,
}

impl State {
    pub fn from_ast(ast: Ast) -> Self {
        Self {
            ast,
            ..Default::default()
        }
    }

    pub fn loc(&self) -> SourceLocation {
        SourceLocation::from((self.file.clone(), self.line, Some(self.pc)))
    }

    #[inline]
    pub fn pc(&self) -> u16 {
        self.pc
    }

    #[inline]
    pub fn set_pc(&mut self, pc: u16) -> &mut Self {
        self.pc = pc;
        self
    }

    #[inline]
    pub fn origin(&self) -> u16 {
        self.origin
    }

    #[inline]
    pub fn set_origin(&mut self, origin: u16) -> &mut Self {
        self.origin = origin;
        self
    }

    #[inline]
    pub fn bin(&self) -> &Vec<u8> {
        &self.bin
    }

    #[inline]
    pub fn bin_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bin
    }

    #[inline]
    pub fn ast(&self) -> &Ast {
        &self.ast
    }

    #[inline]
    pub fn ast_mut(&mut self) -> &mut Ast {
        &mut self.ast
    }

    #[inline]
    pub fn label(&self, label: &str) -> Option<u16> {
        self.labels.get(label).copied()
    }

    #[inline]
    pub fn labels(&self) -> &HashMap<String, u16> {
        &self.labels
    }

    #[inline]
    pub fn labels_mut(&mut self) -> &mut HashMap<String, u16> {
        &mut self.labels
    }

    #[inline]
    pub fn constant(&self, name: &str) -> Option<u16> {
        self.constants.get(name).copied()
    }

    #[inline]
    pub fn constants(&self) -> &HashMap<String, u16> {
        &self.constants
    }

    #[inline]
    pub fn constants_mut(&mut self) -> &mut HashMap<String, u16> {
        &mut self.constants
    }

    #[inline]
    pub fn error(&mut self, error: AssembleError) {
        let loc = self.loc();
        self.errors_mut().push((loc, error));
    }

    #[inline]
    pub fn errors(&self) -> &Vec<(SourceLocation, AssembleError)> {
        &self.errors
    }

    #[inline]
    pub fn errors_mut(&mut self) -> &mut Vec<(SourceLocation, AssembleError)> {
        &mut self.errors
    }

    #[inline]
    pub fn eval(&self, expr: &Expr) -> Result<u16, AssembleError> {
        if let Some(lit) = expr.number_literal_str() {
            if let Some(stripped) = lit.strip_prefix("$") {
                return Ok(u16::from_str_radix(stripped, 16)?);
            } else if let Some(stripped) = lit.strip_prefix("%") {
                return Ok(u16::from_str_radix(stripped, 2)?);
            } else {
                return Ok(u16::from_str_radix(lit, 10)?);
            }
        } else if let Some(chr) = expr.char_literal_str() {
            return Ok((chr.chars().nth(0).unwrap() as u8).into());
        }
        Err(AssembleError::CannotEvaluateExpr(expr.clone()))
    }

    pub fn line(&self) -> LineNum {
        self.line
    }

    pub fn next_line(&mut self) -> &mut Self {
        self.line += 1;
        self
    }

    pub fn inc_pc(&mut self, bytes: u16) -> &mut Self {
        self.pc += bytes;
        self
    }

    pub fn with_file(mut self, file: &str) -> Self {
        self.file = file.into();
        self
    }
}
