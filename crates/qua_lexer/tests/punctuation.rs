#[macro_use]
mod common;

use qua_lexer::token::Token;

#[test]
fn test_punctuation_semicolon() {
    assert_tokens_eq!(";", vec![Token::Semicolon, Token::EOF]);
}

#[test]
fn test_punctuation_multiple_semicolon() {
    assert_tokens_eq!(
        ";; ;",
        vec![
            Token::Semicolon,
            Token::Semicolon,
            Token::Semicolon,
            Token::EOF,
        ]
    );
}

#[test]
fn test_punctuation_comma() {
    assert_tokens_eq!(",", vec![Token::Comma, Token::EOF]);
}

#[test]
fn test_punctuation_multiple_comma() {
    assert_tokens_eq!(
        ",, ,",
        vec![Token::Comma, Token::Comma, Token::Comma, Token::EOF]
    );
}

#[test]
fn test_punctuation_period() {
    assert_tokens_eq!(".", vec![Token::Period, Token::EOF]);
}

#[test]
fn test_punctuation_multiple_period() {
    assert_tokens_eq!(
        ".. .",
        vec![Token::Period, Token::Period, Token::Period, Token::EOF]
    );
}
