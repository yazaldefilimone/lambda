use crate::symbol::SymbolId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
  Identifier(SymbolId),
  Lambda,
  Dot,
  Colon,
  Arrow,
  LeftParen,
  RightParen,
  Eof,
}
