use std::{mem, sync::atomic::Ordering};

use rayon::Scope;
use superr_vm::vm::{State, VM};

use crate::generate_instruction;

use super::{Optimizer, OptimizerArgs};

pub struct DiffingOptimizer {
    pub args: OptimizerArgs,
}

impl Optimizer for DiffingOptimizer {
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

        // start with an empty program
        //let mut current_program = Program::new();
        let mut current_score = DiffingOptimizer::score(&vm.state, &self.args.target);

        while !self.should_stop() {
            vm.reset();

            // TODO: can this be simplified?
            let mut new_program = self.args.optimal.read().unwrap().clone();

            new_program
                .instructions
                .push(generate_instruction(self.args.max_num));
            vm.execute_program(new_program.clone());

            let new_score = DiffingOptimizer::score(&vm.state, &self.args.target);

            if current_score > new_score
                && new_program.instructions.len() < self.current_optimal_length()
            {
                // update optimal program
                eprintln!(
                    "Found more optimal program ({} instructions)",
                    new_program.instructions.len()
                );

                {
                    let mut lock = self.args.optimal.write().unwrap();

                    let _ = mem::replace(&mut *lock, new_program);
                }

                current_score = new_score;

                if new_score == 0.0 {
                    self.should_stop();
                }
            }

            // increment the counter
            counter.fetch_add(1, Ordering::Relaxed);
        }
    }
}

impl DiffingOptimizer {
    /// Randomly generates a single instruction based on the [`DiffingOptimizerOptions`].
    /// Euclidean distance.
    ///
    /// Developer Note: Maybe apply penalty based on length?
    fn score(a: &State, b: &State) -> f32 {
        a.iter()
            .zip(b)
            .map(|(&a, &b)| (a as f32 - b as f32).powi(2))
            .sum::<f32>()
            .sqrt()
    }
}
