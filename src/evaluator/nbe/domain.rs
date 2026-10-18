use crate::{
  core::{
    TermId, TypeId,
    arena::{Arena, Id},
  },
  symbol::SymbolId,
};

/// Binder levels at or above this encode free variables by position, so readback
/// can restore the `depth + position` index the parser assigns to them.
pub const FREE_BASE: u32 = 1 << 30;

pub type ValueId = Id<Value>;
pub type EnvId = Id<Binding>;
pub type Env = Option<EnvId>;

#[derive(Clone, Copy)]
pub struct Closure {
  pub body: TermId,
  pub parameter: Option<SymbolId>,
  pub annotation: Option<TypeId>,
  pub env: Env,
}

#[derive(Clone, Copy)]
pub enum Value {
  Closure(Closure),
  Neutral(Neutral),
  Suspended { term: TermId, env: Env },
}

#[derive(Clone, Copy)]
pub enum Neutral {
  Var(u32),
  App(ValueId, ValueId),
}

pub struct Binding {
  pub value: ValueId,
  pub next: Env,
  pub depth: u32,
}

pub struct Domain {
  values: Arena<Value>,
  envs: Arena<Binding>,
}

impl Default for Domain {
  fn default() -> Self {
    Domain { values: Arena::with_capacity(64), envs: Arena::with_capacity(64) }
  }
}

impl Domain {
  pub fn value(&self, id: ValueId) -> Value {
    *self.values.get(id)
  }

  pub fn add(&mut self, value: Value) -> ValueId {
    self.values.add(value)
  }

  pub fn depth(&self, env: Env) -> u32 {
    match env {
      Some(id) => self.envs.get(id).depth,
      None => 0,
    }
  }

  pub fn cons(&mut self, value: ValueId, env: Env) -> Env {
    let binding = Binding { value, next: env, depth: self.depth(env) + 1 };
    Some(self.envs.add(binding))
  }

  /// Resolves `index` against `env`. The caller guarantees `index < depth(env)`.
  pub fn lookup(&self, env: Env, mut index: u32) -> ValueId {
    let mut current = env;
    while index > 0 {
      current = self.binding(current).next;
      index -= 1;
    }
    self.binding(current).value
  }

  fn binding(&self, env: Env) -> &Binding {
    let id = env.expect("binding within the environment depth");
    self.envs.get(id)
  }
}
