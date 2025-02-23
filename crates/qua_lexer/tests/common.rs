macro_rules! assert_tokens_eq {
    ($input:expr, $expected:expr) => {
        let result = qua_lexer::lexer::lex($input);
        assert_eq!(result, $expected, "Input: `{}`", $input);
    };
}

macro_rules! assert_tokens_ne {
    ($input:expr, $expected:expr) => {
        let result = qua_lexer::lexer::lex($input);
        assert_ne!(result, $expected, "Input: `{}`", $input);
    };
}

macro_rules! assert_identifier_correct_lexing_eq {
    ($input:expr) => {
        assert_tokens_eq!(
            $input,
            vec![
                qua_lexer::token::Token::Identifier($input.to_string()),
                qua_lexer::token::Token::EOF
            ]
        );
    };
}

macro_rules! assert_identifier_correct_lexing_ne {
    ($input:expr) => {
        assert_tokens_ne!(
            $input,
            vec![
                qua_lexer::token::Token::Identifier($input.to_string()),
                qua_lexer::token::Token::EOF
            ]
        );
    };
}

macro_rules! assert_unicode_char_literal_properly_decoded {
    ($input:expr, $expected_value:expr) => {
        assert_tokens_eq!(
            $input,
            vec![
                qua_lexer::token::Token::CharLiteral(char::from_u32($expected_value).unwrap()),
                qua_lexer::token::Token::EOF
            ]
        );
    };
}
