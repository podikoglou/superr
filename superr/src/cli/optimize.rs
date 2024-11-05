use std::io::{self, BufRead};

use superr_optimizers::{
    optimizers::{
        exhaustive::{ExhaustiveOptimizer, ExhaustiveOptimizerOptions},
        random_search::{RandomSearchOptimizer, RandomSearchOptimizerOptions},
    },
    Optimizer,
};
use superr_vm::{instruction::Instruction, program::Program, vm::VM};

use crate::cli::OptimizerType;

use super::OptimizeSubcommand;

pub fn execute(args: OptimizeSubcommand) {
    // properly read program from stdin
    let mut input = Program::new();
    let lines = io::stdin().lock().lines();

    for line in lines {
        match line {
            Ok(v) => input.instructions.push(Instruction::from(v)),
            Err(_) => break,
        }
    }

    println!("*** Input Program ***");
    print_program(&input);
    println!();

    // run the program just to find out what the target state is. we don't need this
    // immediately, we only really use it for the output at the end. in fact,
    // this is computed twice, once hre and once inside the optimize method.
    let target_state = VM::compute_state(&input);

    dbg!(target_state);
    println!();

    let output: Program;

    match args.optimizer {
        // random search
        OptimizerType::RandomSearch => {
            let mut optimizer = RandomSearchOptimizer::new(
                RandomSearchOptimizerOptions {
                    max_instructions: args.max_instructions,
                    max_num: args.max_num,
                    // progress_frequency: args.progress_frequency,
                },
                input.clone(),
            );

            output = optimizer.start_optimization();
        }

        // exhaustive
        OptimizerType::Exhaustive => {
            let mut optimizer = ExhaustiveOptimizer::new(
                ExhaustiveOptimizerOptions {
                    max_instructions: args.max_instructions,
                    max_num: args.max_num,
                },
                input.clone(),
            );

            output = optimizer.start_optimization();
        }
    };

    println!("*** Output Program ***");
    print_program(&output);
    println!();

    println!("Input Program: {} Instructions", input.instructions.len());
    println!("Output Program: {} Instructions", output.instructions.len())
}

fn print_program(program: &Program) {
    if program.instructions.len() > 20 {
        println!("[Program too long to display]");
        return;
    }

    for instruction in &program.instructions {
        println!("{}", instruction.to_string());
    }
}
