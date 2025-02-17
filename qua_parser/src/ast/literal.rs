#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Char(char),
    Float(f32),
    Int(u32),
}
