#[macro_use]
mod common;

use qua_lexer::{keyword::Keyword, token::Token};

#[test]
fn test_keyword_if() {
    assert_tokens_eq!("if", vec![Token::Keyword(Keyword::If), Token::EOF]);
}

#[test]
fn test_keyword_else() {
    assert_tokens_eq!("else", vec![Token::Keyword(Keyword::Else), Token::EOF]);
}

#[test]
fn test_keyword_for() {
    assert_tokens_eq!("for", vec![Token::Keyword(Keyword::For), Token::EOF]);
}

#[test]
fn test_keyword_while() {
    assert_tokens_eq!("while", vec![Token::Keyword(Keyword::While), Token::EOF]);
}

#[test]
fn test_keyword_return() {
    assert_tokens_eq!("return", vec![Token::Keyword(Keyword::Return), Token::EOF]);
}

#[test]
fn test_keyword_break() {
    assert_tokens_eq!("break", vec![Token::Keyword(Keyword::Break), Token::EOF]);
}

#[test]
fn test_keyword_continue() {
    assert_tokens_eq!(
        "continue",
        vec![Token::Keyword(Keyword::Continue), Token::EOF]
    );
}

#[test]
fn test_keyword_in() {
    assert_tokens_eq!("in", vec![Token::Keyword(Keyword::In), Token::EOF]);
}
