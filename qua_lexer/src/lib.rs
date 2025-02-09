// TODO:
//  - char literal
//  - ++, --, +=, *=, /=
pub enum Token {
    Identifier(String), // factorial, for, int

    // Literals
    IntLiteral(u16),       // 21
    FloatLiteral(f32),     // 3.14
    StringLiteral(String), // "Qua!"

    // Delimiters
    OpenParen,  // (
    CloseParen, // )

    OpenBrace,  // {
    CloseBrace, // }

    OpenBracket,  // [
    CloseBracket, // ]

    DoubleQuote,

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
    Plus,         // +
    Minus,        // -
    Slash,        // /
    Asterisk,     // *
    Percent,      // %

    // Special
    EOF,
    Invalid(String),
}
