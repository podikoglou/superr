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

            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '/' => tokens.push(Token::Slash),
            '*' => tokens.push(Token::Asterisk),
            '%' => tokens.push(Token::Percent),

            '"' => {
                // we create an empty string buffer in which we'll put
                // the contents (excluding the quotes) of our string
                let mut string_buffer = String::default();

                while let Some(c2) = chars.next() {
                    match c2 {
                        '"' => break,
                        '\\' => todo!("handle backslash"),

                        c2 => string_buffer.push(c2),
                    }
                }

                tokens.push(Token::StringLiteral(string_buffer));
            }

            c => {
                // handle number literals
                if c.is_digit(10) {
                    // we initialize a string buffer for our number with the
                    // character we've just read, which is the first digit
                    let mut num_buffer = String::from(c);

                    // this is a counter for keeping track of how many decimal
                    // points we've read.
                    // (that doesn't mean they won't appear in the buffer)
                    let mut decimal_points: usize = 0;

                    loop {
                        // consume the next character, ONLY if it is a digit or a
                        // decimal point
                        match chars.next_if(|x| x.is_digit(10) || x == &'.') {
                            Some(c2) => {
                                // if the character is a decimal point, we increase
                                // the counter.
                                if c2 == '.' {
                                    decimal_points += 1;
                                }

                                // if it's a digit, we just add it to the buffer
                                num_buffer += &c2.to_string();
                            }

                            // if the next character isn't a digit or decimal
                            // point, we break out of this loop and finalize the
                            // token.
                            None => break,
                        }
                    }

                    // if we've read up to 1 decimal point, the number is (probably) fine and
                    // we can parse it as a number using rust's std library
                    if decimal_points <= 1 {
                        // if we don't have *any* decimal points, we parse it as an integer
                        if decimal_points == 0 {
                            match num_buffer.parse::<u32>() {
                                Ok(num) => tokens.push(Token::IntLiteral(num)),
                                Err(_) => tokens.push(Token::Invalid(num_buffer)),
                            }
                        } else {
                            // if we *do* have a decimal point, we parse it as a float
                            match num_buffer.parse::<f32>() {
                                Ok(num) => tokens.push(Token::FloatLiteral(num)),
                                Err(_) => tokens.push(Token::Invalid(num_buffer)),
                            }
                        }
                    } else {
                        // if there's more than one decimal point, we can now
                        // emit an invalid token
                        tokens.push(Token::Invalid(num_buffer));
                    }
                } else if c.is_ascii_alphabetic() || c == '_' {
                    // handle identifiers

                    let mut ident_buf = String::from(c);

                    loop {
                        match chars.next_if(|x| x.is_alphanumeric() || x == &'_') {
                            Some(c2) => ident_buf += &c2.to_string(),
                            None => {
                                tokens.push(Token::Identifier(ident_buf));
                                break;
                            }
                        }
                    }
                }
            }
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

    macro_rules! assert_identifier_correct_lexing_eq {
        ($input:expr) => {
            assert_tokens_eq!(
                $input,
                vec![Token::Identifier($input.to_string()), Token::EOF]
            );
        };
    }

    macro_rules! assert_identifier_correct_lexing_ne {
        ($input:expr) => {
            assert_tokens_ne!(
                $input,
                vec![Token::Identifier($input.to_string()), Token::EOF]
            );
        };
    }

    #[test]
    fn test_identifiers() {
        // user defined identifiers
        assert_identifier_correct_lexing_eq!("qua");
        assert_identifier_correct_lexing_eq!("Qua");
        assert_identifier_correct_lexing_eq!("x");
        assert_identifier_correct_lexing_eq!("x_");
        assert_identifier_correct_lexing_eq!("_x");
        assert_identifier_correct_lexing_eq!("____________________Qua_");

        assert_identifier_correct_lexing_ne!("____________________Qua!_");
        assert_identifier_correct_lexing_ne!("!Qua__");
        assert_identifier_correct_lexing_ne!("*qua");

        // qua keywords
        assert_identifier_correct_lexing_eq!("for");
        assert_identifier_correct_lexing_eq!("in");
        assert_identifier_correct_lexing_eq!("if");
        assert_identifier_correct_lexing_eq!("else");
        assert_identifier_correct_lexing_eq!("while");
        assert_identifier_correct_lexing_eq!("range");

        assert_identifier_correct_lexing_eq!("int");
        assert_identifier_correct_lexing_eq!("string");

        assert_identifier_correct_lexing_eq!("print");
    }

    #[test]
    fn test_int_literal() {
        assert_tokens_eq!("0", vec![Token::IntLiteral(0), Token::EOF]);
        assert_tokens_eq!("1", vec![Token::IntLiteral(1), Token::EOF]);
        assert_tokens_eq!("123", vec![Token::IntLiteral(123), Token::EOF]);
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
        assert_tokens_eq!("3.14", vec![Token::FloatLiteral(3.14), Token::EOF]);
        assert_tokens_eq!("0.5", vec![Token::FloatLiteral(0.5), Token::EOF]);
        assert_tokens_eq!("123.456", vec![Token::FloatLiteral(123.456), Token::EOF]);
        assert_tokens_eq!(
            "3.141592653589793238462643383279502884197",
            vec![
                Token::FloatLiteral(3.141592653589793238462643383279502884197),
                Token::EOF,
            ]
        );
        assert_tokens_eq!("999.999", vec![Token::FloatLiteral(999.999), Token::EOF]);
        assert_tokens_eq!("00.11", vec![Token::FloatLiteral(0.11), Token::EOF]);
        assert_tokens_eq!("000.111", vec![Token::FloatLiteral(0.111), Token::EOF]);
        assert_tokens_eq!(
            "0000000.19199191",
            vec![Token::FloatLiteral(0.19199191), Token::EOF]
        );

        // should this be supported?
        // assert_tokens_eq!(".123", vec![Token::FloatLiteral(0.123), Token::EOF]);

        assert_tokens_eq!("123.", vec![Token::FloatLiteral(123.), Token::EOF]);
        assert_tokens_eq!("123.", vec![Token::FloatLiteral(123.0), Token::EOF]);
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
        assert_tokens_eq!("//", vec![Token::Slash, Token::Slash, Token::EOF]);

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
