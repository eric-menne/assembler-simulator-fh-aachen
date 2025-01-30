use crate::frontend::ParseContext;

use super::{tokenize, TokenType};

#[test]
fn test_tokenize() {
    let token_test_code: &str = "
low: ADD (2)
     ADD (15) // test
     BRC up:
up:  STA (15)
     LDA #1
";
    let token_expected_token: [TokenType; 29] = [
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

    let text = String::from(token_test_code);
    let mut context = ParseContext::new_empty();
    let token = tokenize(&text, &mut context);
    assert_eq!(token.len(), token_expected_token.len());

    for (index, t) in token.iter().enumerate() {
        assert_eq!(t.token_type, token_expected_token[index])
    }
}

#[test]
fn test_tokenize_single_line() {
    let token_test_code: &str = "ADD (2)";

    let token_expected_token: [TokenType; 5] = [
        TokenType::Symbol,
        TokenType::ParenthesisOpen,
        TokenType::Number,
        TokenType::ParenthesisClose,
        TokenType::End
    ];

    let text = String::from(token_test_code);
    let mut context = ParseContext::new_empty();
    let token = tokenize(&text, &mut context);
    assert_eq!(token.len(), token_expected_token.len());

    for (index, t) in token.iter().enumerate() {
        assert_eq!(t.token_type, token_expected_token[index])
    }
}


#[test]
fn test_tokenize_empty() {
    let token_test_code: &str = "";

    let token_expected_token: [TokenType; 1] = [
        TokenType::End
    ];

    let text = String::from(token_test_code);
    let mut context = ParseContext::new_empty();
    let token = tokenize(&text, &mut context);
    assert_eq!(token.len(), token_expected_token.len());

    for (index, t) in token.iter().enumerate() {
        assert_eq!(t.token_type, token_expected_token[index])
    }
}
