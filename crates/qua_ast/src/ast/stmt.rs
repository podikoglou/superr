use super::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    VarDecl {
        name: String,
        type_name: Option<String>,
        value: Option<Expr>,
    },
    Assignment {
        name: String,
        value: Expr,
    },
    Return(Option<Expr>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    For {
        var_name: String,
        var_type: Option<String>,
        iterable: Expr,
        body: Box<Stmt>,
    },
    Block(Vec<Stmt>),
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Char,
    Custom(String),
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "int" => Type::Int,
            "float" => Type::Float,
            "string" => Type::String,
            "char" => Type::Char,
            _ => Type::Custom(s.to_string()),
        }
    }
}
