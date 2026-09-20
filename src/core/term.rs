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
    }
  }

  pub fn add_variable(&mut self, variable: Variable) -> TermId {
    let index = self.variables.add(variable);
    TermId::variable(index)
  }

  pub fn add_lambda(&mut self, lambda: Lambda) -> TermId {
    self.add_annotated(lambda, None)
  }

  pub fn add_annotated(&mut self, lambda: Lambda, annotation: Option<TypeId>) -> TermId {
    let is_closed = if lambda.body.is_closed() {
      true
    } else {
      !self.has_free_other_than(lambda.body, lambda.parameter)
    };
    let index = self.lambdas.add(lambda);
    self.annotations.add(annotation);
    TermId::lambda(index, is_closed)
  }

  pub fn annotation(&self, id: LambdaId) -> Option<TypeId> {
    *self.annotations.get(Id::new(id.index()))
  }

  pub fn add_apply(&mut self, apply: Apply) -> TermId {
    let is_closed = apply.function.is_closed() && apply.argument.is_closed();
    let index = self.applies.add(apply);
    TermId::apply(index, is_closed)
  }

  fn has_free_other_than(&self, term: TermId, bound: SymbolId) -> bool {
    if term.is_closed() {
      return false;
    }
    match term.kind() {
      TermKind::Variable(id) => {
        let var = self.variables.get(id);
        var.name != bound
      },
      TermKind::Apply(id) => {
        let apply = self.applies.get(id);
        self.has_free_other_than(apply.function, bound)
          || self.has_free_other_than(apply.argument, bound)
      },
      TermKind::Lambda(id) => {
        let lambda = self.lambdas.get(id);
        if lambda.parameter == bound {
          self.has_any_free(lambda.body)
        } else {
          self.has_free_other_than(lambda.body, bound)
        }
      },
    }
  }

  fn has_any_free(&self, term: TermId) -> bool {
    if term.is_closed() {
      return false;
    }
    match term.kind() {
      TermKind::Variable(_) => true,
      TermKind::Apply(id) => {
        let apply = self.applies.get(id);
        self.has_any_free(apply.function) || self.has_any_free(apply.argument)
      },
      TermKind::Lambda(id) => {
        let lambda = self.lambdas.get(id);
        self.has_free_other_than(lambda.body, lambda.parameter)
      },
    }
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
  pub const CLOSED_FLAG: u32 = 1 << 29;
  pub const INDEX_MASK: u32 = !(Self::KIND_MASK | Self::CLOSED_FLAG);

  #[inline(always)]
  pub fn variable(id: VariableId) -> Self {
    Self(id.index() & Self::INDEX_MASK)
  }

  #[inline(always)]
  pub fn lambda(id: LambdaId, is_closed: bool) -> Self {
    let flag = if is_closed { Self::CLOSED_FLAG } else { 0 };
    Self((1 << 30) | flag | (id.index() & Self::INDEX_MASK))
  }

  #[inline(always)]
  pub fn apply(id: ApplyId, is_closed: bool) -> Self {
    let flag = if is_closed { Self::CLOSED_FLAG } else { 0 };
    Self((2 << 30) | flag | (id.index() & Self::INDEX_MASK))
  }

  #[inline(always)]
  pub fn is_closed(self) -> bool {
    (self.0 & Self::CLOSED_FLAG) != 0
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
  pub name: SymbolId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lambda {
  pub parameter: SymbolId,
  pub body: TermId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Apply {
  pub function: TermId,
  pub argument: TermId,
}
