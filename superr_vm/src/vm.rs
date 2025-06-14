use crate::{instruction::Instruction, program::Program};

pub const MEM_SIZE: usize = 12;

pub type MemValue = u8;
pub type State = [MemValue; MEM_SIZE];

#[derive(Debug, Default)]
pub struct VM {
    pub state: State,
    pub pc: usize,
    pub program: Program,
}

impl VM {
    pub fn reset(&mut self) {
        self.state = [0; MEM_SIZE];
        self.pc = 0;
        self.program = Program::default();
    }

    #[inline(always)]
    pub fn execute_program(&mut self, program: Program) {
        self.program = program;

        while self.pc < self.program.instructions.len() {
            match self.program.instructions[self.pc] {
                Instruction::Load(val) => {
                    self.state[0] = val;

                    self.pc += 1;
                }

                Instruction::Swap(a, b) => {
                    self.state.swap(a, b);

                    self.pc += 1;
                }

                Instruction::XOR(a, b) => {
                    self.state[a] ^= self.state[b];

                    self.pc += 1;
                }

                Instruction::Inc(addr) => {
                    self.state[addr] = self.state[addr].wrapping_add(1);

                    self.pc += 1;
                }

                Instruction::Decr(addr) => {
                    self.state[addr] = self.state[addr].wrapping_sub(1);

                    self.pc += 1;
                }

                Instruction::Add(a, b) => {
                    self.state[a] = self.state[a].wrapping_add(self.state[b]);

                    self.pc += 1;
                }

                Instruction::Sub(a, b) => {
                    self.state[a] = self.state[a].wrapping_sub(self.state[b]);

                    self.pc += 1;
                }

                Instruction::Put(addr) => {
                    // TODO: custom writer which may or may not be stdout, so we can handle
                    // optimization without having to constantly print out
                    println!("{}", self.state[addr]);

                    self.pc += 1;
                }

                Instruction::Jmp(ins) => self.pc = ins,
            }
        }

        self.program = Program::default();
        self.pc = 0;
    }

    #[inline(always)]
    pub fn compute_state(program: &Program) -> State {
        let mut vm = VM::default();

        vm.execute_program(program.clone());

        return vm.state;
    }
}
