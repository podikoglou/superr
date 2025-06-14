#[macro_use]
mod common;

use qua_ast::ast::{expr::Expr, stmt::Stmt};

#[test]
fn return_with_value() {
    let return_stmt = Stmt::Return(Some(litexpr!(int 42)));
    assert_eq!(return_stmt.to_string(), "return 42;");
}

#[test]
fn return_without_value() {
    let return_void = Stmt::Return(None);
    assert_eq!(return_void.to_string(), "return;");
}

#[test]
fn return_with_complex_expression() {
    let return_expr = Stmt::Return(Some(expr!(binary Add (
        Expr::Identifier("x".to_string()),
        litexpr!(int 1)
    ))));
    assert_eq!(return_expr.to_string(), "return (x + 1);");
}
