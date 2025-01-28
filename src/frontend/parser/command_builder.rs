use super::{operant::Operant, Token};

#[derive(Debug, PartialEq)]
pub(crate) struct CommandBuilder<'a> {
    pub(crate) label: Option<&'a Token>,
    pub(crate) instruction: &'a Token,
    pub(crate) operant: Operant<'a>,
}

impl<'a> CommandBuilder<'a> {
    pub(crate) fn new(
        label: Option<&'a Token>,
        instruction: &'a Token,
        operant: Operant<'a>,
    ) -> Self {
        Self {
            label,
            instruction,
            operant,
        }
    }
}
