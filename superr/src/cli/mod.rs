pub mod bench;
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
    Bench(BenchSubcommand),
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
    pub max_num: u8,

    /// optimizer to use (options: random)
    #[argh(option)]
    pub optimizer: OptimizerType,
}

#[derive(Debug, PartialEq)]
pub enum OptimizerType {
    RandomSearch,
    Exhaustive,
    Diffing,
}

impl FromArgValue for OptimizerType {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "random" | "random-search" | "random_search" => Ok(Self::RandomSearch),
            "exhaustive" => Ok(Self::Exhaustive),
            "diffing" => Ok(Self::Diffing),
            _ => Err("invalid optimizer".to_string()),
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Benchmarks the Superr VM by indefinitely running random instructions.
#[argh(subcommand, name = "bench")]
pub struct BenchSubcommand {
    /// amount of instructions to generate at once
    #[argh(option, default = "500")]
    pub buffer: usize,
}
