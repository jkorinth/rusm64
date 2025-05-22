use std::fs;
use std::path::PathBuf;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = PathBuf::from("examples/minimal.asm");
    let source = fs::read_to_string(&file_path)?;
    
    let result = rusm::parse_source(&source);
    
    match result {
        Ok(ast) => {
            println!("Successfully parsed AST:");
            println!("Instructions: {}", ast.instructions().len());
            println!("Labels: {}", ast.labels().len());
            println!("Directives: {}", ast.directives().len());
            println!("Constants: {}", ast.constants().len());
        },
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }
    
    Ok(())
}
