#[macro_use]
mod common;

use qua_lexer::token::Token;

#[test]
fn test_comment_empty() {
    assert_tokens_eq!("//", vec![Token::EOF]);
}

#[test]
fn test_comment_text() {
    assert_tokens_eq!("//hello, world!", vec![Token::EOF]);
}

#[test]
fn test_comment_text_with_space() {
    assert_tokens_eq!("// hello, world!", vec![Token::EOF]);
}

#[test]
fn test_comment_code_after_newline() {
    assert_tokens_eq!(
        "// hello, world!\nint a = 0;",
        vec![
            Token::Identifier("int".to_string()),
            Token::Identifier("a".to_string()),
            Token::Equals,
            Token::IntLiteral(0),
            Token::Semicolon,
            Token::EOF
        ]
    );
}

#[test]
fn test_comment_code_inline_comment() {
    assert_tokens_eq!(
        "// Qua!\nint a = 0;// this creates a variable named int",
        vec![
            Token::Identifier("int".to_string()),
            Token::Identifier("a".to_string()),
            Token::Equals,
            Token::IntLiteral(0),
            Token::Semicolon,
            Token::EOF
        ]
    );
}
