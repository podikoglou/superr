use std::io::{self, BufRead};

use superr_vm::{instruction::Instruction, program::Program};

use super::RunSubcommand;

pub fn execute(_: RunSubcommand) {
    let mut program = Program::new();
    let lines = io::stdin().lock().lines();

    for line in lines {
        match line {
            Ok(v) => program.instructions.push(Instruction::parse(&v).unwrap()),
            Err(_) => break,
        }
    }
}
