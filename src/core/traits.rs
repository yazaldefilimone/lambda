use crate::core::{Apply, ApplyId, Lambda, LambdaId, Variable, VariableId};

pub trait TermView {
  fn variable(&self, id: VariableId) -> &Variable;
  fn lambda(&self, id: LambdaId) -> &Lambda;
  fn apply(&self, id: ApplyId) -> &Apply;
}
