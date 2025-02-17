use std::fmt::Display;

use crate::ast::{
    expr::{BinaryExpr, Expr, UnaryExpr},
    literal::Literal,
};

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(v) => write!(f, "{}", v),
            Expr::Unary(v) => write!(f, "{}", v),
            Expr::Literal(v) => write!(f, "{}", v),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(v) => write!(f, "{}", v),
            Literal::Char(v) => write!(f, "{}", v),
            Literal::Float(v) => write!(f, "{}", v),
            Literal::Int(v) => write!(f, "{}", v),
        }
    }
}

impl Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryExpr::And(l, r) => write!(f, "({} && {})", l, r),
            BinaryExpr::Or(l, r) => write!(f, "({} || {})", l, r),
            BinaryExpr::Equals(l, r) => write!(f, "({} == {})", l, r),
            BinaryExpr::NotEquals(l, r) => write!(f, "({} != {})", l, r),
            BinaryExpr::Greater(l, r) => write!(f, "({} > {})", l, r),
            BinaryExpr::GreaterEqual(l, r) => write!(f, "({} >= {})", l, r),
            BinaryExpr::Less(l, r) => write!(f, "({} < {})", l, r),
            BinaryExpr::LessEqual(l, r) => write!(f, "({} <= {})", l, r),
            BinaryExpr::Add(l, r) => write!(f, "({} + {})", l, r),
            BinaryExpr::Subtract(l, r) => write!(f, "({} - {})", l, r),
            BinaryExpr::Multiply(l, r) => write!(f, "({} * {})", l, r),
            BinaryExpr::Divide(l, r) => write!(f, "({} / {})", l, r),
        }
    }
}

impl Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryExpr::Not(v) => write!(f, "(!{})", v),
            UnaryExpr::Minus(v) => write!(f, "(-{})", v),
        }
    }
}
