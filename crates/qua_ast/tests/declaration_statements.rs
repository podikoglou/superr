#[macro_use]
mod common;

use qua_ast::ast::stmt::Stmt;

#[test]
fn var_decl_with_type_and_value() {
    let var_decl = Stmt::VarDecl {
        name: "x".to_string(),
        type_name: Some("int".to_string()),
        value: Some(litexpr!(int 42)),
    };
    assert_eq!(var_decl.to_string(), "int x = 42;");
}

#[test]
fn var_decl_with_type_no_value() {
    let var_decl = Stmt::VarDecl {
        name: "y".to_string(),
        type_name: Some("float".to_string()),
        value: None,
    };
    assert_eq!(var_decl.to_string(), "float y;");
}

#[test]
fn var_decl_without_type() {
    let var_decl = Stmt::VarDecl {
        name: "z".to_string(),
        type_name: None,
        value: Some(litexpr!(string "hello")),
    };
    assert_eq!(var_decl.to_string(), "z = \"hello\";");
}
