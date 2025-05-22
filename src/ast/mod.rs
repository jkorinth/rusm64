// AST representation for C64 assembly language

use std::collections::HashMap;
use std::fmt;

/// The complete AST representation of an assembly program
#[derive(Debug, Default, Clone)]
pub struct Ast {
    /// List of instructions in the program
    instructions: Vec<Instruction>,
    
    /// Map of labels to their positions
    labels: HashMap<String, Label>,
    
    /// Map of constants to their values
    constants: HashMap<String, String>,
    
    /// List of directives in the program
    directives: Vec<Directive>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
            constants: HashMap::new(),
            directives: Vec::new(),
        }
    }
    
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
    
    pub fn add_label(&mut self, label: Label) {
        self.labels.insert(label.name.clone(), label);
    }
    
    pub fn add_constant(&mut self, name: String, value: String) {
        self.constants.insert(name, value);
    }
    
    pub fn add_directive(&mut self, directive: Directive) {
        self.directives.push(directive);
    }
    
    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
    
    pub fn labels(&self) -> &HashMap<String, Label> {
        &self.labels
    }
    
    pub fn constants(&self) -> &HashMap<String, String> {
        &self.constants
    }
    
    pub fn directives(&self) -> &[Directive] {
        &self.directives
    }
}

/// Represents a 6502 instruction
#[derive(Debug, Clone)]
pub struct Instruction {
    /// The opcode of the instruction
    pub opcode: Opcode,
    
    /// The operand of the instruction (if any)
    pub operand: Option<Operand>,
}

impl Instruction {
    pub fn new(opcode: Opcode, operand: Option<Operand>) -> Self {
        Self { opcode, operand }
    }
}

/// Represents a 6502 opcode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    // Load/Store Operations
    LDA, LDX, LDY, STA, STX, STY,
    
    // Register Transfers
    TAX, TAY, TSX, TXA, TXS, TYA,
    
    // Stack Operations
    PHA, PHP, PLA, PLP,
    
    // Logical Operations
    AND, EOR, ORA, BIT,
    
    // Arithmetic Operations
    ADC, SBC, CMP, CPX, CPY,
    
    // Increments & Decrements
    INC, INX, INY, DEC, DEX, DEY,
    
    // Shifts
    ASL, LSR, ROL, ROR,
    
    // Jumps & Calls
    JMP, JSR, RTS, RTI,
    
    // Branches
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,
    
    // Status Flag Operations
    CLC, CLD, CLI, CLV, SEC, SED, SEI,
    
    // No Operation
    NOP,
    
    // Illegal/Undocumented Opcodes (just a few examples)
    SLO, RLA, SRE, RRA, SAX, LAX, DCP, ISC,
    
    // Halt and Catch Fire
    HCF,
}

