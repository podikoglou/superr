# Superr Bytecode Format
**Version**: 0.1.0

## Instruction Format
Instructions are packed in 32 bit unsigned integers (`u32`)

```
+---------------------------------------------------+
|   opcode   |            |  operand1  |  operand2  |
|            |            |            |            |
|   8 bits   |            |   8 bits   |   8 bits   |
+---------------------------------------------------+
```

In cases where the instruction only takes one operand, the operand is placed in
the second operand's slot.

## Programs
A program is a sequence of instructions prefixed with the amount of
instructions, as a 16 bit unsigned integer.

Since every instruction is 32 bits, the size of a program is `16 + n * 32`
bits, where `n` is the amount of instructions.

Everything (program length, instructions) is Big Endian.
