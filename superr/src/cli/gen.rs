use std::io::Write;

use superr_vm::{
    instruction::{new_inc, new_load, new_swap, new_xor},
    program::Program,
    vm,
};

use super::GenSubcommand;

pub fn execute(args: GenSubcommand) {
    let mut program = Program::new();

    for _ in 0..args.instructions {
        // NOTE: this is unsafe (converting usize to u8)
        let reg1 = fastrand::u8(0..vm::MEM_SIZE as u8);
        let reg2 = fastrand::u8(0..vm::MEM_SIZE as u8);

        let val = fastrand::u8(0..=args.instructions as u8);

        let instruction = fastrand::u8(0..=3);

        let instruction = match instruction {
            0 => new_load(val),
            1 => new_swap(reg1, reg2),
            2 => new_xor(reg1, reg2),
            3 => new_inc(reg1),
            _ => panic!("SUPER unexpected error occurred"),
        };
        program.instructions.push(instruction);
    }

    let output: Vec<u8> = program.into();

    std::io::stdout().write_all(&output).unwrap()
}
