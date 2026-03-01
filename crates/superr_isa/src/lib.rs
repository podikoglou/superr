use std::fmt::Display;

pub type Address = u8;
pub type ImmediateValue = u8;

// This instruction set (along with the assembly format) is heavily based on the one in the below
// project:
//
// https://github.com/AZHenley/superoptimizer
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    /// Loads a value into the memory address 0
    Load(ImmediateValue),

    /// Swaps the values at two memory addresses
    Swap(Address, Address),

    /// XORs the values at two memory addresses and places the result on the first
    XOR(Address, Address),

    /// Increments the value at a given memory address
    Inc(Address),

    /// Decrements the value at a given memory address
    Decr(Address),

    /// Adds the values at two memory addresses and places the result on the first
    Add(Address, Address),

    /// Subtracts the values at two memory addresses and places the result on the first
    Sub(Address, Address),

    /// Prints the value at a given memory address
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Program(pub Vec<Instruction>);
