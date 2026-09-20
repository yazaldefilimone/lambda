use clap::ValueEnum;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Options {
  pub input: PathBuf,
  pub output: Option<PathBuf>,
  pub check: bool,
  pub emit: Option<Emit>,
  pub decode: Option<Encoding>,
  pub trace: bool,
  pub stats: bool,
  pub limit: usize,
  pub verbose: bool,
  pub quiet: bool,
  pub no_color: bool,
}

impl Options {
  pub fn new(input: PathBuf) -> Self {
    Self {
      input,
      output: None,
      check: false,
      emit: None,
      decode: None,
      trace: false,
      stats: false,
      limit: 10000,
      verbose: false,
      quiet: false,
      no_color: false,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Emit {
  Term,
  Ast,
  Types,
  Ir,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Encoding {
  Church,
  Scott,
  Boehm,
}
