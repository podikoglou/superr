use std::{
    io::{self, BufRead},
    str::FromStr,
};

use superr_vm::{instruction::Instruction, program::Program, vm::VM};

use super::RunSubcommand;

pub fn execute(_: RunSubcommand) {
    // properly read program from stdin
    let mut program = Program::new();
    let lines = io::stdin().lock().lines();

    for line in lines {
        match line {
            Ok(v) => program
                .instructions
                .push(Instruction::from_str(&v).unwrap()),
            Err(_) => break,
        }
    }

    // create vm
    let mut vm = VM::default();

    vm.execute_program(&program);

    dbg!(vm);
}
