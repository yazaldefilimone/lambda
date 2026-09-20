use crate::symbol::SymbolId;
use rustc_hash::FxHashMap;

pub struct Interner {
  symbols: Vec<String>,
  map: FxHashMap<String, SymbolId>,
}

impl Interner {
  pub fn new() -> Self {
    Self { symbols: Vec::new(), map: FxHashMap::default() }
  }

  pub fn intern(&mut self, name: &str) -> SymbolId {
    if let Some(id) = self.map.get(name) {
      return *id;
    }

    let id = SymbolId::new(self.symbols.len() as u32);
    self.symbols.push(name.to_string());
    self.map.insert(name.to_string(), id);
    id
  }

  pub fn resolve(&self, id: SymbolId) -> &str {
    &self.symbols[id.index() as usize]
  }
}
