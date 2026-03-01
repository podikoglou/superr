use anyhow::Context;
use clap::ArgMatches;
use clap_stdin::FileOrStdin;
use superr_assembler::assembler::read_program;
use superr_vm::vm::VM;

pub fn execute(matches: &ArgMatches) -> anyhow::Result<()> {
    let input = matches
        .get_one::<FileOrStdin>("input")
        .context("couldn't get input")?
        .clone();

    // read program
    let reader = input.into_reader()?;
    let program = read_program(reader)?;

    // crate vm and run program
    let mut vm = VM::default();

    vm.execute_program(program);

    dbg!(vm);

    Ok(())
}
