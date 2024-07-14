use humantime::{self, format_duration};
use indicatif::{ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};
use std::{
    mem,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

use rand::Rng;
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{self, State, VM},
};

use crate::Optimizer;

pub struct RandomSearchOptimizerOptions {
    pub max_instructions: usize,
    pub max_num: usize,
    pub timeout: Duration,
    pub progress_frequency: u64,
}

pub struct RandomSearchOptimizer {
    pub input: Program,

    pub options: RandomSearchOptimizerOptions,

    pub target_state: Option<State>,

    // state shared between threads
    pub started: Option<Instant>,
    pub counter: Arc<AtomicU64>,
    pub shortest: Arc<RwLock<Program>>,
}

impl RandomSearchOptimizer {
    pub fn new(input: Program, options: RandomSearchOptimizerOptions) -> Self {
        Self {
            input: input.clone(),

            options,

            started: None,
            target_state: Some(VM::compute_state(&input)),
            counter: Arc::new(AtomicU64::new(0)),
            shortest: Arc::new(RwLock::new(input)),
        }
    }
}

impl Optimizer for RandomSearchOptimizer {
    fn optimize(&mut self) -> Program {
        // this is a timer for the timeout, we want to stop searching when it's been x
        // amount of seconds since this instant
        self.started = Some(Instant::now());

        rayon::scope(|scope| {
            // run the progress-reporting thread
            scope.spawn(|_| self.run_progress_loop());

            // run the worker threads for computing the shortest possible program
            for i in 0..rayon::current_num_threads() {
                eprintln!("Starting thread #{}", i);

                scope.spawn(|_| self.run_computation_loop())
            }
        });

        // scope basically joins all the threads, so it blocks until all of them
        // are finished.
        eprintln!("Stopping");

        match Arc::try_unwrap(mem::take(&mut self.shortest)) {
            Ok(shortest) => shortest.into_inner().unwrap(),
            Err(arc) => {
                // this should'nt happen, but if it does, we can still
                // read the value, just by cloning
                arc.read().unwrap().clone()
            }
        }
    }
}

impl RandomSearchOptimizer {
    #[inline(always)]
    fn should_stop(&self) -> bool {
        self.started.unwrap().elapsed() >= self.options.timeout
    }

    fn run_progress_loop(&self) {
        let counter = self.counter.clone();
        let started = self.started.unwrap().clone();

        let progress_frequency = Duration::from_millis(self.options.progress_frequency);

        // create progress bar
        let bar = ProgressBar::new_spinner();

        bar.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
        );

        let mut last_count = 0;

        loop {
            if self.should_stop() {
                break;
            }

            thread::sleep(progress_frequency);

            let current = counter.load(Ordering::Relaxed);
            let elapsed = started.elapsed();

            let programs_per_second =
                ((current - last_count) as f64 / progress_frequency.as_secs_f64()) as u64;

            let message = format!(
                "{} Programs tested | {}/s | {}",
                current.to_formatted_string(&Locale::en),
                programs_per_second.to_formatted_string(&Locale::en),
                format_duration(normalize_duration(elapsed))
            );

            bar.set_message(message);
            bar.tick();

            last_count = current;
        }
    }

    fn run_computation_loop(&self) {
        let counter = self.counter.clone();
        let shortest = self.shortest.clone();

        // gets the length of the shortest program. this is a function because
        // the shortest program can be updated at any time, therefore we need
        // to compute it dynamically.
        let shortest_len = || shortest.read().unwrap().instructions.len();

        // im not actually sure if there's a big overhead when unwrapping `Option`s,
        // but it doesn't hurt to just unwrap it once here
        let target_state = self.target_state.unwrap();

        loop {
            if self.should_stop() {
                break;
            }

            // generate a random program, and execute it in a temporary VM
            // to get the end state
            let program = self.generate_program();
            let state = VM::compute_state(&program);

            if state == target_state {
                // we've found an equivalent program!
                // now let's see if it's more efficient or not

                let len = program.instructions.len();

                if len < shortest_len() && len < self.options.max_instructions {
                    // the program is shorter than the `shortest`! (and also,
                    // shortest than max_instructions) now we just need to
                    // update the shortest variable! (kinda hacky)

                    eprintln!(
                        "Found shorter program ({} less instructions)",
                        shortest_len() - len
                    );

                    {
                        let mut lock = shortest.write().unwrap();

                        let _ = mem::replace(&mut *lock, program);
                    }
                }
            }

            // increase the counter
            counter.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn generate_program(&self) -> Program {
        let mut program = Program::new();

        let mut rng = rand::thread_rng();

        // generate a random amount of instructions for the program to have. this amount is
        // within 0 and the given max_instructions.
        let instructions_amount = rng.gen_range(0..=self.options.max_instructions);

        // generate the instructions of the program
        for _ in 0..instructions_amount {
            let reg1 = rng.gen_range(0..vm::MEM_SIZE);
            let reg2 = rng.gen_range(0..vm::MEM_SIZE);

            let val = rng.gen_range(0..self.options.max_num);

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
