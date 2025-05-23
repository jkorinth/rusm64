use super::{Comment, Instruction, Label};
use derive_more::From;

#[derive(Debug, From, PartialEq)]
pub struct Line(Option<Label>, Option<Instruction>, Option<Comment>);

impl Line {
    #[inline]
    pub fn label(&self) -> &Option<Label> {
        &self.0
    }

    #[inline]
    pub fn instruction(&self) -> &Option<Instruction> {
        &self.1
    }

    #[inline]
    pub fn comment(&self) -> &Option<Comment> {
        &self.2
    }
}

#[derive(Default)]
pub struct LineBuilder {
    label: Option<Label>,
    instruction: Option<Instruction>,
    comment: Option<Comment>,
}

impl LineBuilder {
    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn instruction(mut self, instruction: Instruction) -> Self {
        self.instruction = Some(instruction);
        self
    }

    pub fn comment(mut self, comment: Comment) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn build(self) -> Line {
        Line(self.label, self.instruction, self.comment)
    }
}
