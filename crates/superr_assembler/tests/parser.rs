use chumsky::Parser;
use superr_assembler::{
    ast::{ASTNode, Instruction, Operand},
    parser::{parse_instruction, parser},
};

#[test]
fn test_parse_instruction() {
    let parse = |s| parse_instruction().parse(s).into_result();

    assert_eq!(parse("LOAD 42"), Ok(Instruction::Load(42)));
    assert_eq!(parse("SWAP 4, 2"), Ok(Instruction::Swap(4, 2)));
    assert_eq!(parse("XOR 0, 255"), Ok(Instruction::XOR(0, 255)));
    assert_eq!(parse("INC 27"), Ok(Instruction::Inc(27)));
    assert_eq!(parse("DECR 27"), Ok(Instruction::Decr(27)));
    assert_eq!(parse("ADD 255, 255"), Ok(Instruction::Add(255, 255)));
    assert_eq!(parse("SUB 0, 0"), Ok(Instruction::Sub(0, 0)));
    assert_eq!(parse("PUT 0"), Ok(Instruction::Put(0)));
}

#[test]
fn test_single_instruction() {
    let expected = vec![ASTNode::Instruction {
        opcode: "LOAD".to_string(),
        operands: vec![Operand::ImmediateValue(42)],
    }];

    assert_eq!(parser().parse("LOAD 42").into_result(), Ok(expected));
}

#[test]
fn test_multiple_operands() {
    let expected = vec![ASTNode::Instruction {
        opcode: "ADD".to_string(),
        operands: vec![Operand::ImmediateValue(1), Operand::ImmediateValue((2))],
    }];

    assert_eq!(parser().parse("ADD 1, 2").into_result(), Ok(expected));
}

#[test]
fn test_no_operands() {
    let expected = vec![ASTNode::Instruction {
        opcode: "HALT".to_string(),
        operands: vec![],
    }];

    assert_eq!(parser().parse("HALT ").into_result(), Ok(expected.clone()));
    assert_eq!(parser().parse("HALT").into_result(), Ok(expected.clone()));
}

#[test]
fn test_newlines() {
    let prog = vec![
        ASTNode::Instruction {
            opcode: "LOAD".to_string(),
            operands: vec![Operand::ImmediateValue(5)],
        },
        ASTNode::Instruction {
            opcode: "LOAD".to_string(),
            operands: vec![Operand::ImmediateValue(2)],
        },
    ];

    assert_eq!(
        parser().parse("LOAD 5\nLOAD 2").into_result(),
        Ok(prog.clone())
    );

    assert_eq!(
        parser().parse("LOAD 5\nLOAD 2\n").into_result(),
        Ok(prog.clone())
    );

    assert_eq!(
        parser().parse("LOAD 5\nLOAD 2\n\n").into_result(),
        Ok(prog.clone())
    );

    assert_eq!(
        parser().parse("\n\nLOAD 5\nLOAD 2").into_result(),
        Ok(prog.clone())
    );

    assert_eq!(
        parser().parse("\n\nLOAD 5\nLOAD 2\n\n").into_result(),
        Ok(prog.clone())
    );

    assert!(parser().parse("LOAD 5LOAD 2").into_result().is_err());
}
