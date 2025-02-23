#[macro_use]
mod common;

use qua_lexer::token::Token;

#[test]
fn test_delimiter_open_paren() {
    assert_tokens_eq!("(", vec![Token::OpenParen, Token::EOF]);
}

#[test]
fn test_delimiter_close_paren() {
    assert_tokens_eq!(")", vec![Token::CloseParen, Token::EOF]);
}

#[test]
fn test_delimiter_parens_pair() {
    assert_tokens_eq!("()", vec![Token::OpenParen, Token::CloseParen, Token::EOF]);
}

#[test]
fn test_delimiter_nested_parens() {
    assert_tokens_eq!(
        "(())",
        vec![
            Token::OpenParen,
            Token::OpenParen,
            Token::CloseParen,
            Token::CloseParen,
            Token::EOF,
        ]
    );
}

#[test]
fn test_delimiter_unbalanced_parens() {
    assert_tokens_eq!(
        "(()",
        vec![
            Token::OpenParen,
            Token::OpenParen,
            Token::CloseParen,
            Token::EOF,
        ]
    );
}

#[test]
fn test_delimiter_open_brace() {
    assert_tokens_eq!("{", vec![Token::OpenBrace, Token::EOF]);
}

#[test]
fn test_delimiter_close_brace() {
    assert_tokens_eq!("}", vec![Token::CloseBrace, Token::EOF]);
}

#[test]
fn test_delimiter_braces_pair() {
    assert_tokens_eq!("{}", vec![Token::OpenBrace, Token::CloseBrace, Token::EOF]);
}

#[test]
fn test_delimiter_nested_braces() {
    assert_tokens_eq!(
        "{{}}",
        vec![
            Token::OpenBrace,
            Token::OpenBrace,
            Token::CloseBrace,
            Token::CloseBrace,
            Token::EOF,
        ]
    );
}

#[test]
fn test_delimiter_unbalanced_braces() {
    assert_tokens_eq!(
        "{{}",
        vec![
            Token::OpenBrace,
            Token::OpenBrace,
            Token::CloseBrace,
            Token::EOF,
        ]
    );
}

#[test]
fn test_delimiter_open_bracket() {
    assert_tokens_eq!("[", vec![Token::OpenBracket, Token::EOF]);
}

#[test]
fn test_delimiter_close_bracket() {
    assert_tokens_eq!("]", vec![Token::CloseBracket, Token::EOF]);
}

#[test]
fn test_delimiter_brackets_pair() {
    assert_tokens_eq!(
        "[]",
        vec![Token::OpenBracket, Token::CloseBracket, Token::EOF]
    );
}

#[test]
fn test_delimiter_nested_brackets() {
    assert_tokens_eq!(
        "[[]]",
        vec![
            Token::OpenBracket,
            Token::OpenBracket,
            Token::CloseBracket,
            Token::CloseBracket,
            Token::EOF,
        ]
    );
}

#[test]
fn test_delimiter_unbalanced_brackets() {
    assert_tokens_eq!(
        "[[]",
        vec![
            Token::OpenBracket,
            Token::OpenBracket,
            Token::CloseBracket,
            Token::EOF,
        ]
    );
}
