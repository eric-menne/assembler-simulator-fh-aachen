use super::{operant::Operant, Token};

#[derive(Debug, PartialEq)]
pub(crate) struct CommandBuilder<'a> {
    pub(crate) label: Option<&'a Token>,
    pub(crate) instruction: &'a Token,
    pub(crate) operant: Option<Operant<'a>>,
}

impl<'a> CommandBuilder<'a> {
    pub(crate) fn new(
        label: Option<&'a Token>,
        instruction: &'a Token,
        operant: Option<Operant<'a>>,
    ) -> Self {
        Self {
            label,
            instruction,
            operant,
        }
    }
}
