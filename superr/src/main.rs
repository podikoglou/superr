use std::time::Duration;

use rand::Rng;
use superr_optimizers::{optimizer::Optimizer, superoptimizer::Superoptimizer};
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{self, VM},
};

fn main() {
    // generate program
    let mut original = Program {
        instructions: vec![],
    };

    for _ in 0..5000 {
        let reg1 = rand::thread_rng().gen_range(0..vm::MEM_SIZE);
        let reg2 = rand::thread_rng().gen_range(0..vm::MEM_SIZE);

        let val = rand::thread_rng().gen_range(0..32);

        let instruction = rand::thread_rng().gen_range(0..=3);

        match instruction {
            0 => original.instructions.push(Instruction::Load(val)),
            1 => original.instructions.push(Instruction::Swap(reg1, reg2)),
            2 => original.instructions.push(Instruction::XOR(reg1, reg2)),
            3 => original.instructions.push(Instruction::Inc(reg1)),
            _ => {}
        }
    }

    // run original program
    let mut vm1 = VM::default();
    vm1.execute_program(original.clone());

    dbg!(vm1.state);
    // dbg!(original.clone());
    dbg!(original.instructions.len());

    // optimize program
    let optimizer = Superoptimizer::new(6, Duration::new(2, 0));
    let optimized = optimizer.optimize(original);

    println!("Finished optimizing");

    // run optimized program
    let mut vm2 = VM::default();
    vm2.execute_program(optimized.clone());

    println!("---------------------------");

    dbg!(vm2.state);
    // dbg!(optimized.clone());
    dbg!(optimized.instructions.len());
}
