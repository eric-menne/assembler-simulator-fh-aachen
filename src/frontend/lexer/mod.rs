use super::{LineInfo, LineTable};
use std::{iter::Peekable, usize};
use token::{Token, TokenType};

#[cfg(test)]
mod test;
pub mod token;

pub(super) fn tokenize(text: &str, line_table: &mut LineTable) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut line_start: usize = 0;
    let mut counter: usize = 0;
    let mut cursor = text.chars().enumerate().peekable();

    while let Some((index, c)) = cursor.next() {
        counter += 1;
        let token = match c {
            '#' => Token::new_single(TokenType::Hash, index),
            '(' => Token::new_single(TokenType::ParenthesisOpen, index),
            ')' => Token::new_single(TokenType::ParenthesisClose, index),
            ':' => Token::new_single(TokenType::Colon, index),
            '\n' => {
                line_table.push(LineInfo {
                    start: line_start,
                    end: index,
                });
                line_start = index;
                Token::new_single(TokenType::NewLine, index)
            }
            '/' => {
                let token = check_for_comment(&mut cursor, index);
                if token.token_type == TokenType::NewLine {
                    line_table.push(LineInfo {
                        start: line_start,
                        end: index,
                    });
                    line_start = index;
                }

                token
            }
            c if c.is_ascii_digit() => get_number(&mut cursor, index),
            c if c.is_alphabetic() => get_symbol(&mut cursor, index),
            c if c.is_whitespace() => continue,
            _ => Token::new_single(TokenType::Invalid, index),
        };

        tokens.push(token);
    }

    line_table.push(LineInfo {
        start: line_start,
        end: counter,
    });

    tokens.push(Token::new_single(TokenType::End, text.len()));
    tokens
}

fn check_for_comment<I>(cursor: &mut Peekable<I>, position: usize) -> Token
where
    I: Iterator<Item = (usize, char)>,
{
    if let Some((_, next)) = cursor.peek() {
        if *next == '/' {
            cursor.next();
            let mut new_line_pos = position + 1;
            while let Some((index, c)) = cursor.next() {
                new_line_pos = index;
                if c == '\n' {
                    break;
                }
            }
            return Token::new_single(TokenType::NewLine, new_line_pos);
        }
    }
    Token::new_single(TokenType::Invalid, position)
}

fn get_symbol<I>(cursor: &mut Peekable<I>, start: usize) -> Token
where
    I: Iterator<Item = (usize, char)>,
{
    let mut end = start;

    while let Some((index, c)) = cursor.peek() {
        match c {
            '_' => (),
            c if c.is_alphabetic() => (),
            _ => {
                end = *index;
                break;
            }
        }
        cursor.next();
    }

    Token::new_multi(TokenType::Symbol, start, end)
}

fn get_number<I>(cursor: &mut Peekable<I>, start: usize) -> Token
where
    I: Iterator<Item = (usize, char)>,
{
    let mut end = start;

    while let Some((index, c)) = cursor.peek() {
        match c {
            c if c.is_ascii_digit() => (),
            _ => {
                end = *index;
                break;
            }
        }
        cursor.next();
    }
    if start == end {
        return Token::new_multi(TokenType::Number, start, start + 1);
    }

    Token::new_multi(TokenType::Number, start, end)
}
