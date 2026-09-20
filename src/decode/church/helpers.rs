use crate::{context::Context, core::TermId, symbol::SymbolId};

pub fn get_lambda(ctx: &Context, term: TermId) -> Option<(SymbolId, TermId)> {
  let id = term.as_lambda()?;
  let lambda = ctx.terms.lambdas.get(id);
  let pair = (lambda.parameter, lambda.body);
  Some(pair)
}

pub fn get_apply(ctx: &Context, term: TermId) -> Option<(TermId, TermId)> {
  let id = term.as_apply()?;
  let apply = ctx.terms.applies.get(id);
  let pair = (apply.function, apply.argument);
  Some(pair)
}

pub fn is_variable(ctx: &Context, term: TermId, symbol: SymbolId) -> bool {
  let Some(id) = term.as_variable() else {
    return false;
  };
  let variable = ctx.terms.variables.get(id);
  variable.name == symbol
}

pub fn count_application(
  ctx: &Context,
  term: TermId,
  function: SymbolId,
  base: SymbolId,
) -> Option<u64> {
  if let Some(id) = term.as_apply() {
    let apply = ctx.terms.applies.get(id);
    if is_variable(ctx, apply.function, function) {
      let inner = count_application(ctx, apply.argument, function, base)?;
      let count = inner + 1;
      return Some(count);
    }
    return None;
  }

  if let Some(id) = term.as_variable() {
    let variable = ctx.terms.variables.get(id);
    if variable.name == base {
      let count = 0;
      return Some(count);
    }
    return None;
  }

  None
}
