use std::fmt;
use qua_lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: Token,
    },
    UnexpectedEof,
    InvalidExpression(String),
    InvalidStatement(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "Expected {}, found {:?}", expected, found)
            }
            ParseError::UnexpectedEof => write!(f, "Unexpected end of file"),
            ParseError::InvalidExpression(msg) => write!(f, "Invalid expression: {}", msg),
            ParseError::InvalidStatement(msg) => write!(f, "Invalid statement: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

pub type ParseResult<T> = Result<T, ParseError>;
