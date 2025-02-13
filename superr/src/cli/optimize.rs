use std::{
    mem,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, RwLock,
    },
    thread,
    time::Duration,
};

use anyhow::Context;
use clap::ArgMatches;
use clap_stdin::FileOrStdin;
use indicatif::{ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};
use rayon::ThreadPoolBuilder;
use superr_optimizers::optimizers::{
    diffing::DiffingOptimizer, exhaustive::ExhaustiveOptimizer,
    random_search::RandomSearchOptimizer, Optimizer, OptimizerArgs,
};
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{State, VM},
};

pub fn execute(matches: &ArgMatches) -> anyhow::Result<()> {
    let input = matches
        .get_one::<FileOrStdin>("input")
        .context("couldn't get input")?;

    let contents = input.clone().contents().context("couldn't read input")?;

    let mut program_in = Program::new();

    for line in contents.lines() {
        if !line.is_empty() {
            program_in
                .instructions
                .push(Instruction::from(line.to_string()))
        }
    }

    let length_in = program_in.instructions.len();
    let target = VM::compute_state(&program_in);

    eprintln!("*** Input Program ***");
    print_program(&program_in);
    eprintln!();

    eprintln!("*** Target ***");
    print_state(&target);
    eprintln!();

    // run optimizer
    let program_out = optimize(program_in, matches);
    let length_out = program_out.instructions.len();

    // print results
    eprintln!();
    eprintln!();
    eprintln!("*** Output Program ***");
    print_program_stdout(&program_out);
    eprintln!();

    eprintln!("Input Program: {} Instructions", length_in);
    eprintln!("Output Program: {} Instructions", length_out);

    Ok(())
}

fn optimize(program: Program, matches: &ArgMatches) -> Program {
    // TODO: use min_instructions and min_imm

    // get arguments
    let min_instructions = matches.get_one::<usize>("min-ins").unwrap();
    let max_instructions = matches.get_one::<usize>("max-ins").unwrap();

    let min_imm = matches.get_one::<u8>("min-imm").unwrap();
    let max_imm = matches.get_one::<u8>("max-imm").unwrap();

    let optimizer = matches.get_one::<&str>("optimizer").unwrap();

    // run program to get the target memory & get amount of instructinos,
    // we pass these two to the optimizer.
    let target = VM::compute_state(&program);
    let length = program.instructions.len();

    // create thread pool
    let thread_pool = ThreadPoolBuilder::new().build().unwrap();

    // state sort of
    let optimal = Arc::new(RwLock::new(program));
    let counter = Arc::new(AtomicU64::default());
    let should_stop = Arc::new(AtomicBool::default());

    // create clones of our state which we'll use in the interface
    let mut optimal_2 = optimal.clone();
    let counter_2 = counter.clone();
    let should_stop_2 = should_stop.clone();
    let should_stop_3 = should_stop.clone();

    // ctrl c handler
    ctrlc::set_handler(move || should_stop_2.store(true, Ordering::Relaxed)).unwrap();

    let optimizer_args = OptimizerArgs {
        max_instructions: *max_instructions,
        max_num: *max_imm,

        target,
        length,

        optimal,
        counter,
        should_stop,
    };

    thread_pool.spawn(move || {
        progress_loop(counter_2, should_stop_3);
    });

    match optimizer {
        &"random" => {
            let mut optimizer = RandomSearchOptimizer::new(optimizer_args);

            // start threads
            thread_pool.scope(|scope| {
                optimizer.start_optimization(&scope);
            });
        }
        &"exhaustive" => {
            // initialize optimizer
            let mut optimizer = ExhaustiveOptimizer::new(optimizer_args);

            // start threads
            thread_pool.scope(|scope| {
                optimizer.start_optimization(&scope);
            });
        }
        &"diffing" => {
            // initialize optimizer
            let mut optimizer = DiffingOptimizer::new(optimizer_args);

            // start threads
            thread_pool.scope(|scope| {
                optimizer.start_optimization(&scope);
            });
        }

        _ => unreachable!(),
    }

    // return result
    match Arc::try_unwrap(mem::take(&mut optimal_2)) {
        Ok(optimal) => optimal.into_inner().unwrap(),
        Err(arc) => {
            // this shouldn't happen, but if it does, we can still
            // read the value, just by cloning
            arc.read().unwrap().clone()
        }
    }
}

fn progress_loop(counter: Arc<AtomicU64>, should_stop: Arc<AtomicBool>) {
    let mut last_count = counter.load(Ordering::Relaxed);

    // create progress bar
    let bar = ProgressBar::new_spinner();

    bar.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );

    while !should_stop.load(Ordering::Relaxed) {
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

fn print_program(program: &Program) {
    if program.instructions.len() > 20 {
        eprintln!("[Program too long to display]");
        return;
    }

    for instruction in &program.instructions {
        eprintln!("{}", instruction.to_string());
    }
}

fn print_program_stdout(program: &Program) {
    for instruction in &program.instructions {
        println!("{}", instruction.to_string());
    }
}

fn print_state(state: &State) {
    eprintln!(
        "[{}]",
        state
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
}
