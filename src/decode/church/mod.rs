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
  let order = match ctx.options.prefer {
    Some(DecodePreference::Number) => [
      DecodePreference::Number,
      DecodePreference::Boolean,
      DecodePreference::List,
      DecodePreference::Pair,
    ],
    Some(DecodePreference::Boolean) => [
      DecodePreference::Boolean,
      DecodePreference::Number,
      DecodePreference::List,
      DecodePreference::Pair,
    ],
    Some(DecodePreference::List) => [
      DecodePreference::List,
      DecodePreference::Boolean,
      DecodePreference::Number,
      DecodePreference::Pair,
    ],
    Some(DecodePreference::Pair) => [
      DecodePreference::Pair,
      DecodePreference::Boolean,
      DecodePreference::Number,
      DecodePreference::List,
    ],
    None => [
      DecodePreference::Boolean,
      DecodePreference::Number,
      DecodePreference::List,
      DecodePreference::Pair,
    ],
  };

  for kind in order {
    let decoded = match kind {
      DecodePreference::Boolean => bool::decode(ctx, values, term)?,
      DecodePreference::Number => number::decode(ctx, values, term)?,
      DecodePreference::List => list::decode(ctx, values, term)?,
      DecodePreference::Pair => pair::decode(ctx, values, term)?,
    };

    if let Some(value) = decoded {
      return Ok(Some(value));
    }
  }

  Ok(None)
}
