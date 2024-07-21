use std::str::FromStr;

use anyhow::{anyhow, Error};

use crate::address::MemoryAddress;

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Eq, Hash)]
pub enum Instruction {
    Load(usize),
    Swap(MemoryAddress, MemoryAddress),
    XOR(MemoryAddress, MemoryAddress),
    Inc(MemoryAddress),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.trim().split_whitespace();
        let instruction = parts.next().ok_or(anyhow!("invalid instruction"))?;

        match instruction.to_uppercase().as_str() {
            "LOAD" => Ok(Instruction::Load(
                parts
                    .next()
                    .ok_or(anyhow!("missing argument for LOAD"))?
                    .parse::<usize>()?,
            )),
            "SWAP" => Ok(Instruction::Swap(
                parts
                    .next()
                    .ok_or(anyhow!("missing argument for SWAP"))?
                    .parse::<MemoryAddress>()?,
                parts
                    .next()
                    .ok_or(anyhow!("missing argument for SWAP"))?
                    .parse::<MemoryAddress>()?,
            )),
            "XOR" => Ok(Instruction::XOR(
                parts
                    .next()
                    .ok_or(anyhow!("missing argument for XOR"))?
                    .parse::<MemoryAddress>()?,
                parts
                    .next()
                    .ok_or(anyhow!("missing argument for XOR"))?
                    .parse::<MemoryAddress>()?,
            )),
            "INC" => Ok(Instruction::Inc(
                parts
                    .next()
                    .ok_or(anyhow!("missing argument for INC"))?
                    .parse::<MemoryAddress>()?,
            )),
            _ => Err(anyhow!("invalid instruction")),
        }
    }
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
