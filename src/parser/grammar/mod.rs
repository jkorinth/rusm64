use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use std::str::FromStr;

mod error;
pub use error::ParseError;

#[derive(Parser)]
#[grammar = "parser/grammar/rusm64.pest"]
pub struct RusmParser;

pub use pest::Parser;

use crate::{
    AddressingMode, Ast, BinaryExpr, BinaryExprBuilder, CharLiteral, Comment, Directive, Expr,
    Instruction, LExpr, Label, Line, LineBuilder, LiteralExpr, LowerExpr, NumberLiteral, Op,
    OpBuilder, Opcode, Operand, ParenExpr, RefExpr, UpperExpr, unexpected_rule,
};

impl RusmParser {
    pub fn from_source(src: &str) -> Result<Ast, ParseError> {
        Self::parse_program(Self::parse(Rule::program, src)?)
    }

    pub fn parse_program(pairs: Pairs<'_, Rule>) -> Result<Ast, ParseError> {
        for t in pairs {
            match t.as_rule() {
                Rule::program => {
                    let lines = t
                        .into_inner()
                        .map(Self::parse_line)
                        .collect::<Result<Vec<_>, _>>()?;
                    return Ok(Ast::from(lines));
                }
                _ => {
                    return unexpected_rule!(t.as_rule() => "program");
                }
            }
        }
        Err(ParseError::InvalidSyntax("unexpected end of file".into()))
    }

    pub fn parse_line(pair: Pair<'_, Rule>) -> Result<Line, ParseError> {
        let mut line = LineBuilder::default();
        for t in pair.into_inner() {
            line = match t.as_rule() {
                Rule::label => line.label(Self::parse_label(t)?),
                Rule::instruction => line.instruction(Self::parse_instruction(t)?),
                Rule::comment => line.comment(Self::parse_comment(t)?),
                _ => {
                    return unexpected_rule!(t.as_rule() => "label, instruction, or comment");
                }
            }
        }
        Ok(line.build())
    }

    pub fn parse_label(pair: Pair<'_, Rule>) -> Result<Label, ParseError> {
        match pair.as_rule() {
            Rule::label => {
                println!("pair = {}", pair.as_str());
                return Ok(Self::parse_label(pair.into_inner().nth(0).unwrap())?);
            }
            Rule::label_name => {
                return Ok(pair.as_str().to_string().into());
            }
            _ => {
                return unexpected_rule!(pair.as_rule() => "label_name");
            }
        }
    }

    pub fn parse_instruction(pair: Pair<'_, Rule>) -> Result<Instruction, ParseError> {
        match pair.as_rule() {
            Rule::instruction => {
                return Ok(Self::parse_instruction(pair.into_inner().nth(0).unwrap())?);
            }
            Rule::op => {
                return Ok(Self::parse_op(pair.into_inner())?.into());
            }
            Rule::directive => {
                return Ok(Self::parse_directive(pair.into_inner())?.into());
            }
            _ => {
                return unexpected_rule!(pair.as_rule() => "op or directive");
            }
        }
    }

    pub fn parse_comment(pair: Pair<'_, Rule>) -> Result<Comment, ParseError> {
        match pair.as_rule() {
            Rule::comment => {
                return Ok(pair.as_str().to_string().into());
            }
            _ => {
                return unexpected_rule!(pair.as_rule() => "comment");
            }
        }
    }

    pub fn parse_op(pairs: Pairs<'_, Rule>) -> Result<Op, ParseError> {
        let mut op = OpBuilder::default();
        for t in pairs.clone() {
            op = match t.as_rule() {
                Rule::opcode => op.opcode(Opcode::from_str(t.as_str())?),
                Rule::operand => op.operand(Self::parse_operand(t.into_inner())?),
                _ => {
                    return unexpected_rule!(t.as_rule() => "opcode or operand");
                }
            }
        }
        Ok(op.build())
    }

    pub fn parse_operand(mut pairs: Pairs<'_, Rule>) -> Result<Operand, ParseError> {
        if pairs.len() != 1 {
            return Err(ParseError::InvalidSyntax(
                "expected exactly one operand".into(),
            ));
        }
        let t = pairs.nth(0).unwrap();
        let addrmode = Self::parse_addressing_mode(t.clone())?;
        Ok(Operand::from((addrmode, Self::parse_expr(t.into_inner())?)))
    }

    pub fn parse_addressing_mode(pair: Pair<'_, Rule>) -> Result<AddressingMode, ParseError> {
        use AddressingMode::*;
        match pair.as_rule() {
            Rule::immediate => Ok(Immediate),
            Rule::indexed_x => Ok(AbsoluteX),
            Rule::indexed_y => Ok(AbsoluteY),
            Rule::indirect => Ok(Indirect),
            Rule::indexed_indirect => Ok(IndexedIndirect),
            Rule::indirect_indexed => Ok(IndirectIndexed),
            Rule::abs_zp => Ok(Absolute),
            _ => unexpected_rule!(pair.as_rule() => "addressing mode expression"),
        }
    }

