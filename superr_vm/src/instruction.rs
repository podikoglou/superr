use anyhow::{anyhow, Error};

use crate::address::MemoryAddress;

#[derive(Debug, Clone)]
pub enum Instruction {
    Load(u32),
    Swap(MemoryAddress, MemoryAddress),
    XOR(MemoryAddress, MemoryAddress),
    Inc(MemoryAddress),
}

impl Instruction {
    pub fn parse(input: &str) -> anyhow::Result<Instruction, Error> {
        let mut parts = input.trim().split_whitespace();
        let instruction = parts.next().ok_or(anyhow!("invalid instruction"))?;

        match instruction.to_uppercase().as_str() {
            "LOAD" => Ok(Instruction::Load(
                parts
                    .next()
                    .ok_or(anyhow!("missing argument for LOAD"))?
                    .parse::<u32>()?,
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
