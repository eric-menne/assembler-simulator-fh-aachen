use super::{
    lexer::token::{Token, TokenType},
    ParseContext,
};
use command_builder::CommandBuilder;

use crate::{
    commands::{get_instruction_attribute, InstructionAttribute},
    error::{ParseErrorBuilder, ParseErrorType},
};
use operant::{Operant, OperantKind};
use std::iter::Peekable;

pub mod command_builder;
pub mod operant;
pub fn parse_token<'a>(
    tokens: &'a Vec<Token>,
    context: &mut ParseContext,
) -> Vec<CommandBuilder<'a>> {
    let mut commands: Vec<CommandBuilder> = vec![];
    let mut cursor = tokens.iter().peekable();

    while let Some(peek) = cursor.peek() {
        if peek.token_type == TokenType::End {
            break;
        }
        skip_empty_lines(&mut cursor);
        if let Some(command) = parse_line(&mut cursor, context) {
            commands.push(command);
            finish_line(&mut cursor, context);
        } else {
            recover_to_next_line(&mut cursor);
            skip_empty_lines(&mut cursor);
        }
    }
    commands
}

// Skips empty line until next is not new line
fn skip_empty_lines<'a, I>(cursor: &mut Peekable<I>)
where
    I: Iterator<Item = &'a Token>,
{
    while let Some(next) = cursor.peek() {
        match next.token_type {
            TokenType::NewLine => {
                cursor.next();
            }
            _ => break,
        }
    }
}

// Skips to the next lines
// Reports all symbols on it way as errors
fn finish_line<'a, I>(cursor: &mut Peekable<I>, context: &mut ParseContext)
where
    I: Iterator<Item = &'a Token>,
{
    while let Some(next) = cursor.peek() {
        match next.token_type {
            TokenType::NewLine => {
                break;
            }
            TokenType::End => break,
            _ => {
                context.errors.add(ParseErrorBuilder::new(
                    ParseErrorType::InvalidToken,
                    next.start,
                    next.end,
                ));
                cursor.next();
            }
        }
    }
}

// Moves to the next line or end of file to recover from error
fn recover_to_next_line<'a, I>(cursor: &mut Peekable<I>)
where
    I: Iterator<Item = &'a Token>,
{
    while let Some(next) = cursor.peek() {
        match next.token_type {
            TokenType::NewLine | TokenType::End => break,
            _ => {
                cursor.next();
            }
        }
    }
}

fn parse_line<'a, I>(
    cursor: &mut Peekable<I>,
    context: &mut ParseContext,
) -> Option<CommandBuilder<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    // Check if first token is a symbol
    let first_token = cursor.next()?;
    first_token.ensure_type(TokenType::Symbol, || {
        if first_token.token_type != TokenType::End {
            context.errors.add(ParseErrorBuilder::new(
                ParseErrorType::MissingInstruction,
                first_token.start,
                first_token.end,
            ));
        }
    })?;

    // check if second symbol is an colon
    let second_token = cursor.peek()?;
    Some(match second_token.token_type {
        TokenType::Colon => {
            cursor.next();
            skip_empty_lines(cursor);
            let third_token = cursor.next()?;
            third_token.ensure_type(TokenType::Symbol, || {
                context.errors.add(ParseErrorBuilder::new(
                    ParseErrorType::MissingInstruction,
                    third_token.start,
                    third_token.end,
                ));
            })?;

            parse_command(cursor, context, Some(first_token), third_token)?
        }
        _ => {
            parse_command(cursor, context, None, first_token)?
        }
    })
}

fn parse_command<'a, I>(
    cursor: &mut Peekable<I>,
    context: &mut ParseContext,
    label: Option<&'a Token>,
    instruction: &'a Token,
) -> Option<CommandBuilder<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let attributes = match get_instruction_attribute(
        instruction.resolve(context.text).to_uppercase().as_str(),
    ) {
        Some(attr) => attr,
        None => {
            context.errors.add(ParseErrorBuilder::new(
                ParseErrorType::InvalidInstruction,
                instruction.start,
                instruction.end,
            ));
            return None;
        }
    };

    if attributes.allow_no_operant() {
        return Some(CommandBuilder::new(label, instruction, None));
    }

    let operant = parse_operant(cursor, context, instruction, &attributes)?;

    Some(CommandBuilder::new(label, instruction, Some(operant)))
}

fn parse_operant<'a, I>(
    cursor: &mut Peekable<I>,
    context: &mut ParseContext,
    instruction: &'a Token,
    attributes: &InstructionAttribute,
) -> Option<Operant<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let first_token = match cursor.next() {
        Some(t) => t,
        None => {
            context.errors.add(ParseErrorBuilder::new(
                ParseErrorType::MissingOperant,
                instruction.start,
                instruction.end,
            ));
            return None;
        }
    };

    Some(match first_token.token_type {
        TokenType::ParenthesisOpen => {
            if !attributes.allow_address() {
                context.errors.add(ParseErrorBuilder::new(
                    ParseErrorType::InvalidOperant,
                    first_token.start,
                    first_token.end,
                ));
                return None;
            }
            parse_operant_address(cursor, context)?
        }
        TokenType::Number => {
            if !attributes.allow_fixed_number() {
                context.errors.add(ParseErrorBuilder::new(
                    ParseErrorType::InvalidOperant,
                    first_token.start,
                    first_token.end,
                ));
                return None;
            }
            Operant {
                kind: OperantKind::Fixed,
                value: first_token,
            }
        }
        TokenType::Symbol => {
            if !attributes.allow_label() {
                context.errors.add(ParseErrorBuilder::new(
                    ParseErrorType::InvalidOperant,
                    first_token.start,
                    first_token.end,
                ));
                return None;
            }
            Operant {
                kind: OperantKind::Label,
                value: first_token,
            }
        }
        TokenType::Hash => {
            if !attributes.allow_fixed_number() {
                context.errors.add(ParseErrorBuilder::new(
                    ParseErrorType::InvalidOperant,
                    first_token.start,
                    first_token.end,
                ));
                return None;
            }
            parse_operant_fixed(cursor, context)?
        }
        _ => {
            context.errors.add(ParseErrorBuilder::new(
                ParseErrorType::MissingOperant,
                first_token.start,
                first_token.end,
            ));
            return None;
        }
    })
}

fn parse_operant_fixed<'a, I>(
    cursor: &mut Peekable<I>,
    context: &mut ParseContext,
) -> Option<Operant<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let first_token = cursor.next()?;
    first_token.ensure_type(TokenType::Number, || {
        context.errors.add(ParseErrorBuilder::new(
            ParseErrorType::InvalidOperant,
            first_token.start,
            first_token.end,
        ));
    })?;

    Some(Operant {
        kind: OperantKind::Fixed,
        value: first_token,
    })
}

fn parse_operant_address<'a, I>(
    cursor: &mut Peekable<I>,
    context: &mut ParseContext,
) -> Option<Operant<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let first_token = cursor.next()?;
    first_token.ensure_type(TokenType::Number, || {
        context.errors.add(ParseErrorBuilder::new(
            ParseErrorType::InvalidOperant,
            first_token.start,
            first_token.end,
        ));
    })?;

    let second_token = cursor.next()?;
    second_token.ensure_type(TokenType::ParenthesisClose, || {
        context.errors.add(ParseErrorBuilder::new(
            ParseErrorType::MissingParenthesisClose,
            second_token.start,
            second_token.end,
        ));
    })?;
    Some(Operant {
        kind: OperantKind::Address,
        value: first_token,
    })
}
