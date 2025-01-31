use std::{collections::HashMap, usize};

use crate::error::{ParseErrorReport, ParseErrorReportBuilder};
use lexer::tokenize;
use parser::parse_token;
use resolver::resolve;

use crate::commands::Command;

mod lexer;
mod parser;
mod resolver;

#[cfg(test)]
mod test;

#[allow(unused)]
/// Translate a string to an executable list of commands
///
/// This function takes a str containing source code, tokenizes it, parses the tokens,
/// and resolves them into executable commands.
/// If errors occur an list of error will be returned after all steps has been done.
///
/// # Example
/// ```
/// # use asim::compile;
/// let text: &str = "
/// ADD #1
/// STA (1)
/// ";
///
/// let commands = compile(text).unwrap();
/// ```
pub fn compile(text: &str) -> Result<Vec<Command>, ParseErrorReport> {
    let mut context = ParseContext::new_empty(text);

    let tokens = tokenize(text, &mut context);
    let mut command_builder = parse_token(&tokens, &mut context);
    let commands = resolve(&mut command_builder, &mut context);

    match context.errors.is_successful() {
        true => Ok(commands),
        false => Err(context.errors.build(text, &context.line_table)),
    }
}

#[derive(Debug)]
pub(crate) struct LineTable(Vec<LineInfo>);

impl LineTable {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn push(&mut self, value: LineInfo) {
        self.0.push(value);
    }

    pub fn get_line_index_of(&self, position: usize) -> usize {
        for (index, line) in self.0.iter().enumerate() {
            if position >= line.start && position <= line.end {
                return index;
            }
        }
        self.0.len()
    }

    pub fn get_line_of(&self, position: usize) -> (usize, usize) {
        for line in &self.0 {
            if position >= line.start && position <= (line.end + 1) {
                return (line.start, line.end);
            }
        }
        (0, 0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LineInfo {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub(crate) struct ParseContext<'a> {
    pub errors: ParseErrorReportBuilder,
    pub line_table: LineTable,
    pub labels: HashMap<&'a str, usize>,
    pub text: &'a str,
}

impl <'a>ParseContext<'a> {
    pub fn new_empty(text: &'a str) -> Self {
        Self {
            errors: ParseErrorReportBuilder::new(),
            line_table: LineTable::new(),
            labels: HashMap::new(),
            text
        }
    }
}
