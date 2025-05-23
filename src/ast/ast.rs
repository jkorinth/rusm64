use super::Line;
use derive_more::From;

#[derive(Debug, Default, From, PartialEq)]
pub struct Ast {
    lines: Vec<Line>,
}

impl Ast {
    pub fn add_line(mut self, line: Line) -> Self {
        self.lines.push(line);
        self
    }
}
