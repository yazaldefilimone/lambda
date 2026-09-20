impl std::fmt::Display for super::kind::TokenKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use super::kind::TokenKind::*;

    let text = match self {
      Lambda => "λ",
      Dot => ".",
      Colon => ":",
      Arrow => "->",
      LeftParen => "(",
      RightParen => ")",
      Identifier(_) => "identifier",
      Eof => "end of file",
    };

    write!(f, "{text}")
  }
}
