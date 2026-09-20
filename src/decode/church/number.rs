use crate::{
  context::Context,
  core::TermId,
  decode::{
    ValueId, Values,
    church::helpers::{count_application, get_lambda},
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

  let count = match count_application(ctx, body) {
    Some(value) => value,
    None => {
      let result = None;
      return Ok(result);
    },
  };

  let int_value = values.add_int(count);
  let result = Some(int_value);
  Ok(result)
}
