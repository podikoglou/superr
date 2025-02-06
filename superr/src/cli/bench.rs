use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use indicatif::{ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};
use rayon::ThreadPoolBuilder;
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{self, MemValue, VM},
};

use super::BenchSubcommand;

pub fn execute(args: BenchSubcommand) {
    let thread_pool = ThreadPoolBuilder::new().build().unwrap();

    let counter = Arc::new(AtomicU64::default());
    let should_stop = Arc::new(AtomicBool::default());

    // set ctrl c handler
    let should_stop_2 = should_stop.clone();
    ctrlc::set_handler(move || should_stop_2.store(true, Ordering::Relaxed)).unwrap();

    // create progress thread
    let counter_2 = counter.clone();
    let should_stop_3 = should_stop.clone();

    thread_pool.spawn(move || {
        progress_loop(counter_2, should_stop_3);
    });

    // create benchmark thread
    let should_stop_4 = should_stop.clone();

    thread_pool.scope(|_| {
        bench_loop(args.buffer, counter, should_stop_4);
    });
}

pub fn generate_instruction(max_num: MemValue) -> Instruction {
    let instruction = fastrand::usize(0..=6);

    match instruction {
        0 => {
            let val = fastrand::u8(0..max_num);

            Instruction::Load(val)
        }

        1 | 2 | 5 | 6 => {
            let addr1 = fastrand::usize(0..vm::MEM_SIZE);
            let addr2 = fastrand::usize(0..vm::MEM_SIZE);

            match instruction {
                1 => Instruction::Swap(addr1, addr2),
                2 => Instruction::XOR(addr1, addr2),
                5 => Instruction::Add(addr1, addr2),
                6 => Instruction::Sub(addr1, addr2),

                _ => panic!("SUPER unexpected error occurred"),
            }
        }

        3 | 4 => {
            let addr = fastrand::usize(0..vm::MEM_SIZE);

            match instruction {
                3 => Instruction::Inc(addr),
                4 => Instruction::Decr(addr),

                _ => panic!("SUPER unexpected error occurred"),
            }
        }

        _ => panic!("SUPER unexpected error occurred"),
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
            "{} Instructions Executed | {}/s",
            current.to_formatted_string(&Locale::en),
            programs_per_second.to_formatted_string(&Locale::en),
        );

        bar.set_message(message);
        bar.tick();

        last_count = current;
    }
}

fn bench_loop(buffer: usize, counter: Arc<AtomicU64>, should_stop: Arc<AtomicBool>) {
    let mut vm = VM::default();

    while !should_stop.load(Ordering::Relaxed) {
        let instructions = (0..buffer)
            .map(|_| generate_instruction(8))
            .collect::<Vec<Instruction>>();

        let mut program = Program::new();

        program.instructions = instructions;

        vm.execute_program(program);

        counter.fetch_add(buffer as u64, Ordering::Relaxed);
    }
}
