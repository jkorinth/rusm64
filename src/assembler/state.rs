use super::{Addr, AssembleError, Bin, LineNum, SourceLocation};
use crate::{
    LiteralExpr, NumberLiteral, RefExpr, RhaiExpr,
    ast::{Ast, Expr},
};
use derive_more::Display;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Display, Eq, PartialEq)]
#[display("{}", format!("{:?}", self))]
pub struct State {
    file: String,
    line: LineNum,
    pc: Addr,
    origin: Addr,
    bin: Bin,
    ast: Ast,
    symbols: HashMap<String, i64>,
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

    pub fn reset(&mut self) {
        self.file = "".into();
        self.line = 0;
        self.pc = 0;
        self.bin.clear();
        self.bin.resize(64 * 1024, 0xea);
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
    pub fn bin(&self) -> &Bin {
        &self.bin
    }

    #[inline]
    pub fn bin_mut(&mut self) -> &mut Bin {
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
    pub fn symbol(&self, name: &str) -> Option<i64> {
        self.symbols().get(name).copied()
    }

    #[inline]
    pub fn symbols(&self) -> &HashMap<String, i64> {
        &self.symbols
    }

    #[inline]
    pub fn symbols_mut(&mut self) -> &mut HashMap<String, i64> {
        &mut self.symbols
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

    pub fn eval(&self, expr: &Expr) -> Result<i64, AssembleError> {
        match expr {
            Expr::Ref(e) => match e {
                RefExpr::LabelRef(n) => self
                    .symbol(n)
                    .ok_or_else(|| AssembleError::UnresolvedSymbol(n.clone())),
                RefExpr::SymbolRef(n) => self
                    .symbol(n)
                    .ok_or_else(|| AssembleError::UnresolvedSymbol(n.clone())),
            },
            Expr::Literal(e) => self.eval_literal(e),
            Expr::Upper(e) => Ok((self.eval(e.expr())? >> 8) & 0xff),
            Expr::Lower(e) => Ok(self.eval(e.expr())? & 0xff),
            Expr::Rhai(e) => self.eval_rhai(e),
        }
    }

    fn eval_rhai(&self, e: &RhaiExpr) -> Result<i64, AssembleError> {
        let engine = rhai::Engine::new_raw();
        let mut scope = self.clone().into();
        let res = engine.eval_expression_with_scope::<i64>(&mut scope, e.rhai());
        match res {
            Ok(v) => Ok(v),
            Err(err) => match *err {
                rhai::EvalAltResult::ErrorVariableNotFound(name, _) => {
                    Err(AssembleError::UnresolvedSymbol(name))
                }
                e => Err(AssembleError::RhaiError(format!("{}", e))),
            },
        }
    }

    fn eval_literal(&self, e: &LiteralExpr) -> Result<i64, AssembleError> {
        match e {
            LiteralExpr::NumberLiteral(NumberLiteral::HexLiteral(n)) => {
                Ok(i64::from_str_radix(n, 16)?)
            }
            LiteralExpr::NumberLiteral(NumberLiteral::BinLiteral(n)) => {
                Ok(i64::from_str_radix(n, 2)?)
            }
            LiteralExpr::NumberLiteral(NumberLiteral::DecLiteral(n)) => {
                Ok(i64::from_str_radix(n, 10)?)
            }
            LiteralExpr::CharLiteral(c) => Ok(c.to_string().chars().nth(0).unwrap() as i64),
        }
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

impl From<rhai::Scope<'_>> for State {
    fn from(scope: rhai::Scope) -> Self {
        let mut state = State::default();
        for (name, _, value) in scope.iter() {
            // TODO check duplicates
            state
                .symbols_mut()
                .insert(name.into(), value.as_int().unwrap());
        }
        state
    }
}

impl From<State> for rhai::Scope<'_> {
    fn from(value: State) -> Self {
        let mut scope = rhai::Scope::new();
        let symbols = value.symbols().clone();
        for (name, val) in symbols {
            scope.push_constant(&name, val);
            if let Some(n) = name.strip_prefix(".") {
                scope.push_constant(n, val);
            }
        }
        scope
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::tests::strategies::*;
    use proptest::prelude::*;

    pub fn scope_def_strategy() -> impl Strategy<Value = (String, i64)> {
        (identifier_strategy(), any::<i64>())
    }

    pub fn scope_strategy() -> impl Strategy<Value = rhai::Scope<'static>> {
        prop::collection::vec(scope_def_strategy(), 0..100).prop_map(|e| {
            let mut scope = rhai::Scope::new();
            for (name, val) in e {
                if !scope.contains(&name) {
                    scope.push_constant(name, val);
                }
            }
            scope
        })
    }

    pub fn state_strategy() -> impl Strategy<Value = State> {
        prop::collection::vec(scope_def_strategy(), 0..100).prop_map(|e| {
            let mut state = State::default();
            for (name, val) in e {
                state.symbols_mut().insert(name, val);
            }
            state
        })
    }

    proptest! {
        #[test]
        fn convert_scope_to_state(scope in scope_strategy()) {
            let state: State = scope.clone().into();
            println!("state: {:?}", state);
            for (name, is_constant, value) in scope.iter() {
                if is_constant {
                    assert!(state.symbols().contains_key(name), "missing constant {name}");
                    let state_val = state.symbol(name).unwrap();
                    assert_eq!(state_val, value.as_int().unwrap(), "state value is {state_val}, but scope value is {value}");
                }
            }
        }

        #[test]
        fn convert_state_to_scope(state in state_strategy()) {
            let scope: rhai::Scope = state.clone().into();
            println!("state: {:?}", state);
            for (name, value) in state.symbols() {
                assert!(scope.get(name).is_some(), "missing constant {name}");
                if let Some(d) = scope.get(name) {
                        assert_eq!(d.as_int().unwrap(), *value, "scope value is {}, but state value is {value}", d.as_int().unwrap());
                }
            }
        }
    }
}
