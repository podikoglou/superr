use qua_ast::ast::{expr::Expr, stmt::Stmt};

#[macro_use]
mod common;

#[test]
fn for_with_type() {
    let for_stmt = Stmt::For {
        var_name: "i".to_string(),
        var_type: Some("int".to_string()),
        iterable: Expr::Call {
            name: "range".to_string(),
            args: vec![litexpr!(int 0), litexpr!(int 10)],
        },
        body: Box::new(Stmt::Expression(Expr::Call {
            name: "print".to_string(),
            args: vec![Expr::Identifier("i".to_string())],
        })),
    };
    assert_eq!(
        for_stmt.to_string(),
        "for (int i in range(0, 10)) print(i);"
    );
}

#[test]
fn for_without_type() {
    let for_stmt_no_type = Stmt::For {
        var_name: "item".to_string(),
        var_type: None,
        iterable: Expr::Identifier("items".to_string()),
        body: Box::new(Stmt::Break),
    };
    assert_eq!(for_stmt_no_type.to_string(), "for (item in items) break;");
}
