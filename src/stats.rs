use std::time::Duration;

use crate::{context::Context, core::TermId, evaluator::EvalStats};

#[derive(Debug, Clone, Copy)]
pub struct TermStats {
  pub nodes: usize,
  pub depth: usize,
}

impl TermStats {
  pub fn compute(term: TermId, ctx: &Context) -> Self {
    match term {
      TermId::Variable(_) => {
        let nodes = 1;
        let depth = 1;
        Self { nodes, depth }
      },
      TermId::Lambda(id) => {
        let lambda = ctx.terms.lambdas.get(id);
        let body_stats = Self::compute(lambda.body, ctx);
        let nodes = 1 + body_stats.nodes;
        let depth = 1 + body_stats.depth;
        Self { nodes, depth }
      },
      TermId::Apply(id) => {
        let apply = ctx.terms.applies.get(id);
        let function_stats = Self::compute(apply.function, ctx);
        let argument_stats = Self::compute(apply.argument, ctx);
        let nodes = 1 + function_stats.nodes + argument_stats.nodes;
        let maximum_sub_depth = function_stats.depth.max(argument_stats.depth);
        let depth = 1 + maximum_sub_depth;
        Self { nodes, depth }
      },
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Stats {
  pub term: TermStats,
  pub eval: EvalStats,
}

impl Stats {
  pub fn new(term: TermStats, eval: EvalStats) -> Self {
    Self { term, eval }
  }

  pub fn print(&self) {
    let formatted_time = format_duration(self.eval.duration);
    println!("term");
    println!("  nodes: {}", self.term.nodes);
    println!("  depth: {}", self.term.depth);
    println!();
    println!("eval");
    println!("  steps: {}", self.eval.steps);
    println!("  reductions: {}", self.eval.reductions);
    println!("  time: {formatted_time}");
  }
}

fn format_duration(duration: Duration) -> String {
  let nanoseconds = duration.as_nanos();
  if nanoseconds < 1_000 {
    let formatted = format!("{nanoseconds}ns");
    return formatted;
  }
  if nanoseconds < 1_000_000 {
    let microseconds = nanoseconds as f64 / 1_000.0;
    let formatted = format!("{microseconds:.1}µs");
    return formatted;
  }
  if nanoseconds < 1_000_000_000 {
    let milliseconds = nanoseconds as f64 / 1_000_000.0;
    let formatted = format!("{milliseconds:.1}ms");
    return formatted;
  }
  let seconds = duration.as_secs_f64();
  let formatted = format!("{seconds:.2}s");
  formatted
}
