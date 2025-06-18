use chumsky::{prelude::*, text};

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Instruction { opcode: String, operands: Vec<u64> },
    Block(Vec<ASTNode>),
}

pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<ASTNode>> {
    let number = text::int(10)
        .map(|x| u64::from_str_radix(x, 10))
        .unwrapped();

    // TOOD: strings/characters in the future?
    let operand = number;

    let operands = operand.separated_by(just(',').padded()).collect();

    let opcode = text::ident();

    let instruction = opcode
        .then(
            just(' ')
                .repeated()
                .at_least(1)
                .ignore_then(operands)
                .or_not(),
        )
        .map(
            |(opcode, operands): (&str, Option<Vec<u64>>)| ASTNode::Instruction {
                opcode: opcode.to_string(),
                operands: operands.unwrap_or_default(),
            },
        );

    instruction
        .separated_by(text::newline().repeated().at_least(1))
        .allow_trailing()
        .allow_leading()
        .collect::<Vec<_>>()
}
