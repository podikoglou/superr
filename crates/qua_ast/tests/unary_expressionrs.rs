#[macro_use]
mod common;
#[test]
fn unary_operators() {
    let operand = litexpr!(int 42);

    assert_eq!(expr!(unary Not operand.clone()).to_string(), "(!42)");
    assert_eq!(expr!(unary Minus operand.clone()).to_string(), "(-42)");
}

#[test]
fn unary_expr_with_literal() {
    assert_eq!(expr!(unary Minus litexpr!(int 1)).to_string(), "(-1)");
}
