mod bool;
mod helpers;
mod list;
mod number;
mod pair;

use crate::{
  cli::options::DecodePreference,
  context::Context,
  core::TermId,
  decode::value::{ValueId, Values},
  result,
};

pub fn decode(ctx: &Context, values: &mut Values, term: TermId) -> result::Result<Option<ValueId>> {
  if ctx.options.prefer == Some(DecodePreference::Number) {
    if let Some(value) = number::decode(ctx, values, term)? {
      return Ok(Some(value));
    }
    if let Some(value) = bool::decode(ctx, values, term)? {
      return Ok(Some(value));
    }
  } else {
    if let Some(value) = bool::decode(ctx, values, term)? {
      return Ok(Some(value));
    }
    if let Some(value) = number::decode(ctx, values, term)? {
      return Ok(Some(value));
    }
  }

  if let Some(value) = list::decode(ctx, values, term)? {
    return Ok(Some(value));
  }
  if let Some(value) = pair::decode(ctx, values, term)? {
    return Ok(Some(value));
  }

  Ok(None)
}
