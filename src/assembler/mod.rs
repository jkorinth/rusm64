// Assembler for C64 assembly language

mod opcodes;

use std::collections::HashMap;
use crate::ast::{Ast, Instruction, Opcode, AddressingMode};
use self::opcodes::build_opcode_table;

#[derive(Debug, thiserror::Error)]
pub enum AssemblerError {
    #[error("Unknown opcode: {0}")]
    UnknownOpcode(String),
    
    #[error("Invalid addressing mode for opcode: {0}")]
    InvalidAddressingMode(String),
    
    #[error("Unknown label: {0}")]
    UnknownLabel(String),
    
    #[error("Unknown directive: {0}")]
    UnknownDirective(String),
    
    #[error("Value out of range: {0}")]
    ValueOutOfRange(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Symbol resolution error: {0}")]
    SymbolResolution(String),
    
    #[error("Forward reference error: {0}")]
    ForwardReference(String),
    
    #[error("Duplicate label error: {0}")]
    DuplicateLabel(String),
    
    #[error("Invalid expression: {0}")]
    InvalidExpression(String),
    
    #[error("Error at line {line}: {message}")]
    SourceLineError { line: usize, message: String },
}

/// Assembler for converting AST to binary
pub struct Assembler {
    /// The current program counter
    pc: usize,
    
    /// The resulting binary code
    binary: Vec<u8>,
    
    /// Map of resolved labels to their addresses
    labels: HashMap<String, usize>,
    
    /// Map of unresolved references to labels
    unresolved_refs: Vec<(usize, String, bool)>, // Position, Label name, Is relative?
    
    /// Map of unresolved expression references
    unresolved_expressions: Vec<(usize, String)>,
    
    /// The origin address for the assembly
    origin: usize,
    
    /// The current line number for error reporting
    line_number: usize,
    
    /// Whether to enable verbose output
    verbose: bool,
    
