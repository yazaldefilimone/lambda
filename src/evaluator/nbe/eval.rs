use crate::{
  core::{TermId, TermKind},
  result,
};

use super::{
  Machine,
  domain::{Closure, Env, FREE_BASE, Neutral, Value, ValueId},
};

impl<'a> Machine<'a> {
  pub(super) fn eval(&mut self, term: TermId, env: Env) -> result::Result<ValueId> {
    match term.kind() {
      TermKind::Variable(id) => {
        let index = self.ctx.terms.variables.get(id).index;
        let depth = self.store.depth(env);
        if index < depth {
          Ok(self.store.lookup(env, index))
        } else {
          let level = FREE_BASE + index - depth;
          Ok(self.store.add(Value::Neutral(Neutral::Var(level))))
        }
      },
      TermKind::Lambda(id) => Ok(self.store.add(Value::Closure(Closure {
        body: self.ctx.terms.lambdas.get(id).body,
        parameter: self.ctx.terms.parameter(id),
        annotation: self.ctx.terms.annotation(id),
        env,
      }))),
      TermKind::Apply(id) => {
        let apply = *self.ctx.terms.applies.get(id);
        let function = self.eval(apply.function, env)?;
        let suspended = Value::Suspended { term: apply.argument, env };
        let argument = self.store.add(suspended);
        self.apply(function, argument)
      },
    }
  }

  pub(super) fn force(&mut self, value: ValueId) -> result::Result<ValueId> {
    let mut current = value;
    loop {
      match self.store.value(current) {
        Value::Suspended { term, env } => current = self.eval(term, env)?,
        _ => return Ok(current),
      }
    }
  }

  fn apply(&mut self, function: ValueId, argument: ValueId) -> result::Result<ValueId> {
    let forced = self.force(function)?;
    match self.store.value(forced) {
      Value::Closure(closure) => {
        self.fuel.tick()?;
        let extended = self.store.cons(argument, closure.env);
        self.eval(closure.body, extended)
      },
      _ => {
        let neutral = Value::Neutral(Neutral::App(forced, argument));
        Ok(self.store.add(neutral))
      },
    }
  }
}
