use anyhow::Context;
use clap::ArgMatches;
use clap_stdin::FileOrStdin;
use superr_vm::{instruction::Instruction, program::Program, vm::VM};

pub fn execute(matches: &ArgMatches) -> anyhow::Result<()> {
    let input = matches
        .get_one::<FileOrStdin>("input")
        .context("couldn't get input")?
        .clone();

    let contents = input.contents().context("couldn't read input")?;

    let mut program = Program::new();

    for line in contents.lines() {
        if !line.is_empty() {
            program
                .instructions
                .push(Instruction::from(line.to_string()))
        }
    }

    // create vm
    let mut vm = VM::default();

    vm.execute_program(program);

    dbg!(vm);

    Ok(())
}