    /// The AST being assembled (for accessing constants)
    ast: Option<Ast>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            pc: 0,
            binary: Vec::new(),
            labels: HashMap::new(),
            unresolved_refs: Vec::new(),
            unresolved_expressions: Vec::new(),
            origin: 0x1000, // Default origin
            line_number: 0,
            verbose: false,
            ast: None,
        }
    }
    
    /// Set verbose mode
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    
    /// Set the current line number for error reporting
    pub fn set_line_number(&mut self, line: usize) {
        self.line_number = line;
    }
    
    /// Create an error with the current line number
    fn line_error(&self, message: String) -> AssemblerError {
        AssemblerError::SourceLineError {
            line: self.line_number,
            message,
        }
    }
    
    /// Assemble the AST into binary
    pub fn assemble(&mut self, ast: &Ast) -> Result<Vec<u8>, AssemblerError> {
        // Save the AST for constant lookup
        self.ast = Some(ast.clone());
        
        // First pass: resolve labels
        self.resolve_labels(ast)?;
        
        // Second pass: generate code
        self.generate_code(ast)?;
        
        // Third pass: resolve references
        self.resolve_references()?;
        
        // If there are still unresolved references, try multiple passes
        let mut pass_count = 1;
        while !self.unresolved_refs.is_empty() && pass_count < 5 {
            if self.verbose {
                println!("Pass {}: {} unresolved references remain", pass_count, self.unresolved_refs.len());
            }
            self.resolve_references()?;
            pass_count += 1;
        }
        
        Ok(self.binary.clone())
    }
    
    /// First pass: Resolve labels
    fn resolve_labels(&mut self, ast: &Ast) -> Result<(), AssemblerError> {
        self.pc = self.origin;
        
        // Process directives first (for .org, etc.)
        for directive in ast.directives() {
            match directive.name.as_str() {
                "org" => {
                    let value = self.parse_value(&directive.value)?;
                    self.origin = value;
                    self.pc = value;
                }
                _ => {} // Other directives handled in second pass
            }
        }
        
        // Process labels
        for (name, _label) in ast.labels() {
            self.labels.insert(name.clone(), self.pc);
        }
        
        // Calculate PC for each instruction
        for instruction in ast.instructions() {
            let size = self.instruction_size(instruction)?;
            self.pc += size;
        }
        
        Ok(())
    }
    
    /// Second pass: Generate code
    fn generate_code(&mut self, ast: &Ast) -> Result<(), AssemblerError> {
        self.pc = self.origin;
        self.binary = Vec::new();
        
        // Process directives first
        for directive in ast.directives() {
            self.process_directive(directive)?;
        }
        
        // Process instructions
        let mut i = 0;
        while i < ast.instructions().len() {
            let instruction = &ast.instructions()[i];
            let opcode_bytes = self.encode_instruction(instruction)?;
            
            for &byte in &opcode_bytes {
                self.binary.push(byte);
            }
            
            self.pc += opcode_bytes.len();
            i += 1;
        }
        
        Ok(())
    }
    
    /// Third pass: Resolve references
    fn resolve_references(&mut self) -> Result<(), AssemblerError> {
        let mut remaining_refs = Vec::new();
        
        if self.verbose {
            println!("Resolving references: {:?}", self.unresolved_refs);
            println!("Binary size: {}", self.binary.len());
        }
        
        for (pos, label, is_relative) in &self.unresolved_refs {
            if let Some(&addr) = self.labels.get(label) {
                // Skip references whose position is beyond the binary size
                // This can happen if references were added during value parsing
                // but the actual code generation never reached that point
                if *pos >= self.binary.len() {
                    if self.verbose {
                        println!("Skipping reference to '{}' at ${:04X} (beyond binary size)", label, pos);
                    }
                    continue;
                }
                
                if *is_relative {
                    // Calculate relative address for branch instructions
                    let current_pos = *pos + 2; // PC will be at the next instruction
                    let rel_addr = (addr as isize - current_pos as isize) as i8;
                    
                    // Check if the relative jump is in range (-128 to +127 bytes)
                    if (addr as isize - current_pos as isize) > 127 || (addr as isize - current_pos as isize) < -128 {
                        return Err(AssemblerError::ValueOutOfRange(
                            format!("Branch to '{}' is too far (offset: {})", label, addr as isize - current_pos as isize)
                        ));
                    }
                    
                    if *pos + 1 < self.binary.len() {
                        self.binary[*pos + 1] = rel_addr as u8;
                    }
                } else {
                    // Absolute address
                    if *pos + 1 < self.binary.len() {
                        self.binary[*pos + 1] = (addr & 0xFF) as u8;
                    }
                    
                    // For 2-byte addresses
                    if *pos + 2 < self.binary.len() {
                        self.binary[*pos + 2] = ((addr >> 8) & 0xFF) as u8;
                    }
                }
                
                if self.verbose {
                    println!("Resolved reference to '{}' at ${:04X} -> ${:04X}", label, *pos, addr);
                }
            } else {
                // Still unresolved after multiple passes, keep for the next iteration
                remaining_refs.push((*pos, label.clone(), *is_relative));
            }
        }
        
        // Update unresolved references for the next pass
        self.unresolved_refs = remaining_refs;
        
        // If there are still unresolved references after multiple passes, that's an error
        if !self.unresolved_refs.is_empty() {
            let missing_labels: Vec<String> = self.unresolved_refs
                .iter()
                .map(|(_, label, _)| label.clone())
                .collect();
            
            return Err(AssemblerError::UnknownLabel(format!(
                "Unresolved labels after multiple passes: {:?}", missing_labels
            )));
        }
        
        Ok(())
    }
    
    /// Calculate the size of an instruction in bytes
    fn instruction_size(&self, instruction: &Instruction) -> Result<usize, AssemblerError> {
        if let Some(operand) = &instruction.operand {
            let addr_mode = operand.get_addressing_mode(instruction.opcode);
            
            match addr_mode {
                AddressingMode::Implied | AddressingMode::Accumulator => Ok(1),
                AddressingMode::Immediate | AddressingMode::ZeroPage | 
                AddressingMode::ZeroPageX | AddressingMode::ZeroPageY |
                AddressingMode::Relative | AddressingMode::IndexedIndirect |
                AddressingMode::IndirectIndexed => Ok(2),
                AddressingMode::Absolute | AddressingMode::AbsoluteX |
                AddressingMode::AbsoluteY | AddressingMode::Indirect => Ok(3),
            }
        } else {
            // No operand - implied addressing
            Ok(1)
        }
    }
    
    /// Encode an instruction to bytes
    fn encode_instruction(&mut self, instruction: &Instruction) -> Result<Vec<u8>, AssemblerError> {
        let addr_mode = if let Some(operand) = &instruction.operand {
            operand.get_addressing_mode(instruction.opcode)
        } else {
            AddressingMode::Implied
        };
        
        let opcode_byte = self.get_opcode_byte(instruction.opcode, addr_mode)?;
        let mut bytes = vec![opcode_byte];
        
        if let Some(operand) = &instruction.operand {
            match addr_mode {
                AddressingMode::Implied | AddressingMode::Accumulator => {
                    // No operand bytes
                }
                AddressingMode::Immediate => {
                    let operand_str = operand.to_string();
                    let value_str = operand_str.trim_start_matches('#');
                    let value = self.parse_value(value_str)?;
                    if value > 0xFF {
                        return Err(self.line_error(format!(
                            "Immediate value out of range: {} > 0xFF", value
                        )));
                    }
                    bytes.push((value & 0xFF) as u8);
                }
                AddressingMode::ZeroPage | AddressingMode::ZeroPageX | AddressingMode::ZeroPageY => {
                    let operand_str = operand.to_string();
                    let value_str = operand_str.split(',').next().unwrap_or("");
                    let value = self.parse_value(value_str)?;
                    if value > 0xFF {
                        return Err(self.line_error(format!(
                            "Zero page address out of range: {} > 0xFF", value
                        )));
                    }
                    bytes.push((value & 0xFF) as u8);
                }
                AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => {
                    let operand_str = operand.to_string();
                    let value_str = operand_str.split(',').next().unwrap_or("");
                    let value = self.parse_value(value_str)?;
                    if value > 0xFFFF {
                        return Err(self.line_error(format!(
                            "Absolute address out of range: {} > 0xFFFF", value
                        )));
                    }
                    bytes.push((value & 0xFF) as u8);
                    bytes.push(((value >> 8) & 0xFF) as u8);
                }
                AddressingMode::Indirect => {
                    let operand_str = operand.to_string();
                    let value_str = operand_str.trim_start_matches('(').trim_end_matches(')');
                    let value = self.parse_value(value_str)?;
                    if value > 0xFFFF {
                        return Err(self.line_error(format!(
                            "Indirect address out of range: {} > 0xFFFF", value
                        )));
                    }
                    bytes.push((value & 0xFF) as u8);
                    bytes.push(((value >> 8) & 0xFF) as u8);
                }
                AddressingMode::IndexedIndirect | AddressingMode::IndirectIndexed => {
                    let operand_str = operand.to_string();
                    let value_str = if operand_str.contains(',') {
                        operand_str
                            .trim_start_matches('(')
                            .split(',')
                            .next()
                            .unwrap_or("")
                    } else {
                        operand_str
                            .trim_start_matches('(')
                            .trim_end_matches(")")
                    };
                    
                    let value = self.parse_value(value_str)?;
                    if value > 0xFF {
                        return Err(self.line_error(format!(
                            "Zero page address out of range: {} > 0xFF", value
                        )));
                    }
                    bytes.push((value & 0xFF) as u8);
                }
                AddressingMode::Relative => {
                    // For branch instructions, relative addressing
                    // We'll store the label name and resolve it in the third pass
                    let label = operand.to_string();
                    // Store position, label, and flag that this is a relative jump
                    self.unresolved_refs.push((self.pc, label, true));
                    bytes.push(0); // Placeholder
                }
            }
        }
        
        Ok(bytes)
    }
    
    /// Get the opcode byte for a given opcode and addressing mode
    fn get_opcode_byte(&self, opcode: Opcode, addr_mode: AddressingMode) -> Result<u8, AssemblerError> {
        // Use the complete opcode lookup table
        static OPCODE_TABLE: once_cell::sync::Lazy<HashMap<(Opcode, AddressingMode), self::opcodes::OpcodeEntry>> = 
            once_cell::sync::Lazy::new(|| build_opcode_table());
        
        if let Some(entry) = OPCODE_TABLE.get(&(opcode, addr_mode)) {
            Ok(entry.byte)
        } else {
            Err(AssemblerError::InvalidAddressingMode(format!(
                "Invalid addressing mode {:?} for opcode {:?}", addr_mode, opcode
            )))
        }
    }
    
    /// Parse a value (number, label, constant, etc.)
    fn parse_value(&mut self, value: &str) -> Result<usize, AssemblerError> {
        // Check if it's a string literal
        if value.starts_with('"') && value.ends_with('"') {
            // For string literals, we'll just return the ASCII value of the first character
            let text = &value[1..value.len()-1];
            if !text.is_empty() {
                return Ok(text.bytes().next().unwrap() as usize);
            } else {
                return Err(AssemblerError::Parse("Empty string literal".to_string()));
            }
        }
        
        // First check if it's a numeric literal
        if value.starts_with('$') {
            // Hexadecimal
            let hex_str = &value[1..];
            return usize::from_str_radix(hex_str, 16).map_err(|_| {
                AssemblerError::Parse(format!("Invalid hexadecimal value: {}", value))
            });
        } else if value.starts_with('%') {
            // Binary
            let bin_str = &value[1..];
            return usize::from_str_radix(bin_str, 2).map_err(|_| {
                AssemblerError::Parse(format!("Invalid binary value: {}", value))
            });
        } else if value.chars().all(|c| c.is_digit(10)) {
            // Decimal
            return value.parse::<usize>().map_err(|_| {
                AssemblerError::Parse(format!("Invalid decimal value: {}", value))
            });
        }
        
        // Check if it's a label
        if let Some(&addr) = self.labels.get(value) {
            return Ok(addr);
        }
        
        // Otherwise check for constants
        let mut constant_value = None;
        if let Some(ref ast) = self.ast {
            constant_value = ast.constants().get(value).cloned();
        }
        
        if let Some(const_val) = constant_value {
            // Call parse_value on the constant value
            return self.parse_value(&const_val);
        }
            
        // If we get here, it's likely a forward reference
        self.unresolved_refs.push((self.pc, value.to_string(), false));
        Ok(0) // Placeholder
    }
    
    /// Evaluate an expression (for unresolved expressions)
    fn evaluate_expression(&self, expr: &str) -> Result<usize, AssemblerError> {
        // For now, just a placeholder - would need to implement expression parsing and evaluation
        Err(AssemblerError::InvalidExpression(format!("Invalid expression: {}", expr)))
    }
    
    /// Process a directive
    fn process_directive(&mut self, directive: &crate::ast::Directive) -> Result<(), AssemblerError> {
        match directive.name.as_str() {
            "org" => {
                let value = self.parse_value(&directive.value)?;
                self.origin = value;
                self.pc = value;
                Ok(())
            },
            "byte" | "db" => {
                // Handle byte directive (.byte 1, 2, 3, 4)
                if directive.value.starts_with('"') && directive.value.ends_with('"') {
                    // Handle string literals in byte directives
                    let text = &directive.value[1..directive.value.len()-1];
                    for c in text.bytes() {
                        self.binary.push(c);
                        self.pc += 1;
                    }
                    Ok(())
                } else {
                    // Handle numeric byte values
                    let values: Vec<&str> = directive.value.split(',').map(|v| v.trim()).collect();
                    for value_str in values {
                        let value = self.parse_value(value_str)?;
                        if value > 0xFF {
                            return Err(self.line_error(format!(
                                "Byte value out of range: {} > 0xFF", value
                            )));
                        }
                        self.binary.push((value & 0xFF) as u8);
                        self.pc += 1;
                    }
                    Ok(())
                }
            },
            "word" | "dw" => {
                // Handle word directive (.word $1000, $2000)
                let values: Vec<&str> = directive.value.split(',').map(|v| v.trim()).collect();
                for value_str in values {
                    let value = self.parse_value(value_str)?;
                    if value > 0xFFFF {
                        return Err(self.line_error(format!(
                            "Word value out of range: {} > 0xFFFF", value
                        )));
                    }
                    self.binary.push((value & 0xFF) as u8);
                    self.binary.push(((value >> 8) & 0xFF) as u8);
                    self.pc += 2;
                }
                Ok(())
            },
            "text" | "ascii" => {
                // Handle text directive (.text "Hello, world!")
                let text = if directive.value.starts_with('"') && directive.value.ends_with('"') {
                    &directive.value[1..directive.value.len()-1]
                } else {
                    &directive.value
                };
                
                for c in text.bytes() {
                    self.binary.push(c);
                    self.pc += 1;
                }
                Ok(())
            },
            other => Err(AssemblerError::UnknownDirective(other.to_string()))
        }
    }
}

/// Assemble the AST into binary
pub fn assemble(ast: &Ast) -> Result<Vec<u8>, AssemblerError> {
    let mut assembler = Assembler::new();
    assembler.assemble(ast)
}
