use crate::instruction::Instruction;
use bytes::{Buf, BufMut, Bytes, BytesMut};

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

impl From<Program> for Vec<u8> {
    fn from(value: Program) -> Self {
        let mut buf = BytesMut::new();

        // NOTE: this will fail if we have more than 2^32 - 1
        buf.put_u32(value.instructions.len() as u32);

        for instruction in value.instructions {
            buf.put_u32(instruction);
        }

        buf.to_vec()
    }
}

impl From<Vec<u8>> for Program {
    fn from(value: Vec<u8>) -> Self {
        let mut buf = Bytes::from(value);

        let mut program = Program::default();
        let len = buf.get_u32();

        for _ in 0..len {
            let instruction = buf.get_u32();

            program.instructions.push(instruction);
        }

        program
    }
}
