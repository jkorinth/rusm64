use std::fmt::Display;

use super::EqModAddressing;
use super::Line;
use derive_more::From;

#[derive(Clone, Debug, Default, From, Eq, PartialEq)]
pub struct Ast {
    lines: Vec<Line>,
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self
            .lines
            .iter()
            .map(|l| format!("{}", l))
            .collect::<Vec<_>>()
            .join("\n");
        f.write_str(&format!("{}", lines))
    }
}

impl Ast {
    pub fn add_line(mut self, line: Line) -> Self {
        self.lines.push(line);
        self
    }

    pub fn lines(&self) -> impl Iterator<Item = &Line> {
        self.lines.iter()
    }

    pub fn lines_mut(&mut self) -> impl Iterator<Item = &mut Line> {
        self.lines.iter_mut()
    }

    pub fn line(&self, line_number: usize) -> Option<&Line> {
        self.lines.get(line_number)
    }

    pub fn line_mut(&mut self, line_number: usize) -> Option<&mut Line> {
        self.lines.get_mut(line_number)
    }
    pub fn replace<'a, I: Iterator<Item = &'a Line>>(&mut self, idx: usize, lines: I) -> &mut Self {
        println!("AST: {self:#?}\nreplacing {idx}...");
        if idx + 1 < self.lines.len() {
            self.lines.splice(idx..idx + 1, lines.cloned());
        } else {
            self.lines.splice(idx.., lines.cloned());
        }
        self
    }
}

impl EqModAddressing for Ast {
    fn eq_mod_addressing(&self, other: &Self) -> bool {
        self.lines()
            .filter(|&l| !l.is_empty())
            .zip(other.lines().filter(|&l| !l.is_empty()))
            .map(|(l1, l2)| l1.eq_mod_addressing(l2))
            .reduce(|a, b| a && b)
            .unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::tests::strategies::*;
    use proptest::prelude::*;

    #[test]
    fn empty_lines_are_ignored() {
        let ast1 = Ast::default()
            .add_line(Line::default())
            .add_line(Line::default());
        let ast2 = Ast::default().add_line(Line::default());
        // reflexivity
        assert!(ast1.eq_mod_addressing(&ast1));
        assert!(ast2.eq_mod_addressing(&ast2));
        // symmetry
        assert!(ast1.eq_mod_addressing(&ast2));
        assert!(ast2.eq_mod_addressing(&ast1));
    }

    proptest! {
        #[test]
        fn all_empty_lines_are_ignored(ast1 in ast_strategy()) {
            let ast2: Ast = ast1.lines().zip((0..ast1.lines().count()).map(|_| Line::default())).flat_map(|(l1, l2)| vec![l1.clone(), l2]).collect::<Vec<_>>().into();
            println!("ast1: {:#?}", ast1);
            println!("ast2: {:#?}", ast2);
            assert!(ast1.eq_mod_addressing(&ast2));
        }
    }
}
