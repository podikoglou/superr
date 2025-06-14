use qua_ast::ast::stmt::Type;

#[test]
fn basic_types() {
    assert_eq!(Type::Int.to_string(), "int");
    assert_eq!(Type::Float.to_string(), "float");
    assert_eq!(Type::String.to_string(), "string");
    assert_eq!(Type::Char.to_string(), "char");
}

#[test]
fn custom_type() {
    assert_eq!(Type::Custom("MyType".to_string()).to_string(), "MyType");
}
