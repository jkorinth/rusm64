use super::EqModAddressing;
use derive_more::{Display, From};
use rusm64_macros::EqModAddressing;

#[derive(Clone, Display, Debug, Eq, EqModAddressing, From, Hash, PartialEq)]
#[display("{}:", _0)]
pub struct Label(String);

impl Label {
    #[inline]
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl From<&Label> for String {
    fn from(label: &Label) -> Self {
        label.0.clone()
    }
}
