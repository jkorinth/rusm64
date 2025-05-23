pub mod grammar;

pub use grammar::{RusmParser, from_source};

#[cfg(test)]
mod tests {
    use super::grammar::*;

    #[test]
    fn simple_program() {
        let src = r#"
        ; Hello world!
        .org $8000
        .const BG 7
        .const FG 0
        .const COLOR_RAM $d800

        start:
        lda BG
        sta COLOR_RAM

        _loop:
        jmp _loop
        jmp *
        "#;
        let tokens =
            RusmParser::parse(Rule::program, src).expect("could not tokenize simple program");
        println!("tokens = {}", tokens);
        let prg = RusmParser::parse_program(tokens).expect("could not parse simple program");
        println!("AST = {:?}", prg);
    }

    #[test]
    fn parse_example_minimal() {
        let prg = from_source(
            &std::fs::read_to_string("examples/minimal.asm")
                .expect("failed to read examples/minimal.asm"),
        )
        .expect("failed to parse examples/minimal.asm");
        println!("AST = {:?}", prg);
    }

    #[test]
    fn parse_example_simple() {
        let prg = from_source(
            &std::fs::read_to_string("examples/simple.asm")
                .expect("failed to read examples/simple.asm"),
        )
        .expect("failed to parse examples/simple.asm");
        println!("AST = {:?}", prg);
    }

    #[test]
    fn parse_example_advanced() {
        let prg = from_source(
            &std::fs::read_to_string("examples/advanced.asm")
                .expect("failed to read examples/advanced.asm"),
        )
        .expect("failed to parse examples/advanced.asm");
        println!("AST = {:?}", prg);
    }
}
