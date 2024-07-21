pub mod gen;
pub mod optimize;
pub mod run;

use argh::{FromArgValue, FromArgs};

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
    Optimize(OptimizeSubcommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Runs a superr program. The program is read from stdin.
#[argh(subcommand, name = "run")]
pub struct RunSubcommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// Randomly generates a superr program.
#[argh(subcommand, name = "gen")]
pub struct GenSubcommand {
    /// amount of instructions to generate.
    #[argh(option, default = "8")]
    pub instructions: u32,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Optimizes a program.
#[argh(subcommand, name = "optimize")]
pub struct OptimizeSubcommand {
    /// max amount of instructions the output program should have
    #[argh(option)]
    pub max_instructions: usize,

    /// max number that can be loaded
    #[argh(option)]
    pub max_num: usize,

    /// frequency of progress reports (in milliseconds)
    #[argh(option, default = "250")]
    pub progress_frequency: u64,

    /// optimizer to use (options: random)
    #[argh(option)]
    pub optimizer: OptimizerType,
}

#[derive(Debug, PartialEq)]
pub enum OptimizerType {
    RandomSearch,
    Exhaustive,
}

impl FromArgValue for OptimizerType {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "random" | "random-search" | "random_search" => Ok(Self::RandomSearch),
            "exhaustive" => Ok(Self::Exhaustive),
            _ => Err("invalid optimizer".to_string()),
        }
    }
}
