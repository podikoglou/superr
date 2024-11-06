use std::{mem, sync::atomic::Ordering};

use rayon::Scope;
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{self, VM},
};

use super::{Optimizer, OptimizerArgs};

pub struct RandomSearchOptimizer {
    pub args: OptimizerArgs,
}

impl Optimizer for RandomSearchOptimizer {
    fn new(args: OptimizerArgs) -> Self {
        Self { args }
    }

    fn start_optimization<'a>(&'a mut self, scope: &Scope<'a>) {
        if self.should_stop() {
            return;
        }

        // run the worker threads for computing the shortest possible program
        for _ in 0..rayon::current_num_threads() - 1 {
            scope.spawn(|_| self.worker_loop());
        }
    }

    fn current_optimal_length(&self) -> usize {
        self.args.optimal.read().unwrap().instructions.len()
    }

    fn should_stop(&self) -> bool {
        self.args.should_stop.load(Ordering::Relaxed)
    }

    fn worker_loop(&self) {
        let mut vm = VM::default();

        let counter = self.args.counter.clone();

        while !self.should_stop() {
            vm.reset();

            // generate a completely random program, and compute its state
            let program = self.generate_program();
            vm.execute_program(&program);

            let state = vm.state;

            // let's check if the state we just computed is equal to our target_state
            if self.args.target == state {
                // we now need to check if this program is shorter than the given program
                // (there is a chance that it's not, depending on the options)
                if program.instructions.len() < self.current_optimal_length() {
                    // since the program we found is more efficient, we update the optimal
                    // program to be the one we just found.

                    eprintln!(
                        "Found more optimal program ({} instructions)",
                        program.instructions.len()
                    );

                    {
                        let mut lock = self.args.optimal.write().unwrap();

                        let _ = mem::replace(&mut *lock, program);
                    }
                }
            }

            // increment the counter
            counter.fetch_add(1, Ordering::Relaxed);
        }
    }
}

impl RandomSearchOptimizer {
    /// Randomly generates a program based on the [`RandomSearchOptimizerOptions`].
    fn generate_program(&self) -> Program {
        let mut program = Program::new();

        // generate a random amount of instructions for the program to have. this amount is
        // within 0 and the given max_instructions.
        let instructions_amount = fastrand::usize(0..=self.args.max_instructions);

        // generate the instructions of the program
        for _ in 0..instructions_amount {
            let reg1 = fastrand::usize(0..vm::MEM_SIZE);
            let reg2 = fastrand::usize(0..vm::MEM_SIZE);

            let val = fastrand::usize(0..self.args.max_num);

            let instruction = fastrand::usize(0..=3);

            let instruction = match instruction {
                0 => Instruction::Load(val),
                1 => Instruction::Swap(reg1, reg2),
                2 => Instruction::XOR(reg1, reg2),
                3 => Instruction::Inc(reg1),
                _ => panic!("SUPER unexpected error occurred"),
            };
            program.instructions.push(instruction);
        }

        program
    }
}
