use super::literal::Literal;

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(Literal),
}

pub type ExprBox = Box<Expr>;

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum UnaryExpr {
    Not(ExprBox),
    Minus(ExprBox),
}
