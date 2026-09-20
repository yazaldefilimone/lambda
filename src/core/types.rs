use crate::{
  core::arena::{Arena, Id},
  symbol::SymbolId,
};

pub type TypeVariableId = Id<TypeVariable>;
pub type FunctionTypeId = Id<FunctionType>;
pub type ApplyTypeId = Id<ApplyType>;
pub type NamedTypeId = Id<NamedType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeId {
  Variable(TypeVariableId),
  Function(FunctionTypeId),
  Apply(ApplyTypeId),
  Named(NamedTypeId),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Type {
  pub variables: Arena<TypeVariable>,
  pub functions: Arena<FunctionType>,
  pub applies: Arena<ApplyType>,
  pub named: Arena<NamedType>,
}

impl Type {
  pub fn new() -> Self {
    Self {
      variables: Arena::new(),
      functions: Arena::new(),
      applies: Arena::new(),
      named: Arena::new(),
    }
  }

  pub fn add_variable(&mut self, value: TypeVariable) -> TypeId {
    let index = self.variables.add(value);
    TypeId::Variable(index)
  }

  pub fn add_function(&mut self, value: FunctionType) -> TypeId {
    let index = self.functions.add(value);
    TypeId::Function(index)
  }

  pub fn add_apply(&mut self, value: ApplyType) -> TypeId {
    let index = self.applies.add(value);
    TypeId::Apply(index)
  }

  pub fn add_named(&mut self, value: NamedType) -> TypeId {
    let index = self.named.add(value);
    TypeId::Named(index)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVariable {
  pub name: SymbolId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionType {
  pub parameter: TypeId,
  pub result: TypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApplyType {
  pub constructor: TypeId,
  pub argument: TypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamedType {
  pub name: SymbolId,
}