impl Opcode {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "LDA" => Ok(Opcode::LDA),
            "LDX" => Ok(Opcode::LDX),
            "LDY" => Ok(Opcode::LDY),
            "STA" => Ok(Opcode::STA),
            "STX" => Ok(Opcode::STX),
            "STY" => Ok(Opcode::STY),
            "TAX" => Ok(Opcode::TAX),
            "TAY" => Ok(Opcode::TAY),
            "TSX" => Ok(Opcode::TSX),
            "TXA" => Ok(Opcode::TXA),
            "TXS" => Ok(Opcode::TXS),
            "TYA" => Ok(Opcode::TYA),
            "PHA" => Ok(Opcode::PHA),
            "PHP" => Ok(Opcode::PHP),
            "PLA" => Ok(Opcode::PLA),
            "PLP" => Ok(Opcode::PLP),
            "AND" => Ok(Opcode::AND),
            "EOR" => Ok(Opcode::EOR),
            "ORA" => Ok(Opcode::ORA),
            "BIT" => Ok(Opcode::BIT),
            "ADC" => Ok(Opcode::ADC),
            "SBC" => Ok(Opcode::SBC),
            "CMP" => Ok(Opcode::CMP),
            "CPX" => Ok(Opcode::CPX),
            "CPY" => Ok(Opcode::CPY),
            "INC" => Ok(Opcode::INC),
            "INX" => Ok(Opcode::INX),
            "INY" => Ok(Opcode::INY),
            "DEC" => Ok(Opcode::DEC),
            "DEX" => Ok(Opcode::DEX),
            "DEY" => Ok(Opcode::DEY),
            "ASL" => Ok(Opcode::ASL),
            "LSR" => Ok(Opcode::LSR),
            "ROL" => Ok(Opcode::ROL),
            "ROR" => Ok(Opcode::ROR),
            "JMP" => Ok(Opcode::JMP),
            "JSR" => Ok(Opcode::JSR),
            "RTS" => Ok(Opcode::RTS),
            "RTI" => Ok(Opcode::RTI),
            "BCC" => Ok(Opcode::BCC),
            "BCS" => Ok(Opcode::BCS),
            "BEQ" => Ok(Opcode::BEQ),
            "BMI" => Ok(Opcode::BMI),
            "BNE" => Ok(Opcode::BNE),
            "BPL" => Ok(Opcode::BPL),
            "BVC" => Ok(Opcode::BVC),
            "BVS" => Ok(Opcode::BVS),
            "CLC" => Ok(Opcode::CLC),
            "CLD" => Ok(Opcode::CLD),
            "CLI" => Ok(Opcode::CLI),
            "CLV" => Ok(Opcode::CLV),
            "SEC" => Ok(Opcode::SEC),
            "SED" => Ok(Opcode::SED),
            "SEI" => Ok(Opcode::SEI),
            "NOP" => Ok(Opcode::NOP),
            // Illegal/Undocumented Opcodes
            "SLO" => Ok(Opcode::SLO),
            "RLA" => Ok(Opcode::RLA),
            "SRE" => Ok(Opcode::SRE),
            "RRA" => Ok(Opcode::RRA),
            "SAX" => Ok(Opcode::SAX),
            "LAX" => Ok(Opcode::LAX),
            "DCP" => Ok(Opcode::DCP),
            "ISC" => Ok(Opcode::ISC),
            "HCF" => Ok(Opcode::HCF),
            _ => Err(format!("Unknown opcode: {}", s)),
        }
    }
}

/// Addressing modes for 6502 instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressingMode {
    Implied,            // No operand (e.g., NOP)
    Accumulator,        // Operand is accumulator (e.g., ASL A)
    Immediate,          // Operand is immediate value (e.g., LDA #$10)
    ZeroPage,           // Operand is in zero page (e.g., LDA $10)
    ZeroPageX,          // Indexed zero page with X (e.g., LDA $10,X)
    ZeroPageY,          // Indexed zero page with Y (e.g., LDX $10,Y)
    Absolute,           // Operand is absolute address (e.g., LDA $1234)
    AbsoluteX,          // Indexed absolute with X (e.g., LDA $1234,X)
    AbsoluteY,          // Indexed absolute with Y (e.g., LDA $1234,Y)
    Indirect,           // Indirect addressing (e.g., JMP ($1234))
    IndexedIndirect,    // Indexed indirect (e.g., LDA ($10,X))
    IndirectIndexed,    // Indirect indexed (e.g., LDA ($10),Y)
    Relative,           // Relative addressing for branches (e.g., BNE label)
}

/// Operand type for instructions
#[derive(Debug, Clone)]
pub enum Operand {
    /// Immediate value (#$xx)
    Immediate(String),
    
    /// Absolute or zero page address ($xxxx or $xx)
    Address(String),
    
    /// Zero page,X or Absolute,X
    IndexedX(String),
    
    /// Zero page,Y or Absolute,Y
    IndexedY(String),
    
    /// Indirect address (($xxxx))
    Indirect(String),
    
    /// Indexed indirect (($xx,X))
    IndexedIndirect(String),
    
