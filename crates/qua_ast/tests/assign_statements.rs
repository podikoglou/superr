use qua_ast::ast::{expr::Expr, stmt::Stmt};

#[macro_use]
mod common;

#[test]
fn simple_assignment() {
    let assignment = Stmt::Assignment {
        name: "x".to_string(),
        value: litexpr!(int 100),
    };
    assert_eq!(assignment.to_string(), "x = 100;");
}

#[test]
fn complex_assignment() {
    let complex_assignment = Stmt::Assignment {
        name: "result".to_string(),
        value: expr!(binary Multiply (
            Expr::Identifier("x".to_string()),
            litexpr!(int 2)
        )),
    };
    assert_eq!(complex_assignment.to_string(), "result = (x * 2);");
}
