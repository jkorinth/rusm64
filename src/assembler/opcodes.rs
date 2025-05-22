// Opcode tables for the 6502 CPU
// This file contains the complete opcode mapping for the 6502 processor

use crate::ast::{Opcode, AddressingMode};
use std::collections::HashMap;

/// Represents an opcode lookup entry
#[derive(Debug, Clone, Copy)]
pub struct OpcodeEntry {
    /// The opcode byte
    pub byte: u8,
    
    /// Number of bytes including the opcode byte itself
    pub size: u8,
    
    /// Number of cycles required to execute this instruction
    pub cycles: u8,
}

impl OpcodeEntry {
    pub fn new(byte: u8, size: u8, cycles: u8) -> Self {
        Self { byte, size, cycles }
    }
}

/// Build a complete opcode lookup table for all 6502 instructions
pub fn build_opcode_table() -> HashMap<(Opcode, AddressingMode), OpcodeEntry> {
    let mut table = HashMap::new();
    
    // Load/Store Operations
    // LDA
    table.insert((Opcode::LDA, AddressingMode::Immediate), OpcodeEntry::new(0xA9, 2, 2));
    table.insert((Opcode::LDA, AddressingMode::ZeroPage), OpcodeEntry::new(0xA5, 2, 3));
    table.insert((Opcode::LDA, AddressingMode::ZeroPageX), OpcodeEntry::new(0xB5, 2, 4));
    table.insert((Opcode::LDA, AddressingMode::Absolute), OpcodeEntry::new(0xAD, 3, 4));
    table.insert((Opcode::LDA, AddressingMode::AbsoluteX), OpcodeEntry::new(0xBD, 3, 4));
    table.insert((Opcode::LDA, AddressingMode::AbsoluteY), OpcodeEntry::new(0xB9, 3, 4));
    table.insert((Opcode::LDA, AddressingMode::IndexedIndirect), OpcodeEntry::new(0xA1, 2, 6));
    table.insert((Opcode::LDA, AddressingMode::IndirectIndexed), OpcodeEntry::new(0xB1, 2, 5));
    
    // LDX
    table.insert((Opcode::LDX, AddressingMode::Immediate), OpcodeEntry::new(0xA2, 2, 2));
    table.insert((Opcode::LDX, AddressingMode::ZeroPage), OpcodeEntry::new(0xA6, 2, 3));
    table.insert((Opcode::LDX, AddressingMode::ZeroPageY), OpcodeEntry::new(0xB6, 2, 4));
    table.insert((Opcode::LDX, AddressingMode::Absolute), OpcodeEntry::new(0xAE, 3, 4));
    table.insert((Opcode::LDX, AddressingMode::AbsoluteY), OpcodeEntry::new(0xBE, 3, 4));
    
    // LDY
    table.insert((Opcode::LDY, AddressingMode::Immediate), OpcodeEntry::new(0xA0, 2, 2));
    table.insert((Opcode::LDY, AddressingMode::ZeroPage), OpcodeEntry::new(0xA4, 2, 3));
    table.insert((Opcode::LDY, AddressingMode::ZeroPageX), OpcodeEntry::new(0xB4, 2, 4));
    table.insert((Opcode::LDY, AddressingMode::Absolute), OpcodeEntry::new(0xAC, 3, 4));
    table.insert((Opcode::LDY, AddressingMode::AbsoluteX), OpcodeEntry::new(0xBC, 3, 4));
    
    // STA
    table.insert((Opcode::STA, AddressingMode::ZeroPage), OpcodeEntry::new(0x85, 2, 3));
    table.insert((Opcode::STA, AddressingMode::ZeroPageX), OpcodeEntry::new(0x95, 2, 4));
    table.insert((Opcode::STA, AddressingMode::Absolute), OpcodeEntry::new(0x8D, 3, 4));
    table.insert((Opcode::STA, AddressingMode::AbsoluteX), OpcodeEntry::new(0x9D, 3, 5));
    table.insert((Opcode::STA, AddressingMode::AbsoluteY), OpcodeEntry::new(0x99, 3, 5));
    table.insert((Opcode::STA, AddressingMode::IndexedIndirect), OpcodeEntry::new(0x81, 2, 6));
    table.insert((Opcode::STA, AddressingMode::IndirectIndexed), OpcodeEntry::new(0x91, 2, 6));
    
    // STX
    table.insert((Opcode::STX, AddressingMode::ZeroPage), OpcodeEntry::new(0x86, 2, 3));
    table.insert((Opcode::STX, AddressingMode::ZeroPageY), OpcodeEntry::new(0x96, 2, 4));
    table.insert((Opcode::STX, AddressingMode::Absolute), OpcodeEntry::new(0x8E, 3, 4));
    
    // STY
    table.insert((Opcode::STY, AddressingMode::ZeroPage), OpcodeEntry::new(0x84, 2, 3));
    table.insert((Opcode::STY, AddressingMode::ZeroPageX), OpcodeEntry::new(0x94, 2, 4));
    table.insert((Opcode::STY, AddressingMode::Absolute), OpcodeEntry::new(0x8C, 3, 4));
    
    // Register Transfers
    table.insert((Opcode::TAX, AddressingMode::Implied), OpcodeEntry::new(0xAA, 1, 2));
    table.insert((Opcode::TAY, AddressingMode::Implied), OpcodeEntry::new(0xA8, 1, 2));
    table.insert((Opcode::TSX, AddressingMode::Implied), OpcodeEntry::new(0xBA, 1, 2));
    table.insert((Opcode::TXA, AddressingMode::Implied), OpcodeEntry::new(0x8A, 1, 2));
    table.insert((Opcode::TXS, AddressingMode::Implied), OpcodeEntry::new(0x9A, 1, 2));
    table.insert((Opcode::TYA, AddressingMode::Implied), OpcodeEntry::new(0x98, 1, 2));
    
    // Stack Operations
    table.insert((Opcode::PHA, AddressingMode::Implied), OpcodeEntry::new(0x48, 1, 3));
    table.insert((Opcode::PHP, AddressingMode::Implied), OpcodeEntry::new(0x08, 1, 3));
    table.insert((Opcode::PLA, AddressingMode::Implied), OpcodeEntry::new(0x68, 1, 4));
    table.insert((Opcode::PLP, AddressingMode::Implied), OpcodeEntry::new(0x28, 1, 4));
    
    // Logical Operations
    // AND
    table.insert((Opcode::AND, AddressingMode::Immediate), OpcodeEntry::new(0x29, 2, 2));
    table.insert((Opcode::AND, AddressingMode::ZeroPage), OpcodeEntry::new(0x25, 2, 3));
    table.insert((Opcode::AND, AddressingMode::ZeroPageX), OpcodeEntry::new(0x35, 2, 4));
    table.insert((Opcode::AND, AddressingMode::Absolute), OpcodeEntry::new(0x2D, 3, 4));
    table.insert((Opcode::AND, AddressingMode::AbsoluteX), OpcodeEntry::new(0x3D, 3, 4));
    table.insert((Opcode::AND, AddressingMode::AbsoluteY), OpcodeEntry::new(0x39, 3, 4));
    table.insert((Opcode::AND, AddressingMode::IndexedIndirect), OpcodeEntry::new(0x21, 2, 6));
    table.insert((Opcode::AND, AddressingMode::IndirectIndexed), OpcodeEntry::new(0x31, 2, 5));
    
    // EOR
    table.insert((Opcode::EOR, AddressingMode::Immediate), OpcodeEntry::new(0x49, 2, 2));
    table.insert((Opcode::EOR, AddressingMode::ZeroPage), OpcodeEntry::new(0x45, 2, 3));
    table.insert((Opcode::EOR, AddressingMode::ZeroPageX), OpcodeEntry::new(0x55, 2, 4));
    table.insert((Opcode::EOR, AddressingMode::Absolute), OpcodeEntry::new(0x4D, 3, 4));
    table.insert((Opcode::EOR, AddressingMode::AbsoluteX), OpcodeEntry::new(0x5D, 3, 4));
    table.insert((Opcode::EOR, AddressingMode::AbsoluteY), OpcodeEntry::new(0x59, 3, 4));
    table.insert((Opcode::EOR, AddressingMode::IndexedIndirect), OpcodeEntry::new(0x41, 2, 6));
    table.insert((Opcode::EOR, AddressingMode::IndirectIndexed), OpcodeEntry::new(0x51, 2, 5));
    
    // ORA
    table.insert((Opcode::ORA, AddressingMode::Immediate), OpcodeEntry::new(0x09, 2, 2));
    table.insert((Opcode::ORA, AddressingMode::ZeroPage), OpcodeEntry::new(0x05, 2, 3));
    table.insert((Opcode::ORA, AddressingMode::ZeroPageX), OpcodeEntry::new(0x15, 2, 4));
    table.insert((Opcode::ORA, AddressingMode::Absolute), OpcodeEntry::new(0x0D, 3, 4));
    table.insert((Opcode::ORA, AddressingMode::AbsoluteX), OpcodeEntry::new(0x1D, 3, 4));
    table.insert((Opcode::ORA, AddressingMode::AbsoluteY), OpcodeEntry::new(0x19, 3, 4));
    table.insert((Opcode::ORA, AddressingMode::IndexedIndirect), OpcodeEntry::new(0x01, 2, 6));
    table.insert((Opcode::ORA, AddressingMode::IndirectIndexed), OpcodeEntry::new(0x11, 2, 5));
    
    // BIT
    table.insert((Opcode::BIT, AddressingMode::ZeroPage), OpcodeEntry::new(0x24, 2, 3));
    table.insert((Opcode::BIT, AddressingMode::Absolute), OpcodeEntry::new(0x2C, 3, 4));
    
    // Arithmetic Operations
    // ADC
    table.insert((Opcode::ADC, AddressingMode::Immediate), OpcodeEntry::new(0x69, 2, 2));
    table.insert((Opcode::ADC, AddressingMode::ZeroPage), OpcodeEntry::new(0x65, 2, 3));
    table.insert((Opcode::ADC, AddressingMode::ZeroPageX), OpcodeEntry::new(0x75, 2, 4));
    table.insert((Opcode::ADC, AddressingMode::Absolute), OpcodeEntry::new(0x6D, 3, 4));
    table.insert((Opcode::ADC, AddressingMode::AbsoluteX), OpcodeEntry::new(0x7D, 3, 4));
    table.insert((Opcode::ADC, AddressingMode::AbsoluteY), OpcodeEntry::new(0x79, 3, 4));
    table.insert((Opcode::ADC, AddressingMode::IndexedIndirect), OpcodeEntry::new(0x61, 2, 6));
    table.insert((Opcode::ADC, AddressingMode::IndirectIndexed), OpcodeEntry::new(0x71, 2, 5));
    
    // SBC
    table.insert((Opcode::SBC, AddressingMode::Immediate), OpcodeEntry::new(0xE9, 2, 2));
    table.insert((Opcode::SBC, AddressingMode::ZeroPage), OpcodeEntry::new(0xE5, 2, 3));
    table.insert((Opcode::SBC, AddressingMode::ZeroPageX), OpcodeEntry::new(0xF5, 2, 4));
    table.insert((Opcode::SBC, AddressingMode::Absolute), OpcodeEntry::new(0xED, 3, 4));
    table.insert((Opcode::SBC, AddressingMode::AbsoluteX), OpcodeEntry::new(0xFD, 3, 4));
    table.insert((Opcode::SBC, AddressingMode::AbsoluteY), OpcodeEntry::new(0xF9, 3, 4));
    table.insert((Opcode::SBC, AddressingMode::IndexedIndirect), OpcodeEntry::new(0xE1, 2, 6));
    table.insert((Opcode::SBC, AddressingMode::IndirectIndexed), OpcodeEntry::new(0xF1, 2, 5));
    
    // CMP
    table.insert((Opcode::CMP, AddressingMode::Immediate), OpcodeEntry::new(0xC9, 2, 2));
    table.insert((Opcode::CMP, AddressingMode::ZeroPage), OpcodeEntry::new(0xC5, 2, 3));
    table.insert((Opcode::CMP, AddressingMode::ZeroPageX), OpcodeEntry::new(0xD5, 2, 4));
    table.insert((Opcode::CMP, AddressingMode::Absolute), OpcodeEntry::new(0xCD, 3, 4));
    table.insert((Opcode::CMP, AddressingMode::AbsoluteX), OpcodeEntry::new(0xDD, 3, 4));
    table.insert((Opcode::CMP, AddressingMode::AbsoluteY), OpcodeEntry::new(0xD9, 3, 4));
    table.insert((Opcode::CMP, AddressingMode::IndexedIndirect), OpcodeEntry::new(0xC1, 2, 6));
    table.insert((Opcode::CMP, AddressingMode::IndirectIndexed), OpcodeEntry::new(0xD1, 2, 5));
    
    // CPX
    table.insert((Opcode::CPX, AddressingMode::Immediate), OpcodeEntry::new(0xE0, 2, 2));
    table.insert((Opcode::CPX, AddressingMode::ZeroPage), OpcodeEntry::new(0xE4, 2, 3));
    table.insert((Opcode::CPX, AddressingMode::Absolute), OpcodeEntry::new(0xEC, 3, 4));
    
    // CPY
    table.insert((Opcode::CPY, AddressingMode::Immediate), OpcodeEntry::new(0xC0, 2, 2));
    table.insert((Opcode::CPY, AddressingMode::ZeroPage), OpcodeEntry::new(0xC4, 2, 3));
    table.insert((Opcode::CPY, AddressingMode::Absolute), OpcodeEntry::new(0xCC, 3, 4));
    
    // Increments & Decrements
    // INC
    table.insert((Opcode::INC, AddressingMode::ZeroPage), OpcodeEntry::new(0xE6, 2, 5));
    table.insert((Opcode::INC, AddressingMode::ZeroPageX), OpcodeEntry::new(0xF6, 2, 6));
    table.insert((Opcode::INC, AddressingMode::Absolute), OpcodeEntry::new(0xEE, 3, 6));
    table.insert((Opcode::INC, AddressingMode::AbsoluteX), OpcodeEntry::new(0xFE, 3, 7));
    
    // INX
    table.insert((Opcode::INX, AddressingMode::Implied), OpcodeEntry::new(0xE8, 1, 2));
    
    // INY
    table.insert((Opcode::INY, AddressingMode::Implied), OpcodeEntry::new(0xC8, 1, 2));
    
    // DEC
    table.insert((Opcode::DEC, AddressingMode::ZeroPage), OpcodeEntry::new(0xC6, 2, 5));
    table.insert((Opcode::DEC, AddressingMode::ZeroPageX), OpcodeEntry::new(0xD6, 2, 6));
    table.insert((Opcode::DEC, AddressingMode::Absolute), OpcodeEntry::new(0xCE, 3, 6));
    table.insert((Opcode::DEC, AddressingMode::AbsoluteX), OpcodeEntry::new(0xDE, 3, 7));
    
    // DEX
    table.insert((Opcode::DEX, AddressingMode::Implied), OpcodeEntry::new(0xCA, 1, 2));
    
    // DEY
    table.insert((Opcode::DEY, AddressingMode::Implied), OpcodeEntry::new(0x88, 1, 2));
    
    // Shifts
    // ASL
    table.insert((Opcode::ASL, AddressingMode::Accumulator), OpcodeEntry::new(0x0A, 1, 2));
    table.insert((Opcode::ASL, AddressingMode::ZeroPage), OpcodeEntry::new(0x06, 2, 5));
    table.insert((Opcode::ASL, AddressingMode::ZeroPageX), OpcodeEntry::new(0x16, 2, 6));
    table.insert((Opcode::ASL, AddressingMode::Absolute), OpcodeEntry::new(0x0E, 3, 6));
    table.insert((Opcode::ASL, AddressingMode::AbsoluteX), OpcodeEntry::new(0x1E, 3, 7));
    
    // LSR
    table.insert((Opcode::LSR, AddressingMode::Accumulator), OpcodeEntry::new(0x4A, 1, 2));
    table.insert((Opcode::LSR, AddressingMode::ZeroPage), OpcodeEntry::new(0x46, 2, 5));
    table.insert((Opcode::LSR, AddressingMode::ZeroPageX), OpcodeEntry::new(0x56, 2, 6));
    table.insert((Opcode::LSR, AddressingMode::Absolute), OpcodeEntry::new(0x4E, 3, 6));
    table.insert((Opcode::LSR, AddressingMode::AbsoluteX), OpcodeEntry::new(0x5E, 3, 7));
    
    // ROL
    table.insert((Opcode::ROL, AddressingMode::Accumulator), OpcodeEntry::new(0x2A, 1, 2));
    table.insert((Opcode::ROL, AddressingMode::ZeroPage), OpcodeEntry::new(0x26, 2, 5));
    table.insert((Opcode::ROL, AddressingMode::ZeroPageX), OpcodeEntry::new(0x36, 2, 6));
    table.insert((Opcode::ROL, AddressingMode::Absolute), OpcodeEntry::new(0x2E, 3, 6));
    table.insert((Opcode::ROL, AddressingMode::AbsoluteX), OpcodeEntry::new(0x3E, 3, 7));
    
    // ROR
    table.insert((Opcode::ROR, AddressingMode::Accumulator), OpcodeEntry::new(0x6A, 1, 2));
    table.insert((Opcode::ROR, AddressingMode::ZeroPage), OpcodeEntry::new(0x66, 2, 5));
    table.insert((Opcode::ROR, AddressingMode::ZeroPageX), OpcodeEntry::new(0x76, 2, 6));
    table.insert((Opcode::ROR, AddressingMode::Absolute), OpcodeEntry::new(0x6E, 3, 6));
    table.insert((Opcode::ROR, AddressingMode::AbsoluteX), OpcodeEntry::new(0x7E, 3, 7));
    
    // Jumps & Calls
    table.insert((Opcode::JMP, AddressingMode::Absolute), OpcodeEntry::new(0x4C, 3, 3));
    table.insert((Opcode::JMP, AddressingMode::Indirect), OpcodeEntry::new(0x6C, 3, 5));
    table.insert((Opcode::JSR, AddressingMode::Absolute), OpcodeEntry::new(0x20, 3, 6));
    table.insert((Opcode::RTS, AddressingMode::Implied), OpcodeEntry::new(0x60, 1, 6));
    table.insert((Opcode::RTI, AddressingMode::Implied), OpcodeEntry::new(0x40, 1, 6));
    
    // Branches
    table.insert((Opcode::BCC, AddressingMode::Relative), OpcodeEntry::new(0x90, 2, 2));
    table.insert((Opcode::BCS, AddressingMode::Relative), OpcodeEntry::new(0xB0, 2, 2));
    table.insert((Opcode::BEQ, AddressingMode::Relative), OpcodeEntry::new(0xF0, 2, 2));
    table.insert((Opcode::BMI, AddressingMode::Relative), OpcodeEntry::new(0x30, 2, 2));
    table.insert((Opcode::BNE, AddressingMode::Relative), OpcodeEntry::new(0xD0, 2, 2));
    table.insert((Opcode::BPL, AddressingMode::Relative), OpcodeEntry::new(0x10, 2, 2));
    table.insert((Opcode::BVC, AddressingMode::Relative), OpcodeEntry::new(0x50, 2, 2));
    table.insert((Opcode::BVS, AddressingMode::Relative), OpcodeEntry::new(0x70, 2, 2));
    
    // Status Flag Changes
    table.insert((Opcode::CLC, AddressingMode::Implied), OpcodeEntry::new(0x18, 1, 2));
    table.insert((Opcode::CLD, AddressingMode::Implied), OpcodeEntry::new(0xD8, 1, 2));
    table.insert((Opcode::CLI, AddressingMode::Implied), OpcodeEntry::new(0x58, 1, 2));
    table.insert((Opcode::CLV, AddressingMode::Implied), OpcodeEntry::new(0xB8, 1, 2));
    table.insert((Opcode::SEC, AddressingMode::Implied), OpcodeEntry::new(0x38, 1, 2));
    table.insert((Opcode::SED, AddressingMode::Implied), OpcodeEntry::new(0xF8, 1, 2));
    table.insert((Opcode::SEI, AddressingMode::Implied), OpcodeEntry::new(0x78, 1, 2));
    
    // No Operation
    table.insert((Opcode::NOP, AddressingMode::Implied), OpcodeEntry::new(0xEA, 1, 2));
    
    // A few common illegal/undocumented opcodes
    table.insert((Opcode::SLO, AddressingMode::ZeroPage), OpcodeEntry::new(0x07, 2, 5));
    table.insert((Opcode::RLA, AddressingMode::ZeroPage), OpcodeEntry::new(0x27, 2, 5));
    table.insert((Opcode::SRE, AddressingMode::ZeroPage), OpcodeEntry::new(0x47, 2, 5));
    table.insert((Opcode::RRA, AddressingMode::ZeroPage), OpcodeEntry::new(0x67, 2, 5));
    table.insert((Opcode::SAX, AddressingMode::ZeroPage), OpcodeEntry::new(0x87, 2, 3));
    table.insert((Opcode::LAX, AddressingMode::ZeroPage), OpcodeEntry::new(0xA7, 2, 3));
    table.insert((Opcode::DCP, AddressingMode::ZeroPage), OpcodeEntry::new(0xC7, 2, 5));
    table.insert((Opcode::ISC, AddressingMode::ZeroPage), OpcodeEntry::new(0xE7, 2, 5));
    
    table
}
