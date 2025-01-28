use crate::frontend::LineTable;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ParseErrorReport {
    pub errors: Vec<ParseError>,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub kind: ParseErrorType,
    pub start: usize,
    pub end: usize,
    pub line: Line,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub value: String,
    pub number: usize,
}

#[derive(Debug)]
pub(crate) struct ParseErrorReportBuilder {
    pub error: Vec<ParseErrorBuilder>,
}

impl ParseErrorReportBuilder {
    pub(crate) fn new() -> Self {
        Self { error: vec![] }
    }

    pub(crate) fn is_successful(&mut self) -> bool {
        self.error.len() == 0
    }

    pub(crate) fn add(&mut self, err: ParseErrorBuilder) {
        self.error.push(err);
    }

    pub(crate) fn build(self, text: &str, line_table: &LineTable) -> ParseErrorReport {
        ParseErrorReport {
            errors: self
                .error
                .iter()
                .map(|err| err.build(text, line_table))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseErrorType {
    // Missing
    MissingOperant,
    MissingInstruction,
    MissingLabel,
    MissingParenthesisClose,

    // Invalid
    InvalidFixNumber,
    InvalidAddress,
    // InvalidNumber,
    InvalidInstruction,
    InvalidOperant,
    // InvalidFixNumberType,

    //Not allowed
    NotAllowedAddress,
    NotAllowedFixNumber,
    NotAllowedLabel,

    // label
    LabelReassign,
}

impl Display for ParseErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorType::MissingOperant => write!(f, "Missing operand."),
            ParseErrorType::MissingInstruction => write!(f, "Missing instruction."),
            ParseErrorType::MissingLabel => write!(f, "Missing label."),
            ParseErrorType::MissingParenthesisClose => write!(f, "Missing closing parenthesis."),
            ParseErrorType::InvalidFixNumber => write!(f, "Invalid fix number."),
            ParseErrorType::InvalidAddress => write!(f, "Invalid address."),
            ParseErrorType::InvalidInstruction => write!(f, "Invalid instruction."),
            ParseErrorType::InvalidOperant => write!(f, "Invalid operand."),
            ParseErrorType::NotAllowedAddress => write!(f, "Not allowed address."),
            ParseErrorType::NotAllowedFixNumber => write!(f, "Not allowed fix number."),
            ParseErrorType::NotAllowedLabel => write!(f, "Not allowed label."),
            ParseErrorType::LabelReassign => write!(f, "Label reassignment not allowed."),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ParseErrorBuilder {
    kind: ParseErrorType,
    start: usize,
    end: usize,
}

impl ParseErrorBuilder {
    pub(crate) fn build(self, text: &str, line_table: &LineTable) -> ParseError {
        let line_number = line_table.get_line_index_of(self.start);
        let line_bounds = line_table.get_line_of(self.start);
        let line_text = &text[(line_bounds.0 + 1) ..line_bounds.1];
        ParseError {
            kind: self.kind,
            start: self.start - line_bounds.0 - 1,
            end: self.end - line_bounds.0 - 1,
            line: Line {
                value: line_text.to_string(),
                number: line_number,
            },
        }
    }
}

impl ParseErrorBuilder {
    pub(crate) fn new(kind: ParseErrorType, start: usize, end: usize) -> Self {
        Self { kind, start, end }
    }
}
