use crate::{
    instruction::{decode_op1, decode_op2, decode_opcode, Instruction, INC, LOAD, SWAP, XOR},
    program::Program,
};

pub const MEM_SIZE: usize = 12;

pub type State = [u8; MEM_SIZE];

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

    #[inline(always)]
    pub fn execute_program(&mut self, program: &Program) {
        for instruction in &program.instructions {
            self.execute(*instruction);
        }
    }

    #[inline(always)]
    pub fn execute(&mut self, instruction: Instruction) {
        self.pc += 1;

        let opcode = decode_opcode(instruction);

        match opcode {
            LOAD => {
                let val = decode_op2(instruction);

                self.state[0] = val;
            }
            SWAP => {
                let a = decode_op1(instruction);
                let b = decode_op2(instruction);

                self.state.swap(a as usize, b as usize);
            }
            XOR => {
                let a = decode_op1(instruction);
                let b = decode_op2(instruction);

                self.state[a as usize] ^= self.state[b as usize];
            }
            INC => {
                let addr = decode_op2(instruction);

                self.state[addr as usize] += 1;
            }
            _ => {}
        };
    }

    #[inline(always)]
    pub fn compute_state(program: &Program) -> State {
        let mut vm = VM::default();

        vm.execute_program(program);

        return vm.state;
    }
}
