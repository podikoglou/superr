use anyhow::anyhow;
use itertools::Itertools;
use rayon::{
    iter::{ParallelBridge, ParallelIterator},
    Scope,
};
use std::{mem, sync::atomic::Ordering};
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{MemValue, MEM_SIZE, VM},
};

use super::{Optimizer, OptimizerArgs};

const INSTRUCTIONS: [&'static str; 7] = ["LOAD", "SWAP", "XOR", "INC", "DECR", "ADD", "SUB"];

pub struct ExhaustiveOptimizer {
    pub args: OptimizerArgs,
}

impl Optimizer for ExhaustiveOptimizer {
    fn new(args: OptimizerArgs) -> Self {
        Self { args }
    }

    fn start_optimization<'a>(&'a mut self, _: &Scope<'a>) {
        let programs = self.generate_programs();
        let counter = self.args.counter.clone();

        programs
            .into_iter()
            .par_bridge()
            .try_for_each(|program| -> Result<(), anyhow::Error> {
                // if we need to stop, throw an error.
                // since we're using try_for_each, throwing an error
                // will cause the function to stop.
                //
                // there's literally no other way to stop.
                if self.should_stop() {
                    return Err(anyhow!("Forcefully stopping"));
                }

                // compute the state of the program and compare it to the target state
                let state = VM::compute_state(&program);

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

                Ok(())
            })
            .ok();
    }

    fn current_optimal_length(&self) -> usize {
        self.args.optimal.read().unwrap().instructions.len()
    }

    fn should_stop(&self) -> bool {
        self.args.should_stop.load(Ordering::Relaxed)
    }

    fn worker_loop(&self) {
        todo!()
    }
}

impl ExhaustiveOptimizer {
    fn generate_programs(&self) -> impl Iterator<Item = Program> + '_ {
        let max_length = self.args.max_instructions;

        (1..=max_length).flat_map(move |length| {
            INSTRUCTIONS
                .iter()
                .combinations_with_replacement(length)
                .flat_map(move |inst_combo| {
                    inst_combo
                        .iter()
                        .map(|&inst| self.gen_arg_sets(inst))
                        .multi_cartesian_product()
                        .map(move |args| Program {
                            instructions: inst_combo
                                .iter()
                                .zip(args)
                                .map(|(&inst, args)| self.create_instruction(inst, args))
                                .collect(),
                        })
                })
        })
    }

    fn gen_arg_sets(&self, instruction: &str) -> Vec<[usize; 2]> {
        match instruction {
            "LOAD" => (0..=self.args.max_num as usize)
                .map(|val| [val, 0])
                .collect_vec(),

            "SWAP" | "XOR" | "ADD" | "SUB" => (0..MEM_SIZE)
                .cartesian_product(0..MEM_SIZE)
                .map(|(a, b)| [a, b])
                .collect(),

            "INC" | "DECR" => (0..MEM_SIZE).map(|val| [val, 0]).collect(),

            _ => panic!("Unknown instruction: {}", instruction),
        }
    }

    fn create_instruction(&self, inst: &str, args: [usize; 2]) -> Instruction {
        match inst {
            "LOAD" => Instruction::Load(args[0] as MemValue),

            "SWAP" => Instruction::Swap(args[0], args[1]),

            "XOR" => Instruction::XOR(args[0], args[1]),

            "INC" => Instruction::Inc(args[0]),
            "DECR" => Instruction::Decr(args[0]),

            "ADD" => Instruction::Add(args[0], args[1]),
            "SUB" => Instruction::Sub(args[0], args[1]),

            _ => panic!("Unknown instruction: {}", inst),
        }
    }
}
