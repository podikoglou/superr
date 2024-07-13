use superr_vm::program::Program;

pub trait Optimizer {
    /// Takes a program (which is given when the Optimizer is created) and returns a program
    /// which is equivalent, but optimized optimally, according to the implementation.
    fn optimize(&mut self) -> Program;
}
