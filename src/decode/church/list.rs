use crate::{
  context::Context,
  core::TermId,
  decode::{
    church::helpers::{get_apply, get_lambda, is_variable},
    value::{ListValue, ValueId, Values},
  },
  result,
};

pub fn decode(ctx: &Context, values: &mut Values, term: TermId) -> result::Result<Option<ValueId>> {
  let body = match get_lambda(ctx, term) {
    Some(value) => value,
    None => {
      let result = None;
      return Ok(result);
    },
  };

  let body = match get_lambda(ctx, body) {
    Some(value) => value,
    None => {
      let result = None;
      return Ok(result);
    },
  };

  let mut items = Vec::new();
  let decoded = decode_items(ctx, values, body, &mut items)?;
  if !decoded {
    let result = None;
    return Ok(result);
  }

  let list = ListValue { items };
  let id = values.lists.add(list);
  let value_id = ValueId::List(id);
  let result = Some(value_id);
  Ok(result)
}

fn decode_items(
  ctx: &Context,
  values: &mut Values,
  term: TermId,
  items: &mut Vec<ValueId>,
) -> result::Result<bool> {
  if is_variable(ctx, term, 0) {
    return Ok(true);
  }

  let (left, tail) = match get_apply(ctx, term) {
    Some(value) => value,
    None => return Ok(false),
  };

  let (function, head) = match get_apply(ctx, left) {
    Some(value) => value,
    None => return Ok(false),
  };

  if !is_variable(ctx, function, 1) {
    return Ok(false);
  }

  let value = match super::decode(ctx, values, head)? {
    Some(value) => value,
    None => return Ok(false),
  };

  items.push(value);
  decode_items(ctx, values, tail, items)
}
