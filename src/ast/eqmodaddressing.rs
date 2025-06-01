/// In 6502 assembly some addressing modes depend on the effective operand
/// value, e.g.:
///     ADC $1      ; zero-page addressing
///     ADC $1001   ; absolute adressing
/// After a full lowering and expansion of the AST, the regular Eq should
/// work. To facilitate comparisons before that (e.g., directly after parsing),
/// a relaxed form of equality is implemented in this trait.
pub trait EqModAddressing {
    fn eq_mod_addressing(&self, other: &Self) -> bool;
}
