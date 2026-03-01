use anyhow::Context;
use clap::ArgMatches;
use clap_stdin::FileOrStdin;
use superr_assembler::parser;
use superr_vm::vm::VM;

use crate::reporting::parse_failure;

pub fn execute(matches: &ArgMatches) -> anyhow::Result<()> {
    let input = matches
        .get_one::<FileOrStdin>("input")
        .context("couldn't get input")?
        .clone();

    let file_name = input.filename().to_string();
    let contents = input.contents().context("couldn't read input")?;

    let program = parser::parse_program(&contents)
        .unwrap_or_else(|errs| parse_failure(&errs[0], &contents, file_name));

    // create vm
    let mut vm = VM::default();

    vm.execute_program(program);

    dbg!(vm);

    Ok(())
}
