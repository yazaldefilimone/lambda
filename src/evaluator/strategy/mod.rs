mod applicative;
mod call_by_value;
mod kind;
mod normal;

pub use kind::*;

use crate::{context::Context, core::TermId, result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrategyOptions {
  pub limit: usize,
  pub kind: StrategyKind,
  pub lazy: bool,
}

impl Default for StrategyOptions {
  fn default() -> Self {
    Self { limit: 10000, kind: StrategyKind::Normal, lazy: false }
  }
}

pub struct Strategy {
  pub options: StrategyOptions,
}

impl Strategy {
  pub fn new(options: StrategyOptions) -> Self {
    Self { options }
  }

  pub fn reduce(&self, term: TermId, ctx: &mut Context) -> result::Result<Option<TermId>> {
    match self.options.kind {
      StrategyKind::Normal => normal::reduce(term, ctx, &self.options),
      StrategyKind::Applicative => applicative::reduce(term, ctx, &self.options),
      StrategyKind::CallByValue => call_by_value::reduce(term, ctx, &self.options),
    }
  }
}
