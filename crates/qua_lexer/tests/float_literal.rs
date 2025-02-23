#[macro_use]
mod common;

use qua_lexer::token::Token;

#[test]
fn test_float_literal_zero() {
    assert_tokens_eq!("0.0", vec![Token::FloatLiteral(0.0), Token::EOF]);
}

#[test]
fn test_float_literal_decimal() {
    assert_tokens_eq!("0.1", vec![Token::FloatLiteral(0.1), Token::EOF]);
}

#[test]
fn test_float_literal_one() {
    assert_tokens_eq!("1.0", vec![Token::FloatLiteral(1.0), Token::EOF]);
}

#[test]
fn test_float_literal_one_point_one() {
    assert_tokens_eq!("1.1", vec![Token::FloatLiteral(1.1), Token::EOF]);
}

#[test]
fn test_float_literal_pi() {
    assert_tokens_eq!("3.14", vec![Token::FloatLiteral(3.14), Token::EOF]);
}

#[test]
fn test_float_literal_half() {
    assert_tokens_eq!("0.5", vec![Token::FloatLiteral(0.5), Token::EOF]);
}

#[test]
fn test_float_literal_large_fraction() {
    assert_tokens_eq!("123.456", vec![Token::FloatLiteral(123.456), Token::EOF]);
}

#[test]
fn test_float_literal_long_decimal() {
    assert_tokens_eq!(
        "3.141592653589793238462643383279502884197",
        vec![
            Token::FloatLiteral(3.141592653589793238462643383279502884197),
            Token::EOF,
        ]
    );
}

#[test]
fn test_float_literal_large_integer_part() {
    assert_tokens_eq!("999.999", vec![Token::FloatLiteral(999.999), Token::EOF]);
}

#[test]
fn test_float_literal_leading_zeros() {
    assert_tokens_eq!("00.11", vec![Token::FloatLiteral(0.11), Token::EOF]);
    assert_tokens_eq!("000.111", vec![Token::FloatLiteral(0.111), Token::EOF]);
    assert_tokens_eq!(
        "0000000.19199191",
        vec![Token::FloatLiteral(0.19199191), Token::EOF]
    );
}

#[test]
fn test_float_literal_trailing_dot() {
    assert_tokens_eq!("123.", vec![Token::FloatLiteral(123.), Token::EOF]);
    assert_tokens_eq!("123.", vec![Token::FloatLiteral(123.0), Token::EOF]);
}
