


use rand::{thread_rng, Rng};
use superr_vm::{instruction::Instruction, program::Program, vm::VM};

use crate::optimizer::Optimizer;

pub struct Superoptimizer {
    pub max_instructions: usize,
}

impl Superoptimizer {
    pub fn new(max_instructions: usize) -> Self {
        Self { max_instructions }
    }
}

impl Optimizer for Superoptimizer {
    fn optimize(&self, input: Program) -> Program {
        let mut vm = VM::default();

        vm.execute_program(input);
        let target_state = vm.state.clone();

        vm.reset();

        loop {
            let mut instructions = vec![];

            for _ in 0..8 {
                let instruction_index = thread_rng().gen_range(0..=3);
                let arg01: usize = thread_rng().gen_range(0..4);
                let arg02: usize = thread_rng().gen_range(0..4);

                match instruction_index {
                    0 => instructions.push(Instruction::Load(arg01 as u32)),
                    1 => instructions.push(Instruction::Swap(arg01, arg02)),
                    2 => instructions.push(Instruction::XOR(arg01, arg02)),
                    3 => instructions.push(Instruction::Inc(arg01)),

                    _ => {}
                }
            }

            let program = Program { instructions };

            vm.execute_program(program.clone());

            if vm.state == target_state {
                return program;
            }

            vm.reset();
        }
    }
}

mod test {
    use superr_vm::{instruction::Instruction, program::Program, vm::VM};

    use crate::optimizer::Optimizer;

    use super::Superoptimizer;

    #[test]
    fn optimize_3() {
        let original = Program {
            instructions: vec![
                Instruction::Load(3),
                Instruction::Swap(0x00, 0x01),
                Instruction::Load(3),
                Instruction::Swap(0x00, 0x02),
                Instruction::Load(3),
                Instruction::Swap(0x00, 0x03),
                Instruction::Load(3),
            ],
        };

        let mut vm_original = VM::default();
        vm_original.execute_program(original.clone());

        // optimize
        let optimizer = Superoptimizer::new(4);
        let optimized = optimizer.optimize(original);

        let mut vm_optimized = VM::default();
        vm_optimized.execute_program(optimized);

        assert_eq!(vm_original.state, vm_optimized.state);
    }
}
