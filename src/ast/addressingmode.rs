use derive_more::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressingMode {
    Implied,         // No operand (e.g., NOP)
    Accumulator,     // Operand is accumulator (e.g., ASL A)
    Immediate,       // Operand is immediate value (e.g., LDA #$10)
    ZeroPage,        // Operand is in zero page (e.g., LDA $10)
    ZeroPageX,       // Indexed zero page with X (e.g., LDA $10,X)
    ZeroPageY,       // Indexed zero page with Y (e.g., LDX $10,Y)
    Absolute,        // Operand is absolute address (e.g., LDA $1234)
    AbsoluteX,       // Indexed absolute with X (e.g., LDA $1234,X)
    AbsoluteY,       // Indexed absolute with Y (e.g., LDA $1234,Y)
    Indirect,        // Indirect addressing (e.g., JMP ($1234))
    IndexedIndirect, // Indexed indirect (e.g., LDA ($10,X))
    IndirectIndexed, // Indirect indexed (e.g., LDA ($10),Y)
    Relative,        // Relative addressing for branches (e.g., BNE label)
}
