#[macro_use]
mod common;

use qua_lexer::token::Token;

#[test]
fn test_int_literal_zero() {
    assert_tokens_eq!("0", vec![Token::IntLiteral(0), Token::EOF]);
}

#[test]
fn test_int_literal_single_digit() {
    assert_tokens_eq!("1", vec![Token::IntLiteral(1), Token::EOF]);
}

#[test]
fn test_int_literal_multi_digit() {
    assert_tokens_eq!("123", vec![Token::IntLiteral(123), Token::EOF]);
}

#[test]
fn test_int_literal_max_u32_minus_one() {
    assert_tokens_eq!(
        "4294967294",
        vec![Token::IntLiteral(4294967294), Token::EOF]
    );
}

#[test]
fn test_int_literal_max_u32() {
    assert_tokens_eq!(
        "4294967295",
        vec![Token::IntLiteral(4294967295), Token::EOF]
    );
}
