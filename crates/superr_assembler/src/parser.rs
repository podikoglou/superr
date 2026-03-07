use chumsky::{prelude::*, text};
use superr_isa::{Instruction, Program};

type ParserError<'src> = extra::Err<Rich<'src, char>>;

fn address_operand_parser<'a>() -> impl Parser<'a, &'a str, u8, ParserError<'a>> {
    text::int(10).map(|s: &str| s.parse::<u8>().unwrap())
}

fn immediate_value_operand_parser<'a>() -> impl Parser<'a, &'a str, u8, ParserError<'a>> {
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

pub fn instruction_parser<'a>() -> impl Parser<'a, &'a str, Instruction, ParserError<'a>> {
    // TODO: macro for operands

    choice((
        text::keyword("LOAD")
            .ignore_then(text::whitespace())
            .ignore_then(immediate_value_operand_parser())
            .map(Instruction::Load),
        text::keyword("SWAP")
            .ignore_then(text::whitespace())
            .ignore_then(address_operand_parser())
            .then_ignore(just(",").padded())
            .then(address_operand_parser())
            .map(|(a, b)| Instruction::Swap(a, b)),
        text::keyword("XOR")
            .ignore_then(text::whitespace())
            .ignore_then(address_operand_parser())
            .then_ignore(just(",").padded())
            .then(address_operand_parser())
            .map(|(a, b)| Instruction::XOR(a, b)),
        text::keyword("INC")
            .ignore_then(text::whitespace())
            .ignore_then(address_operand_parser())
            .map(Instruction::Inc),
        text::keyword("DEC")
            .ignore_then(text::whitespace())
            .ignore_then(address_operand_parser())
            .map(Instruction::Dec),
        text::keyword("ADD")
            .ignore_then(text::whitespace())
            .ignore_then(address_operand_parser())
            .then_ignore(just(",").padded())
            .then(address_operand_parser())
            .map(|(a, b)| Instruction::Add(a, b)),
        text::keyword("SUB")
            .ignore_then(text::whitespace())
            .ignore_then(address_operand_parser())
            .then_ignore(just(",").padded())
            .then(address_operand_parser())
            .map(|(a, b)| Instruction::Sub(a, b)),
        text::keyword("PUT")
            .ignore_then(text::whitespace())
            .ignore_then(address_operand_parser())
            .map(Instruction::Put),
    ))
}

pub fn program_parser<'a>() -> impl Parser<'a, &'a str, Program, ParserError<'a>> {
    instruction_parser()
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

pub fn parse_program(input: &str) -> Result<Program, Vec<chumsky::error::Rich<'_, char>>> {
    program_parser().parse(input).into_result()
}
