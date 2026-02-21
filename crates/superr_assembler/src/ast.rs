pub type Address = usize;
pub type ImmediateValue = u64;

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Address(Address),
    ImmediateValue(ImmediateValue),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Instruction {
        opcode: String,
        operands: Vec<Operand>,
    },
    Block(Vec<ASTNode>),
}
