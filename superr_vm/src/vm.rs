use crate::{instruction::Instruction, program::Program};

pub const MEM_SIZE: usize = 6;

#[derive(Debug, Default)]
pub struct VM {
    pub state: [u32; MEM_SIZE],
}

impl VM {
    pub fn reset(&mut self) {
        self.state = [0; MEM_SIZE];
    }

    pub fn execute_program(&mut self, program: Program) {
        for instruction in program.instructions {
            self.execute(instruction);
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
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
}
