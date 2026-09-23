use crate::{
  cli::options::Options,
  core::{LambdaId, Term, TermId, Type},
  loader::Loader,
  messages::Messages,
  symbol::{SymbolId, interner::Interner},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Redex {
  pub lambda: LambdaId,
  pub argument: TermId,
  pub variable: Option<SymbolId>,
}

pub struct Context {
  pub options: Options,
  pub messages: Messages,
  pub loader: Loader,
  pub symbols: Interner,
  pub terms: Term,
  pub types: Type,
  pub redex: Option<Redex>,
  pub current_scope: Vec<LambdaId>,
  pub redex_scope: Vec<LambdaId>,
}

impl Context {
  #[inline]
  pub fn new(options: Options) -> Self {
    Self {
      options,
      messages: Messages::new(),
      loader: Loader::new(),
      symbols: Interner::new(),
      terms: Term::new(),
      types: Type::new(),
      redex: None,
      current_scope: Vec::with_capacity(32),
      redex_scope: Vec::with_capacity(32),
    }
  }
}
