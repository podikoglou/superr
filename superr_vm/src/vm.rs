use crate::{instruction::Instruction, program::Program};

pub const MEM_SIZE: usize = 6;

#[derive(Debug, Default)]
pub struct VM {
    pub state: [usize; MEM_SIZE],
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
        // should we increase the program counter here or in execute_progam?
        self.pc += 1;

        let instruction = instruction.clone();

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
