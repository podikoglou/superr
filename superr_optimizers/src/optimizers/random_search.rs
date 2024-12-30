use std::{
    mem,
    sync::atomic::{AtomicUsize, Ordering},
};

use rayon::Scope;
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{self, VM},
};

use super::{Optimizer, OptimizerArgs};

pub struct RandomSearchOptimizer {
    pub args: OptimizerArgs,
    pub max_instructions: AtomicUsize,
}

impl Optimizer for RandomSearchOptimizer {
    fn new(args: OptimizerArgs) -> Self {
        Self {
            max_instructions: AtomicUsize::new(args.max_instructions),
            args,
        }
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
            vm.execute_program(program.clone());

            let state = vm.state;

            // let's check if the state we just computed is equal to our target_state
            if self.args.target == state {
                // we now need to check if this program is shorter than the given program
                // (there is a chance that it's not, depending on the options)
                if program.instructions.len() < self.current_optimal_length() {
                    // since the program we found is more efficient, we update the optimal
                    // program to be the one we just found.

                    let new_len = program.instructions.len();

                    eprintln!("Found more optimal program ({} instructions)", new_len);

                    {
                        let mut lock = self.args.optimal.write().unwrap();

                        let _ = mem::replace(&mut *lock, program);
                    }

                    // update max_instructions so we can look for programs even
                    // shorter than what we just found
                    self.max_instructions.store(new_len - 1, Ordering::Relaxed);
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
        let max_instructions = self.max_instructions.load(Ordering::Relaxed);
        let instructions_amount = fastrand::usize(0..=max_instructions);

        // generate the instructions of the program
        for _ in 0..instructions_amount {
            let instruction = fastrand::usize(0..=3);

            let instruction = match instruction {
                0 => {
                    let val = fastrand::usize(0..self.args.max_num);

                    Instruction::Load(val)
                }

                1 | 2 => {
                    let addr1 = fastrand::usize(0..vm::MEM_SIZE);
                    let addr2 = fastrand::usize(0..vm::MEM_SIZE);

                    match instruction {
                        1 => Instruction::Swap(addr1, addr2),
                        2 => Instruction::XOR(addr1, addr2),

                        _ => panic!("SUPER unexpected error occurred"),
                    }
                }

                3 => {
                    let addr = fastrand::usize(0..vm::MEM_SIZE);

                    Instruction::Jmp(addr)
                }

                _ => panic!("SUPER unexpected error occurred"),
            };

            program.instructions.push(instruction);
        }

        if Self::is_legal(&program) {
            program
        } else {
            self.generate_program()
        }
    }

    /// Simple filter to filter out programs which cannot possibly be optimal.
    ///
    /// In some cases, this may simply not be worth running (if there aren't
    /// that many instructions in the given program).
    fn is_legal(program: &Program) -> bool {
        let mut previous: Option<&Instruction> = None;

        for (idx, instruction) in program.instructions.iter().enumerate() {
            previous = Some(instruction);

            if let Some(previous) = previous {
                match previous {
                    Instruction::Load(_) => {
                        // if the previous instruction and the current
                        // instruction were both LOAD instructions, filter them out
                        //
                        // (the second LOAD overwrites the value of the first)
                        if let Instruction::Load(_) = instruction {
                            return false;
                        }
                    }
                    Instruction::Swap(a1, b1) | Instruction::XOR(a1, b1) => {
                        if let Instruction::Swap(a2, b2) | Instruction::XOR(a2, b2) = instruction {
                            if (a1 == a2 && b1 == b2)
                                || (a1 == b2 && b1 == a2)
                                || a1 == a2
                                || a1 == b2
                                || b1 == a2
                                || b1 == b2
                            {
                                return false;
                            }
                        }
                    }
                    Instruction::Jmp(line) => {
                        // 1. Does this instruction even exist?
                        if program.instructions.len() <= *line {
                            return false;
                        }

                        // 2. Is this instruction the same we're on right now?
                        if *line == idx {
                            return false;
                        }
                    }

                    _ => {}
                }
            }
        }

        true
    }
}
