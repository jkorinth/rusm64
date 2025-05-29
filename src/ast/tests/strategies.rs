use crate::ast::*;
use proptest::prelude::*;

// Strategy for generating valid labels
pub fn label_name_strategy() -> impl Strategy<Value = String> {
    "[a-z_][a-zA-Z0-9_]*"
        .prop_map(|s| s.to_string())
        .prop_filter("non-empty label_name", |s| !s.is_empty())
}

// Strategy for generating valid identifiers
pub fn identifier_strategy() -> impl Strategy<Value = String> {
    "[A-Z_][A-Z0-9_]*"
        .prop_map(|s| s.to_string())
        .prop_filter("non-empty identifier", |s| !s.is_empty())
}

// Strategy for generating number literals
pub fn number_literal_strategy() -> impl Strategy<Value = NumberLiteral> {
    prop_oneof![
        // Hex literals: $FF, $1234, etc.
        "[0-9A-Fa-f]{1,4}".prop_map(|s| NumberLiteral::HexLiteral(format!("${}", s))),
        // Binary literals: %11110000, etc.
        "[01]{1,8}".prop_map(|s| NumberLiteral::BinLiteral(format!("%{}", s))),
        // Decimal literals: 123, 456, etc.
        "[0-9]{1,5}".prop_map(NumberLiteral::DecLiteral),
    ]
}

// Strategy for generating character literals
pub fn char_literal_strategy() -> impl Strategy<Value = CharLiteral> {
    r"[a-zA-Z0-9!@#\$%\^&\*\(\)_\+-=]".prop_map(|s| CharLiteral::from(format!("'{}'", s)))
}

// Strategy for generating literal expressions
pub fn literal_expr_strategy() -> impl Strategy<Value = LiteralExpr> {
    prop_oneof![
        number_literal_strategy().prop_map(LiteralExpr::NumberLiteral),
        char_literal_strategy().prop_map(LiteralExpr::CharLiteral),
    ]
}

// Strategy for generating reference expressions
pub fn ref_expr_strategy() -> impl Strategy<Value = RefExpr> {
    prop_oneof![
        label_name_strategy().prop_map(RefExpr::LabelRef),
        identifier_strategy().prop_map(RefExpr::SymbolRef),
    ]
}

// Strategy for generating rhai script expressions
// Note: testing actual Rhai script is out of scope.
pub fn rhai_expr_strategy() -> impl Strategy<Value = RhaiExpr> {
    "\\PC*".prop_map(|e| RhaiExpr::from(e.to_string()))
}

// Strategy for generating upper byte expressions
pub fn upper_expr_strategy() -> impl Strategy<Value = UpperExpr> {
    expr_strategy().prop_map(|e| UpperExpr::from(Box::new(e)))
}

// Strategy for generating lower byte expressions
pub fn lower_expr_strategy() -> impl Strategy<Value = LowerExpr> {
    expr_strategy().prop_map(|e| LowerExpr::from(Box::new(e)))
}

// Strategy for generating upper byte expressions
pub fn expr_leaf_strategy() -> impl Strategy<Value = Expr> {
    prop_oneof![
        literal_expr_strategy().prop_map(Expr::Literal),
        ref_expr_strategy().prop_map(Expr::Ref),
        rhai_expr_strategy().prop_map(Expr::Rhai),
    ]
}

// Forward declaration for recursive expression generation
pub fn expr_strategy() -> impl Strategy<Value = Expr> {
    expr_leaf_strategy()
        .prop_recursive(
            2,  // Maximum depth
            32, // Maximum number of nodes
            10, // Items per collection
            |inner| {
                prop_oneof![
                    inner
                        .clone()
                        .prop_map(|e| Expr::Upper(UpperExpr::from(Box::new(e)))),
                    inner
                        .clone()
                        .prop_map(|e| Expr::Lower(LowerExpr::from(Box::new(e)))),
                ]
            },
        )
        .prop_map(|e| {
            println!("generated: {:?}", e);
            e
        })
}

// Strategy for generating addressing modes
pub fn addressing_mode_strategy() -> impl Strategy<Value = AddressingMode> {
    prop_oneof![
        Just(AddressingMode::Implied),
        Just(AddressingMode::Accumulator),
        Just(AddressingMode::Immediate),
        Just(AddressingMode::ZeroPage),
        Just(AddressingMode::ZeroPageX),
        Just(AddressingMode::ZeroPageY),
        Just(AddressingMode::Absolute),
        Just(AddressingMode::AbsoluteX),
        Just(AddressingMode::AbsoluteY),
        Just(AddressingMode::Indirect),
        Just(AddressingMode::IndexedIndirect),
        Just(AddressingMode::IndirectIndexed),
        Just(AddressingMode::Relative),
    ]
}

