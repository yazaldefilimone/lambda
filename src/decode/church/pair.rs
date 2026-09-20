use crate::{
  context::Context,
  core::TermId,
  decode::{
    ValueId, Values,
    church::helpers::{get_apply, get_lambda, is_variable},
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

  let (left, second_term) = match get_apply(ctx, body) {
    Some(value) => value,
    None => {
      let result = None;
      return Ok(result);
    },
  };

  let (projection_term, first_term) = match get_apply(ctx, left) {
    Some(value) => value,
    None => {
      let result = None;
      return Ok(result);
    },
  };

  if !is_variable(ctx, projection_term, 0) {
    let result = None;
    return Ok(result);
  }

  let first = match super::decode(ctx, values, first_term)? {
    Some(value) => value,
    None => {
      let result = None;
      return Ok(result);
    },
  };

  let second = match super::decode(ctx, values, second_term)? {
    Some(value) => value,
    None => {
      let result = None;
      return Ok(result);
    },
  };

  let pair = values.add_pair(first, second);
  let result = Some(pair);
  Ok(result)
}
