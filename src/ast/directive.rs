use std::path::PathBuf;

use pest::{Parser, iterators::Pairs};

use crate::{
    Expr,
    parser::grammar::{ParseError, Rule, RusmParser},
};

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Directive {
    Org(Expr),
    Const(String, Expr),
    Include(PathBuf),
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
            "include" => {
                let path: String = value.expect(".include directive requires a path argument");
                Ok(Directive::Include(PathBuf::from(path)))
            }
            name => Ok(Directive::Unknown(name.into(), value)),
        }
    }
}
