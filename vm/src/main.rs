use vm::Instruction;

pub mod vm;

fn main() {
    let mut vm = vm::VM::default();

    let program = vec![
        Instruction::Load(3),
        Instruction::Swap(0, 1),
        Instruction::Load(3),
        Instruction::Swap(0, 2),
        Instruction::Load(3),
        Instruction::Swap(0, 3),
        Instruction::Load(3),
    ];

    vm.execute_program(program);

    dbg!(vm.state);
}
