use qua_ast::ast::program::Program;
use superr_vm::instruction::Instruction;

#[derive(Debug)]
pub struct Compiler {
    program: Program,
    pub assembly: Vec<Instruction>,
}

impl Compiler {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            assembly: vec![],
        }
    }

    pub fn compile(&mut self) {
        self.emit(Instruction::Inc(0));
        self.emit(Instruction::Inc(0));
        self.emit(Instruction::Inc(1));
        self.emit(Instruction::Inc(1));
    }

    pub fn emit(&mut self, instruction: Instruction) {
        self.assembly.push(instruction);
    }
}
