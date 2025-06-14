use qua_ast::ast::expr::Expr;

#[macro_use]
mod common;

#[test]
fn basic_calls() {
    let call_no_args = Expr::Call {
        name: "func".to_string(),
        args: vec![],
    };
    assert_eq!(call_no_args.to_string(), "func()");

    let call_one_arg = Expr::Call {
        name: "factorial".to_string(),
        args: vec![litexpr!(int 5)],
    };
    assert_eq!(call_one_arg.to_string(), "factorial(5)");
}

#[test]
fn multiple_arguments() {
    let call_multiple_args = Expr::Call {
        name: "range".to_string(),
        args: vec![litexpr!(int 2), litexpr!(int 10)],
    };
    assert_eq!(call_multiple_args.to_string(), "range(2, 10)");

    let call_complex_args = Expr::Call {
        name: "print".to_string(),
        args: vec![
            litexpr!(string "Hello"),
            expr!(binary Add (litexpr!(int 1), litexpr!(int 2))),
        ],
    };
    assert_eq!(call_complex_args.to_string(), "print(\"Hello\", (1 + 2))");
}
