use super::helpers::{get_lambda, is_variable};
use crate::{
  context::Context,
  core::TermId,
  decode::value::{ValueId, Values},
  result,
};

pub fn decode(ctx: &Context, values: &mut Values, term: TermId) -> result::Result<Option<ValueId>> {
  let (true_parameter, body) = match get_lambda(ctx, term) {
    Some(value) => value,
    None => {
      let result = None;
      return Ok(result);
    },
  };

  let (false_parameter, body) = match get_lambda(ctx, body) {
    Some(value) => value,
    None => {
      let result = None;
      return Ok(result);
    },
  };

  if is_variable(ctx, body, true_parameter) {
    let value = values.add_bool(true);
    let result = Some(value);
    return Ok(result);
  }

  if is_variable(ctx, body, false_parameter) {
    let value = values.add_bool(false);
    let result = Some(value);
    return Ok(result);
  }

  let result = None;
  Ok(result)
}
