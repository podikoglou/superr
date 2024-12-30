use rayon::Scope;
use std::{mem, sync::atomic::Ordering};
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{MEM_SIZE, VM},
};

use super::{Optimizer, OptimizerArgs};

pub struct GeneticOptimizer {
    pub args: OptimizerArgs,
    pub population: Vec<Program>,
}

impl Optimizer for GeneticOptimizer {
    fn new(args: OptimizerArgs) -> Self {
        let population = vec![Program::new(); 100]; // Initial population size
        Self { args, population }
    }

    fn start_optimization<'a>(&'a mut self, scope: &Scope<'a>) {
        if self.should_stop() {
            return;
        }

        // Run the worker threads for computing the shortest possible program
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

            // Select, mutate, and crossover programs
            let program = self.evolve_program();
            vm.execute_program(&program);

            let state = vm.state;

            // Check if the state matches the target state
            if self.args.target == state {
                if program.instructions.len() < self.current_optimal_length() {
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

            // Increment the counter
            counter.fetch_add(1, Ordering::Relaxed);
        }
    }
}

impl GeneticOptimizer {
    fn evolve_program(&self) -> Program {
        let mut rng = fastrand::Rng::new();

        // Select two parents
        let parent1 = &self.population[rng.usize(0..self.population.len())];
        let parent2 = &self.population[rng.usize(0..self.population.len())];

        // Crossover
        let mut child = self.crossover(parent1, parent2);

        // Mutate
        self.mutate(&mut child);

        child
    }

    fn crossover(&self, parent1: &Program, parent2: &Program) -> Program {
        let mut child = Program::new();
        let crossover_point =
            fastrand::usize(0..parent1.instructions.len().min(parent2.instructions.len()));

        for i in 0..crossover_point {
            child.instructions.push(parent1.instructions[i].clone());
        }
        for i in crossover_point..parent2.instructions.len() {
            child.instructions.push(parent2.instructions[i].clone());
        }

        child
    }

    fn mutate(&self, program: &mut Program) {
        let mutation_rate = 0.1; // 10% mutation rate
        let mut rng = fastrand::Rng::new();

        for instruction in &mut program.instructions {
            if rng.f64() < mutation_rate {
                *instruction = self.random_instruction();
            }
        }
    }

    fn random_instruction(&self) -> Instruction {
        let reg1 = fastrand::usize(0..MEM_SIZE);
        let reg2 = fastrand::usize(0..MEM_SIZE);
        let val = fastrand::usize(0..self.args.max_num);

        match fastrand::usize(0..=3) {
            0 => Instruction::Load(val),
            1 => Instruction::Swap(reg1, reg2),
            2 => Instruction::XOR(reg1, reg2),
            3 => Instruction::Inc(reg1),
            _ => panic!("Unexpected error occurred"),
        }
    }
}
