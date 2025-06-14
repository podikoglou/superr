use qua_ast::ast::expr::Expr;

#[macro_use]
mod common;

#[test]
fn basic_identifiers() {
    assert_eq!(
        Expr::Identifier("factorial".to_string()).to_string(),
        "factorial"
    );
    assert_eq!(Expr::Identifier("x".to_string()).to_string(), "x");
    assert_eq!(
        Expr::Identifier("_variable".to_string()).to_string(),
        "_variable"
    );
}
