use super::RunSubcommand;
use std::io::{self, BufReader, Read};
use superr_vm::{program::Program, vm::VM};

pub fn execute(_: RunSubcommand) {
    // properly read program from stdin
    let mut program = Program::new();

    let stdin = io::stdin().lock();
    let mut reader = BufReader::new(stdin);

    let mut buf: [u8; 4] = [0; 4];

    // read length of program
    let _ = reader.read(&mut buf);
    let len = u32::from_be_bytes(buf);

    for _ in 0..len {
        match reader.read(&mut buf) {
            Ok(_) => {}
            Err(_) => break,
        };

        let instruction = u32::from_be_bytes(buf);

        program.instructions.push(instruction);
    }

    let mut vm = VM::default();

    vm.execute_program(&program);

    dbg!(vm);
}
