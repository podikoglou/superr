pub enum BinaryOperator {
    And,
    Or,

    Equals,
    NotEquals,

    Greater,
    GreaterEq,
    Lesser,
    LesserEq,

    Add,
    Subtract,
    Divide,
    Multiply,
    // TODO: % and eventually ^
}
pub enum UnaryOperator {
    Not,
    Minus,
}
