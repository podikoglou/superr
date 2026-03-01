use superr_vm::{
    isa::Instruction,
    vm::{self, MemValue},
};

pub mod optimizers;
pub mod vm_pool;

pub fn generate_instruction(max_num: MemValue) -> Instruction {
    let instruction = fastrand::usize(0..=6);

    match instruction {
        0 => {
            let val = fastrand::u8(0..max_num);

            Instruction::Load(val)
        }

        1 | 2 | 5 | 6 => {
            let addr1 = fastrand::u8(0..vm::MEM_SIZE as u8);
            let addr2 = fastrand::u8(0..vm::MEM_SIZE as u8);

            match instruction {
                1 => Instruction::Swap(addr1, addr2),
                2 => Instruction::XOR(addr1, addr2),
                5 => Instruction::Add(addr1, addr2),
                6 => Instruction::Sub(addr1, addr2),

                _ => panic!("SUPER unexpected error occurred"),
            }
        }

        3 | 4 => {
            let addr = fastrand::u8(0..vm::MEM_SIZE as u8);

            match instruction {
                3 => Instruction::Inc(addr),
                4 => Instruction::Decr(addr),

                _ => panic!("SUPER unexpected error occurred"),
            }
        }

        _ => panic!("SUPER unexpected error occurred"),
    }
}
