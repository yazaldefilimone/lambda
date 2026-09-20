use crate::{context::Context, core::TermId};

pub fn get_lambda(ctx: &Context, term: TermId) -> Option<TermId> {
  let id = term.as_lambda()?;
  let lambda = ctx.terms.lambdas.get(id);
  Some(lambda.body)
}

pub fn get_apply(ctx: &Context, term: TermId) -> Option<(TermId, TermId)> {
  let id = term.as_apply()?;
  let apply = ctx.terms.applies.get(id);
  let pair = (apply.function, apply.argument);
  Some(pair)
}

pub fn is_variable(ctx: &Context, term: TermId, index: u32) -> bool {
  let Some(id) = term.as_variable() else {
    return false;
  };
  let variable = ctx.terms.variables.get(id);
  variable.index == index
}

pub fn count_application(ctx: &Context, term: TermId) -> Option<u64> {
  if let Some(id) = term.as_apply() {
    let apply = ctx.terms.applies.get(id);
    if is_variable(ctx, apply.function, 1) {
      let inner = count_application(ctx, apply.argument)?;
      let count = inner + 1;
      return Some(count);
    }
    return None;
  }

  if is_variable(ctx, term, 0) {
    let count = 0;
    return Some(count);
  }

  None
}
