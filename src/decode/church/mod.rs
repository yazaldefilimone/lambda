mod bool;
mod helpers;
mod list;
mod number;
mod pair;

use crate::{
  context::Context,
  core::TermId,
  decode::value::{ValueId, Values},
  result,
};

pub fn decode(ctx: &Context, values: &mut Values, term: TermId) -> result::Result<Option<ValueId>> {
  if let Some(value) = bool::decode(ctx, values, term)? {
    return Ok(Some(value));
  }

  if let Some(value) = number::decode(ctx, values, term)? {
    return Ok(Some(value));
  }

  if let Some(value) = list::decode(ctx, values, term)? {
    return Ok(Some(value));
  }

  if let Some(value) = pair::decode(ctx, values, term)? {
    return Ok(Some(value));
  }

  Ok(None)
}
