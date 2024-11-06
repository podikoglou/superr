use std::{
    mem,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

use indicatif::{ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};
use rayon::ThreadPool;
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{self, State, VM},
};

use super::Optimizer;

pub struct RandomSearchOptimizerState {
    pub optimal: Arc<RwLock<Program>>,
    pub target_state: State,

    pub started: Option<Instant>,
    pub should_stop: Arc<AtomicBool>,

    pub thread_pool: ThreadPool,

    pub counter: Arc<AtomicU64>,
}

pub struct RandomSearchOptimizerOptions {
    pub max_num: usize,
    pub max_instructions: usize,
}

pub struct RandomSearchOptimizer {
    pub state: RandomSearchOptimizerState,
    pub options: RandomSearchOptimizerOptions,
}

impl Optimizer for RandomSearchOptimizer {
    type Options = RandomSearchOptimizerOptions;
    type State = RandomSearchOptimizerState;

    fn new(options: Self::Options, program: superr_vm::program::Program) -> Self {
        let target_state = VM::compute_state(&program);

        Self {
            options,
            state: RandomSearchOptimizerState {
                optimal: Arc::new(RwLock::new(program)),
                target_state: target_state,

                started: None,
                should_stop: Arc::new(AtomicBool::new(false)),

                thread_pool: rayon::ThreadPoolBuilder::new().build().unwrap(),

                counter: Arc::new(AtomicU64::new(0)),
            },
        }
    }

    fn start_optimization(&mut self) -> Program {
        self.state.started = Some(Instant::now());
        self.state.should_stop = Arc::new(AtomicBool::new(false));

        // set ctrl+c handler
        let should_stop = self.state.should_stop.clone();

        ctrlc::set_handler(move || {
            should_stop.store(true, Ordering::Relaxed);
        })
        .expect("Error setting Ctrl-C handler");

        self.state.thread_pool.scope(|scope| {
            // run the progress-reporting thread
            scope.spawn(|_| self.progress_loop());

            // run the worker threads for computing the shortest possible program
            for _ in 0..rayon::current_num_threads() - 1 {
                scope.spawn(|_| self.work());
            }
        });

        self.optimal()
    }

    /// This should *probably* only be called once.
    fn optimal(&mut self) -> superr_vm::program::Program {
        match Arc::try_unwrap(mem::take(&mut self.state.optimal)) {
            Ok(shortest) => shortest.into_inner().unwrap(),
            Err(arc) => {
                // this shouldn't happen, but if it does, we can still
                // read the value, just by cloning
                arc.read().unwrap().clone()
            }
        }
    }

    fn current_optimal_length(&self) -> usize {
        self.state.optimal.read().unwrap().instructions.len()
    }

    fn should_stop(&self) -> bool {
        self.state.should_stop.load(Ordering::Relaxed)
    }

    fn work(&self) {
        let mut vm = VM::default();
        let counter = self.state.counter.clone();

        while !self.should_stop() {
            vm.reset();

            // generate a completely random program, and compute its state
            let program = self.generate_program();
            vm.execute_program(&program);
            let state = vm.state;

            // let's check if the state we just computed is equal to our target_state
            if self.state.target_state == state {
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
                        let mut lock = self.state.optimal.write().unwrap();

                        let _ = mem::replace(&mut *lock, program);
                    }
                }
            }

            // increment the counter
            counter.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn progress_loop(&self) {
        let counter = self.state.counter.clone();

        let mut last_count = 0;

        // create progress bar
        let bar = ProgressBar::new_spinner();

        bar.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );

        while !self.should_stop() {
            thread::sleep(Duration::from_secs(1));

            // load counter
            let current = counter.load(Ordering::Relaxed);

            // calculate programs per second
            let programs_per_second = current - last_count;

            let message = format!(
                "{} Programs tested | {}/s",
                current.to_formatted_string(&Locale::en),
                programs_per_second.to_formatted_string(&Locale::en),
            );

            bar.set_message(message);
            bar.tick();

            last_count = current;
        }
    }
}

impl RandomSearchOptimizer {
    /// Randomly generates a program based on the [`RandomSearchOptimizerOptions`].
    fn generate_program(&self) -> Program {
        let mut program = Program::new();

        // generate a random amount of instructions for the program to have. this amount is
        // within 0 and the given max_instructions.
        let instructions_amount = fastrand::usize(0..=self.options.max_instructions);

        // generate the instructions of the program
        for _ in 0..instructions_amount {
            let reg1 = fastrand::usize(0..vm::MEM_SIZE);
            let reg2 = fastrand::usize(0..vm::MEM_SIZE);

            let val = fastrand::usize(0..self.options.max_num);

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
