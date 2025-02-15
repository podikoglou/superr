use std::{char, iter::Peekable, str::Chars};

// TODO:
//  +=, *=, /=
#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String), // factorial, x
    Keyword(Keyword),   // for, in, int, return

    // Literals
    IntLiteral(u32),       // 21
    FloatLiteral(f32),     // 3.14
    StringLiteral(String), // "Qua!"
    CharLiteral(char),     // 'q'

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

    PlusPlus,
    MinusMinus,

    // Special
    EOF,
    Invalid(String),
}

#[derive(PartialEq, Debug)]
pub enum Keyword {
    If,
    Else,
    For,
    While,
    Return,
    Break,
    Continue,
    In,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pub source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            source,
        }
    }
}

impl Lexer<'_> {
    fn lex_string_literal(&mut self) -> Token {
        // we create an empty string buffer in which we'll put
        // the contents (excluding the quotes) of our string
        let mut string_buffer = String::default();

        let mut closed = false;

        while let Some(c2) = self.chars.next() {
            match c2 {
                '"' => {
                    closed = true;
                    break;
                }
                c2 => string_buffer.push(c2),
            }
        }

        match closed {
            true => Token::StringLiteral(string_buffer),
            false => Token::Invalid("Invalid expression".to_string()),
        }
    }

    /// Lexes a character literal
    fn lex_char_literal(&mut self) -> Token {
        // we create an empty string buffer in which we'll put
        // the contents (excluding the quotes) of our character
        //
        // does that sound weird? well after reading we want to
        // ensure that inside this buffer there's *only* one
        // character, and we want to be able to report the
        // whole contents of whatever is in it otherwise.
        //
        // also, when i implement unicode support, multiple
        // characters *will* actually be needed here.
        let mut char_buffer = String::default();
        let mut closed = false;

        while let Some(c2) = self.chars.next() {
            match c2 {
                '\'' => {
                    closed = true;
                    break;
                }
                c2 => char_buffer.push(c2),
            }
        }

        if !closed {
            return Token::Invalid("Invalid expression".to_string());
        }

        if char_buffer.len() > 1 && char_buffer.starts_with('\\') {
            if char_buffer.starts_with("\\u{") && char_buffer.ends_with('}') {
                let hex_str = &char_buffer[3..char_buffer.len() - 1];
                if hex_str.len() > 6 {
                    return Token::Invalid(format!(
                        "Unicode escape sequence '{}' is too long",
                        char_buffer
                    ));
                }
                if let Ok(code_point) = u32::from_str_radix(hex_str, 16) {
                    if let Some(parsed_char) = char::from_u32(code_point) {
                        return Token::CharLiteral(parsed_char);
                    } else {
                        return Token::Invalid(format!(
                            "Invalid unicode code point: '{}'",
                            char_buffer
                        ));
                    }
                } else {
                    return Token::Invalid(format!(
                        "Invalid unicode escape sequence: '{}'",
                        char_buffer
                    ));
                }
            } else {
                return Token::Invalid(format!("Invalid escape sequence: '{}'", char_buffer));
            }
        } else if char_buffer.len() == 1 {
            // NOTE: it's safe to unwrap here, right?
            let char = char_buffer.chars().next().unwrap();

            return Token::CharLiteral(char);
        } else {
            return Token::Invalid(format!(
                "Character literal '{}' must be one character long",
                char_buffer
            ));
        }
    }

    /// Lexes a number literal, given its first character
    pub fn lex_number_literal(&mut self, first_char: char) -> Token {
        // we initialize a string buffer for our number with the
        // character we've just read, which is the first digit
        // let mut num_buffer = String::from(c);

        let mut num_buffer: Vec<u8> = Vec::with_capacity(10);
        num_buffer.push(first_char as u8);

        // this is a counter for keeping track of how many decimal
        // points we've read.
        // (that doesn't mean they won't appear in the buffer)
        let mut decimal_points: usize = 0;

        loop {
            // consume the next character, ONLY if it is a digit or a
            // decimal point
            match self.chars.next_if(|x| x.is_digit(10) || x == &'.') {
                Some(c2) => {
                    // if the character is a decimal point, we increase
                    // the counter.
                    if c2 == '.' {
                        decimal_points += 1;
                    }

                    // if it's a digit, we just add it to the buffer
                    // num_buffer += &c2.to_string();
                    num_buffer.push(c2 as u8);
                }

                // if the next character isn't a digit or decimal
                // point, we break out of this loop and finalize the
                // token.
                None => break,
            }
        }

        // NOTE: can we skip this? (we'd have to change the way
        // which we store the numbers too)
        let num_string = String::from_utf8(num_buffer)
            .expect("couldn't convert num literal to utf8 string (not read properly)");

        // if we've read up to 1 decimal point, the number is (probably) fine and
        // we can parse it as a number using rust's std library
        if decimal_points <= 1 {
            // if we don't have *any* decimal points, we parse it as an integer
            if decimal_points == 0 {
                match num_string.parse::<u32>() {
                    Ok(num) => return Token::IntLiteral(num),
                    Err(_) => return Token::Invalid(num_string),
                }
            } else {
                // if we *do* have a decimal point, we parse it as a float
                match num_string.parse::<f32>() {
                    Ok(num) => return Token::FloatLiteral(num),
                    Err(_) => return Token::Invalid(num_string),
                }
            }
        } else {
            // if there's more than one decimal point, we can now
            // emit an invalid token
            Token::Invalid(num_string)
        }
    }

    /// Lexes an identifier
    fn lex_identifier(&mut self, first_char: char) -> Token {
        let mut ident_buf = String::from(first_char);

        loop {
            match self.chars.next_if(|x| x.is_alphanumeric() || x == &'_') {
                Some(c2) => ident_buf += &c2.to_string(),
                None => break,
            }
        }

        // NOTE: a map might be faster
        match ident_buf.as_str() {
            "if" => Token::Keyword(Keyword::If),
            "else" => Token::Keyword(Keyword::Else),
            "for" => Token::Keyword(Keyword::For),
            "while" => Token::Keyword(Keyword::While),
            "return" => Token::Keyword(Keyword::Return),
            "break" => Token::Keyword(Keyword::Break),
            "continue" => Token::Keyword(Keyword::Continue),
            "in" => Token::Keyword(Keyword::In),

            _ => Token::Identifier(ident_buf),
        }
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.chars.next() {
            match c {
                '\n' | '\t' | '\r' | ' ' => continue,

                '(' => return Token::OpenParen,
                ')' => return Token::CloseParen,

                '{' => return Token::OpenBrace,
                '}' => return Token::CloseBrace,

                '[' => return Token::OpenBracket,
                ']' => return Token::CloseBracket,

                ';' => return Token::Semicolon,
                ',' => return Token::Comma,
                '.' => return Token::Period,

                '&' => return self.multi_char('&', Token::Invalid(String::from("&")), Token::And),

                '|' => return self.multi_char('|', Token::Invalid(String::from("|")), Token::Or),

                '!' => return self.multi_char('=', Token::Not, Token::NotEquals),
                '=' => return self.multi_char('=', Token::Equals, Token::EqualsEquals),
                '>' => return self.multi_char('=', Token::Greater, Token::GreaterEq),
                '<' => return self.multi_char('=', Token::Lesser, Token::LesserEq),
                '+' => return self.multi_char('+', Token::Plus, Token::PlusPlus),
                '-' => return self.multi_char('-', Token::Minus, Token::MinusMinus),

                '/' => {
                    if let Some(_) = self.chars.next_if_eq(&'/') {
                        // read until we get to a newline
                        while let Some(c2) = self.chars.next() {
                            if c2 == '\n' {
                                break;
                            }
                        }
                        continue;

                    // } else if let Some(_) = chars.next_if_eq(&'*') {
                    } else {
                        return Token::Slash;
                    }
                }
                '*' => return Token::Asterisk,
                '%' => return Token::Percent,

                '"' => return self.lex_string_literal(),
                '\'' => return self.lex_char_literal(),

                '0'..='9' => return self.lex_number_literal(c),

                'A'..='Z' | 'a'..='z' | '_' => return self.lex_identifier(c),

                _ => return Token::Invalid(c.to_string()),
            }
        }

        Token::EOF
    }

    fn multi_char(&mut self, next_char: char, single: Token, multi: Token) -> Token {
        if let Some(_) = self.chars.next_if_eq(&next_char) {
            multi
        } else {
            single
        }
    }
}

