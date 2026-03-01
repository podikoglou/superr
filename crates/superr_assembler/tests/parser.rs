use chumsky::Parser;
use superr_assembler::parser::{instruction_parser, program_parser};
use superr_isa::{Instruction, Program};

#[test]
fn test_parse_instruction() {
    let parse = |s| instruction_parser().parse(s).into_result();

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
    let parse = |s| program_parser().parse(s).into_result();

    assert_eq!(parse("LOAD 42"), Ok(Program(vec![Instruction::Load(42)])));
    assert!(parse("LOAD 42, 0").is_err());
    assert!(parse("LOAD 42 0").is_err());
}

#[test]
fn test_multiple_operands() {
    let parse = |s| program_parser().parse(s).into_result();

    assert_eq!(parse("ADD 1,2"), Ok(Program(vec![Instruction::Add(1, 2)])));
    assert_eq!(parse("ADD 1, 2"), Ok(Program(vec![Instruction::Add(1, 2)])));
    assert_eq!(
        parse("ADD 1,  2"),
        Ok(Program(vec![Instruction::Add(1, 2)]))
    );
    assert!(parse("ADD 1 2").is_err());
    assert!(parse("ADD 1, 2, 3").is_err());
}

#[test]
fn test_newlines() {
    let parse = |s| program_parser().parse(s).into_result();

    let prog = Program(vec![Instruction::Load(5), Instruction::Load(2)]);

    assert_eq!(parse("LOAD 5\nLOAD 2"), Ok(prog.clone()));

    assert_eq!(parse("LOAD 5\nLOAD 2\n"), Ok(prog.clone()));

    assert_eq!(parse("LOAD 5\nLOAD 2\n\n"), Ok(prog.clone()));

    assert_eq!(parse("\n\nLOAD 5\nLOAD 2"), Ok(prog.clone()));

    assert_eq!(parse("\n\nLOAD 5\nLOAD 2\n\n"), Ok(prog.clone()));

    assert!(parse("LOAD 5LOAD 2").is_err());
}
