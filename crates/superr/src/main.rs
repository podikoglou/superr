pub mod cli;

use clap::{arg, command, value_parser, ArgAction};
use clap_stdin::FileOrStdin;

const INSTRUCTIONS: [&str; 8] = [
    "load", "swap", "xor", "inc", "decr", "add", "sub", "put", /* "jump" */
];

const OPTIMIZERS: [&str; 3] = ["random", "exhaustive", "diffing"];

fn main() -> anyhow::Result<()> {
    let program_generation_args = vec![
        arg!(--"min-ins" <val> "Minimum amount of instructions to generate")
            .default_value("0")
            .action(ArgAction::Set)
            .value_parser(value_parser!(usize)),
        arg!(--"max-ins" <val> "Maximum amount of instructions to generate")
            .action(ArgAction::Set)
            .value_parser(value_parser!(usize))
            .required(true),
        arg!(--"min-imm" <val> "Minimum value an intermediate value can take")
            .default_value("1")
            .action(ArgAction::Set)
            .value_parser(value_parser!(u8)),
        arg!(--"max-imm" <val> "Maximum value an intermediate value can take")
            .default_value("255")
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
                        .default_value("-")
                        .value_parser(value_parser!(FileOrStdin<String>)),
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
                .arg(
                    arg!([input] "Superr program to run")
                        .default_value("-")
                        .value_parser(value_parser!(FileOrStdin<String>)),
                )
                .arg(
                    arg!(--optimizer <optimizer> "Optimizer to use")
                        .action(ArgAction::Set)
                        .value_parser(clap::builder::PossibleValuesParser::new(OPTIMIZERS.clone()))
                        .required(true),
                )
                .args(&program_generation_args),
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
        .subcommand(
            command!("qua")
                .about("Qua-related commands")
                .arg_required_else_help(true)
                .subcommand_required(true)
                .subcommand(
                    command!("lex").about("Lexically analyzes a .qua file").arg(
                        arg!([input] "Qua program to analyze")
                            .default_value("-")
                            .value_parser(value_parser!(FileOrStdin<String>)),
                    ),
                )
                .subcommand(
                    command!("ast")
                        .about("Displays the AST of a .qua file")
                        .arg(
                            arg!([input] "Qua program to parse")
                                .default_value("-")
                                .value_parser(value_parser!(FileOrStdin<String>)),
                        ),
                )
                .subcommand(
                    command!("pprint").about("Pretty prints a .qua file").arg(
                        arg!([input] "Qua program to pretty print")
                            .default_value("-")
                            .value_parser(value_parser!(FileOrStdin<String>)),
                    ),
                )
                .subcommand(
                    command!("cc").about("Compiles a .qua file").arg(
                        arg!([input] "Qua program to compile")
                            .default_value("-")
                            .value_parser(value_parser!(FileOrStdin<String>)),
                    ),
                ),
        )
        .subcommand(command!("inspect").about("Launches interactive GUI for Superr"))
        .get_matches();

    match matches.subcommand() {
        Some(("run", matches)) => cli::run::execute(matches),
        Some(("gen", matches)) => cli::gen::execute(matches),
        Some(("optimize", matches)) => cli::optimize::execute(matches),
        Some(("bench", matches)) => cli::bench::execute(matches),
        Some(("inspect", matches)) => cli::inspect::execute(matches),

        Some(("qua", matches)) => match matches.subcommand() {
            Some(("lex", matches)) => cli::qua::lex::execute(matches),
            Some(("ast", matches)) => cli::qua::ast::execute(matches),
            Some(("pprint", matches)) => cli::qua::pprint::execute(matches),
            Some(("cc", matches)) => cli::qua::cc::execute(matches),

            _ => unreachable!("this won't happen"),
        },

        _ => unreachable!("this won't happen"),
    }
}
