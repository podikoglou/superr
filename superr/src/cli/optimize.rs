use std::{
    io::{self, BufRead},
    str::FromStr,
    time::Duration,
};

use superr_optimizers::{optimizer::Optimizer, superoptimizer::Superoptimizer};
use superr_vm::{
    instruction::Instruction,
    program::Program,
    vm::{MEM_SIZE, VM},
};

use super::OptimizeSubcommand;

pub fn execute(args: OptimizeSubcommand) {
    // properly read program from stdin
    let mut input = Program::new();
    let lines = io::stdin().lock().lines();

    for line in lines {
        match line {
            Ok(v) => input.instructions.push(Instruction::from_str(&v).unwrap()),
            Err(_) => break,
        }
    }

    // run the program just to find out what the target state is. we don't need this
    // immediately, we only really use it for the output at the end. in fact,
    // this is computed twice, once hre and once inside the optimize method.
    let target_state = VM::compute_state(&input);

    dbg!(target_state);

    // create and run superoptimizer
    let superoptimizer = Superoptimizer::new(
        args.max_instructions,
        args.max_num,
        Duration::from_secs(args.timeout),
    );

    let output = superoptimizer.optimize(input.clone()); // TODO: ideally don't clone

    // print out instructions for program
    // TODO: ideally don't clone
    for instruction in output.clone().instructions {
        println!("{}", instruction.to_string());
    }

    // these are printed to stderr so it doesn't get in the way of the user if
    //  they're piping the output of this program
    eprintln!("Input Program: {} Instructions", input.instructions.len());
    eprintln!("Output Program: {} Instructions", output.instructions.len());
}
