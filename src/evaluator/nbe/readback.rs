use crate::{
  core::{Apply, Lambda, TermId, Variable},
  result,
};

use super::{
  Machine,
  domain::{FREE_BASE, Neutral, Value, ValueId},
};

impl<'a> Machine<'a> {
  /// `level` is the number of binders read back so far.
  pub(super) fn quote(&mut self, value: ValueId, level: u32) -> result::Result<TermId> {
    let value = self.force(value)?;
    match self.store.value(value) {
      Value::Closure(closure) => {
        let fresh = self.store.add(Value::Neutral(Neutral::Var(level)));
        let extended = self.store.cons(fresh, closure.env);
        let body = self.eval(closure.body, extended)?;
        let body = self.quote(body, level + 1)?;
        let lambda = Lambda { body };
        let parameter = closure.parameter;
        let annotation = closure.annotation;
        let new_id = self.ctx.terms.add_annotated(lambda, parameter, annotation);
        Ok(new_id)
      },
      Value::Neutral(Neutral::Var(variable)) => Ok(self.quote_variable(variable, level)),
      Value::Neutral(Neutral::App(..)) => self.quote_spine(value, level),
      Value::Suspended { .. } => unreachable!("force returns an evaluated value"),
    }
  }

  fn quote_spine(&mut self, value: ValueId, level: u32) -> result::Result<TermId> {
    let base = self.arguments.len();

    let mut current = value;
    let head = loop {
      match self.store.value(current) {
        Value::Neutral(Neutral::App(head, argument)) => {
          self.arguments.push(argument);
          current = head;
        },
        Value::Neutral(Neutral::Var(variable)) => break variable,
        Value::Suspended { .. } => current = self.force(current)?,
        Value::Closure(_) => unreachable!("a neutral spine is headed by a variable"),
      }
    };

    let mut term = self.quote_variable(head, level);
    while self.arguments.len() > base {
      let Some(argument) = self.arguments.pop() else {
        break;
      };
      let normalized = self.quote(argument, level)?;
      let apply = Apply { function: term, argument: normalized };
      term = self.ctx.terms.add_apply(apply);
    }
    Ok(term)
  }

  fn quote_variable(&mut self, neutral: u32, level: u32) -> TermId {
    let index = if neutral >= FREE_BASE {
      level + (neutral - FREE_BASE)
    } else {
      let Some(index) = level.checked_sub(neutral + 1) else {
        unreachable!("variable level is within the readback")
      };
      index
    };
    self.ctx.terms.add_variable(Variable { index })
  }
}
