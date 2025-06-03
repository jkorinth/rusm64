## 0.3.0 (2025-06-03)

### Feat

- **rusm64**: Make assemble command write the .prg file on success
- **Assembler**: Implement KISS-style code generation
- **Assembler**: Replace raw Vec<u8> with Bin
- **Rhai**: Allow multi-line rhai expressions
- Add VALID_ADDRESSING_MODES table for simpler opcode lookup
- **Labels**: Allow leading .'s in label names
- **Assembler**: Rewrite passes using new VisitorMut context
- **VisitorMut**: Add Context to VisitorMut
- **AST**: Add EqModAddressing trait for relaxed identity
- **Expr**: Add numeric_value method on exprs
- **AST**: Rewrite grammar to fix several issues

### Fix

- Strip trailing empty lines in ASTs
- **Pass**: Fix Pass visitor order, visit Ops before increasing PC
- **Assembler**: Fix addressing mode pass by using VALID_ADDRESSING_MODE table
- **Test**: Make round-trip tests use relaxed equality
- **CLI**: Re-enable assembler
- **Test**: Use EqModAdressing for AST comparisons in round-trip tests
- **Test**: Drop pre- and suffixes from literal strategies
- **Test**: Avoid generating invalid opcode + addressing mode combinations
- **Operand**: Fix missing addressing modes in Display impl
- **Parser**: Make parser ignore pre- and suffixes on literals

### Refactor

- Move and improve Errors
- Remove parse_debug bin
- Rename executable to rusm64

## 0.2.0 (2025-05-29)

### Feat

- **AST**: Rewrite grammar to fix several issues

### Refactor

- Move and improve Errors
- Remove parse_debug bin
- Rename executable to rusm64

## 0.1.0 (2025-05-29)
