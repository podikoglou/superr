use superr_vm::{instruction::Program, vm::VM};

use crate::optimizer::Optimizer;

#[derive(Default)]
pub struct Superoptimizer {}

impl Optimizer for Superoptimizer {
    fn optimize(&self, input: &Program) -> Program {
        let vm = VM::default();

        vec![]
    }
}

mod test {
    use superr_vm::{instruction::Instruction, vm::VM};

    use crate::optimizer::Optimizer;

    use super::Superoptimizer;

    #[test]
    fn optimize_3() {
        let original = vec![
            Instruction::Load(3),
            Instruction::Swap(0x00, 0x01),
            Instruction::Load(3),
            Instruction::Swap(0x00, 0x02),
            Instruction::Load(3),
            Instruction::Swap(0x00, 0x03),
            Instruction::Load(3),
        ];

        let mut vm_original = VM::default();
        vm_original.execute_program(original.clone());

        // optimize
        let optimizer = Superoptimizer::default();
        let optimized = optimizer.optimize(&original);

        let mut vm_optimized = VM::default();
        vm_optimized.execute_program(optimized);

        assert_eq!(vm_original.state, vm_optimized.state);
    }
}
