use superr_vm::program::Program;

pub trait Optimizer {
    /// Takes a program and returns a program which is equivalent, but optimized optimally,
    /// according to the implementation.
    fn optimize(&self, input: &Program) -> Program;
}
