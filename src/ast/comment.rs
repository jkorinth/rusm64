use super::EqModAddressing;
use derive_more::{Display, From};
use rusm64_macros::EqModAddressing;

#[derive(Clone, Debug, Display, From, Eq, EqModAddressing, Hash, PartialEq)]
#[display(";{}", _0)]
pub struct Comment(String);

impl Comment {
    pub fn msg(&self) -> &String {
        &self.0
    }

    pub fn msg_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
