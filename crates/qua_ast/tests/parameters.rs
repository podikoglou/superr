use qua_ast::ast::{program::Parameter, stmt::Type};
#[macro_use]
mod common;

#[test]
fn basic_parameters() {
    let param_int = Parameter {
        name: "x".to_string(),
        type_name: Type::Int,
    };
    assert_eq!(param_int.to_string(), "int x");

    let param_custom = Parameter {
        name: "obj".to_string(),
        type_name: Type::Custom("MyClass".to_string()),
    };
    assert_eq!(param_custom.to_string(), "MyClass obj");
}
