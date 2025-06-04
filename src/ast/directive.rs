use super::EqModAddressing;
use derive_more::Display;
use pest::{Parser, iterators::Pairs};
use rusm64_macros::EqModAddressing;

use crate::{
    Expr,
    parser::grammar::{ParseError, Rule, RusmParser},
};

#[derive(Clone, Debug, Display, Eq, EqModAddressing, Hash, PartialEq)]
pub enum Directive {
    #[display(".org {}", _0)]
    Org(Expr),
    #[display(".const {} {}", _0, _1)]
    Const(String, Expr),
    #[display(".script {}", _0)]
    Script(Expr),
    #[display(".byte {}", _0)]
    Byte(Expr),
    #[display(".word {}", _0)]
    Word(Expr),
    #[display(".dword {}", _0)]
    Dword(Expr),
    #[display(".{} {}", _0, _1.as_deref().unwrap_or(""))]
    Unknown(String, Option<String>),
}

fn parse<T, F>(rule: Rule, via: F, input: &str) -> Result<T, ParseError>
where
    F: Fn(Pairs<'_, Rule>) -> Result<T, ParseError>,
{
    via(RusmParser::parse(rule, input)?)
}

impl Directive {
    pub fn from(name: String, value: Option<String>) -> Result<Directive, ParseError> {
        match name.to_lowercase().as_str() {
            "org" => {
                let v = value.expect(".org directive requires an address argument");
                let expr = parse(Rule::expr, RusmParser::parse_expr, &v)?;
                Ok(Directive::Org(expr))
            }
            "const" => {
                let mut v = value.expect(".org directive requires an address argument");
                let mut name = RusmParser::parse(Rule::identifier, &v)?;
                let x = name.nth(0).expect("could not parse name");
                let mut ve = v.split_off(x.as_str().len());
                ve = ve.trim().into();
                println!("ve = {}", ve);
                let expr = parse(Rule::expr, RusmParser::parse_expr, &ve)?;
                Ok(Directive::Const(v, expr))
            }
            "byte" => {
                let v = value.expect(".byte directive requires an argument");
                let expr = parse(Rule::expr, RusmParser::parse_expr, &v)?;
                Ok(Directive::Byte(expr))
            }
            "word" => {
                let v = value.expect(".word directive requires an argument");
                let expr = parse(Rule::expr, RusmParser::parse_expr, &v)?;
                Ok(Directive::Word(expr))
            }
            "dword" => {
                let v = value.expect(".dword directive requires an argument");
                let expr = parse(Rule::expr, RusmParser::parse_expr, &v)?;
                Ok(Directive::Dword(expr))
            }
            name => Ok(Directive::Unknown(name.into(), value)),
        }
    }

    pub fn expr(&self) -> Option<&Expr> {
        match self {
            Directive::Org(e) => Some(e),
            Directive::Const(_, e) => Some(e),
            Directive::Script(e) => Some(e),
            Directive::Byte(e) => Some(e),
            Directive::Word(e) => Some(e),
            Directive::Dword(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq)]
pub struct ConstDirectiveBuilder {
    identifier: Option<String>,
    expr: Option<Expr>,
}

impl ConstDirectiveBuilder {
    pub fn identifier(mut self, identifier: &str) -> Self {
        self.identifier = Some(identifier.into());
        self
    }

    pub fn expr(mut self, expr: Expr) -> Self {
        self.expr = Some(expr);
        self
    }

    pub fn build(self) -> Result<Directive, ParseError> {
        if self.identifier.is_none() {
            Err(ParseError::InvalidSyntax(
                "const directive requires name".into(),
            ))
        } else if self.expr.is_none() {
            Err(ParseError::InvalidSyntax(
                "const directive requires value".into(),
            ))
        } else {
            Ok(Directive::Const(
                self.identifier.unwrap(),
                self.expr.unwrap(),
            ))
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq)]
pub struct OrgDirectiveBuilder {
    expr: Option<Expr>,
}

impl OrgDirectiveBuilder {
    pub fn expr(mut self, expr: Expr) -> Self {
        self.expr = Some(expr);
        self
    }

    pub fn build(self) -> Result<Directive, ParseError> {
        if self.expr.is_none() {
            Err(ParseError::InvalidSyntax(
                "org directive requires value expr".into(),
            ))
        } else {
            Ok(Directive::Org(self.expr.unwrap()))
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq)]
pub struct ScriptDirectiveBuilder {
    expr: Option<Expr>,
}

impl ScriptDirectiveBuilder {
    pub fn expr(mut self, expr: Expr) -> Self {
        self.expr = Some(expr);
        self
    }

    pub fn build(self) -> Result<Directive, ParseError> {
        if self.expr.is_none() {
            Err(ParseError::InvalidSyntax(
                "org directive requires value expr".into(),
            ))
        } else {
            Ok(Directive::Script(self.expr.unwrap()))
        }
    }
}
