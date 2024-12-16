use std::io::{self, BufRead};

use superr_vm::{instruction::Instruction, program::Program, vm::VM};

use super::RunSubcommand;

pub fn execute(_: RunSubcommand) {
    // properly read program from stdin
    let mut program = Program::new();
    let lines = io::stdin().lock().lines();

    for line in lines {
        match line {
            Ok(v) => {
                if !v.is_empty() {
                    program.instructions.push(Instruction::from(v))
                }
            }
            Err(_) => break,
        }
    }

    // create vm
    let mut vm = VM::default();

    vm.execute_program(program);

    dbg!(vm);
}
