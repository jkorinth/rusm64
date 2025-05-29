use derive_more::{Display, From};

#[derive(Clone, Display, Debug, Eq, From, Hash, PartialEq)]
#[display("{}:", _0)]
pub struct Label(String);

impl From<&Label> for String {
    fn from(label: &Label) -> Self {
        label.0.clone()
    }
}
