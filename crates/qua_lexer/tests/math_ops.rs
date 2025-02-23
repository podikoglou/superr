#[macro_use]
mod common;

use qua_lexer::token::Token;

#[test]
fn test_math_equals() {
    assert_tokens_eq!("=", vec![Token::Equals, Token::EOF]);
}

#[test]
fn test_math_multiple_equals() {
    assert_tokens_eq!("= =", vec![Token::Equals, Token::Equals, Token::EOF]);
}

#[test]
fn test_math_equals_equals() {
    assert_tokens_eq!("==", vec![Token::EqualsEquals, Token::EOF]);
}

#[test]
fn test_math_not_equals() {
    assert_tokens_eq!("!=", vec![Token::NotEquals, Token::EOF]);
}

#[test]
fn test_math_greater() {
    assert_tokens_eq!(">", vec![Token::Greater, Token::EOF]);
}

#[test]
fn test_math_lesser() {
    assert_tokens_eq!("<", vec![Token::Lesser, Token::EOF]);
}

#[test]
fn test_math_greater_equal() {
    assert_tokens_eq!(">=", vec![Token::GreaterEq, Token::EOF]);
}

#[test]
fn test_math_lesser_equal() {
    assert_tokens_eq!("<=", vec![Token::LesserEq, Token::EOF]);
}

#[test]
fn test_math_plus() {
    assert_tokens_eq!("+", vec![Token::Plus, Token::EOF]);
}

#[test]
fn test_math_plus_plus() {
    assert_tokens_eq!("++", vec![Token::PlusPlus, Token::EOF]);
}

#[test]
fn test_math_plus_plus_plus() {
    assert_tokens_eq!("++ +", vec![Token::PlusPlus, Token::Plus, Token::EOF]);
}

#[test]
fn test_math_plus_plus_post() {
    assert_tokens_eq!("+ ++", vec![Token::Plus, Token::PlusPlus, Token::EOF]);
}

#[test]
fn test_math_minus() {
    assert_tokens_eq!("-", vec![Token::Minus, Token::EOF]);
}

#[test]
fn test_math_minus_minus() {
    assert_tokens_eq!("--", vec![Token::MinusMinus, Token::EOF]);
}

#[test]
fn test_math_minus_minus_minus() {
    assert_tokens_eq!("-- -", vec![Token::MinusMinus, Token::Minus, Token::EOF]);
}

#[test]
fn test_math_minus_minus_post() {
    assert_tokens_eq!("- --", vec![Token::Minus, Token::MinusMinus, Token::EOF]);
}

#[test]
fn test_math_slash() {
    assert_tokens_eq!("/", vec![Token::Slash, Token::EOF]);
}

#[test]
fn test_math_multiple_slash() {
    assert_tokens_eq!("/ /", vec![Token::Slash, Token::Slash, Token::EOF]);
}

#[test]
fn test_math_asterisk() {
    assert_tokens_eq!("*", vec![Token::Asterisk, Token::EOF]);
}

#[test]
fn test_math_multiple_asterisk() {
    assert_tokens_eq!(
        "* **",
        vec![
            Token::Asterisk,
            Token::Asterisk,
            Token::Asterisk,
            Token::EOF,
        ]
    );
}

#[test]
fn test_math_percent() {
    assert_tokens_eq!("%", vec![Token::Percent, Token::EOF]);
}

#[test]
fn test_math_multiple_percent() {
    assert_tokens_eq!(
        "%% %",
        vec![Token::Percent, Token::Percent, Token::Percent, Token::EOF]
    );
}
