use crate::core::TermId;

/// Structural hash-consing for the term arenas: identical nodes share one
/// `TermId`, so id equality *is* structural equality and reduction graphs stay a
/// compact DAG. Nodes are keyed by the least data that determines them.
#[derive(Clone, Default)]
pub struct TermCache {
  /// Indexed by de Bruijn index; empty slots hold `TermId::EMPTY`.
  variable: Vec<TermId>,
  lambda: rustc_hash::FxHashMap<u64, TermId>,
  apply: rustc_hash::FxHashMap<u64, TermId>,
}

impl TermCache {
  pub fn with_capacity(variables: usize) -> Self {
    Self { variable: Vec::with_capacity(variables), ..Default::default() }
  }

  #[inline(always)]
  pub fn variable(&self, index: u32) -> Option<TermId> {
    match self.variable.get(index as usize) {
      Some(&id) if id != TermId::EMPTY => Some(id),
      _ => None,
    }
  }

  #[inline]
  pub fn set_variable(&mut self, index: u32, id: TermId) {
    let slot = index as usize;
    if slot >= self.variable.len() {
      self.variable.resize(slot + 1, TermId::EMPTY);
    }
    self.variable[slot] = id;
  }

  #[inline(always)]
  pub fn lambda(&self, body: TermId) -> Option<TermId> {
    self.lambda.get(&(body.raw() as u64)).copied()
  }

  #[inline]
  pub fn set_lambda(&mut self, body: TermId, id: TermId) {
    self.lambda.insert(body.raw() as u64, id);
  }

  #[inline(always)]
  pub fn apply(&self, function: TermId, argument: TermId) -> Option<TermId> {
    self.apply.get(&pack(function, argument)).copied()
  }

  #[inline]
  pub fn set_apply(&mut self, function: TermId, argument: TermId, id: TermId) {
    self.apply.insert(pack(function, argument), id);
  }
}

/// Both halves are 32-bit ids, packed into one word so a lookup hashes once.
#[inline(always)]
fn pack(function: TermId, argument: TermId) -> u64 {
  ((function.raw() as u64) << 32) | argument.raw() as u64
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::arena::Id;

  fn variable(index: u32) -> TermId {
    TermId::variable(Id::new(index))
  }

  #[test]
  fn variable_slots_are_sparse_and_miss_on_empty() {
    let mut cache = TermCache::default();
    assert_eq!(cache.variable(0), None);
    assert_eq!(cache.variable(9), None);

    cache.set_variable(9, variable(0));
    assert_eq!(cache.variable(0), None);
    assert_eq!(cache.variable(9), Some(variable(0)));
  }

  #[test]
  fn apply_key_is_injective_in_its_pair() {
    let mut cache = TermCache::default();
    cache.set_apply(variable(1), variable(2), variable(3));
    assert_eq!(cache.apply(variable(1), variable(2)), Some(variable(3)));
    assert_eq!(cache.apply(variable(2), variable(1)), None);
    assert_eq!(cache.apply(variable(1), variable(0)), None);
  }

  #[test]
  fn lambda_key_is_the_body() {
    let mut cache = TermCache::default();
    cache.set_lambda(variable(7), variable(8));
    assert_eq!(cache.lambda(variable(7)), Some(variable(8)));
    assert_eq!(cache.lambda(variable(6)), None);
  }
}
