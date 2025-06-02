use super::EqModAddressing;
use super::{Comment, Directive, Instruction, Label, Op};
use derive_more::{Display, From};
use rusm64_macros::EqModAddressing;

#[derive(Clone, Debug, Default, Display, From, Eq, EqModAddressing, PartialEq)]
#[display("{:<8}{:<16}{}\n",
    _0.as_ref().map(|e| format!("{}", e)).unwrap_or("".to_string()),
    _1.as_ref().map(|e| format!("{}", e)).unwrap_or("".to_string()),
    _2.as_ref().map(|e| format!("{}", e)).unwrap_or("".to_string()))]
pub struct Line(
    pub Option<Label>,
    pub Option<Instruction>,
    pub Option<Comment>,
);

impl Line {
    #[inline]
    pub fn label(&self) -> &Option<Label> {
        &self.0
    }

    #[inline]
    pub fn label_mut(&mut self) -> &mut Option<Label> {
        &mut self.0
    }

    #[inline]
    pub fn instruction(&self) -> &Option<Instruction> {
        &self.1
    }

    #[inline]
    pub fn instruction_mut(&mut self) -> &mut Option<Instruction> {
        &mut self.1
    }

    #[inline]
    pub fn comment(&self) -> &Option<Comment> {
        &self.2
    }

    #[inline]
    pub fn comment_mut(&mut self) -> &mut Option<Comment> {
        &mut self.2
    }

    pub fn directive(&self) -> Option<&Directive> {
        match &self.1 {
            Some(Instruction::Directive(d)) => Some(d),
            _ => None,
        }
    }

    pub fn op(&self) -> Option<&Op> {
        match &self.1 {
            Some(Instruction::Op(op)) => Some(op),
            _ => None,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_none() && self.1.is_none() && self.2.is_none()
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
