use crate::instruction::Instruction;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq, Hash)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            instructions: vec![],
        }
    }
}
