#[macro_use]
mod common;

use qua_lexer::token::Token;

#[test]
fn test_logical_and() {
    assert_tokens_eq!("&&", vec![Token::And, Token::EOF]);
}

#[test]
fn test_logical_multiple_and() {
    assert_tokens_eq!(
        "&& &&&&",
        vec![Token::And, Token::And, Token::And, Token::EOF]
    );
}

#[test]
fn test_logical_or() {
    assert_tokens_eq!("||", vec![Token::Or, Token::EOF]);
}

#[test]
fn test_logical_multiple_or() {
    assert_tokens_eq!("|| ||||", vec![Token::Or, Token::Or, Token::Or, Token::EOF]);
}

#[test]
fn test_logical_not() {
    assert_tokens_eq!("!", vec![Token::Not, Token::EOF]);
}

#[test]
fn test_logical_multiple_not() {
    assert_tokens_eq!("!!", vec![Token::Not, Token::Not, Token::EOF]);
}
