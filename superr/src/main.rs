pub mod cli;

use clap::{arg, command, value_parser, ArgAction};
use clap_stdin::FileOrStdin;

const INSTRUCTIONS: [&str; 8] = [
    "load", "swap", "xor", "inc", "decr", "add", "sub", "put", /* "jump" */
];

const OPTIMIZERS: [&str; 3] = ["random", "exhaustive", "diffing"];

fn main() -> anyhow::Result<()> {
    let program_generation_args = vec![
        arg!(--"min-i" --"min-instructions" <val> "Minimum amount of instructions to generate")
            .default_value("0")
            .required(true)
            .action(ArgAction::Set)
            .value_parser(value_parser!(usize)),
        arg!(--"max-i" --"max-instructions" <val> "Maximum amount of instructions to generate")
            .required(true)
            .action(ArgAction::Set)
            .value_parser(value_parser!(usize)),
        arg!(--"min-imm" <val> "Minimum value an intermediate value can take")
            .default_value("1")
            .required(true)
            .action(ArgAction::Set)
            .value_parser(value_parser!(u8)),
        arg!(--"max-imm" <val> "Maximum value an intermediate value can take")
            .required(true)
            .action(ArgAction::Set)
            .value_parser(value_parser!(u8)),
        arg!(--exclude <instructions> "Instruction to exclude (can be used multiple times)")
            .action(ArgAction::Append)
            .value_parser(clap::builder::PossibleValuesParser::new(
                INSTRUCTIONS.clone(),
            )),
    ];
    let matches = command!()
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            command!("run")
                .aliases(["r", "execute"])
                .about("Runs a Superr program")
                .arg(
                    arg!([input] "Superr program to run")
                        .value_parser(value_parser!(FileOrStdin<String>))
                        .required(true),
                ),
        )
        .subcommand(
            command!("gen")
                .aliases(["g", "generate", "rand", "random"])
                .about("Generates a random program")
                .args(&program_generation_args),
        )
        .subcommand(
            command!("optimize")
                .aliases(["o", "optimise"])
                .about("Optimizes a program")
                .args(&program_generation_args)
                .arg(
                    arg!(--optimizer <optimizer> "Optimizer to use")
                        .action(ArgAction::Set)
                        .value_parser(clap::builder::PossibleValuesParser::new(OPTIMIZERS.clone())),
                ),
        )
        .subcommand(
            command!("bench")
                .aliases(["benchmark"])
                .about("Benchmarks the Superr VM")
                .arg(
                    arg!(--buffer "Amount of instructions to generate at a time")
                        .value_parser(value_parser!(usize))
                        .default_value("512"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", matches)) => cli::run::execute(matches),
        Some(("gen", matches)) => cli::gen::execute(matches),
        Some(("optimize", matches)) => cli::optimize::execute(matches),
        Some(("bench", matches)) => cli::bench::execute(matches),

        _ => unreachable!("this won't happen"),
    }
}
