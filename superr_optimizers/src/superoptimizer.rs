use humantime;
use std::time::{Duration, Instant};

use num_format::{Locale, ToFormattedString};

use rand::Rng;
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{self, VM},
};

use crate::optimizer::Optimizer;

pub struct Superoptimizer {
    pub max_instructions: usize,
    pub max_num: usize,
    pub timeout: Duration,
}

impl Superoptimizer {
    pub fn new(max_instructions: usize, max_num: usize, timeout: Duration) -> Self {
        Self {
            max_instructions,
            max_num,
            timeout,
        }
    }
}

impl Optimizer for Superoptimizer {
    fn optimize(&self, input: Program) -> Program {
        // this is a timer for the timeout, we want to stop searching when it's been x
        // amount of seconds since this instant
        let started = Instant::now();

        // this is a timer for the progress report. we reset it ever 1 second.
        let mut started_progress = Instant::now();
        let mut programs_counter = 0;

        // compute the target state
        let target_state = VM::compute_state(&input);

        // move the original program to the `shortest` variable, as it is the
        // shortest version of the program we have.
        let mut shortest = input;

        // start generating absolutely random programs in hopes that one of them is equivalent to
        // the original one.
        loop {
            let program = self.generate_program();

            // compute the state of the program
            let state = VM::compute_state(&program);

            // check if this program is equivalent to the given one by checking if the states they
            // produce are equal.
            if state == target_state {
                // if the state is equivalent to the target state, check if this is the shortest
                // equivalent program we've encountered. if so, set it to the shortest variable.
                if shortest.instructions.len() > program.instructions.len() {
                    shortest = program.clone();

                    eprintln!(
                        "Found {}-instruction long equivalent program -- continuing search",
                        shortest.instructions.len()
                    );
                } else {
                    eprintln!("Found equivalent program but wasn't shorter")
                }
            }

            // if we're out of time, return the shortest program we've found
            if started.elapsed() >= self.timeout {
                return shortest;
            }

            // progress report
            if started_progress.elapsed() >= Duration::from_secs(1) {
                started_progress = Instant::now();

                eprintln!(
                    "[{} / {} ]: {} programs tested",
                    humantime::format_duration(normalize_duration(started.elapsed())),
                    humantime::format_duration(normalize_duration(self.timeout)),
                    programs_counter.to_formatted_string(&Locale::en)
                );
            }

            programs_counter += 1;
        }
    }
}

impl Superoptimizer {
    fn generate_program(&self) -> Program {
        let mut program = Program::new();

        let mut rng = rand::thread_rng();

        // generate a random amount of instructions for the program to have. this amount is
        // within 0 and the given max_instructions.
        let instructions_amount = rng.gen_range(0..=self.max_instructions);

        // generate the instructions of the program
        for _ in 0..instructions_amount {
            let reg1 = rng.gen_range(0..vm::MEM_SIZE);
            let reg2 = rng.gen_range(0..vm::MEM_SIZE);

            let val = rng.gen_range(0..self.max_num);

            let instruction = rng.gen_range(0..=3);

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

fn normalize_duration(duration: Duration) -> Duration {
    Duration::from_secs(duration.as_secs())
}
