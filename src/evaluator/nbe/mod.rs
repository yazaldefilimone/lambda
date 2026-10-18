mod domain;
mod eval;
mod readback;

use crate::{
  context::Context,
  core::TermId,
  error,
  result::{self, Failed},
};

use domain::{Domain, ValueId};

/// Normalizes `term` by evaluation: evaluate into a semantic value and read it
/// back once, instead of rebuilding the term at every beta step. Returns the
/// normal form and the number of beta steps.
pub fn normalize(ctx: &mut Context, term: TermId, limit: usize) -> result::Result<(TermId, usize)> {
  let mut machine = Machine::new(ctx, limit);

  let value = match machine.eval(term, None) {
    Ok(value) => value,
    Err(_) => return Err(machine.report_limit()),
  };

  match machine.quote(value, 0) {
    Ok(term) => Ok((term, machine.fuel.used)),
    Err(_) => Err(machine.report_limit()),
  }
}

struct Machine<'a> {
  ctx: &'a mut Context,
  store: Domain,
  fuel: Fuel,
  arguments: Vec<ValueId>,
}

impl<'a> Machine<'a> {
  fn new(ctx: &'a mut Context, limit: usize) -> Self {
    Machine { ctx, store: Domain::default(), fuel: Fuel::new(limit), arguments: Vec::new() }
  }

  fn report_limit(&mut self) -> Failed {
    let steps = self.fuel.used;
    let text = format!("eval limit reached\n\nsteps: {steps}\nreductions: {steps}");
    self.ctx.messages.add(error!("{text}"));
    Failed::Abort
  }
}

struct Fuel {
  used: usize,
  limit: usize,
}

impl Fuel {
  fn new(limit: usize) -> Self {
    Fuel { used: 0, limit }
  }

  fn tick(&mut self) -> result::Result<()> {
    if self.used >= self.limit {
      return Err(Failed::Abort);
    }
    self.used += 1;
    Ok(())
  }
}
