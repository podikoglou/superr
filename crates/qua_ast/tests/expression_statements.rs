use qua_ast::ast::{expr::Expr, stmt::Stmt};

#[macro_use]
mod common;

#[test]
fn simple_expression_statement() {
    let stmt = Stmt::Expression(litexpr!(int 42));
    assert_eq!(stmt.to_string(), "42;");
}

#[test]
fn call_expression_statement() {
    let call_stmt = Stmt::Expression(Expr::Call {
        name: "print".to_string(),
        args: vec![litexpr!(string "Hello")],
    });
    assert_eq!(call_stmt.to_string(), "print(\"Hello\");");
}
