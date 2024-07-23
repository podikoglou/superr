use rand::Rng;
use superr_vm::{instruction::Instruction, vm};

use super::GenSubcommand;

pub fn execute(args: GenSubcommand) {
    let mut rng = rand::thread_rng();

    for _ in 0..args.instructions {
        let reg1 = rng.gen_range(0..vm::MEM_SIZE);
        let reg2 = rng.gen_range(0..vm::MEM_SIZE);

        let val = rng.gen_range(0..12);

        let instruction = rng.gen_range(0..=3);

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
