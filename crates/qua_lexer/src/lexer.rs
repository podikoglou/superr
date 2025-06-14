use std::{char, iter::Peekable, str::Chars};

use crate::{keyword, token::Token};

#[derive(Debug)]
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
    /// Reads a string literal from the input, returning [`Token::Invalid`]
    /// if the string is not a valid string literal.
    fn lex_string_literal(&mut self) -> Token {
        // we create an empty string buffer in which we'll put
        // the contents (excluding the quotes) of our string
        let mut string_buffer = String::default();

        let mut closed = false;

        for c2 in &mut self.chars {
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

    /// Reads a character literal from the input, returning [`Token::Invalid`]
    /// if the character is not a valid character literal.
    ///
    /// Unicode code points are supported.
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

        for c2 in &mut self.chars {
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
                        Token::CharLiteral(parsed_char)
                    } else {
                        Token::Invalid(format!("Invalid unicode code point: '{}'", char_buffer))
                    }
                } else {
                    Token::Invalid(format!(
                        "Invalid unicode escape sequence: '{}'",
                        char_buffer
                    ))
                }
            } else {
                Token::Invalid(format!("Invalid escape sequence: '{}'", char_buffer))
            }
        } else if char_buffer.len() == 1 {
            // NOTE: it's safe to unwrap here, right?
            let char = char_buffer.chars().next().unwrap();

            Token::CharLiteral(char)
        } else {
            Token::Invalid(format!(
                "Character literal '{}' must be one character long",
                char_buffer
            ))
        }
    }

    /// Reads a number literal from the input, returning a [`Token::IntLiteral`]
    /// if the number contains no decimal point, and was successfully parsed as an [`u32`].
    ///
    /// Sign is ignored -- it's a separate token.
    fn lex_number_literal(&mut self, first_char: char) -> Token {
        // we initialize a string buffer for our number with the
        // character we've just read, which is the first digit
        let mut num_buffer = String::from(first_char);

        // this is a counter for keeping track of how many decimal
        // points we've read.
        // (that doesn't mean they won't appear in the buffer)
        let mut decimal_points: usize = 0;

        loop {
            // consume the next character, ONLY if it is a digit or a
            // decimal point
            match self.chars.next_if(|x| x.is_ascii_digit() || x == &'.') {
                Some(c2) => {
                    // if the character is a decimal point, we increase
                    // the counter.
                    if c2 == '.' {
                        decimal_points += 1;
                    }

                    // if it's a digit, we just add it to the buffer
                    num_buffer.push(c2);
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
                    Ok(num) => Token::IntLiteral(num),
                    Err(_) => Token::Invalid(num_buffer),
                }
            } else {
                // if we *do* have a decimal point, we parse it as a float
                match num_buffer.parse::<f32>() {
                    Ok(num) => Token::FloatLiteral(num),
                    Err(_) => Token::Invalid(num_buffer),
                }
            }
        } else {
            // if there's more than one decimal point, we can now
            // emit an invalid token
            Token::Invalid(num_buffer)
        }
    }

    /// Reads an identifier from the input (given its first character),
    /// returning a [`Token::Keyword`] token if the identifier is a keyword, or
    /// a [`Token::Identifier`] otherwise.
    fn lex_identifier(&mut self, first_char: char) -> Token {
        let mut ident_buf = String::from(first_char);

        loop {
            match self.chars.next_if(|x| x.is_alphanumeric() || x == &'_') {
                Some(c2) => ident_buf.push(c2),
                None => break,
            }
        }

        match keyword::lookup(&ident_buf) {
            Some(keyword) => Token::Keyword(keyword),
            None => Token::Identifier(ident_buf),
        }
    }

    /// Chooses the next token based on the next character of the input.
    fn multi_char(&mut self, next_char: char, single: Token, multi: Token) -> Token {
        if self.chars.next_if_eq(&next_char).is_some() {
            multi
        } else {
            single
        }
    }

    /// Reads the next token from the input, returning [`Token::Invalid`] for
    /// invalid tokens.
    ///
    /// Whitespace and newlines are skipped.
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
                    if self.chars.next_if_eq(&'/').is_some() {
                        // read until we get to a newline
                        for c2 in &mut self.chars {
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
