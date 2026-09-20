use crate::{
  parser::{
    cursor::Cursor,
    tokens::{Token, kind::TokenKind},
  },
  source::{file::FileId, span::Span},
  symbol::interner::Interner,
};

pub struct Lexer<'a> {
  source: &'a str,
  cursor: Cursor<'a>,
  file: FileId,
  symbols: &'a mut Interner,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str, file: FileId, symbols: &'a mut Interner) -> Self {
    Self { source, cursor: Cursor::new(source), file, symbols }
  }

  pub fn tokenize(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();

    loop {
      self.skip_whitespace();

      if self.cursor.is_end() {
        break;
      }

      tokens.push(self.next_token());
    }

    let end = self.cursor.position();
    tokens.push(Token { kind: TokenKind::Eof, span: self.span(end, end) });
    tokens
  }

  fn next_token(&mut self) -> Token {
    let start = self.cursor.position();

    let kind = match self.cursor.peek() {
      Some(byte) if Self::is_ident_start(byte) => self.identifier(),
      Some(0xCE) if self.cursor.peek_next() == Some(0xBB) => self.lambda_utf8(),
      Some(byte) => self.symbol(byte),
      None => TokenKind::Eof,
    };

    Token { kind, span: self.span(start, self.cursor.position()) }
  }

  fn lambda_utf8(&mut self) -> TokenKind {
    self.cursor.advance();
    self.cursor.advance();
    TokenKind::Lambda
  }

  fn identifier(&mut self) -> TokenKind {
    let start = self.cursor.position();

    while self.cursor.peek().is_some_and(Self::is_ident_continue) {
      self.cursor.advance();
    }

    let text = &self.source[start..self.cursor.position()];
    TokenKind::Identifier(self.symbols.intern(text))
  }

  fn symbol(&mut self, byte: u8) -> TokenKind {
    self.cursor.advance();

    match byte {
      b'.' => TokenKind::Dot,
      b':' => TokenKind::Colon,
      b'(' => TokenKind::LeftParen,
      b')' => TokenKind::RightParen,
      b'-' => {
        if self.cursor.consume(b'>') {
          TokenKind::Arrow
        } else {
          TokenKind::Eof
        }
      },
      b'\\' => TokenKind::Lambda,
      _ => TokenKind::Eof,
    }
  }

  fn skip_whitespace(&mut self) {
    while self
      .cursor
      .peek()
      .is_some_and(|b| matches!(b, b' ' | b'\t' | b'\r' | b'\n'))
    {
      self.cursor.advance();
    }
  }

  fn span(&self, start: usize, end: usize) -> Span {
    Span { file: self.file, start: start as u32, end: end as u32 }
  }

  fn is_ident_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
  }

  fn is_ident_continue(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
  }
}
