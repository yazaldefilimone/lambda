use std::path::PathBuf;

use crate::source::{
  file::{FileId, SourceFile},
  span::Span,
};

pub struct Location {
  pub line: u32,
  pub column: u32,
}

pub struct SourceMap {
  files: Vec<SourceFile>,
}

impl SourceMap {
  pub fn new() -> Self {
    Self { files: Vec::new() }
  }

  pub fn add_file(&mut self, path: PathBuf, source: String) -> FileId {
    let index = self.files.len() as u32;
    let id = FileId(index);
    let file = SourceFile::new(id, path, source);
    self.files.push(file);
    id
  }

  pub fn get(&self, id: FileId) -> Option<&SourceFile> {
    self.files.get(id.usize())
  }

  pub fn slice(&self, span: Span) -> &str {
    let file = self.get(span.file).unwrap();
    &file.source[span.start as usize..span.end as usize]
  }

  pub fn location(&self, span: Span) -> Location {
    let file = self.get(span.file).unwrap();
    let offset = span.start;

    let line = match file.lines.binary_search(&offset) {
      Ok(line) => line,
      Err(line) => line.saturating_sub(1),
    };

    let column = offset - file.lines[line];
    Location { line: line as u32 + 1, column }
  }
}
