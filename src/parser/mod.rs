// Parser module for C64 assembly

mod grammar;

use pest::iterators::{Pair, Pairs};
use pest::error::Error as PestError;
use grammar::{AssemblyParser, Parser, Rule};

use crate::ast::{Ast, Instruction, Opcode, Operand, Label, Directive};

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
    let pairs = AssemblyParser::parse(Rule::program, source)
        .map_err(|e| ParseError::Pest(Box::new(e)))?;
    
    let mut ast = Ast::new();
    parse_program(pairs, &mut ast)?;
    Ok(ast)
}

fn parse_program(pairs: Pairs<Rule>, ast: &mut Ast) -> Result<(), ParseError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                // Program contains lines and EOI, so we need to process its inner pairs
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::line => {
                            parse_line(inner_pair.into_inner(), ast)?;
                        }
                        Rule::EOI => {}, // End of input
                        Rule::COMMENT => {}, // Ignore top-level comments
                        _ => return Err(ParseError::InvalidSyntax(format!("Unexpected rule in program: {:?}", inner_pair.as_rule())))
                    }
                }
            }
            Rule::EOI => {}, // End of input
            _ => return Err(ParseError::InvalidSyntax(format!("Unexpected rule: {:?}", pair.as_rule())))
        }
    }
    Ok(())
}

fn parse_line(pairs: Pairs<Rule>, ast: &mut Ast) -> Result<(), ParseError> {
    let mut label = None;
    let mut instruction = None;
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::label => {
                let label_name = pair.as_str().trim_end_matches(':');
                label = Some(Label::new(label_name));
            }
            Rule::instruction => {
                instruction = Some(parse_instruction(pair)?);
            }
            Rule::directive => {
                let directive = parse_directive(pair)?;
                ast.add_directive(directive);
            }
            Rule::constant => {
                let constant = parse_constant(pair)?;
                ast.add_constant(constant.0, constant.1);
            }
            Rule::COMMENT => {}, // Ignore comments
            _ => return Err(ParseError::InvalidSyntax(format!("Unexpected rule in line: {:?}", pair.as_rule())))
        }
    }
    
    if let Some(l) = label {
        ast.add_label(l);
    }
    
    if let Some(i) = instruction {
        ast.add_instruction(i);
    }
    
    Ok(())
}

fn parse_instruction(pair: Pair<Rule>) -> Result<Instruction, ParseError> {
    let mut inner = pair.into_inner();
    
    let opcode_pair = inner.next().ok_or_else(|| ParseError::InvalidSyntax("Missing opcode".to_string()))?;
    if opcode_pair.as_rule() != Rule::opcode {
        return Err(ParseError::InvalidSyntax(format!("Expected opcode, got {:?}", opcode_pair.as_rule())));
    }
    
    let opcode_str = opcode_pair.as_str().to_uppercase();
    let opcode = Opcode::from_str(&opcode_str)
        .map_err(|_| ParseError::UnknownOpcode(opcode_str))?;
    
    let operand = if let Some(next_pair) = inner.next() {
        if next_pair.as_rule() == Rule::operand {
            Some(parse_operand(next_pair)?)
        } else if next_pair.as_rule() == Rule::COMMENT {
            // Ignore comments
            None
        } else {
            return Err(ParseError::InvalidSyntax(format!("Expected operand, got {:?}", next_pair.as_rule())));
        }
    } else {
        None
    };
    
    Ok(Instruction::new(opcode, operand))
}

fn parse_operand(pair: Pair<Rule>) -> Result<Operand, ParseError> {
    let operand_str = pair.as_str();
    Ok(Operand::parse(operand_str))
}

fn parse_directive(pair: Pair<Rule>) -> Result<Directive, ParseError> {
    let mut inner = pair.into_inner();
    
    let name_pair = inner.next().ok_or_else(|| ParseError::InvalidSyntax("Missing directive name".to_string()))?;
    if name_pair.as_rule() != Rule::directive_name {
        return Err(ParseError::InvalidSyntax(format!("Expected directive name, got {:?}", name_pair.as_rule())));
    }
    
    let name = name_pair.as_str().trim_start_matches('.');
    
    let value_pair = inner.next().ok_or_else(|| ParseError::InvalidSyntax("Missing directive value".to_string()))?;
    if value_pair.as_rule() != Rule::directive_value {
        return Err(ParseError::InvalidSyntax(format!("Expected directive value, got {:?}", value_pair.as_rule())));
    }
    
    let value = value_pair.as_str();
    
    Ok(Directive::new(name, value))
}

fn parse_constant(pair: Pair<Rule>) -> Result<(String, String), ParseError> {
    let mut inner = pair.into_inner();
    
    let name_pair = inner.next().ok_or_else(|| ParseError::InvalidSyntax("Missing constant name".to_string()))?;
    if name_pair.as_rule() != Rule::identifier {
        return Err(ParseError::InvalidSyntax(format!("Expected identifier, got {:?}", name_pair.as_rule())));
    }
    
    let name = name_pair.as_str();
    
    let value_pair = inner.next().ok_or_else(|| ParseError::InvalidSyntax("Missing constant value".to_string()))?;
    if value_pair.as_rule() != Rule::primary {
        return Err(ParseError::InvalidSyntax(format!("Expected primary, got {:?}", value_pair.as_rule())));
    }
    
    let value = value_pair.as_str();
    
    Ok((name.to_string(), value.to_string()))
}
