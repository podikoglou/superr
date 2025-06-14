use anyhow::Context;
use clap::ArgMatches;
use clap_stdin::FileOrStdin;
use qua_lexer::lexer::Lexer;
use qua_parser::Parser;

pub fn execute(matches: &ArgMatches) -> anyhow::Result<()> {
    let input = matches
        .get_one::<FileOrStdin>("input")
        .context("couldn't get input")?
        .clone();

    let contents = input.contents().context("couldn't read input")?;

    let mut parser = Parser::new(Lexer::new(contents.as_str()));

    println!("{}", parser.parse_program()?);

    Ok(())
}