    pub fn parse_expr(pairs: Pairs<'_, Rule>) -> Result<Expr, ParseError> {
        for t in pairs {
            match t.as_rule() {
                // TODO nesting is not correct in lexpr, but this is the simplest solution
                Rule::expr => {
                    return Ok(Self::parse_expr(t.into_inner())?);
                }
                Rule::bin_expr => {
                    return Ok(Expr::Binary(Self::parse_binary_expr(t.into_inner())?));
                }
                Rule::lexpr => {
                    return Ok(Expr::L(Self::parse_lexpr(t.into_inner())?));
                }
                _ => {
                    println!("partial AST: {}", t);
                    return unexpected_rule!(t.as_rule() => "binary or lexpr");
                }
            }
        }
        Err(ParseError::InvalidSyntax("unexpected end of expr".into()))
    }

    pub fn parse_binary_expr(pairs: Pairs<'_, Rule>) -> Result<BinaryExpr, ParseError> {
        let mut bin = BinaryExprBuilder::default();
        for t in pairs {
            match t.as_rule() {
                Rule::lexpr => {
                    bin = bin.lhs(Self::parse_lexpr(t.into_inner())?.into());
                }
                Rule::binop => {
                    bin = bin.op(t.as_str().to_string().into());
                }
                Rule::expr => {
                    bin = bin.rhs(Self::parse_expr(t.into_inner())?);
                }
                _ => {
                    return unexpected_rule!(t.as_rule() => "lexpr, binop or expr");
                }
            }
        }
        Ok(bin.build())
    }

    pub fn parse_lexpr(pairs: Pairs<'_, Rule>) -> Result<LExpr, ParseError> {
        for t in pairs {
            match t.as_rule() {
                Rule::literal_expr => {
                    return Ok(LExpr::LiteralExpr(Self::parse_literal_expr(
                        t.into_inner(),
                    )?));
                }
                Rule::ref_expr => {
                    return Ok(LExpr::RefExpr(Self::parse_ref_expr(t.into_inner())?));
                }
                Rule::paren_expr => {
                    return Ok(LExpr::ParenExpr(ParenExpr::from(Box::new(
                        Self::parse_expr(t.into_inner())?,
                    ))));
                }
                Rule::lower_expr => {
                    return Ok(LExpr::LowerExpr(LowerExpr::from(Box::new(
                        Self::parse_expr(t.into_inner())?,
                    ))));
                }
                Rule::upper_expr => {
                    return Ok(LExpr::UpperExpr(UpperExpr::from(Box::new(
                        Self::parse_expr(t.into_inner())?,
                    ))));
                }
                _ => {
                    return unexpected_rule!(t.as_rule() => "literal_expr, ref_expr, paren_expr, lower_expr or upper_expr");
                }
            }
        }
        Err(ParseError::InvalidSyntax("unexpected end of expr".into()))
    }

    pub fn parse_literal_expr(pairs: Pairs<'_, Rule>) -> Result<LiteralExpr, ParseError> {
        for t in pairs {
            match t.as_rule() {
                Rule::number_literal => {
                    return Ok(LiteralExpr::NumberLiteral(Self::parse_number_literal(
                        t.into_inner(),
                    )?));
                }
                Rule::chr_literal => {
                    return Ok(LiteralExpr::CharLiteral(CharLiteral::from(
                        t.as_str().to_string(),
                    )));
                }
                _ => {
                    return unexpected_rule!(t.as_rule() => "number_literal or chr_literal");
                }
            }
        }
        Err(ParseError::InvalidSyntax(
            "unexpected end of literal_expr".into(),
        ))
    }

    pub fn parse_ref_expr(pairs: Pairs<'_, Rule>) -> Result<RefExpr, ParseError> {
        for t in pairs {
            match t.as_rule() {
                Rule::label_name => {
                    return Ok(RefExpr::LabelRef(t.as_str().into()));
                }
                Rule::identifier => {
                    return Ok(RefExpr::SymbolRef(t.as_str().into()));
                }
                _ => {
                    return unexpected_rule!(t.as_rule() => "label_name or identifier");
                }
            }
        }
        Err(ParseError::InvalidSyntax(
            "unexpected end of ref_expr".into(),
        ))
    }

    pub fn parse_number_literal(pairs: Pairs<'_, Rule>) -> Result<NumberLiteral, ParseError> {
        for t in pairs {
            match t.as_rule() {
                Rule::hex_literal => {
                    return Ok(NumberLiteral::HexLiteral(t.as_str().into()));
                }
                Rule::bin_literal => {
                    return Ok(NumberLiteral::BinLiteral(t.as_str().into()));
                }
                Rule::dec_literal => {
                    return Ok(NumberLiteral::DecLiteral(t.as_str().into()));
                }
                _ => {
                    return unexpected_rule!(t.as_rule() => "hex_literal, bin_literal or dec_literal");
                }
            }
        }
        Err(ParseError::InvalidSyntax(
            "unexpected end of number_literal".into(),
        ))
    }

