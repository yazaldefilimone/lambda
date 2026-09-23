use clap::ValueEnum;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ColorChoice {
  Auto,
  Always,
  Never,
}

#[derive(Debug, Clone)]
pub struct Options {
  pub input: PathBuf,
  #[allow(dead_code)]
  pub output: Option<PathBuf>,
  pub check: bool,
  pub emit: Option<Emit>,
  pub decode: Option<Encoding>,
  pub prefer: Option<DecodePreference>,
  pub trace: bool,
  pub stats: bool,
  pub limit: usize,
  pub verbose: bool,
  pub quiet: bool,
  pub color: ColorChoice,
}

impl Options {
  #[allow(dead_code)]
  pub fn new(input: PathBuf) -> Self {
    Self {
      input,
      output: None,
      check: false,
      emit: None,
      decode: None,
      prefer: None,
      trace: false,
      stats: false,
      limit: 10000,
      verbose: false,
      quiet: false,
      color: ColorChoice::Auto,
    }
  }
}

impl Default for Options {
  fn default() -> Self {
    Self::new(PathBuf::from("<virtual>"))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum DecodePreference {
  #[value(alias = "int", alias = "nat")]
  Number,
  #[value(alias = "bool")]
  Boolean,
  List,
  Pair,
}
