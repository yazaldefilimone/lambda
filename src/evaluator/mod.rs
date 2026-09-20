pub mod alpha;
pub mod beta;
pub mod strategy;
pub mod substitute;

pub use strategy::StrategyOptions;

use std::time::{Duration, Instant};

use crate::{
  context::Context,
  core::TermId,
  error,
  printer::Printer,
  result::{self, Failed},
};
use strategy::Strategy;

#[derive(Debug, Clone, Copy, Default)]
pub struct EvalStats {
  pub steps: usize,
  pub reductions: usize,
  pub created_terms: usize,
  pub duration: Duration,
}

pub struct Evaluator<'a> {
  ctx: &'a mut Context,
  strategy: strategy::Strategy,
  trace: bool,
  pub stats: EvalStats,
}

impl<'a> Evaluator<'a> {
  pub fn new(
    ctx: &'a mut Context,
    strategy_options: strategy::StrategyOptions,
    trace: bool,
  ) -> Self {
    let strategy = Strategy::new(strategy_options);
    let stats = EvalStats::default();
    Self { ctx, strategy, trace, stats }
  }

  pub fn eval(&mut self, mut term: TermId) -> result::Result<TermId> {
    let initial_terms = self.ctx.terms.variables.len()
      + self.ctx.terms.lambdas.len()
      + self.ctx.terms.applies.len();
    let start = Instant::now();
    let mut steps = 0;
    loop {
      if steps >= self.strategy.options.limit {
        let final_terms = self.ctx.terms.variables.len()
          + self.ctx.terms.lambdas.len()
          + self.ctx.terms.applies.len();
        self.stats.created_terms = final_terms.saturating_sub(initial_terms);
        let reductions = self.stats.reductions;
        let text = format!("eval limit reached\n\nsteps: {steps}\nreductions: {reductions}");
        let message = error!("{text}");
        self.ctx.messages.add(message);
        let err = Failed::Abort;
        return Err(err);
      }

      let next_opt = self.strategy.reduce(term, self.ctx)?;
      let Some(next) = next_opt else {
        self.stats.duration = start.elapsed();
        let final_terms = self.ctx.terms.variables.len()
          + self.ctx.terms.lambdas.len()
          + self.ctx.terms.applies.len();
        self.stats.created_terms = final_terms.saturating_sub(initial_terms);
        return Ok(term);
      };

      if self.trace {
        self.debug_step(term, next);
      }

      term = next;
      steps += 1;
      self.stats.steps += 1;
      self.stats.reductions += 1;
    }
  }

  fn debug_step(&self, term: TermId, next: TermId) {
    let printer = Printer::new(self.ctx);
    let term_str = printer.print_term(term);
    let next_str = printer.print_term(next);
    eprintln!("{term_str} -> {next_str}");
  }
}
