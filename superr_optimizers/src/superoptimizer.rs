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
        let started = Instant::now();

        // create VM for testing
        let mut vm = VM::default();

        // execute original program to determine target state
        vm.execute_program(input.clone());

        let target_state = vm.state.clone();

        // move the original program to the `shortest` variable, as it is the shortest version of
        // the program we have.
        let mut shortest = input;

        // start generating absolutely random program in hopes that one of them is equivalent to
        // the original one.
        loop {
            let mut instructions = vec![];

            // generate a random amount of instructions for the program to have. this amount is
            // within 0 and the given max_instructions.
            let instructions_amount = rand::thread_rng().gen_range(0..=self.max_instructions);

            // generate the program instructions
            for _ in 0..instructions_amount {
                // generate a random instruction by its index (0-3)
                let instruction_index = thread_rng().gen_range(0..=3);

                // generate two random addresses. the second one is redundant for instructions like
                // Inc and Load.
                let addr01: usize = thread_rng().gen_range(0..vm::MEM_SIZE);
                let addr02: usize = thread_rng().gen_range(0..vm::MEM_SIZE);

                // generates a random value. this is reundant for instructions other than load.
                let val: usize = thread_rng().gen_range(0..32);

                // convert the instruction_index we generater earlier and the arguments we just
                // generated to an actual instruction and add it to the instructions list.
                match instruction_index {
                    0 => instructions.push(Instruction::Load(val as u32)),
                    1 => instructions.push(Instruction::Swap(addr01, addr02)),
                    2 => instructions.push(Instruction::XOR(addr01, addr02)),
                    3 => instructions.push(Instruction::Inc(addr01)),

                    _ => {}
                }
            }

            // actually create the program
            let program = Program { instructions };

            // reset the memory from the previous iteration and execute the program to see if the
            // state matches.
            vm.reset();
            vm.execute_program(program.clone());

            // check if this program is equivalent to the given one by checking if the states they
            // produce are equal.
            if vm.state == target_state {
                // if the state is equiavlent to the target state, check if this is the shortest
                // equivalent program we've encountered. if so, set it to the shortest variable.
                if shortest.instructions.len() == 0
                    || shortest.instructions.len() > program.instructions.len()
                {
                    shortest = program.clone();
                }
            }

            // if we're out of time, return the shortest program we've found
            if started.elapsed() >= self.max_time {
                return shortest;
            }
        }
    }
}
