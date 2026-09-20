use crate::source::span::Span;
mod display;
pub mod kind;

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: kind::TokenKind,
  pub span: Span,
}

impl Token {
  pub fn is(&self, kind: kind::TokenKind) -> bool {
    self.kind == kind
  }

  pub fn is_identifier(&self) -> bool {
    matches!(self.kind, kind::TokenKind::Identifier(_))
  }

  pub fn length(&self) -> u32 {
    self.span.len()
  }

  pub fn end_of_file(&self) -> bool {
    matches!(self.kind, kind::TokenKind::Eof)
  }
}
