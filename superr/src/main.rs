use cli::SuperrArgs;

pub mod cli;

fn main() {
    let args: SuperrArgs = argh::from_env();

    match args.nested {
        cli::Subcommands::Run(subcommand) => cli::run::execute(subcommand),
        cli::Subcommands::Gen(subcommand) => cli::gen::execute(subcommand),
        cli::Subcommands::Optimize(subcommand) => cli::optimize::execute(subcommand),
        cli::Subcommands::Bench(subcommand) => cli::bench::execute(subcommand),
    }
}
