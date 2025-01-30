use super::{lexer::token::{Token, TokenType}, ParseContext};
use command_builder::CommandBuilder;

use crate::error::{ParseErrorBuilder, ParseErrorReportBuilder, ParseErrorType};
use operant::{Operant, OperantKind};
use std::iter::Peekable;

pub mod command_builder;
pub mod operant;

pub fn parse_token<'a>(
    tokens: &'a Vec<Token>,
    context: &mut ParseContext
) -> Vec<CommandBuilder<'a>> {
    let mut cursor = tokens.iter().peekable();
    let mut nodes: Vec<CommandBuilder> = vec![];

    skip_white_space(&mut cursor);

    loop {
        let command = parse_command(&mut cursor, context);
        match command {
            Some(c) => nodes.push(c),
            None => move_to_next_line(&mut cursor),
        }

        skip_white_space(&mut cursor);

        if let Some(next_peek) = cursor.peek() {
            match next_peek.token_type {
                TokenType::End => break,
                _ => {}
            }
        }
    }

    nodes
}

fn move_to_next_line<'a, I>(cursor: &mut Peekable<I>)
where
    I: Iterator<Item = &'a Token>,
{
    while let Some(t) = cursor.peek() {
        match t.token_type {
            TokenType::NewLine | TokenType::End => break,
            _ => {
                cursor.next();
            }
        }
    }
}

fn skip_white_space<'a, I>(cursor: &mut Peekable<I>)
where
    I: Iterator<Item = &'a Token>,
{
    while let Some(t) = cursor.peek() {
        match t.token_type {
            TokenType::NewLine => {
                cursor.next();
            }
            _ => break,
        }
    }
}

fn parse_command<'a, I>(
    cursor: &mut Peekable<I>,
    context: &mut ParseContext
) -> Option<CommandBuilder<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let token = cursor.next()?;
    token.ensure_type(TokenType::Symbol, || {
        context.errors.add(ParseErrorBuilder::new(
            ParseErrorType::MissingInstruction,
            token.start,
            token.end,
        ));
    })?;

    let next_token = match cursor.peek() {
        Some(t) => t,
        None => {
            context.errors.add(ParseErrorBuilder::new(
                ParseErrorType::MissingOperant,
                token.start,
                token.end,
            ));
            return None;
        }
    };

    match next_token.token_type {
        TokenType::Colon => {
            cursor.next();
            cursor.next_if(|&t| t.token_type == TokenType::NewLine);

            let instruction = match cursor.next() {
                Some(t) => t,
                None => {
                    context.errors.add(ParseErrorBuilder::new(
                        ParseErrorType::MissingInstruction,
                        token.start,
                        token.end,
                    ));
                    return None;
                }
            };
            instruction.ensure_type(TokenType::Symbol, || {
                context.errors.add(ParseErrorBuilder::new(
                    ParseErrorType::MissingInstruction,
                    token.start,
                    token.end,
                ));
            })?;

            let operant = match parse_operant(cursor, context) {
                Some(o) => o,
                None => {
                    context.errors.add(ParseErrorBuilder::new(
                        ParseErrorType::MissingOperant,
                        token.start,
                        token.end,
                    ));
                    return None;
                }
            };
            return Some(CommandBuilder::new(Some(token), instruction, operant));
        }
        _ => {
            let operant = match parse_operant(cursor, context) {
                Some(o) => o,
                None => {
                    context.errors.add(ParseErrorBuilder::new(
                        ParseErrorType::MissingOperant,
                        token.start,
                        token.end,
                    ));
                    return None;
                }
            };
            return Some(CommandBuilder::new(None, token, operant));
        }
    }
}

fn parse_operant<'a, I>(
    cursor: &mut Peekable<I>,
    context: &mut ParseContext
) -> Option<Operant<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let token = cursor.next()?;

    match token.token_type {
        TokenType::Symbol => {
            cursor.next_if(|&t| t.token_type == TokenType::Colon);

            return Some(Operant {
                value: token,
                kind: OperantKind::Label,
            });
        }
        TokenType::Number => {
            return Some(Operant {
                value: token,
                kind: OperantKind::Fixed,
            });
        }
        TokenType::Hash => {
            let number = match cursor.next() {
                Some(t) => t,
                None => {
                    context.errors.add(ParseErrorBuilder::new(
                        ParseErrorType::InvalidFixNumber,
                        token.start,
                        token.end,
                    ));
                    return None;
                }
            };

            number.ensure_type(TokenType::Number, || {
                context.errors.add(ParseErrorBuilder::new(
                    ParseErrorType::InvalidFixNumber,
                    number.start,
                    number.end,
                ));
            })?;

            return Some(Operant {
                value: number,
                kind: OperantKind::Fixed,
            });
        }
        TokenType::ParenthesisOpen => {
            let number_token = match cursor.next() {
                Some(t) => t,
                None => {
                    context.errors.add(ParseErrorBuilder::new(
                        ParseErrorType::InvalidAddress,
                        token.start,
                        token.end,
                    ));
                    return None;
                }
            };

            number_token.ensure_type(TokenType::Number, || {
                context.errors.add(ParseErrorBuilder::new(
                    ParseErrorType::InvalidAddress,
                    token.start,
                    token.end,
                ));
            })?;

            let close = match cursor.next() {
                Some(t) => t,
                None => {
                    context.errors.add(ParseErrorBuilder::new(
                        ParseErrorType::MissingParenthesisClose,
                        token.start,
                        number_token.end,
                    ));
                    return None;
                }
            };

            close.ensure_type(TokenType::ParenthesisClose, || {
                context.errors.add(ParseErrorBuilder::new(
                    ParseErrorType::InvalidAddress,
                    token.start,
                    number_token.end,
                ));
            })?;

            return Some(Operant {
                value: number_token,
                kind: OperantKind::Address,
            });
        }
        _ => {
            return None;
        }
    }
}
