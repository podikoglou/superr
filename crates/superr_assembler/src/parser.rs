use chumsky::{prelude::*, text::whitespace};

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

    let operands = operand.padded().separated_by(just(",")).collect();

    let opcode_parser = text::ident();

    let instruction = opcode_parser
        .then_ignore(whitespace().repeated().at_least(1))
        .then(operands)
        .map(
            |(opcode, operands): (&str, Vec<u64>)| ASTNode::Instruction {
                opcode: opcode.to_string(),
                operands: operands,
            },
        );

    instruction
        .separated_by(text::newline())
        .allow_trailing()
        // .repeated()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_basic_instructions() {
    //     for op1 in 0..255 {
    //         let prog = vec![ASTNode::Instruction {
    //             opcode: "LOAD".to_string(),
    //             operands: vec![op1],
    //         }];

    //         let input = format!("LOAD {}", opcode, op1);

    //         assert_eq!(parser().parse(&input).unwrap(), prog);
    //     }
    // }

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

        // this fails and i dont want it to
        assert_eq!(
            parser().parse("LOAD 5\nLOAD 2").into_result(),
            Ok(prog.clone())
        );

        // this too
        assert_eq!(
            parser().parse("LOAD 5\nLOAD 2\n").into_result(),
            Ok(prog.clone())
        );

        // this doesn't fail and i want it to fail
        assert_ne!(parser().parse("LOAD 5LOAD 2").into_result(), Ok(prog));
    }
}
