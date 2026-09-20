use crate::source::file::FileId;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
  pub file: FileId,
  pub start: u32,
  pub end: u32,
}

impl Span {
  pub const fn new(file: FileId, start: u32, end: u32) -> Self {
    Self { file, start, end }
  }

  pub fn len(&self) -> u32 {
    self.end - self.start
  }

  pub fn is_empty(&self) -> bool {
    self.start == self.end
  }

  pub fn contains(&self, position: u32) -> bool {
    self.start <= position && position < self.end
  }

  pub fn merge(self, other: Self) -> Self {
    assert_eq!(self.file, other.file);
    Self {
      file: self.file,
      start: self.start.min(other.start),
      end: self.end.max(other.end),
    }
  }

  pub fn point(file: FileId, position: u32) -> Self {
    Self { file, start: position, end: position }
  }
}

impl Add for Span {
  type Output = Span;

  fn add(self, other: Span) -> Span {
    self.merge(other)
  }
}
