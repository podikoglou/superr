use crate::address::MemoryAddress;

pub type Instruction = u32;

pub const LOAD: u8 = 0x01;
pub const SWAP: u8 = 0x02;
pub const XOR: u8 = 0x03;
pub const INC: u8 = 0x04;

pub fn new_load(value: u8) -> u32 {
    ((LOAD as u32) << 28) | (value as u32)
}

pub fn new_swap(a: u8, b: u8) -> u32 {
    ((SWAP as u32) << 28) | ((a as u32) << 8) | b as u32
}

pub fn new_xor(a: MemoryAddress, b: MemoryAddress) -> u32 {
    ((XOR as u32) << 28) | ((a as u32) << 8) | b as u32
}

pub fn new_inc(a: u8) -> u32 {
    ((INC as u32) << 28) | (a as u32)
}

pub fn decode_opcode(instruction: Instruction) -> u8 {
    (instruction >> 28) as u8
}

pub fn decode_op1(instruction: Instruction) -> u8 {
    ((instruction >> 8) & 0xFF) as u8
}

pub fn decode_op2(instruction: Instruction) -> u8 {
    (instruction & 0xFF) as u8
}
