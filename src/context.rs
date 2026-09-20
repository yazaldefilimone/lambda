use crate::{
  cli::options::Options,
  core::{Term, Type},
  loader::Loader,
  messages::Messages,
  symbol::interner::Interner,
};

pub struct Context {
  pub options: Options,
  pub messages: Messages,
  pub loader: Loader,
  pub symbols: Interner,
  pub terms: Term,
  pub types: Type,
}

impl Context {
  pub fn new(options: Options) -> Self {
    Self {
      options,
      messages: Messages::new(),
      loader: Loader::new(),
      symbols: Interner::new(),
      terms: Term::new(),
      types: Type::new(),
    }
  }
}
