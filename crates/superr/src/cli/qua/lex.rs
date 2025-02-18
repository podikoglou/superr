use anyhow::Context;
use clap::ArgMatches;
use clap_stdin::FileOrStdin;
use qua_lexer::lexer::lex;

pub fn execute(matches: &ArgMatches) -> anyhow::Result<()> {
    let input = matches
        .get_one::<FileOrStdin>("input")
        .context("couldn't get input")?
        .clone();

    let contents = input.contents().context("couldn't read input")?;

    dbg!(lex(&contents));

    Ok(())
}
