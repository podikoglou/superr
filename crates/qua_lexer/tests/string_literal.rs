#[macro_use]
mod common;

use qua_lexer::token::Token;

#[test]
fn test_string_literal_non_empty() {
    assert_tokens_eq!(
        "\"Qua!\"",
        vec![Token::StringLiteral("Qua!".to_string()), Token::EOF]
    );
}

#[test]
fn test_string_literal_empty() {
    assert_tokens_eq!(
        "\"\"",
        vec![Token::StringLiteral("".to_string()), Token::EOF]
    );
}
