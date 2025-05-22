use std::fmt;

use derive_more::{From, FromStr};

/// The complete AST representation of an assembly program
#[derive(Debug, Default, Clone)]
pub struct Ast(Vec<Line>);

impl Ast {
    pub fn add_line(&mut self, line: Line) {
        self.0.push(line);
    }

    pub fn lines(&self) -> &Vec<Line> {
        &self.0
    }

    pub fn directives(&self) -> Vec<&InstrDir> {
        self.0
            .iter()
            .filter(|line| matches!(line.instrdir(), Some(InstrDir::Directive(_, _))))
            .map(|l| l.instrdir().unwrap())
            .collect()
    }

    pub fn instructions(&self) -> Vec<&InstrDir> {
        self.0
            .iter()
            .filter(|line| matches!(line.instrdir(), Some(InstrDir::Instruction(_, _))))
            .map(|l| l.instrdir().unwrap())
            .collect()
    }

    pub fn constants(&self) -> Vec<&InstrDir> {
        self.0
            .iter()
            .filter(|line| matches!(line.instrdir(), Some(InstrDir::Directive(name, _)) if name == "const"))
            .map(|l| l.instrdir().unwrap())
            .collect()
    }

    pub fn constant(&self, cname: &str) -> Option<String> {
        self.constants()
            .iter()
            .filter_map(|line| match line {
                InstrDir::Directive(dname, args) if dname == "const" && args.len() > 1 => {
                    let name = &args[0];
                    if name == cname {
                        Some(args[1].clone())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .next()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Line(Option<Label>, Option<InstrDir>, Option<Comment>);

impl Line {
    pub fn label(&self) -> Option<&Label> {
        self.0.as_ref()
    }

    pub fn instrdir(&self) -> Option<&InstrDir> {
        self.1.as_ref()
    }

    pub fn comment(&self) -> Option<&Comment> {
        self.2.as_ref()
    }
}

#[derive(Debug, Default, Clone)]
pub struct LineBuilder {
    label: Option<Label>,
    instrdir: Option<InstrDir>,
    comment: Option<Comment>,
}

impl LineBuilder {
    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn instrdir(mut self, instrdir: Option<InstrDir>) -> Self {
        self.instrdir = instrdir;
        self
    }

    pub fn comment(mut self, comment: Comment) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn build(self) -> Line {
        Line(self.label, self.instrdir, self.comment)
    }
}

#[derive(Debug, Clone)]
pub enum InstrDir {
    Instruction(Opcode, Option<Operand>),
    Directive(String, Vec<String>),
}

#[derive(Debug, Clone, Default)]
pub struct DirectiveBuilder {
    name: Option<String>,
    args: Vec<String>,
}

impl DirectiveBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn arg(mut self, arg: String) -> Self {
        self.args.push(arg);
        self
    }

    pub fn build(self) -> InstrDir {
        InstrDir::Directive(self.name.unwrap_or_default(), self.args)
    }
}

#[derive(Debug, Default, From, Clone)]
pub struct Comment(String);

/// Represents a 6502 opcode
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromStr, Hash)]
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

/// Addressing modes for 6502 instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn get_addressing_mode(&self, opcode: &Opcode) -> AddressingMode {
        // Branch instructions always use relative addressing
        if matches!(
            opcode,
            Opcode::BCC
                | Opcode::BCS
                | Opcode::BEQ
                | Opcode::BMI
                | Opcode::BNE
                | Opcode::BPL
                | Opcode::BVC
                | Opcode::BVS
        ) {
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
