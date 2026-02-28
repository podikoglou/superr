use crate::ast::Instruction;

impl From<&Instruction> for u32 {
    fn from(val: &Instruction) -> Self {
        match val {
            Instruction::Load(val) => (0x01 << 28) | (*val as u32),
            Instruction::Swap(a, b) => (0x02 << 28) | ((*a as u32) << 8) | (*b as u32),
            Instruction::XOR(a, b) => (0x03 << 28) | ((*a as u32) << 8) | (*b as u32),
            Instruction::Inc(val) => (0x04 << 28) | (*val as u32),
            Instruction::Decr(val) => (0x05 << 28) | (*val as u32),
            Instruction::Add(a, b) => (0x06 << 28) | ((*a as u32) << 8) | (*b as u32),
            Instruction::Sub(a, b) => (0x07 << 28) | ((*a as u32) << 8) | (*b as u32),
            Instruction::Put(addr) => (0x08 << 28) | (*addr as u32),
        }
    }
}

impl From<&Instruction> for Vec<u8> {
    fn from(val: &Instruction) -> Self {
        Into::<u32>::into(val).to_le_bytes().into()
    }
}
