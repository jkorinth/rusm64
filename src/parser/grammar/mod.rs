use crate::{ast::*, unexpected_rule};
pub use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use std::str::FromStr;

mod error;
pub use error::ParseError;

#[derive(Parser)]
#[grammar = "parser/grammar/rusm64.pest"]
pub struct RusmParser;

impl RusmParser {
    pub fn from_source(src: &str) -> Result<Ast, ParseError> {
        Self::parse_program(Self::parse(Rule::program, src)?)
    }

    pub fn parse_program(pairs: Pairs<'_, Rule>) -> Result<Ast, ParseError> {
        if let Some(t) = pairs.into_iter().next() {
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
                Self::parse_label(pair.into_inner().nth(0).unwrap())
            }
            Rule::label_name => Ok(pair.as_str().to_string().into()),
            _ => {
                unexpected_rule!(pair.as_rule() => "label_name")
            }
        }
    }

    pub fn parse_instruction(pair: Pair<'_, Rule>) -> Result<Instruction, ParseError> {
        match pair.as_rule() {
            Rule::instruction => Self::parse_instruction(pair.into_inner().nth(0).unwrap()),
            Rule::op => Ok(Self::parse_op(pair.into_inner())?.into()),
            Rule::directive => Ok(Self::parse_directive(pair.into_inner())?.into()),
            _ => {
                unexpected_rule!(pair.as_rule() => "op or directive")
            }
        }
    }

    pub fn parse_comment(pair: Pair<'_, Rule>) -> Result<Comment, ParseError> {
        match pair.as_rule() {
            Rule::comment => Self::parse_comment(pair.into_inner().nth(0).unwrap()),
            Rule::comment_msg => Ok(pair.as_str().to_string().into()),
            _ => {
                unexpected_rule!(pair.as_rule() => "comment or comment_msg")
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
        if let Some(t) = pairs.into_iter().next() {
            match t.as_rule() {
                Rule::expr => {
                    return Self::parse_expr(t.into_inner());
                }
                Rule::literal_expr => {
                    return Ok(Self::parse_literal_expr(t.into_inner())?.into());
                }
                Rule::ref_expr => {
                    return Ok(Expr::Ref(Self::parse_ref_expr(t.into_inner())?));
                }
                Rule::lower_expr => {
                    return Ok(Expr::Lower(LowerExpr::from(Box::new(Self::parse_expr(
                        t.into_inner(),
                    )?))));
                }
                Rule::upper_expr => {
                    return Ok(Expr::Upper(UpperExpr::from(Box::new(Self::parse_expr(
                        t.into_inner(),
                    )?))));
                }
                Rule::rhai_expr => {
                    return Ok(Expr::Rhai(Self::parse_rhai_expr(t.into_inner())?));
                }
                _ => {
                    println!("partial AST: {}", t);
                    return unexpected_rule!(t.as_rule() => "literal_expr, ref_expr, lower_expr or upper_expr");
                }
            }
        }
        Err(ParseError::InvalidSyntax("unexpected end of expr".into()))
    }

    pub fn parse_rhai_expr(pairs: Pairs<'_, Rule>) -> Result<RhaiExpr, ParseError> {
        if let Some(t) = pairs.into_iter().next() {
            match t.as_rule() {
                Rule::rhai_content => {
                    return Ok(RhaiExpr::from(t.as_str().to_string()));
                }
                _ => {
                    return unexpected_rule!(t.as_rule() => "rhai_content");
                }
            }
        }
        Err(ParseError::InvalidSyntax(
            "no content found in rhai expr".into(),
        ))
    }

    pub fn parse_literal_expr(pairs: Pairs<'_, Rule>) -> Result<LiteralExpr, ParseError> {
        if let Some(t) = pairs.into_iter().next() {
            match t.as_rule() {
                Rule::number_literal => {
                    let le: LiteralExpr = Self::parse_number_literal(t.into_inner())?.into();
                    return Ok(le);
                }
                Rule::chr_literal => {
                    return Ok(LiteralExpr::CharLiteral(CharLiteral::from(
                        t.as_str()
                            .strip_prefix("'")
                            .unwrap_or(t.as_str())
                            .strip_suffix("'")
                            .unwrap_or(t.as_str())
                            .to_string(),
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
        if let Some(t) = pairs.into_iter().next() {
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
        if let Some(t) = pairs.into_iter().next() {
            match t.as_rule() {
                Rule::hex_literal => {
                    return Ok(NumberLiteral::HexLiteral(
                        t.as_str()
                            .strip_prefix("$")
                            .unwrap_or(t.as_str())
                            .to_string(),
                    ));
                }
                Rule::bin_literal => {
                    return Ok(NumberLiteral::BinLiteral(
                        t.as_str()
                            .strip_prefix("%")
                            .unwrap_or(t.as_str())
                            .to_string(),
                    ));
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
        if let Some(t) = pairs.into_iter().next() {
            match t.as_rule() {
                Rule::const_directive => {
                    return Self::parse_const_directive(t.into_inner());
                }
                Rule::org_directive => {
                    return Self::parse_org_directive(t.into_inner());
                }
                Rule::script_directive => {
                    return Self::parse_script_directive(t.into_inner());
                }
                Rule::generic_directive => {
                    return Self::parse_generic_directive(t.into_inner());
                }
                _ => {
                    return unexpected_rule!(t.as_rule() => "const_directive, org_directive, script_directive or generic_directive");
                }
            }
        }
        Err(ParseError::InvalidSyntax(
            "unexpected end of directive".into(),
        ))
    }

    pub fn parse_const_directive(pairs: Pairs<'_, Rule>) -> Result<Directive, ParseError> {
        let mut builder = ConstDirectiveBuilder::default();

        for t in pairs {
            builder = match t.as_rule() {
                Rule::identifier => builder.identifier(t.as_str()),
                Rule::expr => builder.expr(Self::parse_expr(t.into_inner())?),
                _ => {
                    return unexpected_rule!(t.as_rule() => "identifier or expr");
                }
            }
        }
        builder.build()
    }

    pub fn parse_org_directive(pairs: Pairs<'_, Rule>) -> Result<Directive, ParseError> {
        let mut builder = OrgDirectiveBuilder::default();

        if let Some(t) = pairs.into_iter().next() {
            builder = match t.as_rule() {
                Rule::expr => builder.expr(Self::parse_expr(t.into_inner())?),
                _ => {
                    return unexpected_rule!(t.as_rule() => "expr");
                }
            }
        }

        builder.build()
    }

    pub fn parse_script_directive(pairs: Pairs<'_, Rule>) -> Result<Directive, ParseError> {
        let mut builder = ScriptDirectiveBuilder::default();

        for t in pairs {
            builder = match t.as_rule() {
                Rule::expr => builder.expr(Self::parse_expr(t.into_inner())?),
                _ => {
                    return unexpected_rule!(t.as_rule() => "expr");
                }
            }
        }

        builder.build()
    }

    pub fn parse_generic_directive(pairs: Pairs<'_, Rule>) -> Result<Directive, ParseError> {
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
        Directive::from(name.expect("cannot build Directive without name"), value)
    }
}

pub fn from_source(src: &str) -> Result<Ast, ParseError> {
    RusmParser::from_source(src)
}

#[cfg(test)]
mod tests {
    use super::*;

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
                RusmParser::parse_op(ast.next().unwrap().into_inner()).unwrap()
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
                RusmParser::parse_op(ast.next().unwrap().into_inner()).unwrap()
            );
        }
    }

    #[test]
    fn rule_directive() {
        let tests = [
            ".const PI 3",
            ".const MAX_X    3000   ; maximal width",
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
                RusmParser::parse_directive(ast.next().unwrap().into_inner()).unwrap()
            );
        }
    }

    #[test]
    fn rule_line() {
        pest::set_error_detail(true);
        let tests = [
            "    a: lda x  ; x is a short for\n",
            "a: lda x  ; x is a short for\n",
            ".const     MAX_X           $20     ; ---dkjfkjd\n",
            "    sta (screen_ptr),y ; Write character to screen\n",
        ];
        for t in tests {
            let astr = RusmParser::parse(Rule::line, t);
            println!("<rule_line> result: {:?}", astr);
            let mut ast = astr.unwrap();
            println!("<rule_line> op expr: {}", t);
            println!("<rule_line> AST: {}", &ast);
            println!(
                "<rule_line> {:?}",
                RusmParser::parse_line(ast.next().unwrap()).unwrap()
            );
        }
    }

    #[test]
    fn rule_instruction() {
        pest::set_error_detail(true);
        let tests = ["lda x"];
        for t in tests {
            let mut ast = RusmParser::parse(Rule::instruction, t).unwrap();
            println!("<rule_instruction> op expr: {}", t);
            println!("<rule_instruction> AST: {}", &ast);
            println!(
                "<rule_instruction> {:?}",
                RusmParser::parse_instruction(ast.next().unwrap()).unwrap()
            );
        }
    }

    #[test]
    fn rule_label() {
        pest::set_error_detail(true);
        let tests = ["a:"];
        for t in tests {
            let mut ast = RusmParser::parse(Rule::label, t).unwrap();
            println!("<rule_label> op expr: {}", t);
            println!("<rule_label> AST: {}", &ast);
            println!(
                "<rule_label> {:?}",
                RusmParser::parse_label(ast.next().unwrap()).unwrap()
            );
        }
    }

    #[test]
    fn rule_operand() {
        pest::set_error_detail(true);
        let tests = [
            "#1",         // immediate dec
            "#$1",        // immediate hex
            "#%001",      // immediate bin
            "1",          // zp dec
            "$1",         // zp hex
            "%001",       // zp bin
            "#1",         // immediate dec
            "#$1",        // immediate hex
            "#%001",      // immediate bin
            "($1)",       // indirect hex
            "( 1 )",      // indirect ws
            "( %001 )",   // indirect bin
            "$1, x",      // absolute x
            "$1, y",      // absolute y
            "($1, x)",    // indexed indirect
            "(  $1 , x)", // indexed indirect
            "($1   ), y", // indirect indexed
            "{{ rhai }}",
            "#{{ rhai }}",
            "{{ rhai }}, x",
            "{{ rhai }}, y",
            "({{ rhai }}), y",
            "({{ rhai }}, x)",
        ];
        for t in tests {
            let mut ast = RusmParser::parse(Rule::operand, t).unwrap();
            println!("<rule_operand> expr: {}", t);
            println!("<rule_operand> AST: {}", &ast);
            println!(
                "<rule_operand> {:?}",
                RusmParser::parse_operand(ast.next().unwrap().into_inner()).unwrap()
            );
        }
    }

    #[test]
    fn rule_rhai() {
        pest::set_error_detail(true);
        let tests = ["{{ 1 + 1 }}"];
        for t in tests {
            let ast = RusmParser::parse(Rule::rhai_expr, t).unwrap();
            println!("<rule_rhai> op expr: {}", t);
            println!("<rule_rhai> AST: {}", &ast);
            println!("<rule_rhai> {:?}", RusmParser::parse_expr(ast).unwrap());
        }
    }
}
