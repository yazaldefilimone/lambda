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
  pub annotations: Arena<Option<TypeId>>,
  pub free_variables: Vec<SymbolId>,
  variable_cache: Vec<Option<TermId>>,
  lambda_cache: rustc_hash::FxHashMap<TermId, TermId>,
  apply_cache: rustc_hash::FxHashMap<(TermId, TermId), TermId>,
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
    Self {
      variables: Arena::new(),
      lambdas: Arena::new(),
      applies: Arena::new(),
      annotations: Arena::new(),
      free_variables: Vec::new(),
      variable_cache: Vec::with_capacity(32),
      lambda_cache: rustc_hash::FxHashMap::default(),
      apply_cache: rustc_hash::FxHashMap::default(),
    }
  }

  #[inline(always)]
  pub fn total_terms(&self) -> usize {
    self.variables.len() + self.lambdas.len() + self.applies.len()
  }

  pub fn add_variable(&mut self, variable: Variable) -> TermId {
    let idx = variable.index as usize;
    if idx < self.variable_cache.len() {
      if let Some(id) = self.variable_cache[idx] {
        return id;
      }
    } else {
      self.variable_cache.resize(idx + 1, None);
    }
    let index = self.variables.add(variable);
    let id = TermId::variable(index);
    self.variable_cache[idx] = Some(id);
    id
  }

  pub fn add_lambda(&mut self, lambda: Lambda) -> TermId {
    self.add_annotated(lambda, None)
  }

  pub fn add_annotated(&mut self, lambda: Lambda, annotation: Option<TypeId>) -> TermId {
    if annotation.is_none()
      && let Some(&id) = self.lambda_cache.get(&lambda.body)
    {
      return id;
    }
    let index = self.lambdas.add(lambda);
    self.annotations.add(annotation);
    let id = TermId::lambda(index);
    if annotation.is_none() {
      self.lambda_cache.insert(lambda.body, id);
    }
    id
  }

  pub fn annotation(&self, id: LambdaId) -> Option<TypeId> {
    *self.annotations.get(Id::new(id.index()))
  }

  pub fn add_apply(&mut self, apply: Apply) -> TermId {
    let key = (apply.function, apply.argument);
    if let Some(&id) = self.apply_cache.get(&key) {
      return id;
    }
    let index = self.applies.add(apply);
    let id = TermId::apply(index);
    self.apply_cache.insert(key, id);
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
