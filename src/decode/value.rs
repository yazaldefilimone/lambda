use crate::core::arena::{Arena, Id};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BoolValue {
  pub value: bool,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct IntValue {
  pub value: u64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ListValue {
  pub items: Vec<ValueId>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PairValue {
  pub first: ValueId,
  pub second: ValueId,
}

pub type BoolId = Id<BoolValue>;
pub type IntId = Id<IntValue>;
pub type ListId = Id<ListValue>;
pub type PairId = Id<PairValue>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueId {
  Bool(BoolId),
  Int(IntId),
  List(ListId),
  Pair(PairId),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Values {
  pub bools: Arena<BoolValue>,
  pub ints: Arena<IntValue>,
  pub lists: Arena<ListValue>,
  pub pairs: Arena<PairValue>,
}

impl Values {
  pub fn new() -> Self {
    Self {
      bools: Arena::new(),
      ints: Arena::new(),
      lists: Arena::new(),
      pairs: Arena::new(),
    }
  }

  pub fn add_bool(&mut self, value: bool) -> ValueId {
    let bool_value = BoolValue { value };
    let id = self.bools.add(bool_value);
    let value_id = ValueId::Bool(id);
    value_id
  }

  pub fn add_int(&mut self, value: u64) -> ValueId {
    let int_value = IntValue { value };
    let id = self.ints.add(int_value);
    let value_id = ValueId::Int(id);
    value_id
  }

  pub fn add_list(&mut self, items: Vec<ValueId>) -> ValueId {
    let list_value = ListValue { items };
    let id = self.lists.add(list_value);
    let value_id = ValueId::List(id);
    value_id
  }

  pub fn add_pair(&mut self, first: ValueId, second: ValueId) -> ValueId {
    let pair_value = PairValue { first, second };
    let id = self.pairs.add(pair_value);
    let value_id = ValueId::Pair(id);
    value_id
  }

  pub fn print(&self, id: ValueId) -> String {
    let mut out = String::new();
    self.format_value(id, &mut out);
    out
  }

  fn format_value(&self, id: ValueId, out: &mut String) {
    match id {
      ValueId::Bool(bool_id) => {
        let bool_entry = self.bools.get(bool_id);
        let text = if bool_entry.value { "true" } else { "false" };
        out.push_str(text);
      },
      ValueId::Int(int_id) => {
        let int_entry = self.ints.get(int_id);
        let text = int_entry.value.to_string();
        out.push_str(&text);
      },
      ValueId::Pair(pair_id) => {
        let pair_entry = self.pairs.get(pair_id);
        out.push('(');
        self.format_value(pair_entry.first, out);
        out.push_str(", ");
        self.format_value(pair_entry.second, out);
        out.push(')');
      },
      ValueId::List(list_id) => {
        let list_entry = self.lists.get(list_id);
        out.push('[');
        let mut first = true;
        for item in &list_entry.items {
          if !first {
            out.push_str(", ");
          }
          first = false;
          self.format_value(*item, out);
        }
        out.push(']');
      },
    }
  }
}
