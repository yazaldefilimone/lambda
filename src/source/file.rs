use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(pub u32);

impl FileId {
  pub fn usize(&self) -> usize {
    self.0 as usize
  }
}

pub struct SourceFile {
  pub id: FileId,
  pub path: PathBuf,
  pub source: Arc<str>,
  pub lines: Vec<u32>,
}

impl SourceFile {
  pub fn new(id: FileId, path: PathBuf, source: String) -> Self {
    let lines = Self::compute_lines(&source);
    Self { id, path, source: Arc::from(source), lines }
  }

  pub fn compute_lines(source: &str) -> Vec<u32> {
    let mut starts = vec![0];
    for (index, byte) in source.bytes().enumerate() {
      if byte == b'\n' {
        starts.push((index + 1) as u32);
      }
    }
    starts
  }

  pub fn line_start(&self, line: usize) -> Option<u32> {
    self.lines.get(line).copied()
  }

  pub fn line_count(&self) -> usize {
    self.lines.len()
  }
}
