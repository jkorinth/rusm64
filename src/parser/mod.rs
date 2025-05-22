// Parser module for C64 assembly
pub mod grammar;

use grammar::{AssemblyParser, Parser, Rule};
use pest::error::Error as PestError;
use pest::iterators::{Pair, Pairs};

use crate::ast::*;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Pest error: {0}")]
    Pest(#[from] Box<PestError<Rule>>),

    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),

    #[error("Unknown opcode: {0}")]
    UnknownOpcode(String),
}

/// Parse source code into AST
pub fn parse_source(source: &str) -> Result<Ast, ParseError> {
    pest::set_error_detail(true);
    let pairs =
        AssemblyParser::parse(Rule::program, source).map_err(|e| ParseError::Pest(Box::new(e)))?;

    let mut ast = Ast::default();
    parse_program(pairs, &mut ast)?;
    Ok(ast)
}

fn parse_program(pairs: Pairs<Rule>, ast: &mut Ast) -> Result<(), ParseError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                // Program contains lines and EOI, so we need to process its inner pairs
                let ips = pair.into_inner();
                println!("Program: {:?}", ips);
                for inner_pair in ips {
                    match inner_pair.as_rule() {
                        Rule::line => {
                            ast.add_line(parse_line(inner_pair.into_inner())?);
                        }
                        Rule::EOI => {} // End of input
                        _ => {
                            panic!(
                                "{}",
                                format!(
                                    "Unexpected rule in program: {:?}: {:?}",
                                    inner_pair.as_rule(),
                                    inner_pair
                                )
                            );
                            return Err(ParseError::InvalidSyntax(format!(
                                "Unexpected rule in program: {:?}",
                                inner_pair.as_rule()
                            )));
                        }
                    }
                }
            }
            Rule::EOI => {} // End of input
            _ => {
                panic!(
                    "{}",
                    format!("Unexpected rule in program: {:?}", pair.as_rule())
                );
                return Err(ParseError::InvalidSyntax(format!(
                    "Unexpected rule: {:?}",
                    pair.as_rule()
                )));
            }
        }
    }
    Ok(())
}

fn parse_line(pairs: Pairs<Rule>) -> Result<Line, ParseError> {
    let mut line = LineBuilder::default();

    for pair in pairs {
        match pair.as_rule() {
            Rule::label => {
                let label_name = pair.as_str().trim_end_matches(':');
                line = line.label(Label::new(label_name));
            }
            Rule::instruction => {
                line = line.instrdir(Some(parse_instruction(pair.into_inner())?));
            }
            Rule::directive => {
                line = line.instrdir(Some(parse_directive(pair.into_inner())?));
            }
            Rule::comment => {
                line = line.comment(pair.as_str().to_string().into());
            } // Ignore comments
            _ => {
                panic!(
                    "{}",
                    format!("Unexpected rule in line: {:?}", pair.as_rule())
                );
                return Err(ParseError::InvalidSyntax(format!(
                    "Unexpected rule in line: {:?}",
                    pair.as_rule()
                )));
            }
        }
    }
    Ok(line.build())
}

pub fn parse_instruction(pairs: Pairs<Rule>) -> Result<InstrDir, ParseError> {
    let mut opcode: Option<Opcode> = None;
    let mut operand: Option<Operand> = None;

    for pair in pairs {
        match pair.as_rule() {
            Rule::opcode => {
                let opcode_str = pair.as_str();
                let opcode = Opcode::from_str(opcode_str)
                    .map_err(|_| ParseError::UnknownOpcode(opcode_str.to_string()))?;

                if let Some(operand_pair) = pair.into_inner().next() {
                    if operand_pair.as_rule() == Rule::operand {
                        operand = Some(parse_operand(operand_pair)?);
                    } else {
                        return Err(ParseError::InvalidSyntax(format!(
                            "Expected operand, got {:?}",
                            operand_pair.as_rule()
                        )));
                    }
                }
                return Ok(InstrDir::Instruction(opcode, operand));
            }
            _ => {
                panic!(
                    "{}",
                    format!("Unexpected rule in instruction: {:?}", pair.as_rule())
                );
                return Err(ParseError::InvalidSyntax(format!(
                    "Unexpected rule in instruction: {:?}",
                    pair.as_rule()
                )));
            }
        }
    }
    return Err(ParseError::InvalidSyntax(
        "Instruction parsing failed".to_string(),
    ));
}

fn parse_directive(pairs: Pairs<Rule>) -> Result<InstrDir, ParseError> {
    let mut directive = DirectiveBuilder::default();

    for pair in pairs {
        match pair.as_rule() {
            Rule::directive_name => {
                directive = directive.name(pair.as_str().to_string());
                //return Ok(InstrDir::Directive(pair.as_str().to_string(), Vec::new()));
            }
            Rule::directive_arg => {
                directive = directive.arg(pair.as_str().to_string());
            }
            _ => {
                panic!(
                    "{}",
                    format!("Unexpected rule in directive: {:?}", pair.as_rule())
                );
                return Err(ParseError::InvalidSyntax(format!(
                    "Unexpected rule in directive: {:?}",
                    pair.as_rule()
                )));
            }
        }
    }
    Ok(directive.build())
}

fn parse_operand(pair: Pair<Rule>) -> Result<Operand, ParseError> {
    let operand_str = pair.as_str();
    Ok(Operand::parse(operand_str))
}
