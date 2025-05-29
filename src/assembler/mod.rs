use derive_more::{Display, From};

mod error;
mod opcodes;
mod pass;
mod state;

pub use error::AssembleError;
pub use error::AssembleErrorList;
use pass::Pass;
use state::State as AssemblerState;

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
    passes: Vec<Box<dyn AssemblerPass>>,
    state: AssemblerState,
}

pub trait AssemblerPass: Fn(&AssemblerState) -> Result<AssemblerState, AssembleError> {}

impl<F> AssemblerPass for F where F: Fn(&AssemblerState) -> Result<AssemblerState, AssembleError> {}

impl RusmAssembler {
    fn resolve_labels(state: &AssemblerState) -> Result<AssemblerState, AssembleError> {
        let state = Pass::resolve_labels_pass(state.clone()).execute();
        if state.errors().is_empty() {
            Ok(state)
        } else {
            Err(state.errors().clone().into())
        }
    }

    fn generate_code(state: &AssemblerState) -> Result<AssemblerState, AssembleError> {
        Err(AssembleError::Problem)
    }

    fn resolve_references(state: &AssemblerState) -> Result<AssemblerState, AssembleError> {
        Err(AssembleError::Problem)
    }

    pub fn assemble(&mut self) -> Result<Vec<u8>, AssembleError> {
        self.state.set_pc(0);
        self.state.set_origin(0);
        for pass in &self.passes {
            let state = pass(&self.state)?;
            self.state = state;
        }
        Ok(self.state.bin().clone())
    }
}

impl Default for RusmAssembler {
    fn default() -> Self {
        Self {
            passes: vec![
                Box::new(Self::resolve_labels),
                Box::new(Self::generate_code),
                Box::new(Self::resolve_references),
            ],
            state: AssemblerState::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{RusmParser, assembler::state::State};
    use itertools::Itertools;

    use super::pass::Pass;

    #[test]
    fn test_resolve_labels_pass() {
        let num_labels = 10;
        let src = (0..num_labels)
            .map(|l| format!("l{}: nop", l))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        println!("src = {src}");

        let ast = RusmParser::from_source(&src).unwrap();
        let state: State = State::from_ast(ast);
        let resolve_labels_pass = Pass::resolve_labels_pass(state);
        let state = resolve_labels_pass.execute();

        assert_eq!(state.errors(), &vec![]);

        for (k, v) in state.labels().iter().sorted_by_key(|&(k, _)| k) {
            println!("mapped {} => ${:04x}", k, v);
        }

        for i in 0..num_labels {
            let label = format!("l{i}");
            let addr = state.labels().get(&label);
            assert_eq!(addr, Some(&i));
        }
    }

    #[test]
    fn test_duplicate_labels_are_found() {
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
        let resolve_labels_pass = Pass::resolve_labels_pass(state);
        let state = resolve_labels_pass.execute();

        println!(
            "errors: \n{}",
            state
                .errors()
                .iter()
                .map(|(l, e)| format!("{l} {e}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        assert_ne!(state.errors(), &vec![]);
    }
}
