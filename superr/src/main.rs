use superr_optimizers::{optimizer::Optimizer, superoptimizer::Superoptimizer};
use superr_vm::{program::Program, vm::VM};

fn main() {
    let original: Program = vec![
        vec![3, 0],
        vec![3, 0],
        vec![3, 0],
        vec![1, 0, 1],
        vec![1, 2, 3],
        vec![3, 0],
        vec![1, 0, 2],
        vec![1, 1, 3],
        vec![2, 0, 1],
        vec![2, 2, 3],
    ]
    .into();

    // run original program
    let mut vm1 = VM::default();
    vm1.execute_program(original.clone());

    dbg!(vm1.state);
    dbg!(original.clone());

    // optimize program
    let optimizer = Superoptimizer::new(4); // the optimal program contains 4 instructions
    let optimized = optimizer.optimize(original);

    // run optimized program
    let mut vm2 = VM::default();
    vm2.execute_program(optimized.clone());

    dbg!(vm2.state);
    dbg!(optimized);
}
