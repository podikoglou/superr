use crate::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl Into<Vec<Vec<usize>>> for Program {
    fn into(self) -> Vec<Vec<usize>> {
        self.instructions
            .into_iter()
            .map(|instruction| match instruction {
                Instruction::Load(a) => vec![0, a as usize],
                Instruction::Swap(a, b) => vec![1, a, b],
                Instruction::XOR(a, b) => vec![2, a, b],
                Instruction::Inc(a) => vec![3, a],
            })
            .collect::<Vec<Vec<usize>>>()
    }
}

impl From<Vec<Vec<usize>>> for Program {
    fn from(vec: Vec<Vec<usize>>) -> Self {
        let instructions = vec
            .into_iter()
            .map(|item| match item[0] {
                0 => Instruction::Load(item[1] as u32),
                1 => Instruction::Swap(item[1], item[2]),
                2 => Instruction::XOR(item[1], item[2]),
                3 => Instruction::Inc(item[1]),

                _ => panic!("Invalid instruction code"),
            })
            .collect();

        Program { instructions }
    }
}
