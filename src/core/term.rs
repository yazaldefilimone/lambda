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
    TermId::variable(index)
  }

  pub fn add_lambda(&mut self, lambda: Lambda) -> TermId {
    let index = self.lambdas.add(lambda);
    TermId::lambda(index)
  }

  pub fn add_apply(&mut self, apply: Apply) -> TermId {
    let index = self.applies.add(apply);
    TermId::apply(index)
  }
}

pub type VariableId = Id<Variable>;
pub type LambdaId = Id<Lambda>;
pub type ApplyId = Id<Apply>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TermId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermKind {
  Variable(VariableId),
  Lambda(LambdaId),
  Apply(ApplyId),
}

impl TermId {
  pub const KIND_MASK: u32 = 0b11 << 30;
  pub const INDEX_MASK: u32 = !Self::KIND_MASK;

  #[inline(always)]
  pub fn variable(id: VariableId) -> Self {
    Self(id.index() & Self::INDEX_MASK)
  }

  #[inline(always)]
  pub fn lambda(id: LambdaId) -> Self {
    Self((1 << 30) | (id.index() & Self::INDEX_MASK))
  }

  #[inline(always)]
  pub fn apply(id: ApplyId) -> Self {
    Self((2 << 30) | (id.index() & Self::INDEX_MASK))
  }

  #[inline(always)]
  pub fn kind(self) -> TermKind {
    let index = self.0 & Self::INDEX_MASK;
    match self.0 >> 30 {
      0 => TermKind::Variable(Id::new(index)),
      1 => TermKind::Lambda(Id::new(index)),
      _ => TermKind::Apply(Id::new(index)),
    }
  }

  #[inline(always)]
  pub fn is_variable(self) -> bool {
    (self.0 >> 30) == 0
  }

  #[inline(always)]
  pub fn is_lambda(self) -> bool {
    (self.0 >> 30) == 1
  }

  #[allow(dead_code)]
  #[inline(always)]
  pub fn is_apply(self) -> bool {
    (self.0 >> 30) == 2
  }

  #[inline(always)]
  pub fn as_variable(self) -> Option<VariableId> {
    if (self.0 >> 30) == 0 { Some(Id::new(self.0 & Self::INDEX_MASK)) } else { None }
  }

  #[inline(always)]
  pub fn as_lambda(self) -> Option<LambdaId> {
    if (self.0 >> 30) == 1 { Some(Id::new(self.0 & Self::INDEX_MASK)) } else { None }
  }

  #[inline(always)]
  pub fn as_apply(self) -> Option<ApplyId> {
    if (self.0 >> 30) == 2 { Some(Id::new(self.0 & Self::INDEX_MASK)) } else { None }
  }
}

impl std::fmt::Debug for TermId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.kind())
  }
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
