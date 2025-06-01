use std::fmt::Display;

use super::EqModAddressing;
use super::Line;
use derive_more::From;
use rusm64_macros::EqModAddressing;

#[derive(Clone, Debug, Default, From, Eq, EqModAddressing, PartialEq)]
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
        if !lines.is_empty() {
            f.write_str(&lines)
        } else {
            f.write_str("\n")
        }
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

    pub fn line(&self, line_number: usize) -> Option<&Line> {
        self.lines.get(line_number)
    }

    pub fn line_mut(&mut self, line_number: usize) -> Option<&mut Line> {
        self.lines.get_mut(line_number)
    }
}
