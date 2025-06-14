use anyhow::Context;
use clap::ArgMatches;
use clap_stdin::FileOrStdin;
use qua_compiler::compiler::Compiler;
use qua_lexer::lexer::Lexer;
use qua_parser::Parser;

pub fn execute(matches: &ArgMatches) -> anyhow::Result<()> {
    let input = matches
        .get_one::<FileOrStdin>("input")
        .context("couldn't get input")?
        .clone();

    let contents = input.contents().context("couldn't read input")?;

    let mut parser = Parser::new(Lexer::new(contents.as_str()));
    let program = parser.parse_program()?;

    let mut compiler = Compiler::new(program);

    compiler.compile();

    for instruction in compiler.assembly {
        println!("{}", instruction.to_string());
    }

    Ok(())
}
