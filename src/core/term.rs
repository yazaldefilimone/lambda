use crate::{
  core::{
    TypeId, View,
    arena::{Arena, Id},
    cache::TermCache,
  },
  symbol::SymbolId,
};

#[derive(Clone)]
pub struct Term {
  pub variables: Arena<Variable>,
  pub lambdas: Arena<Lambda>,
  pub applies: Arena<Apply>,
  pub parameters: Arena<Option<SymbolId>>,
  pub annotations: Arena<Option<TypeId>>,
  pub free_variables: Vec<SymbolId>,
  cache: TermCache,
}

impl View for Term {
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
    Self {
      variables: Arena::new(),
      lambdas: Arena::new(),
      applies: Arena::new(),
      parameters: Arena::new(),
      annotations: Arena::new(),
      free_variables: Vec::new(),
      cache: TermCache::with_capacity(32),
    }
  }

  #[inline(always)]
  pub fn total_terms(&self) -> usize {
    self.variables.len() + self.lambdas.len() + self.applies.len()
  }

  #[inline(always)]
  pub fn variable_count(&self) -> usize {
    self.variables.len()
  }

  #[inline(always)]
  pub fn lambda_count(&self) -> usize {
    self.lambdas.len()
  }

  #[inline(always)]
  pub fn apply_count(&self) -> usize {
    self.applies.len()
  }

  pub fn add_variable(&mut self, variable: Variable) -> TermId {
    if let Some(id) = self.cache.variable(variable.index) {
      return id;
    }
    let index = self.variables.add(variable);
    let id = TermId::variable(index);
    self.cache.set_variable(variable.index, id);
    id
  }

  #[allow(dead_code)]
  pub fn add_lambda(&mut self, lambda: Lambda) -> TermId {
    self.add_annotated(lambda, None, None)
  }

  pub fn add_annotated(
    &mut self,
    lambda: Lambda,
    parameter: Option<SymbolId>,
    annotation: Option<TypeId>,
  ) -> TermId {
    let interned = annotation.is_none() && parameter.is_none();
    if interned && let Some(id) = self.cache.lambda(lambda.body) {
      return id;
    }

    let index = self.lambdas.add(lambda);
    self.parameters.add(parameter);
    self.annotations.add(annotation);
    let id = TermId::lambda(index);

    if interned {
      self.cache.set_lambda(lambda.body, id);
    }

    id
  }

  pub fn parameter(&self, id: LambdaId) -> Option<SymbolId> {
    *self.parameters.get(Id::new(id.index()))
  }

  pub fn annotation(&self, id: LambdaId) -> Option<TypeId> {
    *self.annotations.get(Id::new(id.index()))
  }

  pub fn add_apply(&mut self, apply: Apply) -> TermId {
    if let Some(id) = self.cache.apply(apply.function, apply.argument) {
      return id;
    }
    let index = self.applies.add(apply);
    let id = TermId::apply(index);
    self.cache.set_apply(apply.function, apply.argument, id);
    id
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

  /// Reserved id that marks an empty cache slot. Tag `3` is produced by no
  /// constructor, so it can never collide with a real node.
  pub(crate) const EMPTY: Self = Self(3 << 30);

  #[inline(always)]
  pub(crate) fn raw(self) -> u32 {
    self.0
  }

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
  pub fn index(self) -> u32 {
    self.0 & Self::INDEX_MASK
  }

  #[inline(always)]
  pub fn kind(self) -> TermKind {
    let index = self.index();
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
    if (self.0 >> 30) == 0 { Some(Id::new(self.index())) } else { None }
  }

  #[inline(always)]
  pub fn as_lambda(self) -> Option<LambdaId> {
    if (self.0 >> 30) == 1 { Some(Id::new(self.index())) } else { None }
  }

  #[inline(always)]
  pub fn as_apply(self) -> Option<ApplyId> {
    if (self.0 >> 30) == 2 { Some(Id::new(self.index())) } else { None }
  }
}

impl std::fmt::Debug for TermId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.kind())
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Variable {
  pub index: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lambda {
  pub body: TermId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Apply {
  pub function: TermId,
  pub argument: TermId,
}
