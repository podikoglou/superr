use qua_ast::ast::expr::Expr;

#[macro_use]
mod common;

#[test]
fn identifier_edge_cases() {
    // Test various identifier formats
    assert_eq!(Expr::Identifier("simple".to_string()).to_string(), "simple");
    assert_eq!(
        Expr::Identifier("with_underscore".to_string()).to_string(),
        "with_underscore"
    );
    assert_eq!(
        Expr::Identifier("_leading_underscore".to_string()).to_string(),
        "_leading_underscore"
    );
    assert_eq!(
        Expr::Identifier("CamelCase".to_string()).to_string(),
        "CamelCase"
    );
    assert_eq!(
        Expr::Identifier("snake_case_123".to_string()).to_string(),
        "snake_case_123"
    );
    assert_eq!(Expr::Identifier("a".to_string()).to_string(), "a");
    assert_eq!(Expr::Identifier("A".to_string()).to_string(), "A");
}

#[test]
fn nested_function_calls() {
    // Nested function calls
    let nested = Expr::Call {
        name: "outer".to_string(),
        args: vec![
            Expr::Call {
                name: "inner".to_string(),
                args: vec![litexpr!(int 1)],
            },
            Expr::Call {
                name: "other".to_string(),
                args: vec![],
            },
        ],
    };
    assert_eq!(nested.to_string(), "outer(inner(1), other())");
}

#[test]
fn mixed_type_exprs_and_calls() {
    // Mixed argument types function call
    let mixed_call = Expr::Call {
        name: "format".to_string(),
        args: vec![
            litexpr!(string "Number: {}, Char: {}, Bool: {}"),
            litexpr!(int 42),
            litexpr!(char 'x'),
            Expr::Identifier("true".to_string()),
        ],
    };
    assert_eq!(
        mixed_call.to_string(),
        "format(\"Number: {}, Char: {}, Bool: {}\", 42, 'x', true)"
    );
}
