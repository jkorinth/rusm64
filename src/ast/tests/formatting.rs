
use std::fmt::Display;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::RusmParser;
use crate::ast::{tests::strategies::*, *};
use crate::visitors::VisitableFn;
use proptest::prelude::*;
use regex::Regex;

static COUNTER_EXAMPLE_N: LazyLock<AtomicU64> = LazyLock::new(|| AtomicU64::new(0));

static LIT_NUM_HEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$[0-9a-fA-F]{1,4}").unwrap());

static LIT_NUM_DEC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

static LIT_NUM_BIN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"%[01]+").unwrap());

static LIT_CHAR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"'.'").unwrap());

static REF_LABEL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z_]([a-zA-Z0-9_])*").unwrap());

static REF_SYMBOL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[A-Z_]([a-zA-Z0-9_])*").unwrap());

static RHAI: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{\{.*\}\}").unwrap());

static LABEL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z_][a-zA-Z0-9_]*:\s*").unwrap());

fn expect_regex_match<E: VisitableFn<String> + Display>(e: &E, r: &Regex) {
    let res = format!("{}", e);
    println!("formatted: \"{}\"", res);
    assert!(r.is_match(&res));
}

fn check_number_literal(nl: &NumberLiteral) {
    match &nl {
        NumberLiteral::HexLiteral(_) => expect_regex_match(nl, &LIT_NUM_HEX),
        NumberLiteral::DecLiteral(_) => expect_regex_match(nl, &LIT_NUM_DEC),
        NumberLiteral::BinLiteral(_) => expect_regex_match(nl, &LIT_NUM_BIN),
    }
}

fn check_char_literal(cl: &CharLiteral) {
    expect_regex_match(cl, &LIT_CHAR);
}

fn check_literal(l: &LiteralExpr) {
    match l {
        LiteralExpr::NumberLiteral(nl) => check_number_literal(nl),
        LiteralExpr::CharLiteral(cl) => check_char_literal(cl),
    }
}

fn check_ref(r: &RefExpr) {
    match r {
        RefExpr::LabelRef(_) => expect_regex_match(r, &REF_LABEL),
        RefExpr::SymbolRef(_) => expect_regex_match(r, &REF_SYMBOL),
    }
}

fn check_upper(e: &UpperExpr) {
    let res = format!("{}", e);
    println!("res = {}", res);
    assert_ne!(res.len(), 0);
    assert_eq!(res.chars().next().unwrap(), '>');
    check_expr(e.expr());
}

fn check_lower(e: &LowerExpr) {
    let res = format!("{}", e);
    assert_ne!(res.len(), 0);
    assert_eq!(res.chars().next().unwrap(), '<');
    check_expr(e.expr());
}

fn check_expr(e: &Expr) {
    match e {
        Expr::Upper(e) => check_upper(e),
        Expr::Lower(e) => check_lower(e),
        Expr::Rhai(e) => check_rhai(e),
        Expr::Literal(e) => check_literal(e),
        Expr::Ref(e) => check_ref(e),
    }
}

fn check_rhai(e: &RhaiExpr) {
    expect_regex_match(e, &RHAI);
}

proptest! {
    #[test]
    fn fmt_number_literals(nl in number_literal_strategy()) {
        check_number_literal(&nl);
    }

    #[test]
    fn fmt_char_literals(cl in char_literal_strategy()) {
        check_char_literal(&cl);
    }

    #[test]
    fn fmt_literals(l in literal_expr_strategy()) {
        check_literal(&l)
    }

    #[test]
    fn fmt_refs(r in ref_expr_strategy()) {
        check_ref(&r);
    }

    #[test]
    fn fmt_upper(r in upper_expr_strategy()) {
        check_upper(&r);
    }

    #[test]
    fn fmt_lower(r in lower_expr_strategy()) {
        check_lower(&r);
    }

    #[test]
    fn fmt_rhai(r in rhai_expr_strategy()) {
        check_rhai(&r);
    }

    #[test]
    fn fmt_label(l in label_strategy()) {
        expect_regex_match(&l, &LABEL);
    }

    #[test]
    fn fmt_full(ast in ast_strategy()) {
        let src = format!("{}", ast);
        println!("{}\nAST:\n{:?}\n.asm:\n{}\n{}", "*".repeat(80), ast, src, "^".repeat(80));
        match RusmParser::from_source(&src) {
            Ok(re_ast) => {
                println!("{}\nre-AST:\n{:?}\nre-.asm:\n{}\n{}", "*".repeat(80), ast, src, "_".repeat(80));
                if ast != re_ast {
                    let prefix = format!("ex_{}", COUNTER_EXAMPLE_N.fetch_add(1, Ordering::SeqCst));
                    let _ = std::fs::write(format!("{}.orig.asm", prefix), src);
                    let _ = std::fs::write(format!("{}.parsed.asm", prefix), format!("{}", re_ast));
                    println!("programs parsed to different ASTs, saved them in {}.orig.asm and {}.parsed.asm",
                        prefix, prefix);
                }
                assert_eq!(ast, re_ast);
            }
            Err(e) => {
                let prefix = format!("ex_{}", COUNTER_EXAMPLE_N.fetch_add(1, Ordering::SeqCst));
                let _ = std::fs::write(format!("{}.orig.asm", prefix), src);
                println!("parsing generated asm failed: {}\nWrote {}.unparseable.asm.", e, prefix);
            }
        }
    }
}
