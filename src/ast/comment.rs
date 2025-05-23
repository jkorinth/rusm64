use derive_more::{Display, From};

#[derive(Debug, Display, From, Eq, Hash, PartialEq)]
pub struct Comment(String);
