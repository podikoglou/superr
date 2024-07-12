pub mod gen;
pub mod run;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// An experimental VM and superoptimizer.
pub struct SuperrArgs {
    #[argh(subcommand)]
    pub nested: Subcommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Subcommands {
    Run(RunSubcommand),
    Gen(GenSubcommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Runs a superr program. The program is read from stdin.
#[argh(subcommand, name = "run")]
pub struct RunSubcommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// Randomly generates a superr program.
#[argh(subcommand, name = "gen")]
pub struct GenSubcommand {}
