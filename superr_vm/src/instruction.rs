use crate::address::MemoryAddress;

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Eq, Hash)]
pub enum Instruction {
    Load(usize),
    Swap(MemoryAddress, MemoryAddress),
    XOR(MemoryAddress, MemoryAddress),
    Inc(MemoryAddress),
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self {
            Instruction::Load(a) => format!("LOAD {}", a),
            Instruction::Swap(a, b) => format!("SWAP {} {}", a, b),
            Instruction::XOR(a, b) => format!("XOR {} {}", a, b),
            Instruction::Inc(a) => format!("INC {}", a),
        }
    }
}

mod parsers {
    use nom::{
        bytes::complete::tag,
        character::complete::{space0, u8},
        sequence::{preceded, separated_pair},
        Err, IResult,
    };

    use super::Instruction;

    fn load_parser(i: &str) -> IResult<&str, u8> {
        preceded(tag("LOAD"), preceded(space0, u8))(i)
    }

    fn swap_parser(i: &str) -> IResult<&str, (u8, u8)> {
        preceded(
            tag("SWAP"),
            preceded(space0, separated_pair(u8, space0, u8)),
        )(i)
    }

    fn xor_parser(i: &str) -> IResult<&str, (u8, u8)> {
        preceded(tag("XOR"), preceded(space0, separated_pair(u8, space0, u8)))(i)
    }

    fn inc_parser(i: &str) -> IResult<&str, u8> {
        preceded(tag("INC"), preceded(space0, u8))(i)
    }

    pub fn instruction_parser(i: &str) -> IResult<&str, Instruction> {
        match load_parser(i) {
            Ok((_, val)) => return Ok((i, Instruction::Load(val as usize))),
            _ => {}
        }

        match swap_parser(i) {
            Ok((_, (addr1, addr2))) => {
                return Ok((i, Instruction::Swap(addr1 as usize, addr2 as usize)))
            }
            _ => {}
        };

        match swap_parser(i) {
            Ok((_, (addr1, addr2))) => {
                return Ok((i, Instruction::Swap(addr1 as usize, addr2 as usize)))
            }
            _ => {}
        };

        match xor_parser(i) {
            Ok((_, (addr1, addr2))) => {
                return Ok((i, Instruction::XOR(addr1 as usize, addr2 as usize)))
            }
            _ => {}
        };

        match inc_parser(i) {
            Ok((_, addr)) => return Ok((i, Instruction::Inc(addr as usize))),
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
