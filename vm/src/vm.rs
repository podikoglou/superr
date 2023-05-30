use std::usize;

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
    pub fn run_program(&mut self, program: Program) {
        for instruction in program {
            match instruction {
                Instruction::Load(val) => self.load(val),
                Instruction::Swap(a, b) => self.swap(a, b),
                Instruction::XOR(a, b) => self.xor(a, b),
                Instruction::Inc(addr) => self.incr(addr),
            }
        }
    }

    pub fn load(&mut self, val: i32) {
        self.state[0] = val;
    }

    pub fn swap(&mut self, a: MemoryAddress, b: MemoryAddress) {
        self.state.swap(a, b);
    }

    pub fn xor(&mut self, a: MemoryAddress, b: MemoryAddress) {
        self.state[a] = self.state[a] ^ self.state[b];
    }

    pub fn incr(&mut self, addr: MemoryAddress) {
        self.state[addr] += 1;
    }
}
