macro_rules! lit {
    (int $value:expr) => {
        qua_ast::ast::literal::Literal::Int($value)
    };
    (float $value:expr) => {
        qua_ast::ast::literal::Literal::Float($value)
    };
    (string $value:expr) => {
        qua_ast::ast::literal::Literal::String($value.to_string())
    };
    (char $value:expr) => {
        qua_ast::ast::literal::Literal::Char($value)
    };
}

macro_rules! expr {
    (literal $lit:expr) => {
        qua_ast::ast::expr::Expr::Literal($lit)
    };
    (binary $op:ident ($left:expr, $right:expr)) => {
        qua_ast::ast::expr::Expr::Binary(qua_ast::ast::expr::BinaryExpr::$op(
            Box::new($left),
            Box::new($right),
        ))
    };
    (unary $op:ident $expr:expr) => {
        qua_ast::ast::expr::Expr::Unary(qua_ast::ast::expr::UnaryExpr::$op(Box::new($expr)))
    };
}

macro_rules! litexpr {
    (int $value:expr) => {
        expr!(literal lit!(int $value))
    };
    (float $value:expr) => {
        expr!(literal lit!(float $value))
    };
    (string $value:expr) => {
        expr!(literal lit!(string $value))
    };
    (char $value:expr) => {
        expr!(literal lit!(char $value))
    };
}
