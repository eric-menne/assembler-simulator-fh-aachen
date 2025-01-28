use super::Token;

#[derive(Debug, PartialEq)]
pub struct Operant<'a> {
    pub kind: OperantKind,
    pub value: &'a Token,
}

#[derive(Debug, PartialEq)]
pub enum OperantKind {
    Fixed,
    Address,
    Label,
}
