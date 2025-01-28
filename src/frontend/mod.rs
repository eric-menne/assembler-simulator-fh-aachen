use std::usize;

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
    let mut error_report = ParseErrorReportBuilder::new();
    let mut line_table: LineTable = LineTable::new();

    let tokens = tokenize(text, &mut line_table);
    let mut command_builder = parse_token(&tokens, &mut error_report);
    let commands = resolve(
        &text,
        &mut command_builder,
        &mut error_report,
        &mut line_table,
    );

    match error_report.is_successful() {
        true => Ok(commands),
        false => Err(error_report.build(text, &line_table)),
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
            if position >= line.start && position <= line.end {
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
