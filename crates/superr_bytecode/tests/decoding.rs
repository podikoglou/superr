use superr_bytecode::u32_to_instruction;
use superr_isa::Instruction;

#[test]
fn test_decode_load() {
    assert_eq!(u32_to_instruction(0x10000000), Ok(Instruction::Load(0)));
    assert_eq!(u32_to_instruction(0x10000004), Ok(Instruction::Load(4)));
    assert_eq!(u32_to_instruction(0x100000FF), Ok(Instruction::Load(255)));
}

#[test]
fn test_decode_swap() {
    assert_eq!(u32_to_instruction(0x20000000), Ok(Instruction::Swap(0, 0)));
    assert_eq!(u32_to_instruction(0x20000001), Ok(Instruction::Swap(0, 1)));
    assert_eq!(u32_to_instruction(0x20000100), Ok(Instruction::Swap(1, 0)));
    assert_eq!(u32_to_instruction(0x20000101), Ok(Instruction::Swap(1, 1)));
    assert_eq!(
        u32_to_instruction(0x200000FF),
        Ok(Instruction::Swap(0, 255))
    );
    assert_eq!(
        u32_to_instruction(0x200001FF),
        Ok(Instruction::Swap(1, 255))
    );
    assert_eq!(
        u32_to_instruction(0x2000FF00),
        Ok(Instruction::Swap(255, 0))
    );
    assert_eq!(
        u32_to_instruction(0x2000FF01),
        Ok(Instruction::Swap(255, 1))
    );
}

#[test]
fn test_decode_xor() {
    assert_eq!(u32_to_instruction(0x30000000), Ok(Instruction::XOR(0, 0)));
    assert_eq!(u32_to_instruction(0x30000001), Ok(Instruction::XOR(0, 1)));
    assert_eq!(u32_to_instruction(0x30000100), Ok(Instruction::XOR(1, 0)));
    assert_eq!(u32_to_instruction(0x30000101), Ok(Instruction::XOR(1, 1)));
    assert_eq!(u32_to_instruction(0x300000FF), Ok(Instruction::XOR(0, 255)));
    assert_eq!(u32_to_instruction(0x300001FF), Ok(Instruction::XOR(1, 255)));
    assert_eq!(u32_to_instruction(0x3000FF00), Ok(Instruction::XOR(255, 0)));
    assert_eq!(u32_to_instruction(0x3000FF01), Ok(Instruction::XOR(255, 1)));
}

#[test]
fn test_decode_inc() {
    assert_eq!(u32_to_instruction(0x40000000), Ok(Instruction::Inc(0)));
    assert_eq!(u32_to_instruction(0x40000004), Ok(Instruction::Inc(4)));
    assert_eq!(u32_to_instruction(0x400000FF), Ok(Instruction::Inc(255)));
}

#[test]
fn test_decode_dec() {
    assert_eq!(u32_to_instruction(0x50000000), Ok(Instruction::Decr(0)));
    assert_eq!(u32_to_instruction(0x50000004), Ok(Instruction::Decr(4)));
    assert_eq!(u32_to_instruction(0x500000FF), Ok(Instruction::Decr(255)));
}

#[test]
fn test_decode_add() {
    assert_eq!(u32_to_instruction(0x60000000), Ok(Instruction::Add(0, 0)));
    assert_eq!(u32_to_instruction(0x60000001), Ok(Instruction::Add(0, 1)));
    assert_eq!(u32_to_instruction(0x60000100), Ok(Instruction::Add(1, 0)));
    assert_eq!(u32_to_instruction(0x60000101), Ok(Instruction::Add(1, 1)));
    assert_eq!(u32_to_instruction(0x600000FF), Ok(Instruction::Add(0, 255)));
    assert_eq!(u32_to_instruction(0x600001FF), Ok(Instruction::Add(1, 255)));
    assert_eq!(u32_to_instruction(0x6000FF00), Ok(Instruction::Add(255, 0)));
    assert_eq!(u32_to_instruction(0x6000FF01), Ok(Instruction::Add(255, 1)));
}

#[test]
fn test_decode_sub() {
    assert_eq!(u32_to_instruction(0x70000000), Ok(Instruction::Sub(0, 0)));
    assert_eq!(u32_to_instruction(0x70000001), Ok(Instruction::Sub(0, 1)));
    assert_eq!(u32_to_instruction(0x70000100), Ok(Instruction::Sub(1, 0)));
    assert_eq!(u32_to_instruction(0x70000101), Ok(Instruction::Sub(1, 1)));
    assert_eq!(u32_to_instruction(0x700000FF), Ok(Instruction::Sub(0, 255)));
    assert_eq!(u32_to_instruction(0x700001FF), Ok(Instruction::Sub(1, 255)));
    assert_eq!(u32_to_instruction(0x7000FF00), Ok(Instruction::Sub(255, 0)));
    assert_eq!(u32_to_instruction(0x7000FF01), Ok(Instruction::Sub(255, 1)));
}

#[test]
fn test_decode_put() {
    assert_eq!(u32_to_instruction(0x80000000), Ok(Instruction::Put(0)));
    assert_eq!(u32_to_instruction(0x80000004), Ok(Instruction::Put(4)));
    assert_eq!(u32_to_instruction(0x800000FF), Ok(Instruction::Put(255)));
}

// #[test]
// fn test_decode_program() {
//     assert_eq!(program_to_bytes(&Program(vec![])), vec![0x00, 0x00,]);
//
//     assert_eq!(
//         program_to_bytes(&Program(vec![Instruction::Load(255)])),
//         vec![
//             0x00, 0x01, // program length (1)
//             0x10, 0x00, 0x00, 0xFF // LOAD 255
//         ]
//     );
//
//     assert_eq!(
//         program_to_bytes(&Program(vec![
//             Instruction::Load(13),
//             Instruction::Swap(0, 4)
//         ])),
//         vec![
//             0x00, 0x02, // program length (2)
//             0x10, 0x00, 0x00, 0x0D, // LOAD 13
//             0x20, 0x00, 0x00, 0x04, // SWAP 0, 4
//         ]
//     );
// }
