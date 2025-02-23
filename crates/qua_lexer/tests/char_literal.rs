#[macro_use]
mod common;

use qua_lexer::token::Token;

#[test]
fn test_char_literal_valid_char() {
    assert_tokens_eq!("'a'", vec![Token::CharLiteral('a'), Token::EOF]);
}

#[test]
fn test_char_literal_digit() {
    assert_tokens_eq!("'4'", vec![Token::CharLiteral('4'), Token::EOF]);
}

#[test]
fn test_char_literal_invalid_missing_start_quote() {
    assert_tokens_ne!("a'", vec![Token::CharLiteral('a'), Token::EOF]);
}

#[test]
fn test_char_literal_invalid_missing_end_quote() {
    assert_tokens_ne!("'a", vec![Token::CharLiteral('a'), Token::EOF]);
}

#[test]
fn test_char_literal_invalid_no_quotes() {
    assert_tokens_ne!("a", vec![Token::CharLiteral('a'), Token::EOF]);
}
