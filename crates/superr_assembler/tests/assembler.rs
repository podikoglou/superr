use superr_assembler::ast::Instruction;

#[test]
fn test_assemble_load() {
    assert_eq!(Into::<u32>::into(&Instruction::Load(0)), 0x10000000);
    assert_eq!(Into::<u32>::into(&Instruction::Load(4)), 0x10000004);
    assert_eq!(Into::<u32>::into(&Instruction::Load(255)), 0x100000FF);
}

#[test]
fn test_assemble_swap() {
    assert_eq!(Into::<u32>::into(&Instruction::Swap(0, 0)), 0x20000000);
    assert_eq!(Into::<u32>::into(&Instruction::Swap(0, 1)), 0x20000001);
    assert_eq!(Into::<u32>::into(&Instruction::Swap(1, 0)), 0x20000100);
    assert_eq!(Into::<u32>::into(&Instruction::Swap(1, 1)), 0x20000101);
    assert_eq!(Into::<u32>::into(&Instruction::Swap(0, 255)), 0x200000FF);
    assert_eq!(Into::<u32>::into(&Instruction::Swap(1, 255)), 0x200001FF);
    assert_eq!(Into::<u32>::into(&Instruction::Swap(255, 0)), 0x2000FF00);
    assert_eq!(Into::<u32>::into(&Instruction::Swap(255, 1)), 0x2000FF01);
}

#[test]
fn test_assemble_xor() {
    assert_eq!(Into::<u32>::into(&Instruction::XOR(0, 0)), 0x30000000);
    assert_eq!(Into::<u32>::into(&Instruction::XOR(0, 1)), 0x30000001);
    assert_eq!(Into::<u32>::into(&Instruction::XOR(1, 0)), 0x30000100);
    assert_eq!(Into::<u32>::into(&Instruction::XOR(1, 1)), 0x30000101);
    assert_eq!(Into::<u32>::into(&Instruction::XOR(0, 255)), 0x300000FF);
    assert_eq!(Into::<u32>::into(&Instruction::XOR(1, 255)), 0x300001FF);
    assert_eq!(Into::<u32>::into(&Instruction::XOR(255, 0)), 0x3000FF00);
    assert_eq!(Into::<u32>::into(&Instruction::XOR(255, 1)), 0x3000FF01);
}

#[test]
fn test_assemble_inc() {
    assert_eq!(Into::<u32>::into(&Instruction::Inc(0)), 0x40000000);
    assert_eq!(Into::<u32>::into(&Instruction::Inc(4)), 0x40000004);
    assert_eq!(Into::<u32>::into(&Instruction::Inc(255)), 0x400000FF);
}

#[test]
fn test_assemble_dec() {
    assert_eq!(Into::<u32>::into(&Instruction::Decr(0)), 0x50000000);
    assert_eq!(Into::<u32>::into(&Instruction::Decr(4)), 0x50000004);
    assert_eq!(Into::<u32>::into(&Instruction::Decr(255)), 0x500000FF);
}

#[test]
fn test_assemble_add() {
    assert_eq!(Into::<u32>::into(&Instruction::Add(0, 0)), 0x60000000);
    assert_eq!(Into::<u32>::into(&Instruction::Add(0, 1)), 0x60000001);
    assert_eq!(Into::<u32>::into(&Instruction::Add(1, 0)), 0x60000100);
    assert_eq!(Into::<u32>::into(&Instruction::Add(1, 1)), 0x60000101);
    assert_eq!(Into::<u32>::into(&Instruction::Add(0, 255)), 0x600000FF);
    assert_eq!(Into::<u32>::into(&Instruction::Add(1, 255)), 0x600001FF);
    assert_eq!(Into::<u32>::into(&Instruction::Add(255, 0)), 0x6000FF00);
    assert_eq!(Into::<u32>::into(&Instruction::Add(255, 1)), 0x6000FF01);
}
#[test]
fn test_assemble_sub() {
    assert_eq!(Into::<u32>::into(&Instruction::Sub(0, 0)), 0x70000000);
    assert_eq!(Into::<u32>::into(&Instruction::Sub(0, 1)), 0x70000001);
    assert_eq!(Into::<u32>::into(&Instruction::Sub(1, 0)), 0x70000100);
    assert_eq!(Into::<u32>::into(&Instruction::Sub(1, 1)), 0x70000101);
    assert_eq!(Into::<u32>::into(&Instruction::Sub(0, 255)), 0x700000FF);
    assert_eq!(Into::<u32>::into(&Instruction::Sub(1, 255)), 0x700001FF);
    assert_eq!(Into::<u32>::into(&Instruction::Sub(255, 0)), 0x7000FF00);
    assert_eq!(Into::<u32>::into(&Instruction::Sub(255, 1)), 0x7000FF01);
}

#[test]
fn test_assemble_put() {
    assert_eq!(Into::<u32>::into(&Instruction::Put(0)), 0x80000000);
    assert_eq!(Into::<u32>::into(&Instruction::Put(4)), 0x80000004);
    assert_eq!(Into::<u32>::into(&Instruction::Put(255)), 0x800000FF);
}
