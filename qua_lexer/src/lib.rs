// TODO:
//  - char literal
//  - ++, --, +=, *=, /=
#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String), // factorial, for, int

    // Literals
    IntLiteral(u32),       // 21
    FloatLiteral(f32),     // 3.14
    StringLiteral(String), // "Qua!"

    // Delimiters
    OpenParen,  // (
    CloseParen, // )

    OpenBrace,  // {
    CloseBrace, // }

    OpenBracket,  // [
    CloseBracket, // ]

    // Logical Operators
    And, // &&
    Or,  // ||
    Not, // !

    // Punctuation
    Semicolon, // ;
    Comma,     // ,
    Period,    // .

    // Math
    Equals,       // =
    EqualsEquals, // ==
    NotEquals,    // !=
    Greater,      // >
    Lesser,       // <
    GreaterEq,    // >=
    LesserEq,     // <=

    Plus,     // +
    Minus,    // -
    Slash,    // /
    Asterisk, // *
    Percent,  // %

    // Special
    EOF,
    Invalid(String),
}

/// Lexically analyzes a Qua file's textual contents
pub fn lex(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    // state
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),

            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),

            '[' => tokens.push(Token::OpenBracket),
            ']' => tokens.push(Token::CloseBracket),

            ';' => tokens.push(Token::Semicolon),
            ',' => tokens.push(Token::Comma),
            '.' => tokens.push(Token::Period),

            '&' => {
                if let Some(_) = chars.next_if_eq(&'&') {
                    tokens.push(Token::And)
                }
            }

            '|' => {
                if let Some(_) = chars.next_if_eq(&'|') {
                    tokens.push(Token::Or)
                }
            }

            '!' => {
                if let Some(_) = chars.next_if_eq(&'=') {
                    tokens.push(Token::NotEquals)
                } else {
                    tokens.push(Token::Not)
                }
            }

            '=' => {
                if let Some(_) = chars.next_if_eq(&'=') {
                    tokens.push(Token::EqualsEquals)
                } else {
                    tokens.push(Token::Equals)
                }
            }

            '>' => {
                if let Some(_) = chars.next_if_eq(&'=') {
                    tokens.push(Token::GreaterEq)
                } else {
                    tokens.push(Token::Greater)
                }
            }

            '<' => {
                if let Some(_) = chars.next_if_eq(&'=') {
                    tokens.push(Token::LesserEq)
                } else {
                    tokens.push(Token::Lesser)
                }
            }

            _ => {}
        }
    }

    tokens.push(Token::EOF);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_tokens_eq {
        ($input:expr, $expected:expr) => {
            let result = lex($input.to_string());
            assert_eq!(result, $expected, "Input: `{}`", $input);
        };
    }

    macro_rules! assert_tokens_ne {
        ($input:expr, $expected:expr) => {
            let result = lex($input.to_string());
            assert_ne!(result, $expected, "Input: `{}`", $input);
        };
    }

    macro_rules! assert_identifier_correct_lexing {
        ($input:expr) => {
            assert_tokens_eq!(
                $input,
                vec![Token::Identifier($input.to_string()), Token::EOF]
            );
        };
    }

    #[test]
    fn test_identifiers() {
        // user defined identifiers
        assert_identifier_correct_lexing!("qua");
        assert_identifier_correct_lexing!("Qua");
        assert_identifier_correct_lexing!("x");

        // qua keywords
        assert_identifier_correct_lexing!("for");
        assert_identifier_correct_lexing!("in");
        assert_identifier_correct_lexing!("if");
        assert_identifier_correct_lexing!("else");
        assert_identifier_correct_lexing!("while");
        assert_identifier_correct_lexing!("range");

        assert_identifier_correct_lexing!("int");
        assert_identifier_correct_lexing!("string");

        assert_identifier_correct_lexing!("print");
    }

    #[test]
    fn test_int_literal() {
        assert_tokens_eq!("0", vec![Token::IntLiteral(0), Token::EOF]);
        assert_tokens_eq!("1", vec![Token::IntLiteral(1), Token::EOF]);
        assert_tokens_eq!(
            "4294967294",
            vec![Token::IntLiteral(4294967294), Token::EOF]
        );
        assert_tokens_eq!(
            "4294967295",
            vec![Token::IntLiteral(4294967295), Token::EOF]
        );
    }

    #[test]
    fn test_float_literal() {
        assert_tokens_eq!("0.0", vec![Token::FloatLiteral(0.0), Token::EOF]);
        assert_tokens_eq!("0.1", vec![Token::FloatLiteral(0.1), Token::EOF]);
        assert_tokens_eq!("1.0", vec![Token::FloatLiteral(1.0), Token::EOF]);
        assert_tokens_eq!("1.1", vec![Token::FloatLiteral(1.1), Token::EOF]);
        assert_tokens_eq!(
            "3.141592653589793238462643383279502884197",
            vec![
                Token::FloatLiteral(3.141592653589793238462643383279502884197),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_string_literal() {
        assert_tokens_eq!(
            "\"Qua!\"",
            vec![Token::StringLiteral("Qua!".to_string()), Token::EOF]
        );

        assert_tokens_eq!(
            "\"\"",
            vec![Token::StringLiteral("".to_string()), Token::EOF]
        );
    }

    #[test]
    fn test_delimiters() {
        // parentheses
        assert_tokens_eq!("(", vec![Token::OpenParen, Token::EOF]);
        assert_tokens_eq!(")", vec![Token::CloseParen, Token::EOF]);

        assert_tokens_eq!("()", vec![Token::OpenParen, Token::CloseParen, Token::EOF]);
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
        assert_tokens_eq!(
            "(()",
            vec![
                Token::OpenParen,
                Token::OpenParen,
                Token::CloseParen,
                Token::EOF,
            ]
        );

        // braces
        assert_tokens_eq!("{", vec![Token::OpenBrace, Token::EOF]);
        assert_tokens_eq!("}", vec![Token::CloseBrace, Token::EOF]);

        assert_tokens_eq!("{}", vec![Token::OpenBrace, Token::CloseBrace, Token::EOF]);
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
        assert_tokens_eq!(
            "{{}",
            vec![
                Token::OpenBrace,
                Token::OpenBrace,
                Token::CloseBrace,
                Token::EOF,
            ]
        );

        // brackets
        assert_tokens_eq!("[", vec![Token::OpenBracket, Token::EOF]);
        assert_tokens_eq!("]", vec![Token::CloseBracket, Token::EOF]);

        assert_tokens_eq!(
            "[]",
            vec![Token::OpenBracket, Token::CloseBracket, Token::EOF]
        );
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

    #[test]
    fn test_logical_operators() {
        // and
        assert_tokens_eq!("&&", vec![Token::And, Token::EOF]);
        assert_tokens_eq!(
            "&& &&&&",
            vec![Token::And, Token::And, Token::And, Token::EOF]
        );

        // or
        assert_tokens_eq!("||", vec![Token::Or, Token::EOF]);
        assert_tokens_eq!("|| ||||", vec![Token::Or, Token::Or, Token::Or, Token::EOF]);

        // not
        assert_tokens_eq!("!", vec![Token::Not, Token::EOF]);
        assert_tokens_eq!("!!", vec![Token::Not, Token::Not, Token::EOF]);
    }

    #[test]
    fn test_punctuation() {
        // semicolon
        assert_tokens_eq!(";", vec![Token::Semicolon, Token::EOF]);
        assert_tokens_eq!(
            ";; ;",
            vec![
                Token::Semicolon,
                Token::Semicolon,
                Token::Semicolon,
                Token::EOF,
            ]
        );

        // comma
        assert_tokens_eq!(",", vec![Token::Comma, Token::EOF]);
        assert_tokens_eq!(
            ",, ,",
            vec![Token::Comma, Token::Comma, Token::Comma, Token::EOF]
        );

        // period
        assert_tokens_eq!(".", vec![Token::Period, Token::EOF]);
        assert_tokens_eq!(
            ".. .",
            vec![Token::Period, Token::Period, Token::Period, Token::EOF]
        );
    }

    #[test]
    fn test_math() {
        // equals
        assert_tokens_eq!("=", vec![Token::Equals, Token::EOF]);
        assert_tokens_eq!("= =", vec![Token::Equals, Token::Equals, Token::EOF]);

        // equals equals
        assert_tokens_eq!("==", vec![Token::EqualsEquals, Token::EOF]);

        // not equals
        assert_tokens_eq!("!=", vec![Token::NotEquals, Token::EOF]);
        // TODO: how do we want to lex '!=='?

        // greater
        assert_tokens_eq!(">", vec![Token::Greater, Token::EOF]);

        // lesser
        assert_tokens_eq!("<", vec![Token::Lesser, Token::EOF]);

        // greater or equal
        assert_tokens_eq!(">=", vec![Token::GreaterEq, Token::EOF]);

        // lesser or equal
        assert_tokens_eq!("<=", vec![Token::LesserEq, Token::EOF]);

        // plus
        assert_tokens_eq!("+", vec![Token::Plus, Token::EOF]);
        assert_tokens_eq!(
            "++ +",
            vec![Token::Plus, Token::Plus, Token::Plus, Token::EOF]
        );

        // minus
        assert_tokens_eq!("-", vec![Token::Minus, Token::EOF]);
        assert_tokens_eq!(
            "-- -",
            vec![Token::Minus, Token::Minus, Token::Minus, Token::EOF]
        );

        // slash
        assert_tokens_eq!("/", vec![Token::Slash, Token::EOF]);
        assert_tokens_eq!("/ /", vec![Token::Slash, Token::Slash, Token::EOF]);
        assert_tokens_ne!("//", vec![Token::Slash, Token::Slash, Token::EOF]);

        // asterisk
        assert_tokens_eq!("*", vec![Token::Asterisk, Token::EOF]);
        assert_tokens_eq!(
            "* **",
            vec![
                Token::Asterisk,
                Token::Asterisk,
                Token::Asterisk,
                Token::EOF,
            ]
        );

        // percent
        assert_tokens_eq!("%", vec![Token::Percent, Token::EOF]);
        assert_tokens_eq!(
            "%% %",
            vec![Token::Percent, Token::Percent, Token::Percent, Token::EOF]
        );
    }
}
