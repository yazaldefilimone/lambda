use crate::{
  error,
  messages::Messages,
  parser::tokens::{Token, kind::TokenKind},
  result,
  symbol::SymbolId,
};

pub struct Reader<'a> {
  tokens: &'a [Token],
  position: usize,
}

impl<'a> Reader<'a> {
  pub fn new(tokens: &'a [Token]) -> Self {
    Self { tokens, position: 0 }
  }

  #[inline]
  pub fn peek(&self) -> Option<&Token> {
    self.tokens.get(self.position)
  }

  #[inline]
  pub fn peek_offset(&self, offset: usize) -> Option<&Token> {
    self.tokens.get(self.position + offset)
  }

  #[inline]
  pub fn get(&self, position: usize) -> Option<&Token> {
    self.tokens.get(position)
  }

  #[inline]
  pub fn current(&self) -> TokenKind {
    self
      .peek()
      .map(|token| token.kind)
      .unwrap_or(TokenKind::Eof)
  }

  #[inline]
  pub fn advance(&mut self) -> Option<Token> {
    let token = self.peek()?.clone();
    self.position += 1;
    Some(token)
  }

  #[inline]
  pub fn position(&self) -> usize {
    self.position
  }

  #[inline]
  pub fn restore(&mut self, position: usize) {
    self.position = position;
  }

  #[inline]
  pub fn is_end(&self) -> bool {
    self.position >= self.tokens.len()
  }

  #[inline]
  pub fn check(&self, kind: TokenKind) -> bool {
    self.current() == kind
  }

  #[inline]
  pub fn consume(&mut self, kind: TokenKind) -> bool {
    if self.check(kind) {
      self.position += 1;
      true
    } else {
      false
    }
  }

  #[inline]
  pub fn expect(&mut self, kind: TokenKind, messages: &mut Messages) -> result::Result<Token> {
    let token = match self.peek() {
      Some(token) => token,
      None => {
        messages.add(error!("unexpected end of file"));
        return Err(result::Failed::Abort);
      },
    };

    if token.kind != kind {
      let message = error!("expected '{}', found '{}'", kind, token.kind);
      messages.add(message.with_span(token.span));
      return Err(result::Failed::Recover);
    }
    Ok(self.advance().unwrap())
  }

  #[inline]
  pub fn expect_identifier(&mut self, messages: &mut Messages) -> result::Result<SymbolId> {
    let token = match self.advance() {
      Some(token) => token,
      None => {
        messages.add(error!("expected identifier"));
        return Err(result::Failed::Abort);
      },
    };

    match token.kind {
      TokenKind::Identifier(symbol) => Ok(symbol),
      kind => {
        let message = error!("expected identifier, found '{}'", kind);
        messages.add(message.with_span(token.span));
        Err(result::Failed::Recover)
      },
    }
  }

  #[inline]
  pub fn consume_if(&mut self, predicate: impl FnOnce(TokenKind) -> bool) -> Option<Token> {
    let token = self.peek()?;
    if predicate(token.kind) { self.advance() } else { None }
  }
}
