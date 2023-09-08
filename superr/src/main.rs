use superr_vm::{instruction::Instruction, vm::VM};

fn main() {
    let mut vm = VM::default();

    let program = vec![
        Instruction::Load(3),
        Instruction::Swap(0x00, 0x01),
        Instruction::Load(3),
        Instruction::Swap(0x00, 0x02),
        Instruction::Load(3),
        Instruction::Swap(0x00, 0x03),
        Instruction::Load(3),
    ];

    vm.execute_program(program);

    dbg!(vm.state);
}
