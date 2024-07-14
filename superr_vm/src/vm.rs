use crate::{instruction::Instruction, program::Program};

pub const MEM_SIZE: usize = 6;

pub type State = [usize; MEM_SIZE];

#[derive(Debug, Default)]
pub struct VM {
    pub state: State,
    pub pc: usize,
}

impl VM {
    pub fn reset(&mut self) {
        self.state = [0; MEM_SIZE];
        self.pc = 0;
    }

    pub fn execute_program(&mut self, program: &Program) {
        for instruction in &program.instructions {
            self.execute(&instruction);
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        self.pc += 1;

        match *instruction {
            Instruction::Load(val) => {
                self.state[0] = val;
            }

            Instruction::Swap(a, b) => {
                self.state.swap(a, b);
            }

            Instruction::XOR(a, b) => {
                self.state[a] = self.state[a] ^ self.state[b];
            }

            Instruction::Inc(addr) => {
                self.state[addr] += 1;
            }
        }
    }

    pub fn compute_state(program: &Program) -> State {
        let mut vm = VM::default();

        vm.execute_program(program);

        return vm.state;
    }
}
