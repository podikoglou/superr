use qua_ast::ast::stmt::Stmt;

#[macro_use]
mod common;

#[test]
fn break_continue() {
    assert_eq!(Stmt::Break.to_string(), "break;");
    assert_eq!(Stmt::Continue.to_string(), "continue;");
}
