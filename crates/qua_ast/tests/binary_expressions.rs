#[macro_use]
mod common;

macro_rules! test_binary {
    ($($op:ident => $sym:expr),*) => {
        $(
            #[test]
            fn $op() {
                let left = litexpr!(int 1);
                let right = litexpr!(int 2);
                let expected = format!("(1 {} 2)", $sym);
                assert_eq!(
                    expr!(binary $op (left, right)).to_string(),
                    expected
                );
            }
        )*
    };
}

test_binary!(
    And => "&&",
    Or => "||",
    Equals => "==",
    NotEquals => "!=",
    Greater => ">",
    GreaterEqual => ">=",
    Less => "<",
    LessEqual => "<=",
    Add => "+",
    Subtract => "-",
    Multiply => "*",
    Divide => "/",
    Modulo => "%"
);

#[test]
fn nested_binary_expressions() {
    assert_eq!(
        expr!(binary Add (
            expr!(binary Add (litexpr!(int 3), litexpr!(float 0.1415))),
            expr!(binary Divide (litexpr!(int 1), litexpr!(int 2)))
        ))
        .to_string(),
        "((3 + 0.1415) + (1 / 2))"
    );
}
