use derive_more::From;

#[derive(Debug, Eq, From, Hash, PartialEq)]
pub struct Label(String);
