use anyhow::Context;
use clap::ArgMatches;
use superr_vm::{instruction::Instruction, vm};

use crate::INSTRUCTIONS;

pub fn execute(matches: &ArgMatches) -> anyhow::Result<()> {
    let min_instructions = matches.get_one::<usize>("min-ins").unwrap();
    let max_instructions = matches.get_one::<usize>("max-ins").unwrap();

    let min_imm = matches.get_one::<u8>("min-imm").unwrap();
    let max_imm = matches.get_one::<u8>("max-imm").unwrap();

    let excluded: Vec<&String> = match matches.get_many("exclude") {
        Some(v) => v.collect(),
        None => vec![],
    };

    // TODO: clean this up
    let ins_space: Vec<&str> = INSTRUCTIONS
        .into_iter()
        .filter(|ins| !excluded.contains(&&(*ins.to_string()).to_string()))
        .collect();

    for _ in 0..fastrand::usize(*min_instructions..=*max_instructions) {
        let addr1 = fastrand::usize(0..vm::MEM_SIZE);
        let addr2 = fastrand::usize(0..vm::MEM_SIZE);

        let imm = fastrand::u8(min_imm..=max_imm);

        let choice = fastrand::choice(&ins_space).context("invalid iterator length")?;

        let instruction = match *choice {
            "load" => Instruction::Load(imm),
            "swap" => Instruction::Swap(addr1, addr2),
            "xor" => Instruction::XOR(addr1, addr2),
            "inc" => Instruction::Inc(addr1),
            "decr" => Instruction::Decr(addr1),
            "add" => Instruction::Add(addr1, addr2),
            "sub" => Instruction::Sub(addr1, addr2),
            "put" => Instruction::Put(addr1),
            // "jump" => Instruction::Jump(???)
            _ => unreachable!(),
        };

        println!("{}", instruction.to_string());
    }

    Ok(())
}
