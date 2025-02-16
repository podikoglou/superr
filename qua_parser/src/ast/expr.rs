use super::{
    literal::Literal,
    operator::{BinaryOperator, UnaryOperator},
};

pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(Literal),
}

pub type ExprBox = Box<Expr>;

pub struct BinaryExpr {
    pub left: ExprBox,
    pub right: ExprBox,
    pub operator: BinaryOperator,
}

pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub right: ExprBox,
}