/// Helper function to create a lexer and lex a source, and return all tokens.
#[test_fuzz::test_fuzz]
pub fn lex(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let mut tokens = vec![];

    loop {
        let token = lexer.next_token();

        if token == Token::EOF {
            tokens.push(token);
            break;
        }

        tokens.push(token);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_tokens_eq {
        ($input:expr, $expected:expr) => {
            let result = lex($input);
            assert_eq!(result, $expected, "Input: `{}`", $input);
        };
    }

    macro_rules! assert_tokens_ne {
        ($input:expr, $expected:expr) => {
            let result = lex($input);
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

    macro_rules! assert_unicode_char_literal_properly_decoded {
        ($input:expr, $expected_value:expr) => {
            assert_tokens_eq!(
                $input,
                vec![
                    Token::CharLiteral(char::from_u32($expected_value).unwrap()),
                    Token::EOF
                ]
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

        assert_identifier_correct_lexing_eq!("print");
        assert_identifier_correct_lexing_eq!("range");
    }

    #[test]
    fn test_keyword() {
        assert_tokens_eq!("if", vec![Token::Keyword(Keyword::If), Token::EOF]);
        assert_tokens_eq!("else", vec![Token::Keyword(Keyword::Else), Token::EOF]);
        assert_tokens_eq!("for", vec![Token::Keyword(Keyword::For), Token::EOF]);
        assert_tokens_eq!("while", vec![Token::Keyword(Keyword::While), Token::EOF]);
        assert_tokens_eq!("return", vec![Token::Keyword(Keyword::Return), Token::EOF]);
        assert_tokens_eq!("break", vec![Token::Keyword(Keyword::Break), Token::EOF]);
        assert_tokens_eq!(
            "continue",
            vec![Token::Keyword(Keyword::Continue), Token::EOF]
        );
        assert_tokens_eq!("in", vec![Token::Keyword(Keyword::In), Token::EOF]);
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
    fn test_char_literal() {
        assert_tokens_eq!("'a'", vec![Token::CharLiteral('a'), Token::EOF]);
        assert_tokens_ne!("a'", vec![Token::CharLiteral('a'), Token::EOF]);
        assert_tokens_ne!("'a", vec![Token::CharLiteral('a'), Token::EOF]);
        assert_tokens_ne!("a", vec![Token::CharLiteral('a'), Token::EOF]);

        assert_tokens_eq!("'4'", vec![Token::CharLiteral('4'), Token::EOF]);
        assert_tokens_ne!("4'", vec![Token::CharLiteral('4'), Token::EOF]);
        assert_tokens_ne!("'4", vec![Token::CharLiteral('4'), Token::EOF]);
        assert_tokens_ne!("4", vec![Token::CharLiteral('4'), Token::EOF]);
    }

    #[test]
    fn test_char_literal_unicode() {
        assert_tokens_eq!("'\\u{0041}'", vec![Token::CharLiteral('A'), Token::EOF]);
        assert_tokens_eq!("'\\u{0042}'", vec![Token::CharLiteral('B'), Token::EOF]);
        assert_tokens_eq!("'\\u{0043}'", vec![Token::CharLiteral('C'), Token::EOF]);
        assert_tokens_eq!("'\\u{0044}'", vec![Token::CharLiteral('D'), Token::EOF]);
        assert_tokens_eq!("'\\u{0045}'", vec![Token::CharLiteral('E'), Token::EOF]);
        assert_tokens_eq!("'\\u{004a}'", vec![Token::CharLiteral('J'), Token::EOF]);
        assert_tokens_eq!("'\\u{041a}'", vec![Token::CharLiteral('К'), Token::EOF]);
        assert_tokens_eq!("'\\u{20B4}'", vec![Token::CharLiteral('₴'), Token::EOF]);
        assert_tokens_eq!("'\\u{1F600}'", vec![Token::CharLiteral('😀'), Token::EOF]);

        assert_unicode_char_literal_properly_decoded!("'\\u{10FFFF}'", 0x10FFFF);
        assert_unicode_char_literal_properly_decoded!("'\\u{0}'", 0x0);
        assert_unicode_char_literal_properly_decoded!("'\\u{F}'", 0xF);
        assert_unicode_char_literal_properly_decoded!("'\\u{FF}'", 0xFF);
        assert_unicode_char_literal_properly_decoded!("'\\u{FFF}'", 0xFFF);
        assert_unicode_char_literal_properly_decoded!("'\\u{FFFF}'", 0xFFFF);
        assert_unicode_char_literal_properly_decoded!("'\\u{FFFFF}'", 0xFFFFF);
        assert_unicode_char_literal_properly_decoded!("'\\u{10FFFF}'", 0x10FFFF);
        assert_unicode_char_literal_properly_decoded!("'\\u{000000}'", 0x000000);
        assert_unicode_char_literal_properly_decoded!("'\\u{00000F}'", 0x00000F);
        assert_unicode_char_literal_properly_decoded!("'\\u{0000FF}'", 0x0000FF);
        assert_unicode_char_literal_properly_decoded!("'\\u{000FFF}'", 0x000FFF);
        assert_unicode_char_literal_properly_decoded!("'\\u{00FFFF}'", 0x00FFFF);
        assert_unicode_char_literal_properly_decoded!("'\\u{0FFFFF}'", 0x0FFFFF);
        assert_unicode_char_literal_properly_decoded!("'\\u{10FFFF}'", 0x10FFFF);
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
        assert_tokens_eq!("++", vec![Token::PlusPlus, Token::EOF]);
        assert_tokens_eq!("++ +", vec![Token::PlusPlus, Token::Plus, Token::EOF]);
        assert_tokens_eq!("+ ++", vec![Token::Plus, Token::PlusPlus, Token::EOF]);

        // minus
        assert_tokens_eq!("-", vec![Token::Minus, Token::EOF]);
        assert_tokens_eq!("--", vec![Token::MinusMinus, Token::EOF]);
        assert_tokens_eq!("-- -", vec![Token::MinusMinus, Token::Minus, Token::EOF]);
        assert_tokens_eq!("- --", vec![Token::Minus, Token::MinusMinus, Token::EOF]);

        // slash
        assert_tokens_eq!("/", vec![Token::Slash, Token::EOF]);
        assert_tokens_eq!("/ /", vec![Token::Slash, Token::Slash, Token::EOF]);

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

    #[test]
    fn test_comments() {
        assert_tokens_eq!("//", vec![Token::EOF]);
        assert_tokens_eq!("//hello, world!", vec![Token::EOF]);
        assert_tokens_eq!("// hello, world!", vec![Token::EOF]);
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
}
