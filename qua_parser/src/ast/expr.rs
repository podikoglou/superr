use super::{
    literal::Literal,
    operator::{BinaryOperator, UnaryOperator},
};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(Literal),
}

pub type ExprBox = Box<Expr>;

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: ExprBox,
    pub right: ExprBox,
    pub operator: BinaryOperator,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub right: ExprBox,
}
