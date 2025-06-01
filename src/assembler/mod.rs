use derive_more::{Display, From};

mod error;
pub(crate) mod opcodes;
mod pass;
mod state;

pub use error::AssembleError;
pub use error::AssembleErrorList;
pub use pass::Pass as AssemblerPass;
use pass::Pass;
pub use state::State as AssemblerState;

pub type Addr = u16;
pub type LineNum = usize;

#[derive(Clone, Debug, Display, From, Eq, PartialEq)]
#[display("{}:{}[{}]", self.file, self.line, self.pc.map(|pc| format!("${pc:04x}")).unwrap_or_else(|| "".into()))]
pub struct SourceLocation {
    file: String,
    line: LineNum,
    pc: Option<Addr>,
}

pub struct RusmAssembler {
    passes: Vec<Box<Pass>>,
    state: AssemblerState,
}

impl RusmAssembler {
    pub fn new(state: AssemblerState) -> Self {
        Self {
            state,
            ..Default::default()
        }
    }

    pub fn with_passes(mut self, passes: Vec<Box<Pass>>) -> Self {
        self.passes = passes;
        self
    }

    fn compare_states(old: &AssemblerState, new: &AssemblerState) {
        if old.ast() != new.ast() {
            println!("new ast: {:#?}", new.ast());
            println!("old ast: {:#?}", old.ast());
        }
        if old.symbols() != new.symbols() {
            println!("new symbols: {:#?}", new.symbols());
            println!("old symbols: {:#?}", old.symbols());
        }
        if old.errors() != new.errors() {
            println!("new errors: {:#?}", new.errors());
            println!("old errors: {:#?}", new.errors());
        }
    }

    pub fn execute(&mut self) -> Result<AssemblerState, AssembleError> {
        let mut last_state: Option<AssemblerState>;
        let mut cont = true;
        let mut iteration = 0;
        while cont {
            iteration += 1;
            last_state = Some(self.state.clone());
            self.state.errors_mut().clear();
            for (j, pass) in &mut self.passes.iter_mut().enumerate() {
                println!("performing pass #{} iteration #{}", j, iteration);
                self.state = pass.execute(std::mem::replace(
                    &mut self.state,
                    AssemblerState::default(),
                ));
            }
            cont = if let Some(state) = &last_state {
                /* *state.ast() != *self.state.ast() ||*/
                *state.errors() != *self.state.errors() || *state.symbols() != *self.state.symbols()
            } else {
                false
            };
            if cont {
                Self::compare_states(&last_state.unwrap(), &self.state);
            }
        }
        // abort if errors persist at the end of the pass loop
        if self.state.errors().len() > 0 {
            return Err(self.state.errors().clone().into());
        }
        Ok(std::mem::take(&mut self.state))
    }

    pub fn assemble(&mut self) -> Result<Vec<u8>, AssembleError> {
        self.state = self.execute()?;
        Ok(self.state.bin().clone())
    }
}

impl Default for RusmAssembler {
    fn default() -> Self {
        Self {
            passes: vec![
                Box::new(Pass::resolve_labels_pass()),
                Box::new(Pass::resolve_constants_pass()),
                Box::new(Pass::resolve_references_pass()),
                Box::new(Pass::resolve_rhai_pass()),
                Box::new(Pass::determine_addressing_pass()),
                Box::new(Pass::generate_code_pass()),
            ],
            state: AssemblerState::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, Line, Op, Operand, RusmParser, assembler::state::State};
    use itertools::Itertools;

    use super::{RusmAssembler, pass::Pass};

    #[test]
    fn resolve_labels() {
        let num_labels = 10;
        let src = (0..num_labels)
            .map(|l| format!("l{}: nop", l))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        println!("src = {src}");

        let ast = RusmParser::from_source(&src).unwrap();
        let state: State = State::from_ast(ast);
        let mut asm =
            RusmAssembler::new(state).with_passes(vec![Pass::resolve_labels_pass().into()]);
        let state = asm.execute().unwrap();

        assert_eq!(state.errors(), &vec![]);

        for (k, v) in state.symbols().iter().sorted_by_key(|&(k, _)| k) {
            println!("mapped {} => ${:04x}", k, v);
        }

        for i in 0..num_labels {
            let label = format!("l{i}");
            let addr = state.symbols().get(&label);
            assert_eq!(addr, Some(&i));
        }
    }

    #[test]
    fn duplicate_labels_are_found() {
        let src = r#"
          start:
            nop
            nop
          loop:
            nop
            nop
          start2:
            nop
            nop
          loop:
            nop
          start:
            nop
        "#;
        let ast = RusmParser::from_source(src).unwrap();
        let state: State = State::from_ast(ast).with_file("<local>");
        let mut asm =
            RusmAssembler::new(state).with_passes(vec![Pass::validate_labels_pass().boxed()]);
        let res = asm.execute();
        assert!(res.is_err());
    }

    #[test]
    fn resolve_labels_and_constants() {
        let src = r#"
            .org 42
        test:
            .const Y <X
            .const Z >X
            .const X {{ 0x1000 + E }}
            .const D {{ A + B + C }}
            .const C {{ B + 23 }}
            .const B {{ A + 12 }}
            .const A 12
            .const E {{ D + test }}
        "#;
        println!("src = {src}");

        let ast = RusmParser::from_source(&src).unwrap();
        let state: State = State::from_ast(ast);
        let mut asm = RusmAssembler::new(state).with_passes(vec![
            Pass::resolve_labels_pass().boxed(),
            Pass::resolve_constants_pass().boxed(),
        ]);

        let state = asm.execute().unwrap();
        assert_eq!(state.errors(), &vec![]);
        println!("symbols: {:?}", state.symbols());
        assert_eq!(state.symbol("A"), Some(12));
        assert_eq!(state.symbol("B"), Some(24));
        assert_eq!(state.symbol("C"), Some(47));
        assert_eq!(state.symbol("D"), Some(83));
        assert_eq!(state.symbol("E"), Some(125));
        assert_eq!(state.symbol("test"), Some(42));
        assert_eq!(state.symbol("X"), Some(4221));
        assert_eq!(state.symbol("Z"), Some(4221 >> 8));
        assert_eq!(state.symbol("Y"), Some(4221 & 0xff));
    }

    #[test]
    fn determine_addressing_pass() {
        let src = r#"
            LDA $1
        "#;
        println!("src = {src}");

        let ast = RusmParser::from_source(&src).unwrap();
        let state: State = State::from_ast(ast);
        let mut asm =
            RusmAssembler::new(state).with_passes(vec![Pass::determine_addressing_pass().boxed()]);

        let state = asm.execute().unwrap();
        assert_eq!(state.errors(), &vec![]);
        println!("new AST: {:#?}", state.ast());
        use crate::ast::*;
        assert!(matches!(
            state.ast().line(1),
            Some(Line(
                _,
                Some(Instruction::Op(Op(
                    _,
                    Some(Operand(AddressingMode::ZeroPage, _))
                ))),
                _
            ))
        ));
    }
}
