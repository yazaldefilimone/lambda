use std::{path::PathBuf, sync::Arc};

use crate::{
  error,
  messages::Messages,
  result,
  source::{file::FileId, map::SourceMap},
};

pub struct Loader {
  pub sources: SourceMap,
}

impl Loader {
  pub fn new() -> Self {
    Self { sources: SourceMap::new() }
  }

  pub fn load(&mut self, path: PathBuf, messages: &mut Messages) -> result::Result<FileId> {
    let content = match std::fs::read_to_string(&path) {
      Ok(content) => content,
      Err(_) => {
        messages.add(error!("cannot load '{}'", path.display()));
        return Err(result::Failed::Load);
      },
    };
    let file_id = self.sources.add_file(path, content);
    Ok(file_id)
  }

  pub fn add_virtual(&mut self, name: &str, content: String) -> FileId {
    self.sources.add_file(PathBuf::from(name), content)
  }

  pub fn file_source(&self, file_id: FileId) -> result::Result<Arc<str>> {
    self
      .sources
      .get(file_id)
      .map(|f| f.source.clone())
      .ok_or(result::Failed::Abort)
  }
}
