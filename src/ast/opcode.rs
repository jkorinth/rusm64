use derive_more::{Display, FromStr};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, FromStr, Hash)]
pub enum Opcode {
    // Load/Store Operations
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,

    // Register Transfers
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,

    // Stack Operations
    PHA,
    PHP,
    PLA,
    PLP,

    // Logical Operations
    AND,
    EOR,
    ORA,
    BIT,

    // Arithmetic Operations
    ADC,
    SBC,
    CMP,
    CPX,
    CPY,

    // Increments & Decrements
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,

    // Shifts
    ASL,
    LSR,
    ROL,
    ROR,

    // Jumps & Calls
    JMP,
    JSR,
    RTS,
    RTI,

    // Branches
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,

    // Status Flag Operations
    CLC,
    CLD,
    CLI,
    CLV,
    SEC,
    SED,
    SEI,

    // No Operation
    NOP,

    // Illegal/Undocumented Opcodes (just a few examples)
    SLO,
    RLA,
    SRE,
    RRA,
    SAX,
    LAX,
    DCP,
    ISC,

    // Halt and Catch Fire
    HCF,
}
