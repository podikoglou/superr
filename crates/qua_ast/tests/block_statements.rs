use qua_ast::ast::{expr::Expr, stmt::Stmt};

#[macro_use]
mod common;

#[test]
fn block_statement() {
    let block = Stmt::Block(vec![
        Stmt::VarDecl {
            name: "x".to_string(),
            type_name: Some("int".to_string()),
            value: Some(litexpr!(int 1)),
        },
        Stmt::Assignment {
            name: "x".to_string(),
            value: expr!(binary Add (
                Expr::Identifier("x".to_string()),
                litexpr!(int 1)
            )),
        },
        Stmt::Return(Some(Expr::Identifier("x".to_string()))),
    ]);

    let expected = "{\n    int x = 1;\n    x = (x + 1);\n    return x;\n}";
    assert_eq!(block.to_string(), expected);
}

#[test]
fn empty_block() {
    let empty_block = Stmt::Block(vec![]);
    assert_eq!(empty_block.to_string(), "{\n}");
}
