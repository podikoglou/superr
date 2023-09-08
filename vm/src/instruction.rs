use crate::address::MemoryAddress;

#[derive(Debug)]
pub enum Instruction {
    Load(u32),
    Swap(MemoryAddress, MemoryAddress),
    XOR(MemoryAddress, MemoryAddress),
    Inc(MemoryAddress),
}
