use std::marker::PhantomData;

#[derive(Debug)]
pub struct Id<T> {
  index: u32,
  _marker: PhantomData<fn() -> T>,
}

impl<T> Clone for Id<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
  fn eq(&self, other: &Self) -> bool {
    self.index == other.index
  }
}

impl<T> Eq for Id<T> {}

impl<T> std::hash::Hash for Id<T> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.index.hash(state);
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Arena<T> {
  items: Vec<T>,
}

impl<T> Id<T> {
  pub fn new(index: u32) -> Self {
    Self { index, _marker: PhantomData }
  }

  pub fn index(self) -> u32 {
    self.index
  }
}

impl<T> Arena<T> {
  pub fn new() -> Self {
    Self::with_capacity(1024)
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self { items: Vec::with_capacity(capacity) }
  }

  pub fn add(&mut self, value: T) -> Id<T> {
    let id = Id::new(self.items.len() as u32);
    self.items.push(value);
    id
  }

  pub fn get(&self, id: Id<T>) -> &T {
    &self.items[id.index() as usize]
  }

  #[allow(dead_code)]
  pub fn get_mut(&mut self, id: Id<T>) -> &mut T {
    &mut self.items[id.index() as usize]
  }

  #[allow(dead_code)]
  pub fn len(&self) -> usize {
    self.items.len()
  }

  #[allow(dead_code)]
  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }
}

impl<T> Default for Arena<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> std::ops::Index<Id<T>> for Arena<T> {
  type Output = T;

  fn index(&self, id: Id<T>) -> &Self::Output {
    &self.items[id.index() as usize]
  }
}

impl<T> std::ops::IndexMut<Id<T>> for Arena<T> {
  fn index_mut(&mut self, id: Id<T>) -> &mut Self::Output {
    &mut self.items[id.index() as usize]
  }
}

impl<T> std::fmt::Display for Id<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Id {{ index: {} }}", self.index)
  }
}
