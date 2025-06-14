use qua_ast::ast::expr::Expr;

#[macro_use]
mod common;

#[test]
fn deeply_nested_expressions() {
    // Create a deeply nested expression: ((((1 + 2) * 3) / 4) % 5)
    let deeply_nested = expr!(binary Modulo (
        expr!(binary Divide (
            expr!(binary Multiply (
                expr!(binary Add (litexpr!(int 1), litexpr!(int 2))),
                litexpr!(int 3)
            )),
            litexpr!(int 4)
        )),
        litexpr!(int 5)
    ));
    assert_eq!(deeply_nested.to_string(), "((((1 + 2) * 3) / 4) % 5)");
}

#[test]
fn complex_logical_expressions() {
    let complex_logical = expr!(binary Or (
        expr!(binary And (
            Expr::Call {
                name: "is_valid".to_string(),
                args: vec![Expr::Identifier("x".to_string())],
            },
            expr!(binary Greater (
                Expr::Call {
                    name: "length".to_string(),
                    args: vec![Expr::Identifier("x".to_string())],
                },
                litexpr!(int 0)
            ))
        )),
        expr!(binary Equals (
            Expr::Identifier("x".to_string()),
            litexpr!(string "default")
        ))
    ));
    assert_eq!(
        complex_logical.to_string(),
        "((is_valid(x) && (length(x) > 0)) || (x == \"default\"))"
    );
}
