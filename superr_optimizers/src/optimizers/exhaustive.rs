use anyhow::anyhow;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};
use rayon::{
    iter::{ParallelBridge, ParallelIterator},
    ThreadPool,
};
use std::{
    mem,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, RwLock,
    },
    thread,
    time::{Duration, Instant},
};
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{State, MEM_SIZE, VM},
};

use crate::Optimizer;

const INSTRUCTIONS: [&'static str; 4] = ["LOAD", "SWAP", "XOR", "INC"];

pub struct ExhaustiveOptimizerState {
    pub optimal: Arc<RwLock<Program>>,
    pub target_state: State,

    pub started: Option<Instant>,
    pub should_stop: Arc<AtomicBool>,

    pub thread_pool: ThreadPool,

    pub counter: Arc<AtomicU64>,
}

pub struct ExhaustiveOptimizerOptions {
    pub max_instructions: usize,
    pub max_num: usize,
}

pub struct ExhaustiveOptimizer {
    pub state: ExhaustiveOptimizerState,
    pub options: ExhaustiveOptimizerOptions,
}

impl Optimizer for ExhaustiveOptimizer {
    type Options = ExhaustiveOptimizerOptions;
    type State = ExhaustiveOptimizerState;

    fn new(options: Self::Options, program: superr_vm::program::Program) -> Self {
        let target_state = VM::compute_state(&program);

        Self {
            options,
            state: ExhaustiveOptimizerState {
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

            // create programs iterator, run workers
            let programs = self.generate_programs();
            let counter = self.state.counter.clone();

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
                    Ok(())
                })
                .ok();
        });

        self.optimal()
    }

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
        todo!()
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

impl ExhaustiveOptimizer {
    fn generate_programs(&self) -> impl Iterator<Item = Program> + '_ {
        let max_length = self.options.max_instructions;

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
            "LOAD" => (0..=self.options.max_num).map(|val| [val, 0]).collect_vec(),

            "SWAP" | "XOR" => (0..MEM_SIZE)
                .cartesian_product(0..MEM_SIZE)
                .map(|(a, b)| [a, b])
                .collect(),

            "INC" => (0..MEM_SIZE).map(|val| [val, 0]).collect(),

            _ => panic!("Unknown instruction: {}", instruction),
        }
    }

    fn create_instruction(&self, inst: &str, args: [usize; 2]) -> Instruction {
        match inst {
            "LOAD" => Instruction::Load(args[0]),
            "SWAP" => Instruction::Swap(args[0], args[1]),
            "XOR" => Instruction::XOR(args[0], args[1]),
            "INC" => Instruction::Inc(args[0]),

            _ => panic!("Unknown instruction: {}", inst),
        }
    }
}
