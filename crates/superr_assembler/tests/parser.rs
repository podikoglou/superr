use chumsky::Parser;
use superr_assembler::{
    ast::{ASTNode, Operand},
    parser::parser,
};

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
