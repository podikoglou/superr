use superr_bytecode::{instruction_to_u32, program_to_bytes};
use superr_isa::{Instruction, Program};

#[test]
fn test_encode_load() {
    assert_eq!(instruction_to_u32(&Instruction::Load(0)), 0x10000000);
    assert_eq!(instruction_to_u32(&Instruction::Load(4)), 0x10000004);
    assert_eq!(instruction_to_u32(&Instruction::Load(255)), 0x100000FF);
}

#[test]
fn test_encode_swap() {
    assert_eq!(instruction_to_u32(&Instruction::Swap(0, 0)), 0x20000000);
    assert_eq!(instruction_to_u32(&Instruction::Swap(0, 1)), 0x20000001);
    assert_eq!(instruction_to_u32(&Instruction::Swap(1, 0)), 0x20000100);
    assert_eq!(instruction_to_u32(&Instruction::Swap(1, 1)), 0x20000101);
    assert_eq!(instruction_to_u32(&Instruction::Swap(0, 255)), 0x200000FF);
    assert_eq!(instruction_to_u32(&Instruction::Swap(1, 255)), 0x200001FF);
    assert_eq!(instruction_to_u32(&Instruction::Swap(255, 0)), 0x2000FF00);
    assert_eq!(instruction_to_u32(&Instruction::Swap(255, 1)), 0x2000FF01);
}

#[test]
fn test_encode_xor() {
    assert_eq!(instruction_to_u32(&Instruction::XOR(0, 0)), 0x30000000);
    assert_eq!(instruction_to_u32(&Instruction::XOR(0, 1)), 0x30000001);
    assert_eq!(instruction_to_u32(&Instruction::XOR(1, 0)), 0x30000100);
    assert_eq!(instruction_to_u32(&Instruction::XOR(1, 1)), 0x30000101);
    assert_eq!(instruction_to_u32(&Instruction::XOR(0, 255)), 0x300000FF);
    assert_eq!(instruction_to_u32(&Instruction::XOR(1, 255)), 0x300001FF);
    assert_eq!(instruction_to_u32(&Instruction::XOR(255, 0)), 0x3000FF00);
    assert_eq!(instruction_to_u32(&Instruction::XOR(255, 1)), 0x3000FF01);
}

#[test]
fn test_encode_inc() {
    assert_eq!(instruction_to_u32(&Instruction::Inc(0)), 0x40000000);
    assert_eq!(instruction_to_u32(&Instruction::Inc(4)), 0x40000004);
    assert_eq!(instruction_to_u32(&Instruction::Inc(255)), 0x400000FF);
}

#[test]
fn test_encode_dec() {
    assert_eq!(instruction_to_u32(&Instruction::Decr(0)), 0x50000000);
    assert_eq!(instruction_to_u32(&Instruction::Decr(4)), 0x50000004);
    assert_eq!(instruction_to_u32(&Instruction::Decr(255)), 0x500000FF);
}

#[test]
fn test_encode_add() {
    assert_eq!(instruction_to_u32(&Instruction::Add(0, 0)), 0x60000000);
    assert_eq!(instruction_to_u32(&Instruction::Add(0, 1)), 0x60000001);
    assert_eq!(instruction_to_u32(&Instruction::Add(1, 0)), 0x60000100);
    assert_eq!(instruction_to_u32(&Instruction::Add(1, 1)), 0x60000101);
    assert_eq!(instruction_to_u32(&Instruction::Add(0, 255)), 0x600000FF);
    assert_eq!(instruction_to_u32(&Instruction::Add(1, 255)), 0x600001FF);
    assert_eq!(instruction_to_u32(&Instruction::Add(255, 0)), 0x6000FF00);
    assert_eq!(instruction_to_u32(&Instruction::Add(255, 1)), 0x6000FF01);
}

#[test]
fn test_encode_sub() {
    assert_eq!(instruction_to_u32(&Instruction::Sub(0, 0)), 0x70000000);
    assert_eq!(instruction_to_u32(&Instruction::Sub(0, 1)), 0x70000001);
    assert_eq!(instruction_to_u32(&Instruction::Sub(1, 0)), 0x70000100);
    assert_eq!(instruction_to_u32(&Instruction::Sub(1, 1)), 0x70000101);
    assert_eq!(instruction_to_u32(&Instruction::Sub(0, 255)), 0x700000FF);
    assert_eq!(instruction_to_u32(&Instruction::Sub(1, 255)), 0x700001FF);
    assert_eq!(instruction_to_u32(&Instruction::Sub(255, 0)), 0x7000FF00);
    assert_eq!(instruction_to_u32(&Instruction::Sub(255, 1)), 0x7000FF01);
}

#[test]
fn test_encode_put() {
    assert_eq!(instruction_to_u32(&Instruction::Put(0)), 0x80000000);
    assert_eq!(instruction_to_u32(&Instruction::Put(4)), 0x80000004);
    assert_eq!(instruction_to_u32(&Instruction::Put(255)), 0x800000FF);
}

#[test]
fn test_encode_program() {
    assert_eq!(program_to_bytes(&Program(vec![])), vec![0x00, 0x00,]);

    assert_eq!(
        program_to_bytes(&Program(vec![Instruction::Load(255)])),
        vec![
            0x00, 0x01, // program length (1)
            0x10, 0x00, 0x00, 0xFF // LOAD 255
        ]
    );

    assert_eq!(
        program_to_bytes(&Program(vec![
            Instruction::Load(13),
            Instruction::Swap(0, 4)
        ])),
        vec![
            0x00, 0x02, // program length (2)
            0x10, 0x00, 0x00, 0x0D, // LOAD 13
            0x20, 0x00, 0x00, 0x04, // SWAP 0, 4
        ]
    );
}
