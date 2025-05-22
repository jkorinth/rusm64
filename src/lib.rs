// rusm - A C64 assembler written in Rust

pub mod parser;
pub mod ast;
pub mod assembler;

// Re-export main functions for easier access
pub use crate::parser::parse_source;
pub use crate::assembler::assemble;
use crate::ast::Ast;

/// Result type for the assembler operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the assembler operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parse error: {0}")]
    Parse(#[from] parser::ParseError),
    
    #[error("Assembly error: {0}")]
    Assembly(#[from] assembler::AssemblerError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Assemble the AST into binary with verbose output
pub fn assemble_verbose(ast: &Ast) -> Result<Vec<u8>> {
    let mut assembler = assembler::Assembler::new().verbose(true);
    assembler.assemble(ast).map_err(Error::Assembly)
}