// Strategy for generating operands
pub fn operand_strategy() -> impl Strategy<Value = Operand> {
    (addressing_mode_strategy(), expr_strategy())
        .prop_map(|(addr_mode, expr)| Operand::from((addr_mode, expr)))
}

// Strategy for generating opcodes
pub fn opcode_strategy() -> impl Strategy<Value = Opcode> {
    prop_oneof![
        // Load/Store
        Just(Opcode::LDA),
        Just(Opcode::LDX),
        Just(Opcode::LDY),
        Just(Opcode::STA),
        Just(Opcode::STX),
        Just(Opcode::STY),
        // Transfers
        Just(Opcode::TAX),
        Just(Opcode::TAY),
        Just(Opcode::TSX),
        Just(Opcode::TXA),
        Just(Opcode::TXS),
        Just(Opcode::TYA),
        // Stack
        Just(Opcode::PHA),
        Just(Opcode::PHP),
        Just(Opcode::PLA),
        Just(Opcode::PLP),
        // Logical
        Just(Opcode::AND),
        Just(Opcode::EOR),
        Just(Opcode::ORA),
        Just(Opcode::BIT),
        // Arithmetic
        Just(Opcode::ADC),
        Just(Opcode::SBC),
        Just(Opcode::CMP),
        Just(Opcode::CPX),
        Just(Opcode::CPY),
        // Inc/Dec
        Just(Opcode::INC),
        Just(Opcode::INX),
        Just(Opcode::INY),
        Just(Opcode::DEC),
        Just(Opcode::DEX),
        Just(Opcode::DEY),
        // Shifts
        Just(Opcode::ASL),
        Just(Opcode::LSR),
        Just(Opcode::ROL),
        Just(Opcode::ROR),
        // Jumps
        Just(Opcode::JMP),
        Just(Opcode::JSR),
        Just(Opcode::RTS),
        Just(Opcode::RTI),
        // Branches
        Just(Opcode::BCC),
        Just(Opcode::BCS),
        Just(Opcode::BEQ),
        Just(Opcode::BMI),
        Just(Opcode::BNE),
        Just(Opcode::BPL),
        Just(Opcode::BVC),
        Just(Opcode::BVS),
        // Status
        Just(Opcode::CLC),
        Just(Opcode::CLD),
        Just(Opcode::CLI),
        Just(Opcode::CLV),
        Just(Opcode::SEC),
        Just(Opcode::SED),
        Just(Opcode::SEI),
        // Other
        Just(Opcode::NOP),
    ]
}

// Strategy for generating operations
pub fn op_strategy() -> impl Strategy<Value = Op> {
    (opcode_strategy(), proptest::option::of(operand_strategy())).prop_map(|(opcode, operand)| {
        let mut builder = OpBuilder::default().opcode(opcode);
        if let Some(op) = operand {
            builder = builder.operand(op);
        }
        builder.build()
    })
}

// Strategy for generating directives
pub fn directive_strategy() -> impl Strategy<Value = Directive> {
    prop_oneof![
        expr_strategy().prop_map(Directive::Org),
        (identifier_strategy(), expr_strategy())
            .prop_map(|(name, expr)| Directive::Const(name, expr)),
        (
            identifier_strategy(),
            proptest::option::of(r"[a-zA-Z0-9_]+")
        )
            .prop_map(|(name, value)| Directive::Unknown(name, value)),
    ]
}

// Strategy for generating instructions
pub fn instruction_strategy() -> impl Strategy<Value = Instruction> {
    prop_oneof![
        directive_strategy().prop_map(Instruction::Directive),
        op_strategy().prop_map(Instruction::Op),
    ]
}

// Strategy for generating labels
pub fn label_strategy() -> impl Strategy<Value = Label> {
    label_name_strategy().prop_map(Label::from)
}

// Strategy for generating comments
pub fn comment_strategy() -> impl Strategy<Value = Comment> {
    "\\PC*"
        //r"[a-zA-Z0-9 _.,!?\-]*"
        .prop_map(|s| Comment::from(format!("; {}", s)))
}

// Strategy for generating lines
pub fn line_strategy() -> impl Strategy<Value = Line> {
    (
        proptest::option::of(label_strategy()),
        proptest::option::of(instruction_strategy()),
        proptest::option::of(comment_strategy()),
    )
        .prop_map(|(label, instruction, comment)| {
            let mut builder = LineBuilder::default();
            if let Some(l) = label {
                builder = builder.label(l);
            }
            if let Some(i) = instruction {
                builder = builder.instruction(i);
            }
            if let Some(c) = comment {
                builder = builder.comment(c);
            }
            builder.build()
        })
}

// Strategy for generating ASTs
pub fn ast_strategy() -> impl Strategy<Value = Ast> {
    prop::collection::vec(line_strategy(), 1..20).prop_map(|lines| {
        lines
            .into_iter()
            .fold(Ast::default(), |ast, line| ast.add_line(line))
    })
}
