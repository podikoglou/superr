#[macro_use]
mod common;

use qua_ast::ast::expr::Expr;

#[test]
fn simple_groupings() {
    let grouped = Expr::Grouping(Box::new(litexpr!(int 42)));
    assert_eq!(grouped.to_string(), "(42)");

    let nested_grouped = Expr::Grouping(Box::new(Expr::Grouping(Box::new(litexpr!(int 5)))));
    assert_eq!(nested_grouped.to_string(), "((5))");
}

#[test]
fn complex_groupings() {
    let complex_grouped = Expr::Grouping(Box::new(expr!(binary Add (
        litexpr!(int 1),
        litexpr!(int 2)
    ))));
    assert_eq!(complex_grouped.to_string(), "((1 + 2))");
}
