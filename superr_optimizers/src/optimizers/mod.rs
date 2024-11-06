use std::sync::{
    atomic::{AtomicBool, AtomicU64},
    Arc, RwLock,
};

use rayon::Scope;
use superr_vm::{program::Program, vm::State};

pub mod exhaustive;
pub mod random_search;

pub struct OptimizerArgs {
    /// Target state which we want our program to have.
    pub target: State,

    /// Length of the program we're trying to optimize.
    pub length: usize,

    /// Largest possible number that can appear in instructions such as LOAD.
    pub max_num: usize,

    /// Max amount of instructions a program should have. Typically our
    /// original program's length minus 1.
    pub max_instructions: usize,

    /// Container for our most optimal program.
    ///
    /// NOTE: We could have a history rather than storing a single
    /// optimal program and discarding the others.
    pub optimal: Arc<RwLock<Program>>,

    /// Counter for the amount of programs checked.
    ///
    /// This is used for the progress bar and other statistics.
    pub counter: Arc<AtomicU64>,

    /// Switch for stopping the optimization process.
    ///
    /// This is used for the interface.
    pub should_stop: Arc<AtomicBool>,
}

pub trait Optimizer {
    /// Creates a new instance of the Optimizer.
    fn new(args: OptimizerArgs) -> Self;

    /// Starts the optimization process.
    ///
    /// It starts multiple threads using rayon:
    ///   - one for reporting the progress
    ///   - and the rest of the available threads, for computing the optimal program.
    ///
    /// It also joins the threads, meaning that this function is blocking, until
    /// the threads are stopped.
    ///
    /// Returns the program using [`Optimizer::optimal`] when finished.
    fn start_optimization<'a>(&'a mut self, scope: &Scope<'a>);

    /// Returns the length of the optimal program. 'current' refers to the fact that
    /// we're not necessarily returning the optimal length of the program, but the
    /// length of what we know to be the optimal program at this point.
    fn current_optimal_length(&self) -> usize;

    /// This function is used within the threads of the optimizer, and checks
    /// whether to stop based on the state of the program.
    ///
    /// Implementations should use this method function, rather than using the
    /// state's should_stop variable directly, as in some implementations there
    /// may be other variables involved with whether the program should stop.
    fn should_stop(&self) -> bool;

    /// Runs the worker loop, constantly generating and checking
    /// programs until it finds an optimal program.
    fn worker_loop(&self);
}
