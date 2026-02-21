use crate::ast::{ASTNode, Operand};
use chumsky::{prelude::*, text};

pub fn parse_operand_address<'a>() -> impl Parser<'a, &'a str, usize> {
    text::int(10).map(|s: &str| s.parse::<usize>().unwrap())
}

pub fn parse_operand_immediate<'a>() -> impl Parser<'a, &'a str, u64> {
    text::int(10).map(|s: &str| s.parse::<u64>().unwrap())
}

pub fn parse_operand<'a>() -> impl Parser<'a, &'a str, Operand> {
    choice((
        parse_operand_address().map(Operand::Address),
        parse_operand_immediate().map(Operand::ImmediateValue),
    ))
}

pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<ASTNode>> {
    let operand = parse_operand();

    let operands = operand.padded().separated_by(just(',')).collect();

    let opcode = text::ident();

    let instruction = opcode.then(just(' ').repeated().ignore_then(operands)).map(
        |(opcode, operands): (&str, Vec<Operand>)| ASTNode::Instruction {
            opcode: opcode.to_string(),
            operands,
        },
    );

    instruction
        .separated_by(text::newline().repeated().at_least(1))
        .allow_trailing()
        .allow_leading()
        .collect::<Vec<_>>()
}
