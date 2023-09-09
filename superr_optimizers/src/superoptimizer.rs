use std::time::{Duration, Instant};

use rand::{thread_rng, Rng};
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{self, VM},
};

use crate::optimizer::Optimizer;

pub struct Superoptimizer {
    pub max_instructions: usize,
    pub max_time: Duration,
}

impl Superoptimizer {
    pub fn new(max_instructions: usize, max_time: Duration) -> Self {
        Self {
            max_instructions,
            max_time,
        }
    }
}

impl Optimizer for Superoptimizer {
    fn optimize(&self, input: Program) -> Program {
        // create vm
        let mut vm = VM::default();

        vm.execute_program(input.clone());

        let target_state = vm.state.clone();

        dbg!(target_state);

        // state
        let started = Instant::now();

        let mut shortest: Program = Program {
            instructions: vec![],
        };

        // start bruteforcing optimal programs
        loop {
            // generate program
            let mut instructions = vec![];

            let instructions_amount = rand::thread_rng().gen_range(1..=self.max_instructions);

            for _ in 0..instructions_amount {
                let instruction_index = thread_rng().gen_range(0..=3);

                let addr01: usize = thread_rng().gen_range(0..vm::MEM_SIZE);
                let addr02: usize = thread_rng().gen_range(0..vm::MEM_SIZE);

                let val: usize = thread_rng().gen_range(0..32);

                match instruction_index {
                    0 => instructions.push(Instruction::Load(val as u32)),
                    1 => instructions.push(Instruction::Swap(addr01, addr02)),
                    2 => instructions.push(Instruction::XOR(addr01, addr02)),
                    3 => instructions.push(Instruction::Inc(addr01)),

                    _ => {}
                }
            }

            // execute program
            let program = Program { instructions };

            vm.reset();
            vm.execute_program(program.clone());

            dbg!(vm.state);

            // is it equivalent to the input program?
            // if so, check if it's also the shortest equivalent
            if vm.state == target_state {
                // if it's the shortest one encountered (or te first equivalent), set it as the shortest
                if shortest.instructions.len() == 0
                    || shortest.instructions.len() > program.instructions.len()
                {
                    shortest = program.clone();
                }
            }

            // if we're out of time return, return the shortest one we found
            if started.elapsed() >= self.max_time {
                // if we haven't found any optimized version of the given program, just return the
                // original.
                if shortest.instructions.len() == 0 {
                    return input;
                }
                return shortest;
            }
        }
    }
}

mod test {
    use superr_vm::{instruction::Instruction, program::Program, vm::VM};

    use crate::optimizer::Optimizer;

    use super::Superoptimizer;

    #[test]
    fn optimize_3() {
        let original = Program {
            instructions: vec![
                Instruction::Load(3),
                Instruction::Swap(0x00, 0x01),
                Instruction::Load(3),
                Instruction::Swap(0x00, 0x02),
                Instruction::Load(3),
                Instruction::Swap(0x00, 0x03),
                Instruction::Load(3),
            ],
        };

        let mut vm_original = VM::default();
        vm_original.execute_program(original.clone());

        // optimize
        let optimizer = Superoptimizer::new(4);
        let optimized = optimizer.optimize(original);

        let mut vm_optimized = VM::default();
        vm_optimized.execute_program(optimized);

        assert_eq!(vm_original.state, vm_optimized.state);
    }
}
