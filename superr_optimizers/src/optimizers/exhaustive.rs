use std::{
    mem,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

use anyhow::anyhow;
use humantime::format_duration;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use superr_vm::{
    instruction::{Instruction},
    program::Program,
    vm::{State, MEM_SIZE, VM},
};

use crate::Optimizer;

const INSTRUCTIONS: [&'static str; 4] = ["LOAD", "SWAP", "XOR", "INC"];

pub struct ExhaustiveOptimizerOptions {
    pub max_instructions: usize,
    pub max_num: usize,
    pub progress_frequency: u64,
}

pub struct ExhaustiveOptimizer {
    pub input: Program,
    pub options: ExhaustiveOptimizerOptions,
    pub target_state: State,

    // state
    pub started: Option<Instant>,
    pub counter: Arc<AtomicUsize>,
    pub shortest: Arc<RwLock<Program>>,
    pub stop: Arc<AtomicBool>,
}

impl ExhaustiveOptimizer {
    pub fn new(input: Program, options: ExhaustiveOptimizerOptions) -> Self {
        Self {
            input: input.clone(),
            options,

            target_state: VM::compute_state(&input),
            started: None,
            counter: Arc::new(AtomicUsize::new(0)),
            shortest: Arc::new(RwLock::new(input)),
            stop: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Optimizer for ExhaustiveOptimizer {
    fn optimize(&mut self) -> Program {
        self.started = Some(Instant::now());

        eprintln!("Generating Programs");

        let counter = self.counter.clone();
        let shortest = self.shortest.clone();
        let stop = self.stop.clone();

        // let's set a ctrl c handler, which makes the program stop when
        // ctrl c is pressed
        ctrlc::set_handler(move || {
            stop.store(true, Ordering::Relaxed);
        })
        .expect("Error setting Ctrl-C handler");

        // gets the length of the shortest program. this is a function because
        // the shortest program can be updated at any time, therefore we need
        // to compute it dynamically.
        let shortest_len = || shortest.read().unwrap().instructions.len();

        rayon::scope(|scope| {
            // run the progress-reporting thread
            scope.spawn(|_| self.run_progress_loop());

            let programs = self.generate_programs();

            // we use try_for_each so that we can stop the processing if
            // ctrl c is pressed by the user, by returning an error.
            programs
                .par_bridge()
                .try_for_each(|program| -> Result<(), anyhow::Error> {
                    if self.should_stop() {
                        return Err(anyhow!("Forcefully stopping"));
                    }

                    // execute it in a temporary VM to get the end state
                    let state = VM::compute_state(&program);

                    if state == self.target_state {
                        // we've found an equivalent program!
                        // now let's see if it's more efficient or not

                        let len = program.instructions.len();

                        if len < shortest_len() && len <= self.options.max_instructions {
                            // the program is shorter than the `shortest`! (and also,
                            // shortest than max_instructions) now we just need to
                            // update the shortest variable! (kinda hacky)

                            eprintln!(
                                "Found shorter program ({} less instructions)",
                                shortest_len() - len
                            );

                            {
                                let mut lock = shortest.write().unwrap();

                                let _ = mem::replace(&mut *lock, program.clone());
                            }
                        }
                    }

                    // increase the counter
                    counter.fetch_add(1, Ordering::Relaxed);

                    Ok(())
                })
                .ok();

            let stop = self.stop.clone();

            stop.store(true, Ordering::Relaxed);
        });

        // scope basically joins all the threads, so it blocks until all of them
        // are finished.
        eprintln!("Stopping");

        match Arc::try_unwrap(mem::take(&mut self.shortest)) {
            Ok(shortest) => shortest.into_inner().unwrap(),
            Err(arc) => {
                // this shouldn't happen, but if it does, we can still
                // read the value, just by cloning
                arc.read().unwrap().clone()
            }
        }
    }
}

impl ExhaustiveOptimizer {
    #[inline(always)]
    fn should_stop(&self) -> bool {
        let stop = self.stop.clone();

        stop.load(Ordering::Relaxed)
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
                                .map(|(&inst, args)| self.create_instruction(inst, &args))
                                .collect(),
                        })
                })
        })
    }

    fn gen_arg_sets(&self, instruction: &str) -> Vec<Vec<usize>> {
        match instruction {
            "LOAD" => (0..=self.options.max_num)
                .map(|val| vec![val])
                .collect_vec(),

            "SWAP" | "XOR" => (0..MEM_SIZE)
                .cartesian_product(0..MEM_SIZE)
                .map(|(a, b)| vec![a, b])
                .collect(),

            "INC" => (0..MEM_SIZE).map(|val| vec![val]).collect(),

            _ => panic!("Unknown instruction: {}", instruction),
        }
    }

    fn create_instruction(&self, inst: &str, args: &[usize]) -> Instruction {
        match inst {
            "LOAD" => Instruction::Load(args[0]),
            "SWAP" => Instruction::Swap(args[0], args[1]),
            "XOR" => Instruction::XOR(args[0], args[1]),
            "INC" => Instruction::Inc(args[0]),

            _ => panic!("Unknown instruction: {}", inst),
        }
    }
}

fn normalize_duration(duration: Duration) -> Duration {
    Duration::from_secs(duration.as_secs())
}
