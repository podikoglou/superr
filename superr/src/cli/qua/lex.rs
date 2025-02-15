use anyhow::Context;
use clap::ArgMatches;
use clap_stdin::FileOrStdin;

pub fn execute(matches: &ArgMatches) -> anyhow::Result<()> {
    let input = matches
        .get_one::<FileOrStdin>("input")
        .context("couldn't get input")?
        .clone();

    let contents = input.contents().context("couldn't read input")?;

    dbg!(qua_lexer::lex(&contents));

    Ok(())
}
