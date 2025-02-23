pub mod char_literal;
pub mod comments;
pub mod delimeter;
pub mod float_literal;
pub mod identifier;
pub mod int_literal;
pub mod keyword;
pub mod logical_ops;
pub mod math_ops;
pub mod punctuation;
pub mod string_literal;
pub mod unicode_char_literal;

macro_rules! assert_tokens_eq {
    ($input:expr, $expected:expr) => {
        let result = crate::lexer::lex($input);
        assert_eq!(result, $expected, "Input: `{}`", $input);
    };
}

macro_rules! assert_tokens_ne {
    ($input:expr, $expected:expr) => {
        let result = crate::lexer::lex($input);
        assert_ne!(result, $expected, "Input: `{}`", $input);
    };
}

macro_rules! assert_identifier_correct_lexing_eq {
    ($input:expr) => {
        crate::tests::assert_tokens_eq!(
            $input,
            vec![
                crate::token::Token::Identifier($input.to_string()),
                crate::token::Token::EOF
            ]
        );
    };
}

macro_rules! assert_identifier_correct_lexing_ne {
    ($input:expr) => {
        crate::tests::assert_tokens_ne!(
            $input,
            vec![
                crate::token::Token::Identifier($input.to_string()),
                crate::token::Token::EOF
            ]
        );
    };
}

macro_rules! assert_unicode_char_literal_properly_decoded {
    ($input:expr, $expected_value:expr) => {
        assert_tokens_eq!(
            $input,
            vec![
                crate::token::Token::CharLiteral(char::from_u32($expected_value).unwrap()),
                crate::token::Token::EOF
            ]
        );
    };
}

pub(crate) use assert_identifier_correct_lexing_eq;
pub(crate) use assert_identifier_correct_lexing_ne;
pub(crate) use assert_tokens_eq;
pub(crate) use assert_tokens_ne;
pub(crate) use assert_unicode_char_literal_properly_decoded;
