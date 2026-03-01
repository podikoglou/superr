use std::io::{stdout, Write};

use anyhow::Context;
use clap::ArgMatches;
use clap_stdin::FileOrStdin;
use superr_assembler::{assembler::program_to_bytes, parser};

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

    let program_bytes = program_to_bytes(&program);

    stdout().write_all(&program_bytes)?;

    Ok(())
}
