use qua_ast::ast::{
    expr::Expr,
    program::{Function, Parameter},
    stmt::{Stmt, Type},
};

#[macro_use]
mod common;

#[test]
fn function_with_return_type_and_parameters() {
    let function = Function {
        name: "factorial".to_string(),
        parameters: vec![Parameter {
            name: "x".to_string(),
            type_name: Type::Int,
        }],
        return_type: Some(Type::Int),
        body: Stmt::Block(vec![
            Stmt::VarDecl {
                name: "result".to_string(),
                type_name: Some("int".to_string()),
                value: Some(litexpr!(int 1)),
            },
            Stmt::Return(Some(Expr::Identifier("result".to_string()))),
        ]),
    };

    let expected = "int factorial(int x) {\n    int result = 1;\n    return result;\n}";
    assert_eq!(function.to_string(), expected);
}

#[test]
fn function_without_return_type() {
    let void_function = Function {
        name: "print_hello".to_string(),
        parameters: vec![],
        return_type: None,
        body: Stmt::Expression(Expr::Call {
            name: "print".to_string(),
            args: vec![litexpr!(string "Hello")],
        }),
    };
    assert_eq!(void_function.to_string(), "print_hello() print(\"Hello\");");
}

#[test]
fn function_with_multiple_parameters() {
    let multi_param_function = Function {
        name: "add".to_string(),
        parameters: vec![
            Parameter {
                name: "a".to_string(),
                type_name: Type::Int,
            },
            Parameter {
                name: "b".to_string(),
                type_name: Type::Int,
            },
        ],
        return_type: Some(Type::Int),
        body: Stmt::Return(Some(expr!(binary Add (
            Expr::Identifier("a".to_string()),
            Expr::Identifier("b".to_string())
        )))),
    };
    assert_eq!(
        multi_param_function.to_string(),
        "int add(int a, int b) return (a + b);"
    );
}
