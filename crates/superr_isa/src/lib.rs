pub type Address = u8;
pub type ImmediateValue = u8;

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Address(Address),
    ImmediateValue(ImmediateValue),
}

// This instruction set (along with the assembly format) is heavily based on the one in the below
// project:
//
// https://github.com/AZHenley/superoptimizer
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Load(ImmediateValue),

    Swap(Address, Address),

    XOR(Address, Address),

    Inc(Address),
    Decr(Address),

    Add(Address, Address),
    Sub(Address, Address),

    Put(Address),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program(pub Vec<Instruction>);
