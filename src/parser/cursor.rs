pub struct Cursor<'a> {
  source: &'a [u8],
  position: usize,
}

impl<'a> Cursor<'a> {
  pub fn new(source: &'a str) -> Self {
    Self { source: source.as_bytes(), position: 0 }
  }

  #[inline]
  pub fn peek(&self) -> Option<u8> {
    self.source.get(self.position).copied()
  }

  #[inline]
  pub fn peek_next(&self) -> Option<u8> {
    self.source.get(self.position + 1).copied()
  }

  #[inline]
  pub fn advance(&mut self) -> Option<u8> {
    let byte = self.peek()?;
    self.position += 1;
    Some(byte)
  }

  #[inline]
  pub fn consume(&mut self, expected: u8) -> bool {
    if self.peek() == Some(expected) {
      self.position += 1;
      true
    } else {
      false
    }
  }

  #[inline]
  pub fn position(&self) -> usize {
    self.position
  }

  #[inline]
  pub fn is_end(&self) -> bool {
    self.position >= self.source.len()
  }

  #[allow(dead_code)]
  pub fn slice(&self, start: usize, end: usize) -> &[u8] {
    &self.source[start..end]
  }
}
