use chumsky::{prelude::*, text};
use superr_isa::{Instruction, Program};

pub type ParserError<'src> = extra::Err<Rich<'src, char>>;

pub fn parse_operand_address<'a>() -> impl Parser<'a, &'a str, u8, ParserError<'a>> {
    text::int(10).map(|s: &str| s.parse::<u8>().unwrap())
}

pub fn parse_operand_immediate<'a>() -> impl Parser<'a, &'a str, u8, ParserError<'a>> {
    text::int(10).map(|s: &str| s.parse::<u8>().unwrap())
}
//
// pub fn parse_operand<'a>() -> impl Parser<'a, &'a str, Operand, ParserError<'a>> {
//     choice((
//         parse_operand_address().map(Operand::Address),
//         parse_operand_immediate().map(Operand::ImmediateValue),
//     ))
// }
//
// pub fn parse_operands<'a>() -> impl Parser<'a, &'a str, Vec<Operand>, ParserError<'a>> {
//     parse_operand().padded().separated_by(just(',')).collect()
// }

pub fn parse_instruction<'a>() -> impl Parser<'a, &'a str, Instruction, ParserError<'a>> {
    // TODO: macro for operands

    choice((
        text::keyword("LOAD")
            .ignore_then(text::whitespace())
            .ignore_then(parse_operand_immediate())
            .map(Instruction::Load),
        text::keyword("SWAP")
            .ignore_then(text::whitespace())
            .ignore_then(parse_operand_address())
            .then_ignore(just(",").padded())
            .then(parse_operand_address())
            .map(|(a, b)| Instruction::Swap(a, b)),
        text::keyword("XOR")
            .ignore_then(text::whitespace())
            .ignore_then(parse_operand_address())
            .then_ignore(just(",").padded())
            .then(parse_operand_address())
            .map(|(a, b)| Instruction::XOR(a, b)),
        text::keyword("INC")
            .ignore_then(text::whitespace())
            .ignore_then(parse_operand_address())
            .map(Instruction::Inc),
        text::keyword("DECR") // like this for backwards compatibility. TODO: DEC
            .ignore_then(text::whitespace())
            .ignore_then(parse_operand_address())
            .map(Instruction::Decr),
        text::keyword("ADD")
            .ignore_then(text::whitespace())
            .ignore_then(parse_operand_address())
            .then_ignore(just(",").padded())
            .then(parse_operand_address())
            .map(|(a, b)| Instruction::Add(a, b)),
        text::keyword("SUB")
            .ignore_then(text::whitespace())
            .ignore_then(parse_operand_address())
            .then_ignore(just(",").padded())
            .then(parse_operand_address())
            .map(|(a, b)| Instruction::Sub(a, b)),
        text::keyword("PUT")
            .ignore_then(text::whitespace())
            .ignore_then(parse_operand_address())
            .map(Instruction::Put),
    ))
}

pub fn parser<'a>() -> impl Parser<'a, &'a str, Program, ParserError<'a>> {
    parse_instruction()
        .separated_by(text::newline().repeated().at_least(1))
        .allow_trailing()
        .allow_leading()
        .collect::<Vec<_>>()
        .map(Program)

    // let operand = parse_operand();
    //
    // let opcode = text::ident();
    //
    // // let instruction = opcode.then(just(' ').repeated().ignore_then(operands)).map(
    // //     |(opcode, operands): (&str, Vec<Operand>)| ASTNode::Instruction {
    // //         opcode: opcode.to_string(),
    // //         operands,
    // //     },
    // // );
    //
    // instruction
    //     .separated_by(text::newline().repeated().at_least(1))
    //     .allow_trailing()
    //     .allow_leading()
    //     .collect::<Vec<_>>()
}
