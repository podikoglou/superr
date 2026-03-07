use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};
use superr_isa::{Instruction, Program};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BytecodeDecodingError {
    #[error("io error")]
    IOError(#[from] std::io::Error),

    #[error("invalid opcode: {0}")]
    InvalidOpcode(u8),

    #[error("invalid operand")]
    InvalidOperand,
}

impl PartialEq for BytecodeDecodingError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::IOError(_), Self::IOError(_)) => false,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

pub fn instruction_to_u32(instruction: &Instruction) -> u32 {
    match instruction {
        Instruction::Load(val) => (0x01 << 28) | (*val as u32),
        Instruction::Swap(a, b) => (0x02 << 28) | ((*a as u32) << 8) | (*b as u32),
        Instruction::XOR(a, b) => (0x03 << 28) | ((*a as u32) << 8) | (*b as u32),
        Instruction::Inc(val) => (0x04 << 28) | (*val as u32),
        Instruction::Dec(val) => (0x05 << 28) | (*val as u32),
        Instruction::Add(a, b) => (0x06 << 28) | ((*a as u32) << 8) | (*b as u32),
        Instruction::Sub(a, b) => (0x07 << 28) | ((*a as u32) << 8) | (*b as u32),
        Instruction::Put(addr) => (0x08 << 28) | (*addr as u32),
    }
}

pub fn u32_to_instruction(input: u32) -> Result<Instruction, BytecodeDecodingError> {
    let opcode = (input & 0xF0000000) >> 28;

    // TODO: error handling
    let first_op = || ((input & 0x0000FF00) >> 8).try_into().unwrap();
    let second_op = || (input & 0x000000FF).try_into().unwrap();

    match opcode {
        0x01 => Ok(Instruction::Load(second_op())),
        0x02 => Ok(Instruction::Swap(first_op(), second_op())),
        0x03 => Ok(Instruction::XOR(first_op(), second_op())),
        0x04 => Ok(Instruction::Inc(second_op())),
        0x05 => Ok(Instruction::Dec(second_op())),
        0x06 => Ok(Instruction::Add(first_op(), second_op())),
        0x07 => Ok(Instruction::Sub(first_op(), second_op())),
        0x08 => Ok(Instruction::Put(second_op())),
        _ => Err(BytecodeDecodingError::InvalidOpcode(opcode as u8)),
    }
}

pub fn instruction_to_bytes(instruction: &Instruction) -> Vec<u8> {
    instruction_to_u32(instruction).to_be_bytes().into()
}

pub fn program_to_bytes(program: &Program) -> Vec<u8> {
    // encode length as an unsigned 16 bit big endian number
    let len = program.0.len() as u16;
    let len_bytes = len.to_be_bytes().to_vec();

    // encode every instruction into a flat array (see From<&Instruction> for Vec<u8>)
    let instructions_bytes = program
        .0
        .iter()
        .flat_map(instruction_to_bytes)
        .collect::<Vec<u8>>();

    [len_bytes, instructions_bytes].concat()
}

pub fn read_program(mut reader: impl Read) -> Result<Program, BytecodeDecodingError> {
    // read length
    let len = reader
        .read_u16::<BigEndian>()
        .map_err(BytecodeDecodingError::IOError)?;

    // read instructions
    let mut instructions = vec![];

    for _ in 0..len {
        let read = reader
            .read_u32::<BigEndian>()
            .map_err(BytecodeDecodingError::IOError)?;

        let instruction = u32_to_instruction(read)?;

        instructions.push(instruction);
    }

    Ok(Program(instructions))
}
