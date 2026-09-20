use crate::{context::Context, core::TermId, symbol::SymbolId};

pub fn get_lambda(ctx: &Context, term: TermId) -> Option<(SymbolId, TermId)> {
  match term {
    TermId::Lambda(id) => {
      let lambda = ctx.terms.lambdas.get(id);
      let pair = (lambda.parameter, lambda.body);
      Some(pair)
    },
    _ => None,
  }
}

pub fn get_apply(ctx: &Context, term: TermId) -> Option<(TermId, TermId)> {
  match term {
    TermId::Apply(id) => {
      let apply = ctx.terms.applies.get(id);
      let pair = (apply.function, apply.argument);
      Some(pair)
    },
    _ => None,
  }
}

pub fn is_variable(ctx: &Context, term: TermId, symbol: SymbolId) -> bool {
  match term {
    TermId::Variable(id) => {
      let variable = ctx.terms.variables.get(id);
      variable.name == symbol
    },
    _ => false,
  }
}

pub fn count_application(
  ctx: &Context,
  term: TermId,
  function: SymbolId,
  base: SymbolId,
) -> Option<u64> {
  match term {
    TermId::Apply(id) => {
      let apply = ctx.terms.applies.get(id);
      if is_variable(ctx, apply.function, function) {
        let inner = count_application(ctx, apply.argument, function, base)?;
        let count = inner + 1;
        Some(count)
      } else {
        None
      }
    },
    TermId::Variable(id) => {
      let variable = ctx.terms.variables.get(id);
      if variable.name == base {
        let count = 0;
        Some(count)
      } else {
        None
      }
    },
    _ => None,
  }
}
