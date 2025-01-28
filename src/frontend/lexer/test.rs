use crate::frontend::LineTable;

use super::{tokenize, TokenType};

const TOKEN_TEST_CODE: &str = "
low: ADD (2)
     ADD (15) // test
     BRC up:
up:  STA (15)
     LDA #1
";

const TOKEN_EXPECTED_TOKEN: [TokenType; 29] = [
    TokenType::NewLine,
    TokenType::Symbol,
    TokenType::Colon,
    TokenType::Symbol,
    TokenType::ParenthesisOpen,
    TokenType::Number,
    TokenType::ParenthesisClose,
    TokenType::NewLine,
    TokenType::Symbol,
    TokenType::ParenthesisOpen,
    TokenType::Number,
    TokenType::ParenthesisClose,
    TokenType::NewLine,
    TokenType::Symbol,
    TokenType::Symbol,
    TokenType::Colon,
    TokenType::NewLine,
    TokenType::Symbol,
    TokenType::Colon,
    TokenType::Symbol,
    TokenType::ParenthesisOpen,
    TokenType::Number,
    TokenType::ParenthesisClose,
    TokenType::NewLine,
    TokenType::Symbol,
    TokenType::Hash,
    TokenType::Number,
    TokenType::NewLine,
    TokenType::End,
];

#[test]
fn test_tokenize() {
    let text = String::from(TOKEN_TEST_CODE);
    let mut line_table = LineTable::new();
    let token = tokenize(&text, &mut line_table);
    assert_eq!(token.len(), TOKEN_EXPECTED_TOKEN.len());

    for (index, t) in token.iter().enumerate() {
        assert_eq!(t.token_type, TOKEN_EXPECTED_TOKEN[index])
    }
}
