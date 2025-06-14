use super::literal::Literal;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(Literal),
    Identifier(String),
    Call { name: String, args: Vec<Expr> },
    Grouping(Box<Expr>),
}

pub type ExprBox = Box<Expr>;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryExpr {
    // Logical operators
    And(ExprBox, ExprBox),
    Or(ExprBox, ExprBox),

    // Comparison operators
    Equals(ExprBox, ExprBox),
    NotEquals(ExprBox, ExprBox),

    Greater(ExprBox, ExprBox),
    GreaterEqual(ExprBox, ExprBox),

    Less(ExprBox, ExprBox),
    LessEqual(ExprBox, ExprBox),

    // Arithmetic operators
    Add(ExprBox, ExprBox),
    Subtract(ExprBox, ExprBox),
    Multiply(ExprBox, ExprBox),
    Divide(ExprBox, ExprBox),
    Modulo(ExprBox, ExprBox),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryExpr {
    Not(ExprBox),
    Minus(ExprBox),
}
