const MEM_SIZE: usize = 4;

pub type MemoryAddress = usize;

pub type Program = Vec<Instruction>;

#[derive(Debug)]
pub enum Instruction {
    Load(i32),
    Swap(MemoryAddress, MemoryAddress),
    XOR(MemoryAddress, MemoryAddress),
    Inc(MemoryAddress),
}

#[derive(Debug)]
pub struct VM {
    pub state: [i32; MEM_SIZE],
}

impl Default for VM {
    fn default() -> Self {
        VM {
            state: [0; MEM_SIZE],
        }
    }
}

impl VM {
    pub fn execute_program(&mut self, program: Program) {
        for instruction in program {
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
