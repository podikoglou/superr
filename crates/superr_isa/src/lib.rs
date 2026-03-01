use std::fmt::Display;

pub type Address = u8;
pub type ImmediateValue = u8;

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Address(Address),
    ImmediateValue(ImmediateValue),
}

// This instruction set (along with the assembly format) is heavily based on the one in the below
// project:
//
// https://github.com/AZHenley/superoptimizer
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Load(ImmediateValue),

    Swap(Address, Address),

    XOR(Address, Address),

    Inc(Address),
    Decr(Address),

    Add(Address, Address),
    Sub(Address, Address),

    Put(Address),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Load(a) => write!(f, "LOAD {}", a),

            Instruction::Swap(a, b) => write!(f, "SWAP {}, {}", a, b),

            Instruction::XOR(a, b) => write!(f, "XOR {}, {}", a, b),

            Instruction::Inc(a) => write!(f, "INC {}", a),
            Instruction::Decr(a) => write!(f, "DECR {}", a),

            Instruction::Add(a, b) => write!(f, "ADD {}, {}", a, b),
            Instruction::Sub(a, b) => write!(f, "SUB {}, {}", a, b),

            Instruction::Put(a) => write!(f, "PUT {}", a),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program(pub Vec<Instruction>);
