use crate::{address::MemoryAddress, vm::MemValue};

// This instruction set (along with the assembly format) is heavily based on the one in the below
// project:
//
// https://github.com/AZHenley/superoptimizer
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Eq, Hash)]
pub enum Instruction {
    Load(MemValue),

    Swap(MemoryAddress, MemoryAddress),

    XOR(MemoryAddress, MemoryAddress),

    Inc(MemoryAddress),
    Decr(MemoryAddress),

    Add(MemoryAddress, MemoryAddress),
    Sub(MemoryAddress, MemoryAddress),

    Put(MemoryAddress),

    Jmp(usize),
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self {
            Instruction::Load(a) => format!("LOAD {}", a),

            Instruction::Swap(a, b) => format!("SWAP {} {}", a, b),

            Instruction::XOR(a, b) => format!("XOR {} {}", a, b),

            Instruction::Inc(a) => format!("INC {}", a),
            Instruction::Decr(a) => format!("DECR {}", a),

            Instruction::Add(a, b) => format!("ADD {} {}", a, b),
            Instruction::Sub(a, b) => format!("SUB {} {}", a, b),

            Instruction::Put(a) => format!("PUT {}", a),
            Instruction::Jmp(a) => format!("JMP {}", a),
        }
    }
}

mod parsers {
    use nom::{
        bytes::complete::tag,
        character::complete::{space0, u8},
        sequence::separated_pair,
        Err, IResult,
    };

    use crate::vm::MemValue;

    use super::Instruction;

    fn load_parser(i: &str) -> IResult<&str, (&str, u8)> {
        separated_pair(tag("LOAD"), space0, u8)(i)
    }

    fn swap_parser(i: &str) -> IResult<&str, (&str, (u8, u8))> {
        separated_pair(tag("SWAP"), space0, separated_pair(u8, space0, u8))(i)
    }

    fn xor_parser(i: &str) -> IResult<&str, (&str, (u8, u8))> {
        separated_pair(tag("XOR"), space0, separated_pair(u8, space0, u8))(i)
    }

    fn inc_parser(i: &str) -> IResult<&str, (&str, u8)> {
        separated_pair(tag("INC"), space0, u8)(i)
    }

    fn decr_parser(i: &str) -> IResult<&str, (&str, u8)> {
        separated_pair(tag("DECR"), space0, u8)(i)
    }

    fn add_parser(i: &str) -> IResult<&str, (&str, (u8, u8))> {
        separated_pair(tag("ADD"), space0, separated_pair(u8, space0, u8))(i)
    }

    fn sub_parser(i: &str) -> IResult<&str, (&str, (u8, u8))> {
        separated_pair(tag("SUB"), space0, separated_pair(u8, space0, u8))(i)
    }

    fn put_parser(i: &str) -> IResult<&str, (&str, u8)> {
        separated_pair(tag("PUT"), space0, u8)(i)
    }

    fn jmp_parser(i: &str) -> IResult<&str, (&str, u8)> {
        separated_pair(tag("JMP"), space0, u8)(i)
    }

    pub fn instruction_parser(i: &str) -> IResult<&str, Instruction> {
        match load_parser(i) {
            Ok((_, (_, val))) => return Ok((i, Instruction::Load(val as MemValue))),
            _ => {}
        }

        match swap_parser(i) {
            Ok((_, (_, (addr1, addr2)))) => {
                return Ok((i, Instruction::Swap(addr1 as usize, addr2 as usize)))
            }
            _ => {}
        };

        match xor_parser(i) {
            Ok((_, (_, (addr1, addr2)))) => {
                return Ok((i, Instruction::XOR(addr1 as usize, addr2 as usize)))
            }
            _ => {}
        };

        match inc_parser(i) {
            Ok((_, (_, addr))) => return Ok((i, Instruction::Inc(addr as usize))),
            _ => {}
        };

        match decr_parser(i) {
            Ok((_, (_, addr))) => return Ok((i, Instruction::Decr(addr as usize))),
            _ => {}
        };

        match add_parser(i) {
            Ok((_, (_, (addr1, addr2)))) => {
                return Ok((i, Instruction::Add(addr1 as usize, addr2 as usize)))
            }
            _ => {}
        };

        match sub_parser(i) {
            Ok((_, (_, (addr1, addr2)))) => {
                return Ok((i, Instruction::Sub(addr1 as usize, addr2 as usize)))
            }
            _ => {}
        };

        match put_parser(i) {
            Ok((_, (_, addr))) => return Ok((i, Instruction::Put(addr as usize))),
            _ => {}
        };

        match jmp_parser(i) {
            Ok((_, (_, ins))) => return Ok((i, Instruction::Jmp(ins as usize))),
            _ => {}
        };

        Err(Err::Failure(nom::error::make_error(
            i,
            nom::error::ErrorKind::Alt,
        )))
    }
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let (_, instruction) = parsers::instruction_parser(&value).expect("invalid instruction");

        instruction
    }
}
