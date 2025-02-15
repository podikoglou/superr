pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(Literal),
}

pub type ExprBox = Box<Expr>;

pub struct BinaryExpr {
    pub left: ExprBox,
    pub right: ExprBox,
    pub operator: BinaryOperator,
}

pub enum BinaryOperator {
    And,
    Or,

    Equals,
    NotEquals,

    Greater,
    GreaterEq,
    Lesser,
    LesserEq,

    Add,
    Subtract,
    Divide,
    Multiply,
    // TODO: % and eventually ^
}

pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub right: ExprBox,
}

pub enum UnaryOperator {
    Not,
    Minus,
}

pub enum Literal {
    String(String),
    Char(char),
    Float(f32),
    Int(f32),
}
