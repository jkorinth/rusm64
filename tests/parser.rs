use pest::Parser;
use rusm::{
    Ast, from_source,
    grammar::{ParseError, Rule, RusmParser},
};
use std::{fs, path::PathBuf};

fn parse_str(source: &String) -> Result<Ast, ParseError> {
    let ast = from_source(source)?;

    // Print the AST for debugging
    println!("Parsed AST:");
    println!("{:#?}", ast);

    Ok(ast)
}

fn parse_file(input: &PathBuf) -> Result<Ast, ParseError> {
    let source = fs::read_to_string(input).unwrap();
    parse_str(&source)
}

#[test]
fn parse_comments() {
    let source = r#"
        ; This is a comment

        ; Empty lines are ok.
        ;
        ; Empty comments, too.
;;;
    "#;
    let ast = parse_str(&source.to_string());
    println!("Parsed AST: {:#?}", ast);
    let ast = ast.expect("Failed to parse comments");
    assert_eq!(ast.lines().count(), 8);
}

#[test]
fn parse_simple_directives() {
    let source = r#"
        .const PI 3
        .const E 3048
        .const Z $d0cd
        .org $5001
        .for
        .const MASK %00000000
    "#;
    let ast = parse_str(&source.to_string());
    println!("Parsed AST: {:#?}", ast);
    ast.expect("Failed to parse directives");
}

#[test]
fn parse_problematic_directives() {
    pest::set_error_detail(true);
    let src = [
        ".  const MAX_X 40\n",
        ".const MAX_X       40\n",
        ".const MAX_X       40      \n",
        ".const MAX_X       40      ; Screen width\n",
    ];
    for l in src {
        let r = RusmParser::parse(Rule::directive, l);
        println!("rule directive: {:?}", r);
        let ast = RusmParser::parse_directive(
            r.expect("failed to parse directive")
                .next()
                .unwrap()
                .into_inner(),
        )
        .unwrap();
        RusmParser::parse(Rule::line, l).expect("uncool");
    }
}

#[test]
fn parse_labels() {
    let source = r#"
    start:
    end: ; another label
    
fucker:
fucker2:
    "#;
    let ast = parse_str(&source.to_string());
    println!("Parsed AST: {:#?}", ast);
    ast.expect("Failed to parse labels");
}

#[test]
fn parse_number_literals() {
    for source in [
        "12345",
        "$0",
        "$0000",
        "%00000000",
        "0",
        "%00",
        "$0",
        "$0000",
        "$00000000",
    ] {
        let parse_res = RusmParser::parse(Rule::number_literal, source);
        println!("Parsed literal: {:#?}", parse_res);
        parse_res.expect("Failed to parse number literals");
    }
}

#[test]
fn parse_instruction() {
    for source in [
        "LDA $00",
        "sty #$00",
        "bne loop",
        "jmp (1000)",
        "nop",
        "stx ($10),y",
        "stx ($10,x)",
    ] {
        let parse_res = RusmParser::parse(Rule::instruction, source);
        println!("Parsed instruction: {:#?}", parse_res);
        for pair in parse_res.expect("Failed to parse instruction") {
            println!("Parsed instruction pair: {:#?}", pair);
            let ast = RusmParser::parse_instruction(pair.into_inner().next().unwrap());
            println!("Parsed AST: {:#?}", ast);
            ast.expect("Failed to parse instruction");
        }
    }
}

#[test]
fn parse_single_instructions() {
    for source in [
        "LDA $00\n",
        "sty #$00\n",
        "bne loop\n",
        "jmp (1000)\n",
        "nop\n",
        "stx ($10),y\n",
        "stx ($10,x)\n",
    ] {
        let ast = parse_str(&source.to_string());
        ast.expect("Failed to parse instructions");
    }
}

#[test]
fn parse_instructions() {
    let source = r#"
        LDA #$00
        LDX $00
        STY $00
        STX $00
        sty #%001
        jmp label
        jmp ($0010)
        lda $00
        stx $00
        bne loop
        nop
    "#;
    let ast = parse_str(&source.to_string());
    println!("Parsed AST: {:#?}", ast);
    ast.expect("Failed to parse instructions");
}

#[test]
fn parse_minimal() {
    let source = r#"
        ; Minimal example program.
        .org $8000
    start:
        jmp start
        jmp *
    "#;
    let ast = parse_str(&source.to_string());
    println!("Parsed AST: {:#?}", ast);
    ast.expect("Failed to parse labels");
}

#[test]
fn parse_minimal_asm() {
    parse_file(&PathBuf::from("examples/minimal.asm")).unwrap();
}

#[test]
fn parse_simple_asm() {
    parse_file(&PathBuf::from("examples/simple.asm")).unwrap();
}

#[test]
fn parse_advanced_asm() {
    parse_file(&PathBuf::from("examples/advanced.asm")).unwrap();
}
