use qua_ast::ast::{expr::Expr, stmt::Stmt};

#[macro_use]
mod common;

#[test]
fn simple_while() {
    let while_stmt = Stmt::While {
        condition: expr!(binary Greater (
            Expr::Identifier("i".to_string()),
            litexpr!(int 0)
        )),
        body: Box::new(Stmt::Assignment {
            name: "i".to_string(),
            value: expr!(binary Subtract (
                Expr::Identifier("i".to_string()),
                litexpr!(int 1)
            )),
        }),
    };
    assert_eq!(while_stmt.to_string(), "while ((i > 0)) i = (i - 1);");
}