    pub fn parse_directive(pairs: Pairs<'_, Rule>) -> Result<Directive, ParseError> {
        let mut name: Option<String> = None;
        let mut value: Option<String> = None;
        for t in pairs {
            match t.as_rule() {
                Rule::dir_name => {
                    name = Some(t.as_str().into());
                }
                Rule::dir_arg => {
                    value = Some(t.as_str().into());
                }
                _ => {
                    return unexpected_rule!(t.as_rule() => "dir_name or dir_arg");
                }
            }
        }
        Ok(Directive::from(
            name.expect("cannot build Directive without name"),
            value,
        )?)
    }
}

pub fn from_source(src: &str) -> Result<Ast, ParseError> {
    RusmParser::from_source(src)
}

#[cfg(test)]
mod tests {
    use crate::{BinOp, OperandBuilder};

    use super::*;

    #[test]
    fn absolutex_with_binary_expr() {
        pest::set_error_detail(true);
        let src = r#"sta SCREEN_BASE+$400,x
        "#;
        let expected = Ast::from(vec![
            LineBuilder::default()
                .instruction(
                    OpBuilder::default()
                        .opcode(Opcode::STA)
                        .operand(
                            OperandBuilder::default()
                                .addressing_mode(AddressingMode::AbsoluteX)
                                .expr(Expr::Binary(
                                    BinaryExprBuilder::default()
                                        .lhs(Expr::L(LExpr::RefExpr(RefExpr::SymbolRef(
                                            "SCREEN_BASE".into(),
                                        ))))
                                        .op(BinOp::from("+".to_string()))
                                        .rhs(Expr::L(LExpr::LiteralExpr(
                                            LiteralExpr::NumberLiteral(NumberLiteral::HexLiteral(
                                                "$400".into(),
                                            )),
                                        )))
                                        .build(),
                                ))
                                .build(),
                        )
                        .build()
                        .into(),
                )
                .build(),
            LineBuilder::default().build(),
        ]);
        let res = from_source(src);
        match res {
            Err(ParseError::Pest(ref pe)) => {
                println!("pest error: {}", pe);
                println!("parse attempts: {:?}", pe.parse_attempts().unwrap());
            }
            _ => {}
        }
        let ast = res.unwrap();
        assert_eq!(ast, expected);
    }

    #[test]
    fn rule_op() {
        let tests = [
            "ldx",
            "ldx 1",
            "ldx $1",
            "ldx %1111",
            "lda #1",
            "lda #$1",
            "lda #%11010",
            "lda $fffe,y",
            "lda $fffe, y",
            "lda $fffe , y",
            "lda ($ff),y",
            "lda ($ff), y",
            "lda ( $ff ), y",
            "lda $fffe,x",
            "lda $fffe, x",
            "lda $fffe , ",
            "lda ($ff,x)",
            "lda ($ff, x)",
            "lda ( $ff , x)",
            "jmp ($fdee)",
            "jmp start",
            "sta SCREEN_BASE+$400,x",
            "sta SCREEN_BASE + $100, x",
        ];
        for t in tests {
            let mut ast = RusmParser::parse(Rule::op, t).unwrap();
            println!("<rule_op> op expr: {}", t);
            println!("<rule_op> AST: {}", &ast);
            println!(
                "<rule_op> {}",
                RusmParser::parse_op(ast.nth(0).unwrap().into_inner()).unwrap()
            );
        }
    }

    #[test]
    fn rule_op_expr() {
        let tests = [
            "ldx #<start",
            "ldy #>start",
            "ldy #<start + 1",
            "ldy #>start-1",
            "ldy start + (end - 1)",
            "ldy start/12, x",
        ];
        for t in tests {
            let mut ast = RusmParser::parse(Rule::op, t).unwrap();
            println!("<rule_op_expr> op expr: {}", t);
            println!("<rule_op_expr> AST: {}", &ast);
            println!(
                "<rule_op_expr> {}",
                RusmParser::parse_op(ast.nth(0).unwrap().into_inner()).unwrap()
            );
        }
    }

    #[test]
    fn rule_directive() {
        let tests = [
            ".const PI 3",
            ".org $6400",
            ".include \"tst.asm\"",
            ".for i = 12, i < 100, i++",
        ];
        for t in tests {
            let mut ast = RusmParser::parse(Rule::directive, t).unwrap();
            println!("<rule_directive> op expr: {}", t);
            println!("<rule_directive> AST: {}", &ast);
            println!(
                "<rule_directive> {:?}",
                RusmParser::parse_directive(ast.nth(0).unwrap().into_inner()).unwrap()
            );
        }
    }
}
