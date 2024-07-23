use superr_vm::program::Program;

pub trait Optimizer {
    /// The options of the program, such as the biggest value an instruction
    /// operand can be, max instructions a program can have, etc.
    type Options;

    /// The state of the program, holding things such as the optimal program, whether
    /// to stop, programs checked, etc.
    type State;

    /// Creates a new instance of the Optimizer.
    fn new(options: Self::Options, program: Program) -> Self;

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
    fn start_optimization(&mut self) -> Program;

    /// Gets the optimal version of the program.
    ///
    /// When the superoptimizer is created, the variable behind this is initialized
    /// with the initially given program, so if no optimal program was found,
    /// the given program is returned, thus not needing to return an [Option]
    fn optimal(&mut self) -> Program;

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
    fn work(&self);

    /// Runs the progress loop, constantly updating the progress bar.
    ///
    /// TODO: Figure out a universal way to have a progress bar
    /// without needing to write a new one for each optimizer?
    fn progress_loop(&self);
}

pub mod optimizers;