    /// Indirect indexed (($xx),Y)
    IndirectIndexed(String),
}

// Implement Display for the Operand enum so it can be converted to string
impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Immediate(val) => write!(f, "#{}", val),
            Operand::Address(addr) => write!(f, "{}", addr),
            Operand::IndexedX(addr) => write!(f, "{},X", addr),
            Operand::IndexedY(addr) => write!(f, "{},Y", addr),
            Operand::Indirect(addr) => write!(f, "({})", addr),
            Operand::IndexedIndirect(addr) => write!(f, "({},X)", addr),
            Operand::IndirectIndexed(addr) => write!(f, "({},Y)", addr),
        }
    }
}

impl Operand {
    pub fn parse(s: &str) -> Self {
        if s.starts_with('#') {
            Operand::Immediate(s[1..].to_string())
        } else if s.ends_with(",x") || s.ends_with(",X") {
            let addr = &s[..s.len() - 2];
            Operand::IndexedX(addr.to_string())
        } else if s.ends_with(",y") || s.ends_with(",Y") {
            let addr = &s[..s.len() - 2];
            Operand::IndexedY(addr.to_string())
        } else if s.starts_with('(') && s.ends_with("),y") || s.ends_with("),Y") {
            let addr = &s[1..s.len() - 3];
            Operand::IndirectIndexed(addr.to_string())
        } else if s.starts_with('(') && (s.ends_with(",x)") || s.ends_with(",X)")) {
            let addr = &s[1..s.len() - 3];
            Operand::IndexedIndirect(addr.to_string())
        } else if s.starts_with('(') && s.ends_with(')') {
            let addr = &s[1..s.len() - 1];
            Operand::Indirect(addr.to_string())
        } else {
            Operand::Address(s.to_string())
        }
    }
    
    pub fn get_addressing_mode(&self, opcode: Opcode) -> AddressingMode {
        // Branch instructions always use relative addressing
        if matches!(opcode, 
            Opcode::BCC | Opcode::BCS | Opcode::BEQ | 
            Opcode::BMI | Opcode::BNE | Opcode::BPL | 
            Opcode::BVC | Opcode::BVS) {
            return AddressingMode::Relative;
        }
            
        match self {
            Operand::Immediate(_) => AddressingMode::Immediate,
            Operand::Address(addr) => {
                // Try to determine if it's zero page or absolute
                // This is a simplification; you'd need to evaluate expressions
                if addr.starts_with('$') && addr.len() <= 3 {
                    AddressingMode::ZeroPage
                } else {
                    AddressingMode::Absolute
                }
            }
            Operand::IndexedX(addr) => {
                if addr.starts_with('$') && addr.len() <= 3 {
                    AddressingMode::ZeroPageX
                } else {
                    AddressingMode::AbsoluteX
                }
            }
            Operand::IndexedY(addr) => {
                if addr.starts_with('$') && addr.len() <= 3 {
                    AddressingMode::ZeroPageY
                } else {
                    AddressingMode::AbsoluteY
                }
            }
            Operand::Indirect(_) => AddressingMode::Indirect,
            Operand::IndexedIndirect(_) => AddressingMode::IndexedIndirect,
            Operand::IndirectIndexed(_) => AddressingMode::IndirectIndexed,
        }
    }
}

/// Represents a label in the assembly code
#[derive(Debug, Clone)]
pub struct Label {
    /// Name of the label
    pub name: String,
    
    /// Position of the label (to be filled during assembly)
    pub position: Option<usize>,
}

impl Label {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            position: None,
        }
    }
    
    pub fn with_position(name: &str, position: usize) -> Self {
        Self {
            name: name.to_string(),
            position: Some(position),
        }
    }
}

/// Represents an assembly directive (like .byte, .word, etc.)
#[derive(Debug, Clone)]
pub struct Directive {
    /// Name of the directive
    pub name: String,
    
    /// Value of the directive
    pub value: String,
}

impl Directive {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}
