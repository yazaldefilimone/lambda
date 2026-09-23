pub mod beta;
pub mod strategy;
pub mod substitute;

pub use strategy::StrategyOptions;

use std::time::{Duration, Instant};

use crate::{
  context::Context,
  core::{Apply, Lambda, LambdaId, TermId},
  error,
  result::{self, Failed},
  trace::{Reduction, ReductionRule, TextTracer},
};
use strategy::Strategy;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EvalStats {
  pub steps: usize,
  pub reductions: usize,
  pub created_terms: usize,
  pub max_spine_depth: usize,
  pub duration: Duration,
}

pub struct Evaluator<'a> {
  ctx: &'a mut Context,
  strategy: strategy::Strategy,
  trace: bool,
  spine: Vec<TermId>,
  tracer: Option<TextTracer>,
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
    let spine = Vec::with_capacity(128);
    let tracer = if trace { Some(TextTracer::new(ctx)) } else { None };
    Self { ctx, strategy, trace, spine, tracer, stats }
  }

  pub fn eval(&mut self, mut term: TermId) -> result::Result<TermId> {
    let initial_terms = self.ctx.terms.total_terms();
    let start = Instant::now();

    if !self.trace && self.strategy.options.kind == strategy::StrategyKind::Normal {
      let result = self.normalize(term);
      self.stats.duration = start.elapsed();
      self.stats.created_terms = self.ctx.terms.total_terms().saturating_sub(initial_terms);
      return result;
    }

    let mut steps = 0;
    loop {
      if steps >= self.strategy.options.limit {
        self.stats.created_terms = self.ctx.terms.total_terms().saturating_sub(initial_terms);
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
        self.stats.created_terms = self.ctx.terms.total_terms().saturating_sub(initial_terms);
        if let Some(tracer) = &mut self.tracer {
          tracer.on_finish(self.ctx, term);
        }
        return Ok(term);
      };

      if let Some(tracer) = &mut self.tracer {
        let redex = self.ctx.redex.take();
        let reduction = Reduction {
          step: steps + 1,
          rule: ReductionRule::Beta,
          before: term,
          after: next,
          redex,
        };
        tracer.on_step(self.ctx, &reduction);
      }

      term = next;
      steps += 1;
      self.stats.steps += 1;
      self.stats.reductions += 1;
    }
  }

  fn normalize(&mut self, mut term: TermId) -> result::Result<TermId> {
    let spine_base = self.spine.len();

    loop {
      term = self.descend_spine(term);

      let Some((lambda, argument)) = self.next_redex(term, spine_base) else {
        break;
      };

      self.check_limit()?;
      term = self.reduce_redex(lambda, argument)?;
    }

    if let Some(lambda_id) = term.as_lambda() {
      return self.normalize_lambda(lambda_id, term);
    }

    self.reconstruct_spine(term, spine_base)
  }

  #[inline(always)]
  fn descend_spine(&mut self, mut term: TermId) -> TermId {
    while let Some(apply_id) = term.as_apply() {
      let apply = *self.ctx.terms.applies.get(apply_id);
      self.spine.push(apply.argument);
      term = apply.function;
    }
    if self.spine.len() > self.stats.max_spine_depth {
      self.stats.max_spine_depth = self.spine.len();
    }
    term
  }

  #[inline(always)]
  fn next_redex(&mut self, term: TermId, spine_base: usize) -> Option<(LambdaId, TermId)> {
    if self.spine.len() == spine_base {
      return None;
    }
    let lambda_id = term.as_lambda()?;
    let argument = self.spine.pop()?;
    Some((lambda_id, argument))
  }

  #[inline(always)]
  fn check_limit(&mut self) -> result::Result<()> {
    if self.stats.reductions < self.strategy.options.limit {
      return Ok(());
    }
    let reductions = self.stats.reductions;
    let text = format!("eval limit reached\n\nsteps: {reductions}\nreductions: {reductions}");
    let message = error!("{text}");
    self.ctx.messages.add(message);
    Err(Failed::Abort)
  }

  #[inline(always)]
  fn reduce_redex(&mut self, lambda: LambdaId, argument: TermId) -> result::Result<TermId> {
    let term = beta::reduce(self.ctx, lambda, argument)?;
    self.stats.steps += 1;
    self.stats.reductions += 1;
    Ok(term)
  }

  fn normalize_lambda(&mut self, lambda_id: LambdaId, term: TermId) -> result::Result<TermId> {
    if self.strategy.options.lazy {
      return Ok(term);
    }
    let lambda = *self.ctx.terms.lambdas.get(lambda_id);
    let normalized_body = self.normalize(lambda.body)?;
    if normalized_body == lambda.body {
      return Ok(term);
    }
    let new_lambda = Lambda { body: normalized_body };
    let parameter = self.ctx.terms.parameter(lambda_id);
    let annotation = self.ctx.terms.annotation(lambda_id);
    let new_id = self
      .ctx
      .terms
      .add_annotated(new_lambda, parameter, annotation);
    Ok(new_id)
  }

  fn reconstruct_spine(&mut self, mut term: TermId, spine_base: usize) -> result::Result<TermId> {
    while self.spine.len() > spine_base {
      let Some(argument) = self.spine.pop() else {
        break;
      };
      let normalized_argument = self.normalize(argument)?;
      let new_apply = Apply { function: term, argument: normalized_argument };
      term = self.ctx.terms.add_apply(new_apply);
    }
    Ok(term)
  }
}
