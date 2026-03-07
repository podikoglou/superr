use superr_bytecode::{instruction_to_u32, u32_to_instruction};
use superr_isa::Instruction;

fn assert_roundtrip(instruction: Instruction) {
    let encoded = instruction_to_u32(&instruction);
    let decoded = u32_to_instruction(encoded).expect("expected valid instruction");
    assert_eq!(decoded, instruction);
}

#[test]
fn test_roundtrip_encode_decode() {
    for value in 0u8..=u8::MAX {
        assert_roundtrip(Instruction::Load(value));
        assert_roundtrip(Instruction::Inc(value));
        assert_roundtrip(Instruction::Dec(value));
        assert_roundtrip(Instruction::Put(value));
    }

    for a in 0u8..=u8::MAX {
        for b in 0u8..=u8::MAX {
            assert_roundtrip(Instruction::Swap(a, b));
            assert_roundtrip(Instruction::XOR(a, b));
            assert_roundtrip(Instruction::Add(a, b));
            assert_roundtrip(Instruction::Sub(a, b));
        }
    }
}
