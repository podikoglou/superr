use chumsky::Parser;
use superr_assembler::{ast::ASTNode, parser::parser};

#[test]
fn test_single_instruction() {
    let expected = vec![ASTNode::Instruction {
        opcode: "LOAD".to_string(),
        operands: vec![42],
    }];

    assert_eq!(parser().parse("LOAD 42").into_result(), Ok(expected));
}

#[test]
fn test_multiple_operands() {
    let expected = vec![ASTNode::Instruction {
        opcode: "ADD".to_string(),
        operands: vec![1, 2, 3],
    }];

    assert_eq!(parser().parse("ADD 1, 2, 3").into_result(), Ok(expected));
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
            operands: vec![5],
        },
        ASTNode::Instruction {
            opcode: "LOAD".to_string(),
            operands: vec![2],
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

#[test]
fn test_large_numbers() {
    let expected = vec![ASTNode::Instruction {
        opcode: "SWAP".to_string(),
        operands: vec![u64::MAX],
    }];

    let input = format!("SWAP {}", u64::MAX);
    assert_eq!(parser().parse(&input).into_result(), Ok(expected));
}

#[test]
fn test_zero_operand() {
    let expected = vec![ASTNode::Instruction {
        opcode: "LOAD".to_string(),
        operands: vec![0],
    }];

    assert_eq!(parser().parse("LOAD 0").into_result(), Ok(expected));
}

#[test]
fn test_multiple_spaces() {
    let expected = vec![ASTNode::Instruction {
        opcode: "JMP".to_string(),
        operands: vec![100],
    }];

    assert_eq!(parser().parse("JMP    100").into_result(), Ok(expected));
}

#[test]
fn test_mixed_instructions() {
    let expected = vec![
        ASTNode::Instruction {
            opcode: "LOAD".to_string(),
            operands: vec![10],
        },
        ASTNode::Instruction {
            opcode: "ADD".to_string(),
            operands: vec![5, 15],
        },
        ASTNode::Instruction {
            opcode: "HALT".to_string(),
            operands: vec![],
        },
    ];

    assert_eq!(
        parser().parse("LOAD 10\nADD 5, 15\nHALT").into_result(),
        Ok(expected)
    );
}

#[test]
fn test_empty_input() {
    assert_eq!(parser().parse("").into_result(), Ok(vec![]));
}

#[test]
fn test_invalid_syntax_fails() {
    assert!(parser().parse("123 LOAD").into_result().is_err());
    assert!(parser().parse("LOAD abc").into_result().is_err());
    assert!(parser().parse("LOAD 5,").into_result().is_err());
}
