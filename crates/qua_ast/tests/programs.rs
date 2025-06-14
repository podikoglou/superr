use qua_ast::ast::{
    expr::Expr,
    program::{Function, Parameter, Program},
    stmt::{Stmt, Type},
};

#[macro_use]
mod common;

#[test]
fn program_with_multiple_functions() {
    let program = Program {
        functions: vec![
            Function {
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
            },
            Function {
                name: "main".to_string(),
                parameters: vec![],
                return_type: Some(Type::Int),
                body: Stmt::Block(vec![
                    Stmt::Expression(Expr::Call {
                        name: "print".to_string(),
                        args: vec![Expr::Call {
                            name: "add".to_string(),
                            args: vec![litexpr!(int 2), litexpr!(int 3)],
                        }],
                    }),
                    Stmt::Return(Some(litexpr!(int 0))),
                ]),
            },
        ],
    };

    let result = program.to_string();

    // Check that both functions are present
    assert!(result.contains("int add(int a, int b)"));
    assert!(result.contains("int main()"));
    assert!(result.contains("return (a + b);"));
    assert!(result.contains("print(add(2, 3));"));
    assert!(result.contains("return 0;"));
}

#[test]
fn empty_program() {
    let empty_program = Program { functions: vec![] };
    assert_eq!(empty_program.to_string(), "");
}
