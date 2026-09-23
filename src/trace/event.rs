use crate::{context::Redex, core::TermId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ReductionRule {
  Alpha,
  Beta,
  Eta,
  Delta,
}

impl ReductionRule {
  pub fn symbol(&self) -> &'static str {
    match self {
      Self::Alpha => "α",
      Self::Beta => "β",
      Self::Eta => "η",
      Self::Delta => "δ",
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Reduction {
  #[allow(dead_code)]
  pub step: usize,
  pub rule: ReductionRule,
  pub before: TermId,
  pub after: TermId,
  pub redex: Option<Redex>,
}
