use qua_ast::ast::{expr::Expr, stmt::Stmt};

#[macro_use]
mod common;

#[test]
fn simple_if() {
    let if_stmt = Stmt::If {
        condition: expr!(binary Greater (
            Expr::Identifier("x".to_string()),
            litexpr!(int 0)
        )),
        then_branch: Box::new(Stmt::Return(Some(litexpr!(int 1)))),
        else_branch: None,
    };
    assert_eq!(if_stmt.to_string(), "if ((x > 0)) return 1;");
}

#[test]
fn if_else() {
    let if_else_stmt = Stmt::If {
        condition: expr!(binary Equals (
            Expr::Identifier("x".to_string()),
            litexpr!(int 0)
        )),
        then_branch: Box::new(Stmt::Return(Some(litexpr!(int 0)))),
        else_branch: Some(Box::new(Stmt::Return(Some(litexpr!(int 1))))),
    };
    assert_eq!(
        if_else_stmt.to_string(),
        "if ((x == 0)) return 0; else return 1;"
    );
}
