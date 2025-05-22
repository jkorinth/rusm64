// Grammar for C64 assembly language using pest
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/assembly.pest"]
pub struct AssemblyParser;

pub use pest::Parser;
