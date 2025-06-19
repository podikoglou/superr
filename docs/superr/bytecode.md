# Superr Bytecode Format
**Version**: 0.0.1

## Instructions
Each instruction contains a few pieces of data:
- The opcode (8 bits)
- Operands (12 bits each)

The amount of operands depends on the opcode. Some opcodes don't require
operands. There can be up to 2 operands.

This means that each instruction can be packed in **32 bits**.
