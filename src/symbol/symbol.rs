use crate::core::arena::Id;

pub type SymbolId = Id<Symbol>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Symbol;
