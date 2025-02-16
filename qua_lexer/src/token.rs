// TODO:
//  +=, *=, /=
//  true, false (?)
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
