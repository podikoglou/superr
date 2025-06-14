use std::fmt::Display;

use crate::ast::{
    expr::{BinaryExpr, Expr, UnaryExpr},
    literal::Literal,
    program::{Function, Parameter, Program},
    stmt::{Stmt, Type},
};

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(v) => write!(f, "{}", v),
            Expr::Unary(v) => write!(f, "{}", v),
            Expr::Literal(v) => write!(f, "{}", v),
            Expr::Identifier(name) => write!(f, "{}", name),
            Expr::Call { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expr::Grouping(expr) => write!(f, "({})", expr),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(v) => write!(f, "\"{}\"", v),
            Literal::Char(v) => write!(f, "'{}'", v),
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
            BinaryExpr::Modulo(l, r) => write!(f, "({} % {})", l, r),
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

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expression(expr) => write!(f, "{};", expr),
            Stmt::VarDecl {
                name,
                type_name,
                value,
            } => {
                if let Some(type_name) = type_name {
                    write!(f, "{} {}", type_name, name)?;
                } else {
                    write!(f, "{}", name)?;
                }
                if let Some(value) = value {
                    write!(f, " = {}", value)?;
                }
                write!(f, ";")
            }
            Stmt::Assignment { name, value } => write!(f, "{} = {};", name, value),
            Stmt::Return(expr) => {
                write!(f, "return")?;
                if let Some(expr) = expr {
                    write!(f, " {}", expr)?;
                }
                write!(f, ";")
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                write!(f, "if ({}) {}", condition, then_branch)?;
                if let Some(else_branch) = else_branch {
                    write!(f, " else {}", else_branch)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => write!(f, "while ({}) {}", condition, body),
            Stmt::For {
                var_name,
                var_type,
                iterable,
                body,
            } => {
                write!(f, "for (")?;
                if let Some(var_type) = var_type {
                    write!(f, "{} ", var_type)?;
                }
                write!(f, "{} in {}) {}", var_name, iterable, body)
            }
            Stmt::Block(stmts) => {
                writeln!(f, "{{")?;
                for stmt in stmts {
                    writeln!(f, "    {}", stmt)?;
                }
                write!(f, "}}")
            }
            Stmt::Break => write!(f, "break;"),
            Stmt::Continue => write!(f, "continue;"),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(return_type) = &self.return_type {
            write!(f, "{} ", return_type)?;
        }
        write!(f, "{}(", self.name)?;
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", param)?;
        }
        write!(f, ") {}", self.body)
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.type_name, self.name)
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, function) in self.functions.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            writeln!(f, "{}", function)?;
        }
        Ok(())
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::Char => write!(f, "char"),
            Type::Custom(name) => write!(f, "{}", name),
        }
    }
}
