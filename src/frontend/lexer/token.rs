#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType {
    Symbol,
    Number,
    Hash,
    Colon,
    ParenthesisOpen,
    ParenthesisClose,
    Invalid,
    NewLine,
    End,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub(crate) fn new_single(token_type: TokenType, position: usize) -> Self {
        Self {
            token_type,
            start: position,
            end: position + 1,
        }
    }

    pub(crate) fn new_multi(token_type: TokenType, start: usize, end: usize) -> Self {
        Self {
            token_type,
            start,
            end,
        }
    }

    pub(crate) fn ensure_type<F>(&self, token_type: TokenType, then: F) -> Option<&Self>
    where
        F: FnOnce(),
    {
        if self.token_type == token_type {
            Some(self)
        } else {
            then();
            None
        }
    }

    pub(crate) fn resolve<'a>(&self, text: &'a str) -> &'a str {
        &text[self.start..self.end]
    }
}
