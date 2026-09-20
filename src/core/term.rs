use crate::{
  core::{
    TermView, TypeId,
    arena::{Arena, Id},
  },
  symbol::SymbolId,
};

pub struct Term {
  pub variables: Arena<Variable>,
  pub lambdas: Arena<Lambda>,
  pub applies: Arena<Apply>,
}

impl TermView for Term {
  fn variable(&self, id: VariableId) -> &Variable {
    self.variables.get(id)
  }

  fn lambda(&self, id: LambdaId) -> &Lambda {
    self.lambdas.get(id)
  }

  fn apply(&self, id: ApplyId) -> &Apply {
    self.applies.get(id)
  }
}

impl Term {
  pub fn new() -> Self {
    Self { variables: Arena::new(), lambdas: Arena::new(), applies: Arena::new() }
  }

  pub fn add_variable(&mut self, variable: Variable) -> TermId {
    let index = self.variables.add(variable);
    TermId::Variable(index)
  }

  pub fn add_lambda(&mut self, lambda: Lambda) -> TermId {
    let index = self.lambdas.add(lambda);
    TermId::Lambda(index)
  }

  pub fn add_apply(&mut self, apply: Apply) -> TermId {
    let index = self.applies.add(apply);
    TermId::Apply(index)
  }
}

pub type VariableId = Id<Variable>;
pub type LambdaId = Id<Lambda>;
pub type ApplyId = Id<Apply>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermId {
  Variable(VariableId),
  Lambda(LambdaId),
  Apply(ApplyId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Variable {
  pub name: SymbolId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lambda {
  pub parameter: SymbolId,
  pub annotation: Option<TypeId>,
  pub body: TermId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Apply {
  pub function: TermId,
  pub argument: TermId,
}
