#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Instruction { opcode: String, operands: Vec<u64> },
    Block(Vec<ASTNode>),
}
