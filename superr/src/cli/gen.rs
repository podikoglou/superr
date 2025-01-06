use superr_vm::{instruction::Instruction, vm};

use super::GenSubcommand;

pub fn execute(args: GenSubcommand) {
    for _ in 0..args.instructions {
        let reg1 = fastrand::usize(0..vm::MEM_SIZE);
        let reg2 = fastrand::usize(0..vm::MEM_SIZE);

        // TODO: cli args
        let val = fastrand::u8(0..12);
        let instruction = fastrand::usize(0..=3);

        let instruction = match instruction {
            0 => Instruction::Load(val),
            1 => Instruction::Swap(reg1, reg2),
            2 => Instruction::XOR(reg1, reg2),
            3 => Instruction::Inc(reg1),
            _ => panic!("SUPER unexpected error occurred"),
        };

        println!("{}", instruction.to_string());
    }
}
